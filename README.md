# EVM Wallet API

A comprehensive Rust-based EVM wallet API that supports multiple blockchain networks with decimal amount transfers and fixed gas pricing.

## Features

- **Multi-Network Support**: Add and manage custom EVM networks
- **Decimal Amounts**: Send tokens with decimal precision (e.g., 0.1, 0.01 ETH)
- **Fixed Gas Pricing**: Consistent 1.2 Gwei gas price across all transactions
- **Wallet Management**: Generate mnemonics, create wallets, and manage private keys
- **Transaction History**: Query native and ERC20 transaction history
- **Balance Queries**: Check native and ERC20 token balances

## 1. Health Check

**Handler**: `health_check`  
**Description**: Check server status  
**Example**:
```bash
GET /health
```
**Response**:
```
EVM Wallet API is running!
```

## 2. Address From Private Key

**Handler**: `address_from_private_key`  
**Description**: Extract address from private key (use this to verify wallet address)
**Example**:
```bash
POST /wallet/getAddress
Content-Type: application/json

{
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
}
```
**Response**:
```json
{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa"
}
```

## 3. Generate Mnemonic

**Handler**: `generate_mnemonic`  
**Description**: Generate 24-word mnemonic (BIP-39 standard mnemonic generation)
**Example**:
```bash
POST /wallet/generateMnemonic
```
**Response**:
```json
{
  "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident account accuse achieve acid acoustic acquire across act action actor actress actual"
}
```

## 4. Generate Mnemonic Custom

**Handler**: `generate_mnemonic_with_words`  
**Description**: Generate mnemonic with custom word count (BIP-39 standard mnemonic generation with adjustable word count. Default is 24 words)
**Example**:
```bash
POST /wallet/generateMnemonicCustom
Content-Type: application/json

{
  "word_count": 12
}
```
**Response**:
```json
{
  "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident"
}
```

## 5. Create Wallet From Mnemonic

**Handler**: `create_wallet_from_mnemonic`  
**Description**: Create wallet from mnemonic (logic to create wallet from generated mnemonic: mnemonic > wallet)
**Example**:
```bash
POST /wallet/fromMnemonic
Content-Type: application/json

{
  "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident account accuse achieve acid acoustic acquire across act action actor actress actual"
}
```
**Response**:
```json
{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "public_key": "0x04abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident account accuse achieve acid acoustic acquire across act action actor actress actual"
}
```

## 6. Send Native Coin

**Handler**: `send_native_coin`  
**Description**: Native coin transfer (supports decimal amounts)  
**Example**:
```bash
POST /transaction/sendNative
Content-Type: application/json

{
  "to": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "amount": 1.0,
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "network": "ethereum"
}
```
**Response**:
```json
{
  "hash": "0xabc123def456789abc123def456789abc123def456789abc123def456789abc123",
  "status": "confirmed"
}
```

## 7. Send ERC20 Token

**Handler**: `send_erc20_token`  
**Description**: ERC20 token transfer (supports decimal amounts)  
**Example**:
```bash
POST /transaction/sendErc20
Content-Type: application/json

{
  "to": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "amount": 100.0,
  "token_address": "0xA0b86a33E6441f8C7f9d51e6B8ff0C6a2e4E5F2c",
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "network": "ethereum"
}
```
**Response**:
```json
{
  "hash": "0xdef456789abc123def456789abc123def456789abc123def456789abc123def456",
  "status": "confirmed"
}
```

## 8. Estimate Gas

**Handler**: `estimate_gas`  
**Description**: Gas estimation and cost calculation (fixed at 1.2 Gwei)  
**Example**:
```bash
POST /transaction/estimateGas
Content-Type: application/json

{
  "to": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "amount": 1.0,
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "network": "ethereum"
}
```
**Response**:
```json
{
  "gas_limit": 21000,
  "gas_price": "1200000000",
  "total_fee": "25200000000000"
}
```

## 9. Get Transaction Details

**Handler**: `get_transaction_details`  
**Description**: Get transaction details (check if transaction was successful)
**Example**:
```bash
POST /transaction/details
Content-Type: application/json

{
  "tx_hash": "0xabc123def456789abc123def456789abc123def456789abc123def456789abc123",
  "network": "ethereum"
}
```
**Response**:
```json
{
  "transaction": {
    "transaction_hash": "0xabc123def456789abc123def456789abc123def456789abc123def456789abc123",
    "block_number": 12345678,
    "from_address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
    "to_address": "0x8ba1f109551bD432803012645Hac136c61c45aa",
    "amount": "1000000000000000000",
    "gas_used": 21000,
    "gas_price": "1200000000",
    "transaction_index": 5,
    "timestamp": 1703123456
  }
}
```

