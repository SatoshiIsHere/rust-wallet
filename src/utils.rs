use std::env;
use std::collections::HashMap;
use std::sync::Mutex;
use alloy::primitives::U256;
use crate::types::NetworkInfo;

pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_default_rpc_url() -> String {
    get_env_var("RPC_ENDPOINT").unwrap_or_else(|| "http://localhost:8545".to_string())
}

pub fn get_rpc_url_for_network(network: Option<&str>) -> String {
    match network {
        None => get_default_rpc_url(),
        Some(network_name) => {
            if let Ok(custom_networks) = get_custom_networks().lock() {
                if let Some(rpc_url) = custom_networks.get(network_name) {
                    return rpc_url.clone();
                }
            }
            get_default_rpc_url()
        }
    }
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

fn get_custom_networks() -> &'static Mutex<HashMap<String, String>> {
    static CUSTOM_NETWORKS: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
    CUSTOM_NETWORKS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn add_custom_network(name: String, rpc_url: String) -> Result<(), String> {
    let mut networks = get_custom_networks().lock().map_err(|_| "Failed to lock networks")?;
    networks.insert(name, rpc_url);
    Ok(())
}

pub fn remove_custom_network(name: &str) -> Result<(), String> {
    
    let mut networks = get_custom_networks().lock().map_err(|_| "Failed to lock networks")?;
    if networks.remove(name).is_none() {
        return Err(format!("Network '{}' not found", name));
    }
    Ok(())
}

pub fn get_all_networks() -> Vec<NetworkInfo> {
    let mut networks = Vec::new();
    
    if let Ok(custom_networks) = get_custom_networks().lock() {
        for (name, rpc_url) in custom_networks.iter() {
            networks.push(NetworkInfo { 
                name: name.clone(), 
                rpc_url: rpc_url.clone() 
            });
        }
    }
    
    networks
} 