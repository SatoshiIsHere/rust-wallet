use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
};
use tracing::warn;
use crate::wallet::*;
use crate::types::*;
use crate::utils::*;

pub async fn get_native_balance(
    Json(payload): Json<BalanceRequest>,
) -> Result<ResponseJson<BalanceResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let rpc_url = get_default_rpc_url();
    match EvmWallet::get_native_balance(&payload.address, &rpc_url).await {
        Ok(balance) => Ok(ResponseJson(BalanceResponse {
            balance: wei_to_eth(balance),
        })),
        Err(e) => {
            warn!("Failed to get native balance: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

pub async fn get_erc20_balance(
    Json(payload): Json<Erc20BalanceRequest>,
) -> Result<ResponseJson<BalanceResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let rpc_url = get_default_rpc_url();
    match EvmWallet::get_erc20_balance(&payload.address, &payload.token_address, &rpc_url).await {
        Ok(balance) => Ok(ResponseJson(BalanceResponse {
            balance: wei_to_eth(balance),
        })),
        Err(e) => {
            warn!("Failed to get ERC20 balance: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

 