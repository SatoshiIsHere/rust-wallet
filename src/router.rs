use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/wallet/getAddress", post(handlers::address_from_private_key))
        .route("/wallet/generateMnemonic", post(handlers::generate_mnemonic))
        .route("/wallet/generateMnemonicCustom", post(handlers::generate_mnemonic_with_words))
        .route("/wallet/fromMnemonic", post(handlers::create_wallet_from_mnemonic))
        .route("/transaction/sendNative", post(handlers::send_native_coin))
        .route("/transaction/sendErc20", post(handlers::send_erc20_token))
        .route("/transaction/estimateGas", post(handlers::estimate_gas))
        .route("/transaction/receipt", post(handlers::get_transaction_receipt))
        .route("/transaction/details", post(handlers::get_transaction_details))
        .route("/transaction/history", post(handlers::get_native_transaction_history))
        .route("/transaction/history/all", post(handlers::get_all_native_transaction_history))
        .route("/balance/native", post(handlers::get_native_balance))
        .route("/balance/erc20", post(handlers::get_erc20_balance))
        .route("/events/erc20Transfers", post(handlers::get_erc20_events))
        .route("/block/current", get(handlers::get_current_block))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        )
} 
