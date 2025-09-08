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
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "ethereum"
  }'
```

## Network Management

### 1. Adding Networks

You can add new networks. Here's an example of adding Ethereum mainnet:

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

**Check ETH balance on Ethereum:**
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "ethereum"
  }'
```

**Check BNB balance on BSC:**
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "bsc"
  }'
```

### Token Transfers

**Send ETH on Ethereum:**
```bash
curl -X POST http://localhost:3000/transaction/sendNative \
  -H "Content-Type: application/json" \
  -d '{
    "to": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "amount": 1.0,
    "private_key": "your_private_key_here",
    "network": "ethereum"
  }'
```

**Send ERC20 token on BSC:**
```bash
curl -X POST http://localhost:3000/transaction/sendErc20 \
  -H "Content-Type: application/json" \
  -d '{
    "to": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "amount": 1.0,
    "token_address": "0x55d398326f99059fF775485246999027B3197955",
    "private_key": "your_private_key_here",
    "network": "bsc"
  }'
```

### Transaction History

**Query transaction history on specific network:**
```bash
curl -X POST http://localhost:3000/transaction/history \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "from_block": 18000000,
    "to_block": 18001000,
    "network": "ethereum"
  }'
```

## Supported APIs

The following APIs all support the `network` parameter:

- **Balance Queries**: `/balance/native`, `/balance/erc20`
- **Transactions**: `/transaction/sendNative`, `/transaction/sendErc20`, `/transaction/estimateGas`
- **History**: `/transaction/history`, `/transaction/history/all`, `/transaction/details`
- **Events**: `/events/erc20Transfers`

## Tips

1. **Network names can be set freely**. Use any name you want like `ethereum`, `bsc`, `my-custom-network`, etc.

2. **RPC URLs must be accurate**. Using incorrect URLs will cause API calls to fail.

3. **To change the default network**, set the `RPC_ENDPOINT` environment variable.

4. **If no network is specified**, the default VERY network will always be used.

Now you can freely use multiple blockchain networks! 🚀
