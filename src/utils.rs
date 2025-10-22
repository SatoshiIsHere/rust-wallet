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

pub fn is_non_eip1559_network(rpc_url: &str) -> bool {
    let rpc_lower = rpc_url.to_lowercase();
    rpc_lower.contains("bsc") || 
    rpc_lower.contains("binance")
}

pub async fn get_dynamic_gas_price(rpc_url: &str) -> Result<U256, Box<dyn std::error::Error>> {
    let provider = ProviderBuilder::new()
        .connect_http(rpc_url.parse()?);
    
    let gas_price = provider.get_gas_price().await?;
    Ok(U256::from(gas_price))
}

pub async fn get_current_base_fee(rpc_url: &str) -> Result<U256, Box<dyn std::error::Error>> {
    let provider = ProviderBuilder::new()
        .connect_http(rpc_url.parse()?);
    
    if let Some(block) = provider.get_block_by_number(alloy::eips::BlockNumberOrTag::Latest).await? {
        if let Some(base_fee) = block.header.base_fee_per_gas {
            return Ok(U256::from(base_fee));
        }
    }
    
    let gas_price = provider.get_gas_price().await?;
    Ok(U256::from(gas_price))
}


pub fn get_network_fallback_gas_price(rpc_url: &str) -> U256 {
    let rpc_lower = rpc_url.to_lowercase();
    
    if rpc_lower.contains("polygon") || rpc_lower.contains("matic") {
        U256::from(35_000_000_000u64)
    } else if rpc_lower.contains("bsc") || rpc_lower.contains("binance") {
        U256::from(8_000_000_000u64) 
    } else if rpc_lower.contains("arbitrum") {
        U256::from(2_000_000_000u64)
    } else if rpc_lower.contains("optimism") {
        U256::from(10_000_000u64)
    } else if rpc_lower.contains("avalanche") || rpc_lower.contains("avax") {
        U256::from(30_000_000_000u64)
    } else if rpc_lower.contains("fantom") {
        U256::from(2_000_000_000u64)
    } else if rpc_lower.contains("ethereum") || rpc_lower.contains("mainnet") {
        U256::from(30_000_000_000u64)
    } else {
        U256::from(25_000_000_000u64)
    }
}


pub async fn get_dynamic_gas_price_with_retry(rpc_url: &str, max_retries: u32) -> Result<U256, Box<dyn std::error::Error>> {
    let mut last_error = None;
    
    for attempt in 1..=max_retries {
        match get_dynamic_gas_price(rpc_url).await {
            Ok(price) => {

                let min_gas_price = U256::from(1_000_000u64);
                let max_gas_price = U256::from(1_000_000_000_000u64);
                
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

pub async fn get_dynamic_gas_price_with_margin(rpc_url: &str, margin_percent: u32) -> Result<U256, Box<dyn std::error::Error>> {
    let base_price = get_dynamic_gas_price(rpc_url).await?;
    let margin = base_price * U256::from(margin_percent) / U256::from(100);
    let adjusted_price = base_price + margin;
    
    info!("Dynamic gas price: {} wei, margin: {}%, adjusted: {} wei", 
          base_price, margin_percent, adjusted_price);
    Ok(adjusted_price)
}

pub async fn get_eip1559_gas_price(rpc_url: &str) -> (U256, U256, U256) {
    if is_very_network(rpc_url) {
        info!("Using VERY network fixed gas price (EIP-1559)");
        let base_price = U256::from(1_200_000_000u64);
        let priority_fee = U256::from(1_000_000_000u64);
        let max_fee = base_price * U256::from(2);
        return (max_fee, priority_fee, base_price);
    }
    
    let provider = match ProviderBuilder::new().connect_http(rpc_url.parse().ok().unwrap_or_else(|| "http://localhost:8545".parse().unwrap())) {
        provider => provider
    };
    
    let base_price = match provider.get_gas_price().await {
        Ok(price) => U256::from(price),
        Err(_) => get_network_fallback_gas_price(rpc_url)
    };
    
    let max_fee = base_price * U256::from(2);
    
    let priority_fee = match provider.raw_request::<(), String>("eth_maxPriorityFeePerGas".into(), ()).await {
        Ok(hex_value) => {
            match U256::from_str(&hex_value) {
                Ok(value) => {
                    info!("Got priority fee from network API (eth_maxPriorityFeePerGas): {} wei", value);
                    value
                }
                Err(_) => {
                    warn!("Failed to parse priority fee, calculating from base_price");
                    calculate_priority_fee(rpc_url, base_price)
                }
            }
        }
        Err(_) => {
            warn!("Network doesn't support eth_maxPriorityFeePerGas, calculating from base_price");
            calculate_priority_fee(rpc_url, base_price)
        }
    };
    
    info!("EIP-1559 gas prices - base: {} wei, max_fee: {} wei (2x), priority_fee: {} wei", 
          base_price, max_fee, priority_fee);
    
    (max_fee, priority_fee, base_price)
}

fn calculate_priority_fee(rpc_url: &str, base_price: U256) -> U256 {
    // base_price의 10%를 priority fee로 사용 (완전히 네트워크 기반!)
    let calculated = base_price / U256::from(10); // 10%
    
    let rpc_lower = rpc_url.to_lowercase();
    
    // 매우 낮은 최소값만 설정 (하드코딩 최소화)
    let min_priority = if rpc_lower.contains("arbitrum") || rpc_lower.contains("optimism") {
        U256::from(1_000_000u64) // 0.001 Gwei (L2는 매우 낮게)
    } else if rpc_lower.contains("polygon") {
        U256::from(10_000_000u64) // 0.01 Gwei
    } else {
        U256::from(10_000_000u64) // 0.01 Gwei (기본값)
    };
    
    // 계산된 값 우선, 최소값은 안전장치로만
    if calculated < min_priority {
        min_priority
    } else {
        calculated // 네트워크 기반 값 사용!
    }
}

pub async fn get_smart_gas_price(rpc_url: &str) -> U256 {
    if is_very_network(rpc_url) {
        info!("Using VERY network fixed gas price");
        return U256::from(1_200_000_000u64);
    }
    
    match get_dynamic_gas_price_with_margin(rpc_url, 10).await {
        Ok(price) => {
            info!("Successfully got dynamic gas price with 10% margin: {} wei", price);
            price
        }
        Err(e) => {
            warn!("Failed to get dynamic gas price with margin: {}, using fallback", e);
            get_network_fallback_gas_price(rpc_url)
        }
    }
}

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