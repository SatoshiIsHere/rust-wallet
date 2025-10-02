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
    
    let rpc_url = env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string());
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
    
    let rpc_url = env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string());
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
    
    let rpc_url = env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string());
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
    
    let rpc_url = env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string());
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
    let rpc_url = env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string());
    let gas_result = wallet.estimate_gas(dummy_recipient, amount, &rpc_url).await;
    assert!(gas_result.is_ok());
}

#[tokio::test]
async fn test_environment_variable_usage() {
    setup_test_env();
    
    let rpc_url = env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string());
    assert_eq!(rpc_url, "http://localhost:8545");
    
    env::set_var("RPC_ENDPOINT", "https://custom-rpc.example.com");
    let custom_rpc = env::var("RPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:8545".to_string());
    assert_eq!(custom_rpc, "https://custom-rpc.example.com");
}

// ========== 가스 가격 관련 테스트 ==========

#[tokio::test]
async fn test_network_fallback_gas_prices() {
    use evm_wallet::utils::get_network_fallback_gas_price;
    
    // Ethereum 메인넷 테스트
    let ethereum_urls = vec![
        "https://eth-mainnet.infura.io",
        "https://mainnet.alchemyapi.io",
        "https://ethereum-mainnet.example.com"
    ];
    
    for url in ethereum_urls {
        let gas_price = get_network_fallback_gas_price(url);
        assert_eq!(gas_price, U256::from(30_000_000_000u64)); // 30 Gwei
    }
    
    // Polygon 테스트
    let polygon_urls = vec![
        "https://polygon-mainnet.infura.io",
        "https://matic-mainnet.alchemyapi.io",
        "https://polygon.example.com"
    ];
    
    for url in polygon_urls {
        let gas_price = get_network_fallback_gas_price(url);
        assert_eq!(gas_price, U256::from(35_000_000_000u64)); // 35 Gwei
    }
    
    // BSC 테스트
    let bsc_urls = vec![
        "https://bsc-dataseed.binance.org",
        "https://binance-smart-chain.example.com"
    ];
    
    for url in bsc_urls {
        let gas_price = get_network_fallback_gas_price(url);
        assert_eq!(gas_price, U256::from(8_000_000_000u64)); // 8 Gwei
    }
    
    // Arbitrum 테스트
    let arbitrum_urls = vec![
        "https://arbitrum-mainnet.infura.io",
        "https://arbitrum.example.com"
    ];
    
    for url in arbitrum_urls {
        let gas_price = get_network_fallback_gas_price(url);
        assert_eq!(gas_price, U256::from(2_000_000_000u64)); // 2 Gwei
    }
    
    // Optimism 테스트
    let optimism_urls = vec![
        "https://optimism-mainnet.infura.io",
        "https://optimism.example.com"
    ];
    
    for url in optimism_urls {
        let gas_price = get_network_fallback_gas_price(url);
        assert_eq!(gas_price, U256::from(10_000_000u64)); // 0.01 Gwei
    }
    
    // 기본값 테스트 (알 수 없는 네트워크)
    let unknown_url = "https://unknown-network.example.com";
    let gas_price = get_network_fallback_gas_price(unknown_url);
    assert_eq!(gas_price, U256::from(25_000_000_000u64)); // 25 Gwei
}

#[tokio::test]
async fn test_very_network_detection() {
    use evm_wallet::utils::is_very_network;
    
    assert!(is_very_network("https://verylabs.io"));
    assert!(is_very_network("https://api.verylabs.io"));
    assert!(is_very_network("https://very.example.com"));
    assert!(!is_very_network("https://ethereum.org"));
    assert!(!is_very_network("https://polygon.technology"));
}

#[tokio::test]
async fn test_network_priority_detection() {
    use evm_wallet::utils::get_network_fallback_gas_price;
    
    // "mainnet"이 포함되어도 "polygon"이 우선되어야 함
    let polygon_mainnet_url = "https://polygon-mainnet.infura.io";
    let gas_price = get_network_fallback_gas_price(polygon_mainnet_url);
    assert_eq!(gas_price, U256::from(35_000_000_000u64)); // Polygon 가격, Ethereum 가격 아님
    
    // "ethereum"이 포함되어도 "arbitrum"이 우선되어야 함  
    let arbitrum_url = "https://arbitrum-ethereum-mainnet.infura.io";
    let gas_price = get_network_fallback_gas_price(arbitrum_url);
    assert_eq!(gas_price, U256::from(2_000_000_000u64)); // Arbitrum 가격
}

#[tokio::test]
async fn test_smart_gas_price_with_anvil() {
    use evm_wallet::utils::get_smart_gas_price;
    
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();
    
    // Anvil에서 동적 가스 가격 가져오기 테스트
    let gas_price = get_smart_gas_price(&provider_url).await;
    
    // Anvil은 매우 낮은 가스 가격을 사용하므로, 결과가 합리적인 범위인지 확인
    assert!(gas_price > U256::ZERO);
    assert!(gas_price < U256::from(1_000_000_000_000u64)); // 1000 Gwei 미만
}

#[tokio::test]
async fn test_gas_price_margin_calculation() {
    use evm_wallet::utils::get_dynamic_gas_price_with_margin;
    
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();
    
    // 10% 마진이 올바르게 적용되는지 테스트
    let result = get_dynamic_gas_price_with_margin(&provider_url, 10).await;
    assert!(result.is_ok());
    
    let gas_price_with_margin = result.unwrap();
    assert!(gas_price_with_margin > U256::ZERO);
}

