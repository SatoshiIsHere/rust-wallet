use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
};
use tracing::warn;
use crate::wallet::*;
use crate::types::*;

pub async fn address_from_private_key(
    Json(payload): Json<CreateFromPrivateKeyRequest>,
) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<ErrorResponse>)> {
    match EvmWallet::address_from_private_key(&payload.private_key) {
        Ok(address) => {
            let response = serde_json::json!({
                "address": address
            });
            Ok(ResponseJson(response))
        }
        Err(e) => {
            warn!("Failed to get address from private key: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

pub async fn generate_mnemonic() -> Result<ResponseJson<MnemonicResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    match EvmWallet::generate_mnemonic() {
        Ok(mnemonic) => Ok(ResponseJson(MnemonicResponse { mnemonic })),
        Err(e) => {
            warn!("Failed to generate mnemonic: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

pub async fn generate_mnemonic_with_words(
    Json(payload): Json<GenerateMnemonicRequest>,
) -> Result<ResponseJson<MnemonicResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let word_count = payload.word_count.unwrap_or(24);
    
    match EvmWallet::generate_mnemonic_with_words(word_count) {
        Ok(mnemonic) => Ok(ResponseJson(MnemonicResponse { mnemonic })),
        Err(e) => {
            warn!("Failed to generate mnemonic: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
}

pub async fn create_wallet_from_mnemonic(
    Json(payload): Json<CreateFromMnemonicRequest>,
) -> Result<ResponseJson<WalletResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    match EvmWallet::from_mnemonic(&payload.mnemonic) {
        Ok(wallet) => {
            let response = WalletResponse {
                address: wallet.address,
                private_key: wallet.private_key,
                public_key: wallet.public_key,
                mnemonic: wallet.mnemonic,
            };
            Ok(ResponseJson(response))
        }
        Err(e) => {
            warn!("Failed to create wallet from mnemonic: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ErrorResponse { error: e.to_string() }),
            ))
        }
    }
} 