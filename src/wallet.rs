use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256, Bytes, TxHash, FixedBytes},
    providers::{Provider, ProviderBuilder},
    rpc::types::{TransactionRequest, Filter},
    signers::local::PrivateKeySigner,
    consensus::Transaction,
    network::TransactionResponse,
};
use anyhow::Result;
use bip39::{Language, Mnemonic};
use k256::ecdsa::SigningKey;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use hex;
use tracing::{debug, warn};
use crate::types::{Erc20TransferEvent, TransactionReceipt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmWallet {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
    pub mnemonic: Option<String>,
    #[serde(skip)]
    pub signer: Option<PrivateKeySigner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletResponse {
    pub address: String,
    pub private_key: String,
    pub public_key: String,
    pub mnemonic: Option<String>,
}

impl EvmWallet {

    pub fn new_random() -> Result<Self> {
        let signing_key = SigningKey::random(&mut thread_rng());
        let private_key_bytes = signing_key.to_bytes();
        let private_key = hex::encode(private_key_bytes);
        
        let signer = PrivateKeySigner::from_str(&private_key)?;
        let address = signer.address();
        
        let public_key = hex::encode(signing_key.verifying_key().to_encoded_point(false).as_bytes());
        
        Ok(EvmWallet {
            private_key,
            public_key,
            address: format!("{:#x}", address),
            mnemonic: None,
            signer: Some(signer),
        })
    }

    pub fn address_from_private_key(private_key: &str) -> Result<String> {
        let signer = PrivateKeySigner::from_str(private_key)?;
        let address = signer.address();
        Ok(format!("{:#x}", address))
    }

    pub fn create_wallet_from_private_key(private_key: &str) -> Result<Self> {
        let trimmed_key = private_key.trim_start_matches("0x");
        if trimmed_key.len() != 64 {
            return Err(anyhow::anyhow!("Invalid private key length. Expected 64 characters (32 bytes), got {}", trimmed_key.len()));
        }
        
        if !trimmed_key.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!("Invalid private key format. Must be hexadecimal"));
        }
        
        let signer = PrivateKeySigner::from_str(private_key)
            .map_err(|e| anyhow::anyhow!("Failed to create signer from private key: {}", e))?;
        let address = signer.address();
        let private_key_bytes = hex::decode(trimmed_key)
            .map_err(|e| anyhow::anyhow!("Failed to decode private key hex: {}", e))?;
        let signing_key = SigningKey::from_slice(&private_key_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to create signing key: {}", e))?;
        let public_key = hex::encode(signing_key.verifying_key().to_encoded_point(false).as_bytes());
        
        Ok(EvmWallet {
            private_key: private_key.to_string(),
            public_key,
            address: format!("{:#x}", address),
            mnemonic: None,
            signer: Some(signer),
        })
    }

    pub fn generate_mnemonic() -> Result<String> {
        Self::generate_mnemonic_with_words(24)
    }

    pub fn generate_mnemonic_with_words(word_count: usize) -> Result<String> {
        let entropy_bits = match word_count {
            12 => 128,
            15 => 160,
            18 => 192, 
            21 => 224,
            24 => 256,
            _ => return Err(anyhow::anyhow!("Only support 12, 15, 18, 21, 24.")),
        };
        
        let entropy_bytes = entropy_bits / 8;
        let mut entropy = vec![0u8; entropy_bytes];
        getrandom::getrandom(&mut entropy)?;
        
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;
        Ok(mnemonic.to_string())
    }

    pub fn from_mnemonic(mnemonic_phrase: &str) -> Result<Self> {
        let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_phrase)?;
        let seed = mnemonic.to_seed("");
        let signing_key = SigningKey::from_slice(&seed[0..32])?;
        let private_key = hex::encode(signing_key.to_bytes());
        let signer = PrivateKeySigner::from_str(&private_key)?;
        let address = signer.address();
        
        let public_key = hex::encode(signing_key.verifying_key().to_encoded_point(false).as_bytes());
        
        Ok(EvmWallet {
            private_key,
            public_key,
            address: format!("{:#x}", address),
            mnemonic: Some(mnemonic_phrase.to_string()),
            signer: Some(signer),
        })
    }

    pub async fn send_native_coin(
        &self,
        to: &str,
        amount_wei: U256,
        rpc_url: &str,
    ) -> Result<TxHash> {
        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(self.signer.clone().unwrap()))
            .connect_http(rpc_url.parse()?);

        let to_address = Address::from_str(to)?;

        let estimate_tx = TransactionRequest::default()
            .to(to_address)
            .value(amount_wei);
        let gas_limit = provider.estimate_gas(estimate_tx).await?;
        
        let (max_fee_per_gas, max_priority_fee_per_gas) = crate::utils::get_eip1559_gas_price(rpc_url).await;

        let tx = TransactionRequest::default()
            .to(to_address)
            .value(amount_wei)
            .gas_limit(gas_limit)
            .max_fee_per_gas(max_fee_per_gas.to::<u128>())
            .max_priority_fee_per_gas(max_priority_fee_per_gas.to::<u128>());

        let pending_tx = provider.send_transaction(tx).await?;
        let tx_hash = *pending_tx.tx_hash();
        Ok(tx_hash)
    }

    pub async fn send_erc20_token(
        &self,
        to: &str,
        amount_token_wei: U256,
        token_address: &str,
        rpc_url: &str,
    ) -> Result<TxHash> {
        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(self.signer.clone().unwrap()))
            .connect_http(rpc_url.parse()?);

        let token_addr = Address::from_str(token_address)?;
        let to_address = Address::from_str(to)?;

        let function_selector = "a9059cbb";
        let to_padded = format!("{:0>64}", format!("{:x}", to_address));
        let amount_padded = format!("{:0>64x}", amount_token_wei);
        
        let data = format!("{}{}{}", function_selector, to_padded, amount_padded);
        let call_data = Bytes::from(hex::decode(data)?);

        let estimate_tx = TransactionRequest::default()
            .to(token_addr)
            .input(call_data.clone().into());
        let gas_limit = provider.estimate_gas(estimate_tx).await?;
        
        let (max_fee_per_gas, max_priority_fee_per_gas) = crate::utils::get_eip1559_gas_price(rpc_url).await;

        let tx = TransactionRequest::default()
            .to(token_addr)
            .input(call_data.into())
            .gas_limit(gas_limit)
            .max_fee_per_gas(max_fee_per_gas.to::<u128>())
            .max_priority_fee_per_gas(max_priority_fee_per_gas.to::<u128>());

        let pending_tx = provider.send_transaction(tx).await?;
        let tx_hash = *pending_tx.tx_hash();
        Ok(tx_hash)
    }

    pub async fn get_native_balance(address: &str, rpc_url: &str) -> Result<U256> {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
        let addr = Address::from_str(address)?;
        let balance = provider.get_balance(addr).await?;
        Ok(balance)
    }

    pub async fn get_erc20_balance(
        address: &str,
        token_address: &str,
        rpc_url: &str,
    ) -> Result<U256> {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
        
        let token_addr = Address::from_str(token_address)?;
        let user_addr = Address::from_str(address)?;

        let function_selector = "70a08231";
        let address_padded = format!("{:0>64}", format!("{:x}", user_addr));
        let data = format!("{}{}", function_selector, address_padded);
        let call_data = Bytes::from(hex::decode(data)?);

        let call_request = TransactionRequest::default()
            .to(token_addr)
            .input(call_data.into());

        let result = provider.call(call_request).await?;
        let balance = U256::from_be_slice(&result);
        Ok(balance)
    }

    pub async fn estimate_gas(
        &self,
        to: &str,
        _amount_eth: U256,
        rpc_url: &str,
    ) -> Result<(u64, String, String)> {
        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(self.signer.clone().unwrap()))
            .connect_http(rpc_url.parse()?);
        let to_address = Address::from_str(to)?;
        let estimate_amount = U256::from(10u64.pow(16));

        
        let tx = TransactionRequest::default()
            .to(to_address)
            .value(estimate_amount);
        let gas_limit = provider.estimate_gas(tx).await?;
        
        let (max_fee_per_gas, _) = crate::utils::get_eip1559_gas_price(rpc_url).await;
        
        let from_address = self.signer.as_ref().unwrap().address();
        let balance = provider.get_balance(from_address).await?;
        let max_gas_cost = U256::from(gas_limit) * max_fee_per_gas;
        let total_needed = estimate_amount + max_gas_cost;
        
        if balance < total_needed {
            return Err(anyhow::anyhow!(
                "Insufficient funds: balance {} wei, needed {} wei (amount: {}, max gas cost: {})",
                balance, total_needed, estimate_amount, max_gas_cost
            ));
        }
        
        let estimated_gas_price = max_fee_per_gas;        
        let total_fee = U256::from(gas_limit) * estimated_gas_price;
        Ok((gas_limit as u64, estimated_gas_price.to_string(), total_fee.to_string()))
    }

    pub async fn estimate_erc20_gas(
        &self,
        to: &str,
        amount_token_wei: U256,
        token_address: &str,
        rpc_url: &str,
    ) -> Result<(u64, String, String)> {
        warn!("=== ERC20 Gas Estimation Started ===");
        warn!("RPC URL: {}", rpc_url);
        warn!("Token Address: {}", token_address);
        warn!("To Address: {}", to);
        warn!("Amount: {}", amount_token_wei);
        
        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(self.signer.clone().unwrap()))
            .connect_http(rpc_url.parse()?);

        let token_addr = Address::from_str(token_address)?;
        let to_address = Address::from_str(to)?;

        let function_selector = "a9059cbb";
        let to_padded = format!("{:0>64}", format!("{:x}", to_address));
        let amount_padded = format!("{:0>64x}", amount_token_wei);
        
        let data = format!("{}{}{}", function_selector, to_padded, amount_padded);
        let call_data = Bytes::from(hex::decode(data)?);

        warn!("Estimating gas limit...");
        let tx = TransactionRequest::default()
            .to(token_addr)
            .input(call_data.into());

        let gas_limit = match provider.estimate_gas(tx).await {
            Ok(limit) => {
                warn!("Gas limit estimated successfully: {}", limit);
                limit as u64
            },
            Err(e) => {
                warn!("Gas estimation failed: {}", e);
                warn!("Error details: {:?}", e);
                return Err(e.into());
            }
        };
        
        let (max_fee_per_gas, _) = crate::utils::get_eip1559_gas_price(rpc_url).await;
        
        let from_address = self.signer.as_ref().unwrap().address();
        let balance = provider.get_balance(from_address).await?;
        let max_gas_cost = U256::from(gas_limit) * max_fee_per_gas;
        
        if balance < max_gas_cost {
            return Err(anyhow::anyhow!(
                "Insufficient ETH for gas: balance {} wei, max gas cost {} wei (gas_limit: {}, max_fee: {})",
                balance, max_gas_cost, gas_limit, max_fee_per_gas
            ));
        }
        
        let estimated_gas_price = max_fee_per_gas;
        warn!("Estimated gas price: {} wei (using max_fee_per_gas)", estimated_gas_price);        
        let total_fee = U256::from(gas_limit) * estimated_gas_price;
        Ok((gas_limit as u64, estimated_gas_price.to_string(), total_fee.to_string()))
    }

    pub async fn get_erc20_transfer_events(
        token_address: &str,
        from_block: Option<u64>,
        to_block: Option<u64>,
        address_filter: Option<&str>,
        rpc_url: &str,
    ) -> Result<Vec<Erc20TransferEvent>> {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
        let token_addr = Address::from_str(token_address)?;
    
        let transfer_topic = FixedBytes::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")?;
        
        let mut filter = Filter::new()
            .address(token_addr)
            .event_signature(transfer_topic);
            
        if let Some(from) = from_block {
            filter = filter.from_block(from);
        }
        
        if let Some(to) = to_block {
            filter = filter.to_block(to);
        }
        
        if let Some(addr_filter) = address_filter {
            let filter_addr = Address::from_str(addr_filter)?;
            let padded_addr = FixedBytes::from_slice(&[&[0u8; 12], filter_addr.as_slice()].concat());
            filter = filter.topic1(padded_addr);
        }
        
        let logs = provider.get_logs(&filter).await?;
        let mut events = Vec::new();
        
        for log in logs {
            if log.topics().len() >= 3 {
                let from_addr = Address::from_slice(&log.topics()[1].as_slice()[12..]);
                let to_addr = Address::from_slice(&log.topics()[2].as_slice()[12..]);
                let amount = U256::from_be_slice(&log.data().data);
                
                let event = Erc20TransferEvent {
                    transaction_hash: format!("{:#x}", log.transaction_hash.unwrap_or_default()),
                    block_number: log.block_number.unwrap_or_default(),
                    from_address: format!("{:#x}", from_addr),
                    to_address: format!("{:#x}", to_addr),
                    amount: amount.to_string(),
                    log_index: log.log_index.unwrap_or_default(),
                };
                
                events.push(event);
            }
        }
        
        Ok(events)
    }

    pub async fn get_native_transactions_by_block_range(
        address: &str,
        from_block: Option<u64>,
        to_block: Option<u64>,
        rpc_url: &str,
    ) -> Result<Vec<TransactionReceipt>> {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
        let target_addr = Address::from_str(address)?;
        
        let latest_block = provider.get_block_number().await?;
        let from_block = from_block.unwrap_or(latest_block.saturating_sub(100));
        let to_block = to_block.unwrap_or(latest_block);
        
        let mut native_transactions = Vec::new();
        

        for block_num in from_block..=to_block {
            if let Ok(Some(block)) = provider.get_block_by_number(block_num.into()).await {
                let block_number = block.header.number;
                let block_timestamp = block.header.timestamp;
                
                if let Some(transactions) = block.transactions.as_transactions() {
                    let tx_vec: Vec<_> = transactions.iter().cloned().collect();
                    for tx in tx_vec {
                        if tx.value() > U256::ZERO {
                            let from_addr = tx.from();
                            let to_addr = tx.to().unwrap_or_default();
                            
                            if from_addr == target_addr || to_addr == target_addr {
                                let tx_hash = tx.tx_hash();
                                
                                if let Ok(Some(receipt)) = provider.get_transaction_receipt(tx_hash).await {
                                    let status = if receipt.status() {
                                        "success".to_string()
                                    } else {
                                        "failed".to_string()
                                    };
                                    let gas_used = receipt.gas_used as u64;
                                    let effective_gas_price = receipt.effective_gas_price.to_string();
                                    
                                    let effective_gas_price_u128 = receipt.effective_gas_price;
                                    let transaction_fee = effective_gas_price_u128 * gas_used as u128;
                                    
                                    let burnt_fees = if let Ok(Some(block)) = provider.get_block_by_number(block_number.into()).await {
                                        if let Some(base_fee) = block.header.base_fee_per_gas {
                                            let burnt = base_fee as u128 * gas_used as u128;
                                            burnt.to_string()
                                        } else {
                                            "0".to_string()
                                        }
                                    } else {
                                        "0".to_string()
                                    };
                                    
                                    let receipt = TransactionReceipt {
                                        transaction_hash: format!("{:#x}", tx_hash),
                                        block_number,
                                        from_address: format!("{:#x}", from_addr),
                                        to_address: format!("{:#x}", to_addr),
                                        amount: tx.value().to_string(),
                                        gas_used,
                                        gas_limit: tx.gas_limit() as u64,
                                        gas_price: effective_gas_price.clone(),
                                        effective_gas_price,
                                        transaction_fee: transaction_fee.to_string(),
                                        burnt_fees,
                                        transaction_index: tx.transaction_index.unwrap_or_default(),
                                        timestamp: Some(block_timestamp),
                                        status,
                                    };
                                    native_transactions.push(receipt);
                                    continue;
                                }

                            }
                        }
                    }
                }
            }
        }
        
        Ok(native_transactions)
    }
    
    pub async fn get_all_native_transactions_by_block_range(
        from_block: u64,
        to_block: Option<u64>,
        rpc_url: &str,
    ) -> Result<Vec<TransactionReceipt>> {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
        
        let latest_block = provider.get_block_number().await?;
        let to_block = to_block.unwrap_or(latest_block);
        
        debug!("Searching from block {} to block {}", from_block, to_block);
        
        let mut all_transactions = Vec::new();
        
        for block_num in from_block..=to_block {
            if let Ok(Some(block)) = provider.get_block_by_number(block_num.into()).await {
                let block_number = block.header.number;
                let block_timestamp = block.header.timestamp;
                
                debug!("Processing block {} with {} transactions", block_number, block.transactions.len());
                
                let transactions = if let Some(txs) = block.transactions.as_transactions() {
                    debug!("Using as_transactions() - found {} transactions", txs.len());
                    Some(txs.iter().cloned().collect())
                } else if let Some(tx_hashes) = block.transactions.as_hashes() {
                    debug!("Using as_hashes() - found {} transaction hashes", tx_hashes.len());
                    let mut txs = Vec::new();
                    for hash in tx_hashes {
                        if let Ok(Some(tx)) = provider.get_transaction_by_hash(*hash).await {
                            txs.push(tx);
                        }
                    }
                    Some(txs)
                } else {
                    debug!("No transactions found in block {}", block_number);
                    None
                };
                
                if let Some(tx_vec) = transactions {
                    debug!("Found {} native transactions in block {}", tx_vec.len(), block_number);
                    
                    for tx in tx_vec {
                        debug!("Transaction value: {}", tx.value());
                        let tx_hash = tx.tx_hash();
                        debug!("Processing transaction hash: {:#x}", tx_hash);
                        
                        if let Ok(Some(receipt)) = provider.get_transaction_receipt(tx_hash).await {
                            debug!("Found receipt for transaction {:#x}", tx_hash);
                            let status = if receipt.status() {
                                "success".to_string()
                            } else {
                                "failed".to_string()
                            };
                            let gas_used = receipt.gas_used as u64;
                            let effective_gas_price = receipt.effective_gas_price.to_string();
                            
                            let effective_gas_price_u128 = receipt.effective_gas_price;
                            let transaction_fee = effective_gas_price_u128 * gas_used as u128;
                            
                            let burnt_fees = if let Ok(Some(block)) = provider.get_block_by_number(block_number.into()).await {
                                if let Some(base_fee) = block.header.base_fee_per_gas {
                                    let burnt = base_fee as u128 * gas_used as u128;
                                    burnt.to_string()
                                } else {
                                    "0".to_string()
                                }
                            } else {
                                "0".to_string()
                            };
                            
                            let receipt_data = TransactionReceipt {
                                transaction_hash: format!("{:#x}", tx_hash),
                                block_number,
                                from_address: format!("{:#x}", tx.from()),
                                to_address: format!("{:#x}", tx.to().unwrap_or_default()),
                                amount: tx.value().to_string(),
                                gas_used,
                                gas_limit: tx.gas_limit() as u64,
                                gas_price: effective_gas_price.clone(),
                                effective_gas_price,
                                transaction_fee: transaction_fee.to_string(),
                                burnt_fees,
                                transaction_index: tx.transaction_index.unwrap_or_default(),
                                timestamp: Some(block_timestamp),
                                status,
                            };
                            all_transactions.push(receipt_data);
                            debug!("Added transaction {} to results", format!("{:#x}", tx_hash));
                        } else {
                            debug!("No receipt found for transaction {:#x}", tx_hash);
                        }
                    }
                } else {
                    debug!("No native transactions found in block {}", block_number);
                }
            }
        }
        
        debug!("Found {} total transactions", all_transactions.len());
        Ok(all_transactions)
    }
    
    pub async fn get_native_transaction_details(
        tx_hash: &str,
        rpc_url: &str,
    ) -> Result<Option<TransactionReceipt>> {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
        let hash = TxHash::from_str(tx_hash)?;
        
        if let Ok(Some(tx)) = provider.get_transaction_by_hash(hash).await {
            if let Ok(Some(receipt)) = provider.get_transaction_receipt(hash).await {
                let block = provider.get_block_by_number(
                    receipt.block_number.unwrap_or_default().into()
                ).await?;
                
                let status = if receipt.status() {
                    "success".to_string()
                } else {
                    "failed".to_string()
                };
                
                let effective_gas_price = receipt.effective_gas_price.to_string();
                let gas_used = receipt.gas_used as u64;
                let transaction_fee = receipt.effective_gas_price * gas_used as u128;
                
                let burnt_fees = if let Ok(Some(block)) = provider.get_block_by_number(receipt.block_number.unwrap_or_default().into()).await {
                    if let Some(base_fee) = block.header.base_fee_per_gas {
                        let burnt = base_fee as u128 * gas_used as u128;
                        burnt.to_string()
                    } else {
                        "0".to_string()
                    }
                } else {
                    "0".to_string()
                };
                
                let receipt_data = TransactionReceipt {
                    transaction_hash: format!("{:#x}", tx.tx_hash()),
                    block_number: receipt.block_number.unwrap_or_default(),
                    from_address: format!("{:#x}", tx.from()),
                    to_address: format!("{:#x}", tx.to().unwrap_or_default()),
                    amount: tx.value().to_string(),
                    gas_used,
                    gas_limit: tx.gas_limit() as u64,
                    gas_price: effective_gas_price.clone(),
                    effective_gas_price,
                    transaction_fee: transaction_fee.to_string(),
                    burnt_fees,
                    transaction_index: receipt.transaction_index.unwrap_or_default(),
                    timestamp: block.map(|b| b.header.timestamp),
                    status,
                };
                
                return Ok(Some(receipt_data));
            }
        }
        
        Ok(None)
    }
    
    pub async fn get_current_block(rpc_url: &str) -> Result<u64> {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
        let latest_block = provider.get_block_number().await?;
        Ok(latest_block)
    }
} 