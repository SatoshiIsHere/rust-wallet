use std::env;
use tracing::info;
use evm_wallet::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        // .with_max_level(tracing::Level::DEBUG)
        .init();
    let app = create_router();
    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Wallet API Server starting on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
} 