use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
};
use tracing::warn;
use crate::wallet::*;
use crate::types::*;
use crate::utils::*;

pub async fn health_check() -> &'static str {
    "EVM Wallet API is running!"
}


pub async fn get_transaction_details(
    Json(payload): Json<TransactionDetailsRequest>,
) -> Result<ResponseJson<TransactionDetailsResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
    match EvmWallet::get_native_transaction_details(&payload.tx_hash, &rpc_url).await {
        Ok(Some(transaction)) => {
            Ok(ResponseJson(TransactionDetailsResponse { transaction }))
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                ResponseJson(ErrorResponse { error: "Transaction not found".to_string() }),
            ))
        }
        Err(e) => {
            warn!("Failed to get transaction details: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

pub async fn get_native_transaction_history(
    Json(payload): Json<NativeTransactionHistoryRequest>,
) -> Result<ResponseJson<TransactionHistoryResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
    match EvmWallet::get_native_transactions_by_block_range(
        &payload.address,
        payload.from_block,
        payload.to_block,
        &rpc_url,
    ).await {
        Ok(transactions) => {
            Ok(ResponseJson(TransactionHistoryResponse { transactions }))
        }
        Err(e) => {
            warn!("Failed to get native transaction history: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

pub async fn get_erc20_events(
    Json(payload): Json<Erc20EventsRequest>,
) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
    match EvmWallet::get_erc20_transfer_events(
        &payload.token_address,
        payload.from_block,
        payload.to_block,
        payload.address_filter.as_deref(),
        &rpc_url,
    ).await {
        Ok(events) => {
            let response = serde_json::json!({
                "events": events
            });
            Ok(ResponseJson(response))
        }
        Err(e) => {
            warn!("Failed to get ERC20 events: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

pub async fn get_all_native_transaction_history(
    Json(payload): Json<AllTransactionHistoryRequest>,
) -> Result<ResponseJson<TransactionHistoryResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let rpc_url = get_rpc_url_for_network(payload.network.as_deref());
    match EvmWallet::get_all_native_transactions_by_block_range(
        payload.from_block,
        payload.to_block,
        &rpc_url,
    ).await {
        Ok(transactions) => {
            Ok(ResponseJson(TransactionHistoryResponse { transactions }))
        }
        Err(e) => {
            warn!("Failed to get all native transaction history: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

pub async fn get_current_block() -> Result<ResponseJson<CurrentBlockResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let rpc_url = get_rpc_url_for_network(None);
    match EvmWallet::get_current_block(&rpc_url).await {
        Ok(current_block) => {
            Ok(ResponseJson(CurrentBlockResponse { current_block }))
        }
        Err(e) => {
            warn!("Failed to get current block: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}