## 10. Get Native Balance

**Handler**: `get_native_balance`  
**Description**: Get native coin balance
**Example**:
```bash
POST /balance/native
Content-Type: application/json

{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "network": "ethereum"
}
```
**Response**:
```json
{
  "balance": "98.991502669199115334"
}
```

## 11. Get ERC20 Balance

**Handler**: `get_erc20_balance`  
**Description**: Get ERC20 token balance
**Example**:
```bash
POST /balance/erc20
Content-Type: application/json

{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "token_address": "0xA0b86a33E6441f8C7f9d51e6B8ff0C6a2e4E5F2c",
  "network": "ethereum"
}
```
**Response**:
```json
{
  "balance": "1000.0"
}
```

## 12. Get Native Transaction History

**Handler**: `get_native_transaction_history`  
**Description**: Get native transaction history   
**Example**:
```bash
POST /transaction/history
Content-Type: application/json

{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "from_block": 12345600,
  "to_block": 12345700,
  "network": "ethereum"
}
```
**Response**:
```json
{
  "transactions": [
    {
      "transaction_hash": "0xabc123def456789abc123def456789abc123def456789abc123def456789abc123",
      "block_number": 12345678,
      "from_address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
      "to_address": "0x8ba1f109551bD432803012645Hac136c61c45aa",
      "amount": "1000000000000000000",
      "gas_used": 21000,
      "gas_price": "1200000000",
      "transaction_index": 5,
      "timestamp": 1703123456
    }
  ]
}
```

## 13. Get ERC20 Events

**Handler**: `get_erc20_events`  
**Description**: Get ERC20 transfer events  
**Example**:
```bash
POST /events/erc20Transfers
Content-Type: application/json

{
  "token_address": "0xA0b86a33E6441f8C7f9d51e6B8ff0C6a2e4E5F2c",
  "from_block": 12345600,
  "to_block": 12345700,
  "address_filter": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "network": "ethereum"
}
```
**Response**:
```json
{
  "events": [
    {
      "transaction_hash": "0xdef456789abc123def456789abc123def456789abc123def456789abc123def456",
      "block_number": 12345678,
      "from_address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
      "to_address": "0x8ba1f109551bD432803012645Hac136c61c45aa",
      "amount": "100000000000000000000",
      "log_index": 2
    }
  ]
}
```

## 14. Get All Native Transaction History

**Handler**: `get_all_native_transaction_history`  
**Description**: Get all native transaction history (from_block required, to_block optional - defaults to latest block)
**Example**:
```bash
POST /transaction/history/all
Content-Type: application/json

{
  "from_block": 12345600,
  "to_block": 12345700,
  "network": "ethereum"
}
```
**Response**:
```json
{
  "transactions": [
    {
      "transaction_hash": "0xabc123def456789abc123def456789abc123def456789abc123def456789abc123",
      "block_number": 12345678,
      "from_address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
      "to_address": "0x8ba1f109551bD432803012645Hac136c61c45aa",
      "amount": "1000000000000000000",
      "gas_used": 21000,
      "gas_price": "1200000000",
      "effective_gas_price": "1200000000",
      "transaction_fee": "252000000000000",
      "burnt_fees": "0",
      "gas_limit": 21000,
      "transaction_index": 5,
      "timestamp": 1703123456,
      "status": "success"
    }
  ]
}
```

## 15. Get Current Block

**Handler**: `get_current_block`  
**Description**: Get current latest block number
**Example**:
```bash
GET /block/current
```
**Response**:
```json
{
  "current_block": 12345678
}
```

## 16. Network Management

### Add Network
```bash
POST /networks/add
Content-Type: application/json

{
  "name": "ethereum",
  "rpc_url": "https://eth-mainnet.g.alchemy.com/v2/demo"
}
```

### List Networks
```bash
GET /networks
```

### Remove Network
```bash
POST /networks/remove
Content-Type: application/json

{
  "name": "ethereum"
}
```

## Environment Configuration

Create a `.env` file in the project root:

```env
# Default RPC endpoint (used when no network is specified)
RPC_ENDPOINT=https://rpc.verylabs.io

# Server port (default: 3000)
PORT=3000
```

## Key Features

- **Decimal Amount Support**: Use `1.0`, `0.1`, `0.01` instead of wei strings
- **Fixed Gas Price**: All transactions use 1.2 Gwei gas price
- **Multi-Network**: Add custom networks via API
- **Network Parameter**: All APIs support optional `network` parameter

## Quick Start

1. Clone the repository
2. Create `.env` file with your RPC endpoint
3. Run `cargo run`
4. Server starts on `http://localhost:3000`

For detailed network management, see [docs/network.md](docs/network.md).
``` 