use std::env;
use alloy::primitives::{U256, Address, Bytes};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{warn, info};

pub fn get_rpc_url_for_network(network: Option<&str>) -> String {
    match network {
        None => env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string()),
        Some(network_input) => network_input.to_string(),
    }
}

pub fn is_very_network(rpc_url: &str) -> bool {
    rpc_url.contains("verylabs.io") || rpc_url.contains("very")
}

pub async fn get_dynamic_gas_price(rpc_url: &str) -> Result<U256, Box<dyn std::error::Error>> {
    let provider = ProviderBuilder::new()
        .connect_http(rpc_url.parse()?);
    
    let gas_price = provider.get_gas_price().await?;
    Ok(U256::from(gas_price))
}


pub fn get_network_fallback_gas_price(rpc_url: &str) -> U256 {
    let rpc_lower = rpc_url.to_lowercase();
    
    if rpc_lower.contains("ethereum") || rpc_lower.contains("mainnet") {
        U256::from(20_000_000_000u64) 
    } else if rpc_lower.contains("polygon") || rpc_lower.contains("matic") {
        U256::from(30_000_000_000u64) 
    } else if rpc_lower.contains("bsc") || rpc_lower.contains("binance") {
        U256::from(5_000_000_000u64) 
    } else if rpc_lower.contains("arbitrum") {
        U256::from(100_000_000u64) 
    } else if rpc_lower.contains("optimism") {
        U256::from(1_000_000u64) 
    } else if rpc_lower.contains("avalanche") || rpc_lower.contains("avax") {
        U256::from(25_000_000_000u64) 
    } else if rpc_lower.contains("fantom") {
        U256::from(1_000_000_000u64) 
    } else {
        U256::from(20_000_000_000u64) 
    }
}


pub async fn get_dynamic_gas_price_with_retry(rpc_url: &str, max_retries: u32) -> Result<U256, Box<dyn std::error::Error>> {
    let mut last_error = None;
    
    for attempt in 1..=max_retries {
        match get_dynamic_gas_price(rpc_url).await {
            Ok(price) => {

                let min_gas_price = U256::from(1_000_000u64); // 0.001 Gwei
                let max_gas_price = U256::from(1_000_000_000_000u64); // 1000 Gwei
                
                if price >= min_gas_price && price <= max_gas_price {
                    return Ok(price);
                } else {
                    warn!("Gas price out of range: {}, using fallback", price);
                    return Ok(get_network_fallback_gas_price(rpc_url));
                }
            }
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    let delay = Duration::from_millis(500 * attempt as u64);
                    warn!("Attempt {} failed to get gas price, retrying in {:?}: {}", attempt, delay, last_error.as_ref().unwrap());
                    sleep(delay).await;
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}

/// 안전한 가스비 가져오기 (재시도 + fallback)
pub async fn get_safe_gas_price(rpc_url: &str) -> U256 {
    match get_dynamic_gas_price_with_retry(rpc_url, 3).await {
        Ok(price) => {
            info!("Successfully got dynamic gas price: {} wei", price);
            price
        }
        Err(e) => {
            let fallback_price = get_network_fallback_gas_price(rpc_url);
            warn!("Failed to get dynamic gas price after retries: {}, using network fallback: {} wei", e, fallback_price);
            fallback_price
        }
    }
}

pub fn wei_to_eth(wei: U256) -> String {
    let wei_str = wei.to_string();
    let len = wei_str.len();
    
    let eth_str = if len <= 18 {
        format!("0.{}", format!("{:0>18}", wei_str))
    } else {
        let (integer_part, fractional_part) = wei_str.split_at(len - 18);
        format!("{}.{}", integer_part, fractional_part)
    };
    
    eth_str.trim_end_matches('0').trim_end_matches('.').to_string()
}

pub async fn get_token_decimals(token_address: &str, rpc_url: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let provider = ProviderBuilder::new()
        .connect_http(rpc_url.parse()?);
    
    let token_addr = Address::from_str(token_address)?;
    
    let function_selector = "313ce567";
    let data = format!("{}", function_selector);
    let call_data = Bytes::from(hex::decode(data)?);

    let call_request = TransactionRequest::default()
        .to(token_addr)
        .input(call_data.into());

    let result = provider.call(call_request).await?;
    let decimals = result[31];
    Ok(decimals)
}

pub fn token_amount_to_readable(amount: U256, decimals: u8) -> String {
    let amount_str = amount.to_string();
    let len = amount_str.len();
    let decimal_places = decimals as usize;
    
    let readable_str = if len <= decimal_places {
        format!("0.{}", format!("{:0>width$}", amount_str, width = decimal_places))
    } else {
        let (integer_part, fractional_part) = amount_str.split_at(len - decimal_places);
        format!("{}.{}", integer_part, fractional_part)
    };
    
    readable_str.trim_end_matches('0').trim_end_matches('.').to_string()
}