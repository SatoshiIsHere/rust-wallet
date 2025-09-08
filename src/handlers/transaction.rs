use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
};
use tracing::warn;
use alloy::primitives::U256;
use crate::wallet::*;
use crate::types::*;
use crate::utils::*;

pub async fn send_native_coin(
    Json(payload): Json<SendTransactionRequest>,
) -> Result<ResponseJson<TransactionResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    match EvmWallet::create_wallet_from_private_key(&payload.private_key) {
        Ok(wallet) => {
            let wei_amount = (payload.amount * 1_000_000_000_000_000_000.0) as u128;
            let amount = U256::from(wei_amount);

            let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
            match wallet.send_native_coin(&payload.to, amount, &rpc_url).await {
                Ok(hash) => Ok(ResponseJson(TransactionResponse {
                    hash: format!("{:#x}", hash),
                    status: "confirmed".to_string(),
                })),
                Err(e) => {
                    warn!("Failed to send native coin: {}", e);
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
                    status: "confirmed".to_string(),
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

