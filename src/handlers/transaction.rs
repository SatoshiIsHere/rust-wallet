use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
};
use tracing::{warn, info, debug};
use alloy::primitives::U256;
use crate::wallet::*;
use crate::types::*;
use crate::utils::*;

pub async fn send_native_coin(
    Json(payload): Json<SendTransactionRequest>,
) -> Result<ResponseJson<TransactionResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    info!("Native coin transfer request: to={}, amount={}, network={:?}", 
          payload.to, payload.amount, payload.network);
    
    match EvmWallet::create_wallet_from_private_key(&payload.private_key) {
        Ok(wallet) => {
            let wei_amount = (payload.amount * 1_000_000_000_000_000_000.0) as u128;
            let amount = U256::from(wei_amount);
            debug!("Converted amount: {} ETH -> {} wei", payload.amount, wei_amount);

            let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
            debug!("Using RPC URL: {}", rpc_url);
            
            match wallet.send_native_coin(&payload.to, amount, &rpc_url).await {
                Ok(hash) => {
                    info!("Native coin transfer successful: tx_hash={:#x}", hash);
                    Ok(ResponseJson(TransactionResponse {
                        hash: format!("{:#x}", hash),
                    }))
                },
                Err(e) => {
                    warn!("Failed to send native coin: {}", e);
                    let error_msg = if e.to_string().contains("insufficient") {
                        "Insufficient funds for transaction. Please check your balance and gas requirements."
                    } else if e.to_string().contains("network") || e.to_string().contains("connection") {
                        "Network connection failed. Please check your network configuration and try again."
                    } else if e.to_string().contains("gas") {
                        "Gas estimation failed. The transaction may be too complex or the network is congested."
                    } else {
                        &e.to_string()
                    };
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseJson(ErrorResponse { error: error_msg.to_string() }),
                    ))
                }
            }
        }
        Err(e) => {
            warn!("Invalid private key: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ErrorResponse { error: format!("Invalid private key: {}", e) }),
            ))
        }
    }
}

pub async fn send_erc20_token(
    Json(payload): Json<SendErc20Request>,
) -> Result<ResponseJson<TransactionResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    match EvmWallet::create_wallet_from_private_key(&payload.private_key) {
        Ok(wallet) => {
            let wei_amount = (payload.amount * 1_000_000_000_000_000_000.0) as u128;
            let amount = U256::from(wei_amount);

            let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
            match wallet.send_erc20_token(&payload.to, amount, &payload.token_address, &rpc_url).await {
                Ok(hash) => Ok(ResponseJson(TransactionResponse {
                    hash: format!("{:#x}", hash),
                })),
                Err(e) => {
                    warn!("Failed to send ERC20 token: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseJson(ErrorResponse { error: e.to_string() }),
                    ))
                }
            }
        }
        Err(e) => {
            warn!("Invalid private key: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ErrorResponse { error: format!("Invalid private key: {}", e) }),
            ))
        }
    }
}

pub async fn estimate_gas(
    Json(payload): Json<EstimateGasRequest>,
) -> Result<ResponseJson<GasEstimateResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    match EvmWallet::create_wallet_from_private_key(&payload.private_key) {
        Ok(wallet) => {
            let wei_amount = (payload.amount * 1_000_000_000_000_000_000.0) as u128;
            let amount = U256::from(wei_amount);

            let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
            match wallet.estimate_gas(&payload.to, amount, &rpc_url).await {
                Ok((gas_limit, gas_price, total_fee)) => Ok(ResponseJson(GasEstimateResponse { 
                    gas_limit, 
                    gas_price,
                    total_fee
                })),
                Err(e) => {
                    warn!("Failed to estimate gas: {}", e);
                    let error_msg = if e.to_string().contains("network") || e.to_string().contains("connection") {
                        "Network connection failed. Please check your network configuration and try again."
                    } else if e.to_string().contains("revert") {
                        "Transaction would fail. Please check the recipient address and contract state."
                    } else {
                        &e.to_string()
                    };
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseJson(ErrorResponse { error: error_msg.to_string() }),
                    ))
                }
            }
        }
        Err(e) => {
            warn!("Invalid private key: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ErrorResponse { error: format!("Invalid private key: {}", e) }),
            ))
        }
    }
}

pub async fn get_transaction_receipt(
    Json(payload): Json<TransactionReceiptRequest>,
) -> Result<ResponseJson<TransactionReceiptResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    info!("Transaction receipt request: tx_hash={}, network={:?}", payload.tx_hash, payload.network);
    
    let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
    debug!("Using RPC URL: {}", rpc_url);
    
    match EvmWallet::get_native_transaction_details(&payload.tx_hash, &rpc_url).await {
        Ok(Some(receipt)) => {
            info!("Transaction receipt found: tx_hash={}, status={}", payload.tx_hash, receipt.status);
            Ok(ResponseJson(TransactionReceiptResponse {
                tx_hash: payload.tx_hash,
                status: "confirmed".to_string(),
                block_number: Some(receipt.block_number),
                gas_used: Some(receipt.gas_used),
                transaction_fee: Some(receipt.transaction_fee),
            }))
        },
        Ok(None) => {
            info!("Transaction not found or not confirmed yet: tx_hash={}", payload.tx_hash);
            Ok(ResponseJson(TransactionReceiptResponse {
                tx_hash: payload.tx_hash,
                status: "pending".to_string(),
                block_number: None,
                gas_used: None,
                transaction_fee: None,
            }))
        },
        Err(e) => {
            warn!("Failed to get transaction receipt: {}", e);
            let error_msg = if e.to_string().contains("network") || e.to_string().contains("connection") {
                "Network connection failed. Please check your network configuration and try again."
            } else if e.to_string().contains("invalid") && e.to_string().contains("hash") {
                "Invalid transaction hash format. Please provide a valid transaction hash."
            } else {
                &e.to_string()
            };
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: error_msg.to_string() }),
            ))
        }
    }
}