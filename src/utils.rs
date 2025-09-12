use std::env;
use alloy::primitives::U256;

pub fn get_rpc_url_for_network(network: Option<&str>) -> String {
    match network {
        None => env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string()),
        Some(network_input) => network_input.to_string(),
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