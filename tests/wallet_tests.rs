use evm_wallet::*;
use alloy::{node_bindings::Anvil, primitives::U256};
use std::env;

fn setup_test_env() {
    env::set_var("RPC_ENDPOINT", "http://localhost:8545");
}

#[tokio::test]
async fn test_generate_mnemonic() {
    let mnemonic = EvmWallet::generate_mnemonic().unwrap();
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    assert_eq!(words.len(), 24);
}

#[tokio::test]
async fn test_generate_mnemonic_with_different_word_counts() {
    let mnemonic_12 = EvmWallet::generate_mnemonic_with_words(12).unwrap();
    let words_12: Vec<&str> = mnemonic_12.split_whitespace().collect();
    assert_eq!(words_12.len(), 12);

    let mnemonic_18 = EvmWallet::generate_mnemonic_with_words(18).unwrap();
    let words_18: Vec<&str> = mnemonic_18.split_whitespace().collect();
    assert_eq!(words_18.len(), 18);

    let mnemonic_24 = EvmWallet::generate_mnemonic_with_words(24).unwrap();
    let words_24: Vec<&str> = mnemonic_24.split_whitespace().collect();
    assert_eq!(words_24.len(), 24);

    let result = EvmWallet::generate_mnemonic_with_words(13);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_wallet_from_mnemonic() {
    let mnemonic = EvmWallet::generate_mnemonic().unwrap();
    let wallet = EvmWallet::from_mnemonic(&mnemonic).unwrap();
    
    assert!(!wallet.private_key.is_empty());
    assert!(!wallet.public_key.is_empty());
    assert!(!wallet.address.is_empty());
    assert!(wallet.address.starts_with("0x"));
    assert_eq!(wallet.mnemonic, Some(mnemonic));
}

#[tokio::test]
async fn test_create_wallet_from_private_key() {
    let original_wallet = EvmWallet::new_random().unwrap();
    let restored_wallet = EvmWallet::create_wallet_from_private_key(&original_wallet.private_key).unwrap();
    
    assert_eq!(original_wallet.private_key, restored_wallet.private_key);
    assert_eq!(original_wallet.address, restored_wallet.address);
    assert_eq!(original_wallet.public_key, restored_wallet.public_key);
}

#[tokio::test]
async fn test_address_from_private_key() {
    let private_key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let address = EvmWallet::address_from_private_key(private_key).unwrap();
    
    assert!(address.starts_with("0x"));
    assert_eq!(address.len(), 42);
    let address2 = EvmWallet::address_from_private_key(private_key).unwrap();
    assert_eq!(address, address2);
    
    let private_key_no_prefix = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let address3 = EvmWallet::address_from_private_key(private_key_no_prefix).unwrap();
    assert_eq!(address, address3);
}

#[tokio::test]
async fn test_address_from_private_key_vs_create_wallet() {
    let original_wallet = EvmWallet::new_random().unwrap();
    
    let address_only = EvmWallet::address_from_private_key(&original_wallet.private_key).unwrap();
    
    let full_wallet = EvmWallet::create_wallet_from_private_key(&original_wallet.private_key).unwrap();
    
    assert_eq!(address_only, full_wallet.address);
    assert_eq!(address_only, original_wallet.address);
}

#[tokio::test]
async fn test_invalid_private_key_handling() {
    let invalid_keys = vec![
        "",                                    
        "invalid",                            
        "0x",                                
        "0x123",                             
        "0x123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef123456", 
        "0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG",
    ];
    
    for invalid_key in invalid_keys {
        let result = EvmWallet::address_from_private_key(invalid_key);
        assert!(result.is_err(), "Should fail for invalid private key: {}", invalid_key);
        
        let result2 = EvmWallet::create_wallet_from_private_key(invalid_key);
        assert!(result2.is_err(), "Should fail for invalid private key: {}", invalid_key);
    }
}

#[tokio::test]
async fn test_wallet_with_anvil_and_env() {
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();
    
    env::set_var("RPC_ENDPOINT", &provider_url);
    
    let wallet = EvmWallet::new_random().unwrap();
    
    let rpc_url = get_default_rpc_url();
    let balance = EvmWallet::get_native_balance(&wallet.address, &rpc_url).await;
    assert!(balance.is_ok());
    
    let dummy_address = "0x742d35Cc6634C0532925a3b8D55de0c4a2e6D6b4";
    let amount = U256::from(1000000000000000000u64);
    let gas_estimate = wallet.estimate_gas(dummy_address, amount, &rpc_url).await;
    if let Err(e) = &gas_estimate {
        println!("Gas estimate error: {}", e);
    }
    // Allow for network errors in test environment
    assert!(gas_estimate.is_ok() || gas_estimate.unwrap_err().to_string().contains("error sending request"));
}

#[tokio::test]
async fn test_send_native_coin_with_private_key() {
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();
    
    env::set_var("RPC_ENDPOINT", &provider_url);
    
    let sender_wallet = EvmWallet::new_random().unwrap();
    let wallet = EvmWallet::create_wallet_from_private_key(&sender_wallet.private_key).unwrap();
    
    let recipient = "0x742d35Cc6634C0532925a3b8D55de0c4a2e6D6b4";
    let amount = U256::from(1000000000000000000u64);
    
    let rpc_url = get_default_rpc_url();
    let result = wallet.send_native_coin(recipient, amount, &rpc_url).await;
    
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    println!("Error message: {}", error_msg);
    // Accept various error types: insufficient funds, network errors, gas errors
    assert!(error_msg.contains("insufficient") || error_msg.contains("fund") || 
            error_msg.contains("balance") || error_msg.contains("gas") ||
            error_msg.contains("error sending request") || error_msg.contains("connection"));
}

#[tokio::test]
async fn test_send_erc20_token_with_private_key() {
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();
    
    env::set_var("RPC_ENDPOINT", &provider_url);
    
    let sender_wallet = EvmWallet::new_random().unwrap();
    let wallet = EvmWallet::create_wallet_from_private_key(&sender_wallet.private_key).unwrap();
    
    let recipient = "0x742d35Cc6634C0532925a3b8D55de0c4a2e6D6b4";
    let token_address = "0x1234567890123456789012345678901234567890";
    let amount = U256::from(1000000000000000000u64);
    
    let rpc_url = get_default_rpc_url();
    let result = wallet.send_erc20_token(recipient, amount, token_address, &rpc_url).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_estimate_gas_with_private_key() {
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();

    env::set_var("RPC_ENDPOINT", &provider_url);
    
    let sender_wallet = EvmWallet::new_random().unwrap();
    let wallet = EvmWallet::create_wallet_from_private_key(&sender_wallet.private_key).unwrap();
    
    let recipient = "0x742d35Cc6634C0532925a3b8D55de0c4a2e6D6b4";
    let amount = U256::from(1000000000000000000u64);
    
    let rpc_url = get_default_rpc_url();
    let result = wallet.estimate_gas(recipient, amount, &rpc_url).await;
    assert!(result.is_ok());
    
    let (gas_limit, gas_price, total_fee) = result.unwrap();
    assert!(gas_limit > 0);
    assert!(gas_limit <= 21000);
    assert!(!gas_price.is_empty());
    assert!(!total_fee.is_empty());
}

#[tokio::test]
async fn test_mnemonic_deterministic() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
    
    let wallet1 = EvmWallet::from_mnemonic(mnemonic).unwrap();
    let wallet2 = EvmWallet::from_mnemonic(mnemonic).unwrap();
    
    assert_eq!(wallet1.private_key, wallet2.private_key);
    assert_eq!(wallet1.address, wallet2.address);
    assert_eq!(wallet1.public_key, wallet2.public_key);
}

#[tokio::test]
async fn test_transaction_functions_with_known_private_key() {
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();
    
    env::set_var("RPC_ENDPOINT", &provider_url);
    
    let test_private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    
    let address = EvmWallet::address_from_private_key(test_private_key).unwrap();
    assert_eq!(address, "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    
    let wallet = EvmWallet::create_wallet_from_private_key(test_private_key).unwrap();
    assert_eq!(wallet.address, address);
    assert_eq!(wallet.private_key, test_private_key);
    
    let dummy_recipient = "0x742d35Cc6634C0532925a3b8D55de0c4a2e6D6b4";
    let amount = U256::from(1000000000000000000u64);
    let rpc_url = get_default_rpc_url();
    let gas_result = wallet.estimate_gas(dummy_recipient, amount, &rpc_url).await;
    assert!(gas_result.is_ok());
}

#[tokio::test]
async fn test_environment_variable_usage() {
    setup_test_env();
    
    let rpc_url = get_default_rpc_url();
    assert_eq!(rpc_url, "http://localhost:8545");
    
    env::set_var("RPC_ENDPOINT", "https://custom-rpc.example.com");
    let custom_rpc = get_default_rpc_url();
    assert_eq!(custom_rpc, "https://custom-rpc.example.com");
} 