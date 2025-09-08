use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
};
use tracing::{warn, info, debug};
use crate::wallet::*;
use crate::types::*;
use crate::utils::*;

pub async fn get_native_balance(
    Json(payload): Json<BalanceRequest>,
) -> Result<ResponseJson<BalanceResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    info!("Native balance request: address={}, network={:?}", payload.address, payload.network);
    
    let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
    debug!("Using RPC URL: {}", rpc_url);
    
    match EvmWallet::get_native_balance(&payload.address, &rpc_url).await {
        Ok(balance) => {
            let eth_balance = wei_to_eth(balance);
            debug!("Retrieved balance: {} wei -> {} ETH", balance, eth_balance);
            Ok(ResponseJson(BalanceResponse {
                balance: eth_balance,
            }))
        },
        Err(e) => {
            warn!("Failed to get native balance: {}", e);
            let error_msg = if e.to_string().contains("network") || e.to_string().contains("connection") {
                "Network connection failed. Please check your network configuration and try again."
            } else if e.to_string().contains("invalid") && e.to_string().contains("address") {
                "Invalid address format. Please provide a valid Ethereum address."
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

pub async fn get_erc20_balance(
    Json(payload): Json<Erc20BalanceRequest>,
) -> Result<ResponseJson<BalanceResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
    match EvmWallet::get_erc20_balance(&payload.address, &payload.token_address, &rpc_url).await {
        Ok(balance) => Ok(ResponseJson(BalanceResponse {
            balance: wei_to_eth(balance),
        })),
        Err(e) => {
            warn!("Failed to get ERC20 balance: {}", e);
            let error_msg = if e.to_string().contains("network") || e.to_string().contains("connection") {
                "Network connection failed. Please check your network configuration and try again."
            } else if e.to_string().contains("invalid") && e.to_string().contains("address") {
                "Invalid address format. Please provide a valid Ethereum address."
            } else if e.to_string().contains("contract") {
                "Invalid token contract address or contract not found."
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

 