use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateFromPrivateKeyRequest {
    pub private_key: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFromMnemonicRequest {
    pub mnemonic: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateMnemonicRequest {
    pub word_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct EstimateGasRequest {
    pub to: String,
    pub amount: f64,
    pub private_key: String,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EstimateErc20GasRequest {
    pub to: String,
    pub amount: f64,
    pub token_address: String,
    pub private_key: String,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NativeTransactionHistoryRequest {
    pub address: String,
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AllTransactionHistoryRequest {
    pub from_block: u64,
    pub to_block: Option<u64>,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionDetailsRequest {
    pub tx_hash: String,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionReceiptRequest {
    pub tx_hash: String,
    pub network: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct MnemonicResponse {
    pub mnemonic: String,
}

#[derive(Debug, Serialize)]
pub struct GasEstimateResponse {
    pub gas_limit: u64,
    pub gas_price: String,
    pub total_fee: String,
}


#[derive(Debug, Deserialize)]
pub struct SendTransactionRequest {
    pub to: String,
    pub amount: f64,
    pub private_key: String,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendErc20Request {
    pub to: String,
    pub amount: f64,
    pub token_address: String,
    pub private_key: String,
    pub network: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub hash: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionReceiptResponse {
    pub tx_hash: String,
    pub status: String,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
    pub transaction_fee: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BalanceRequest {
    pub address: String,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Erc20BalanceRequest {
    pub address: String,
    pub token_address: String,
    pub network: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub balance: String,
}

#[derive(Debug, Deserialize)]
pub struct Erc20EventsRequest {
    pub token_address: String,
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
    pub address_filter: Option<String>,
    pub network: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Erc20TransferEvent {
    pub transaction_hash: String,
    pub block_number: u64,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub log_index: u64,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub block_number: u64,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub gas_price: String,
    pub effective_gas_price: String,
    pub transaction_fee: String,
    pub burnt_fees: String,
    pub transaction_index: u64,
    pub timestamp: Option<u64>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionHistoryResponse {
    pub transactions: Vec<TransactionReceipt>,
}

#[derive(Debug, Serialize)]
pub struct TransactionDetailsResponse {
    pub transaction: TransactionReceipt,
}

#[derive(Debug, Serialize)]
pub struct CurrentBlockResponse {
    pub current_block: u64,
}



 