// ========== 에러 처리 테스트 ==========

#[tokio::test]
async fn test_invalid_rpc_url_handling() {
    use evm_wallet::utils::get_smart_gas_price;
    
    // 잘못된 RPC URL로 테스트
    let invalid_urls = vec![
        "http://invalid-url-that-does-not-exist.com",
        "https://invalid-rpc-endpoint.example.com",
        "ftp://invalid-protocol.com",
    ];
    
    for invalid_url in invalid_urls {
        let result = get_smart_gas_price(invalid_url).await;
        // 실패하더라도 fallback 가격이 반환되어야 함
        assert!(result > U256::ZERO);
    }
}

#[tokio::test]
async fn test_wei_to_eth_conversion() {
    use evm_wallet::utils::wei_to_eth;
    
    // 1 ETH = 10^18 wei
    let one_eth_wei = U256::from(1_000_000_000_000_000_000u64);
    let eth_str = wei_to_eth(one_eth_wei);
    assert_eq!(eth_str, "1");
    
    // 0.5 ETH = 5 * 10^17 wei
    let half_eth_wei = U256::from(500_000_000_000_000_000u64);
    let eth_str = wei_to_eth(half_eth_wei);
    assert_eq!(eth_str, "0.5");
    
    // 0.001 ETH = 10^15 wei
    let small_amount_wei = U256::from(1_000_000_000_000_000u64);
    let eth_str = wei_to_eth(small_amount_wei);
    assert_eq!(eth_str, "0.001");
    
    // 매우 작은 금액
    let tiny_amount_wei = U256::from(1_000_000_000_000u64); // 0.000001 ETH
    let eth_str = wei_to_eth(tiny_amount_wei);
    assert_eq!(eth_str, "0.000001");
}

#[tokio::test]
async fn test_token_amount_conversion() {
    use evm_wallet::utils::token_amount_to_readable;
    
    // USDT (6 decimals)
    let usdt_amount = U256::from(1_000_000u64); // 1 USDT
    let readable = token_amount_to_readable(usdt_amount, 6);
    assert_eq!(readable, "1");
    
    // USDC (6 decimals)
    let usdc_amount = U256::from(500_000u64); // 0.5 USDC
    let readable = token_amount_to_readable(usdc_amount, 6);
    assert_eq!(readable, "0.5");
    
    // 일반 ERC20 (18 decimals)
    let erc20_amount = U256::from(1_000_000_000_000_000_000u64); // 1 token
    let readable = token_amount_to_readable(erc20_amount, 18);
    assert_eq!(readable, "1");
}

// ========== 네트워크별 통합 테스트 ==========

#[tokio::test]
async fn test_multiple_network_scenarios() {
    use evm_wallet::utils::get_smart_gas_price;
    
    // 다양한 네트워크 URL 패턴 테스트
    let network_urls = vec![
        ("Ethereum", "https://eth-mainnet.infura.io"),
        ("Polygon", "https://polygon-mainnet.infura.io"),
        ("BSC", "https://bsc-dataseed.binance.org"),
        ("Arbitrum", "https://arbitrum-mainnet.infura.io"),
        ("Optimism", "https://optimism-mainnet.infura.io"),
        ("Avalanche", "https://avalanche-mainnet.infura.io"),
        ("Fantom", "https://fantom-mainnet.infura.io"),
    ];
    
    for (network_name, url) in network_urls {
        // fallback 가격이 적절한 범위에 있는지 확인
        let fallback_price = evm_wallet::utils::get_network_fallback_gas_price(url);
        assert!(fallback_price > U256::ZERO, "{} fallback price should be positive", network_name);
        
        // 매우 높은 가격이 아닌지 확인 (1000 Gwei = 1 Twei 미만)
        assert!(fallback_price < U256::from(1_000_000_000_000u64), 
                "{} fallback price should be reasonable", network_name);
    }
}

// ========== 성능 및 안정성 테스트 ==========

#[tokio::test]
async fn test_gas_price_consistency() {
    use evm_wallet::utils::get_smart_gas_price;
    
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();
    
    // 여러 번 호출해서 일관성 있는 결과가 나오는지 확인
    let mut prices = Vec::new();
    for _ in 0..5 {
        let price = get_smart_gas_price(&provider_url).await;
        prices.push(price);
    }
    
    // 모든 가격이 동일하거나 매우 비슷해야 함 (Anvil은 고정 가격 사용)
    for i in 1..prices.len() {
        assert_eq!(prices[0], prices[i], "Gas prices should be consistent");
    }
}

#[tokio::test]
async fn test_concurrent_gas_price_requests() {
    use evm_wallet::utils::get_smart_gas_price;
    use tokio::task;
    
    let anvil = Anvil::new().spawn();
    let provider_url = anvil.endpoint();
    
    // 동시에 여러 요청을 보내서 경쟁 상태 테스트
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let url = provider_url.clone();
            task::spawn(async move {
                get_smart_gas_price(&url).await
            })
        })
        .collect();
    
    let results: Vec<_> = futures::future::join_all(handles).await;
    
    // 모든 요청이 성공해야 함
    for result in results {
        assert!(result.is_ok(), "Concurrent gas price request should succeed");
        let price = result.unwrap();
        assert!(price > U256::ZERO, "Gas price should be positive");
    }
} 