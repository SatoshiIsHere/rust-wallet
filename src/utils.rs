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
    
    // 더 구체적인 키워드를 먼저 확인 (우선순위 순서)
    if rpc_lower.contains("polygon") || rpc_lower.contains("matic") {
        U256::from(35_000_000_000u64) // 30 → 35 Gwei (17% 인상)
    } else if rpc_lower.contains("bsc") || rpc_lower.contains("binance") {
        U256::from(8_000_000_000u64)  // 5 → 8 Gwei (60% 인상)
    } else if rpc_lower.contains("arbitrum") {
        U256::from(2_000_000_000u64)  // 0.1 → 2 Gwei (20배 인상)
    } else if rpc_lower.contains("optimism") {
        U256::from(10_000_000u64)     // 0.001 → 0.01 Gwei (10배 인상)
    } else if rpc_lower.contains("avalanche") || rpc_lower.contains("avax") {
        U256::from(30_000_000_000u64) // 25 → 30 Gwei (20% 인상)
    } else if rpc_lower.contains("fantom") {
        U256::from(2_000_000_000u64)  // 1 → 2 Gwei (100% 인상)
    } else if rpc_lower.contains("ethereum") || rpc_lower.contains("mainnet") {
        U256::from(30_000_000_000u64) // 20 → 30 Gwei (50% 인상)
    } else {
        U256::from(25_000_000_000u64) // 20 → 25 Gwei (25% 인상)
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

/// 동적 가스 가격에 마진 추가 (안정성 확보)
pub async fn get_dynamic_gas_price_with_margin(rpc_url: &str, margin_percent: u32) -> Result<U256, Box<dyn std::error::Error>> {
    let base_price = get_dynamic_gas_price(rpc_url).await?;
    let margin = base_price * U256::from(margin_percent) / U256::from(100);
    let adjusted_price = base_price + margin;
    
    info!("Dynamic gas price: {} wei, margin: {}%, adjusted: {} wei", 
          base_price, margin_percent, adjusted_price);
    Ok(adjusted_price)
}

/// EIP-1559 가스 가격 정보 (max_fee_per_gas, max_priority_fee_per_gas)
pub async fn get_eip1559_gas_price(rpc_url: &str) -> (U256, U256) {
    // VERY 네트워크는 기존 로직 유지
    if is_very_network(rpc_url) {
        info!("Using VERY network fixed gas price (EIP-1559)");
        let fixed_price = U256::from(1_200_000_000u64);
        return (fixed_price, U256::from(1_000_000_000u64)); // max_fee, priority_fee
    }
    
    match get_dynamic_gas_price(rpc_url).await {
        Ok(base_price) => {
            // max_fee_per_gas: base_price의 2배 (여유있게 설정)
            let max_fee = base_price * U256::from(2);
            
            // max_priority_fee_per_gas: 네트워크별로 적절한 팁 설정
            let priority_fee = calculate_priority_fee(rpc_url, base_price);
            
            info!("EIP-1559 gas prices - base: {} wei, max_fee: {} wei (2x), priority_fee: {} wei", 
                  base_price, max_fee, priority_fee);
            
            (max_fee, priority_fee)
        }
        Err(e) => {
            warn!("Failed to get dynamic gas price: {}, using fallback", e);
            let fallback = get_network_fallback_gas_price(rpc_url);
            let max_fee = fallback * U256::from(2); // 2배로 여유있게
            let priority_fee = fallback / U256::from(20); // 5% 정도를 팁으로
            
            info!("Using fallback EIP-1559 gas prices - max_fee: {} wei, priority_fee: {} wei", 
                  max_fee, priority_fee);
            
            (max_fee, priority_fee)
        }
    }
}

/// 네트워크별 적절한 priority fee 계산
fn calculate_priority_fee(rpc_url: &str, base_price: U256) -> U256 {
    let rpc_lower = rpc_url.to_lowercase();
    
    // L2 네트워크들은 낮은 priority fee
    if rpc_lower.contains("arbitrum") || rpc_lower.contains("optimism") {
        U256::from(100_000_000u64) // 0.1 Gwei
    } else if rpc_lower.contains("polygon") {
        U256::from(30_000_000_000u64) // 30 Gwei (Polygon은 높게)
    } else if rpc_lower.contains("bsc") {
        U256::from(1_000_000_000u64) // 1 Gwei
    } else {
        // 기본값: base_price의 5% 정도 (최소 1 Gwei, 최대 5 Gwei)
        let calculated = base_price / U256::from(20); // 5%
        let min_priority = U256::from(1_000_000_000u64); // 1 Gwei
        let max_priority = U256::from(5_000_000_000u64); // 5 Gwei
        
        if calculated < min_priority {
            min_priority
        } else if calculated > max_priority {
            max_priority
        } else {
            calculated
        }
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

/// 안전한 가스비 가져오기 (재시도 + fallback) - 기존 함수 유지 (호환성)
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