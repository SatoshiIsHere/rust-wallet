# Network Management Guide

## Using without Network (Default)
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1"
  }'
```

## Using with Specific Network

### Method 1: Using Network Name (Registered Networks)
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "ethereum"
  }'
```

### Method 2: Using Direct RPC URL (No Registration Required)
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "https://eth-mainnet.g.alchemy.com/v2/demo"
  }'
```

## Network Management

### Two Ways to Use Networks

1. **Direct RPC URL** (Recommended for one-time use): Simply put the RPC URL directly in the `network` field
2. **Registered Network Name** (Recommended for repeated use): Register a network with a name and use the name

### 1. Adding Networks (Optional - for repeated use)

You can add new networks for convenience. Here's an example of adding Ethereum mainnet:

```bash
curl -X POST http://localhost:3000/networks/add \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ethereum",
    "rpc_url": "https://eth-mainnet.g.alchemy.com/v2/demo"
  }'
```

Adding BSC network:
```bash
curl -X POST http://localhost:3000/networks/add \
  -H "Content-Type: application/json" \
  -d '{
    "name": "bsc",
    "rpc_url": "https://bsc-dataseed1.binance.org"
  }'
```

### 2. Viewing Registered Networks

You can check all currently registered networks:

```bash
curl http://localhost:3000/networks
```

Response example:
```json
{
  "networks": [
    {
      "name": "ethereum",
      "rpc_url": "https://eth-mainnet.g.alchemy.com/v2/demo"
    },
    {
      "name": "bsc",
      "rpc_url": "https://bsc-dataseed1.binance.org"
    }
  ]
}
```

### 3. Removing Networks

You can delete networks that are no longer needed:

```bash
curl -X POST http://localhost:3000/networks/remove \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ethereum"
  }'
```

## Network-Specific API Usage Examples

### Balance Queries

**Check ETH balance on Ethereum (using registered network name):**
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "ethereum"
  }'
```

**Check ETH balance on Ethereum (using direct RPC URL):**
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "https://eth-mainnet.g.alchemy.com/v2/demo"
  }'
```

**Check BNB balance on BSC (using direct RPC URL):**
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "https://bsc-dataseed1.binance.org"
  }'
```

### Token Transfers

**Send ETH on Ethereum (using direct RPC URL):**
```bash
curl -X POST http://localhost:3000/transaction/sendNative \
  -H "Content-Type: application/json" \
  -d '{
    "to": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "amount": 1.0,
    "private_key": "your_private_key_here",
    "network": "https://eth-mainnet.g.alchemy.com/v2/demo"
  }'
```

**Send ERC20 token on BSC (using direct RPC URL):**
```bash
curl -X POST http://localhost:3000/transaction/sendErc20 \
  -H "Content-Type: application/json" \
  -d '{
    "to": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "amount": 1.0,
    "token_address": "0x55d398326f99059fF775485246999027B3197955",
    "private_key": "your_private_key_here",
    "network": "https://bsc-dataseed1.binance.org"
  }'
```

### Transaction History

**Query transaction history on specific network (using direct RPC URL):**
```bash
curl -X POST http://localhost:3000/transaction/history \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "from_block": 18000000,
    "to_block": 18001000,
    "network": "https://eth-mainnet.g.alchemy.com/v2/demo"
  }'
```

## Supported APIs

The following APIs all support the `network` parameter:

- **Balance Queries**: `/balance/native`, `/balance/erc20`
- **Transactions**: `/transaction/sendNative`, `/transaction/sendErc20`, `/transaction/estimateGas`
- **History**: `/transaction/history`, `/transaction/history/all`, `/transaction/details`
- **Events**: `/events/erc20Transfers`

## Tips

1. **Direct RPC URLs**: You can now use RPC URLs directly in the `network` field without registration. Just put the full URL like `https://eth-mainnet.g.alchemy.com/v2/demo`.

2. **Network names**: You can still register networks with custom names for convenience if you use them frequently.

3. **RPC URLs must be accurate**. Using incorrect URLs will cause API calls to fail.

4. **To change the default network**, set the `RPC_ENDPOINT` environment variable.

5. **If no network is specified**, the default VERY network will always be used.

6. **URL Detection**: The system automatically detects if the `network` field contains a URL (starts with `http://` or `https://`) and uses it directly, otherwise it looks up registered network names.

Now you can freely use multiple blockchain networks without the hassle of registration! 🚀
