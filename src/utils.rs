use std::env;
use alloy::primitives::{U256, Address, Bytes};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use std::str::FromStr;

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