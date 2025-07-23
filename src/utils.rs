use std::env;
use alloy::primitives::U256;

pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_default_rpc_url() -> String {
    get_env_var("RPC_ENDPOINT").unwrap_or_else(|| "https://rpc.verylabs.io".to_string())
}

pub fn get_default_private_key() -> Option<String> {
    get_env_var("PRIVATE_KEY")
}

pub fn get_server_port() -> u16 {
    get_env_var("SERVER_PORT")
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000)
}

pub fn wei_to_eth(wei: U256) -> String {
    let wei_str = wei.to_string();
    let len = wei_str.len();
    
    if len <= 18 {
        let padded = format!("{:0>18}", wei_str);
        let eth_str = format!("0.{}", padded);
        trim_trailing_zeros(&eth_str)
    } else {
        let (integer_part, fractional_part) = wei_str.split_at(len - 18);
        let eth_str = format!("{}.{}", integer_part, fractional_part);
        trim_trailing_zeros(&eth_str)
    }
}

fn trim_trailing_zeros(s: &str) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
} 