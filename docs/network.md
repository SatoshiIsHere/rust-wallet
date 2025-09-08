### 네트워크 없이 사용 (기본값)
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1"
  }'
```

### 특정 네트워크 지정해서 사용
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "ethereum"
  }'
```

## 네트워크 관리

### 1. 네트워크 추가하기

새로운 네트워크를 추가할 수 있습니다. 이더리움 메인넷을 추가하는 예시:

```bash
curl -X POST http://localhost:3000/networks/add \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ethereum",
    "rpc_url": "https://eth-mainnet.g.alchemy.com/v2/demo"
  }'
```

BSC 네트워크 추가:
```bash
curl -X POST http://localhost:3000/networks/add \
  -H "Content-Type: application/json" \
  -d '{
    "name": "bsc",
    "rpc_url": "https://bsc-dataseed1.binance.org"
  }'
```

### 2. 등록된 네트워크 목록 보기

현재 등록된 모든 네트워크를 확인할 수 있습니다:

```bash
curl http://localhost:3000/networks
```

응답 예시:
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

### 3. 네트워크 삭제하기

더 이상 사용하지 않는 네트워크를 삭제할 수 있습니다:

```bash
curl -X POST http://localhost:3000/networks/remove \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ethereum"
  }'
```

## 네트워크별 API 사용 예시

### 잔액 조회

**이더리움에서 ETH 잔액 확인:**
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "ethereum"
  }'
```

**BSC에서 BNB 잔액 확인:**
```bash
curl -X POST http://localhost:3000/balance/native \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x742d35Cc6634C0532925a3b8D4C0C8C0d1c0b5e1",
    "network": "bsc"
  }'
```

### 토큰 전송

**이더리움에서 ETH 전송:**
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

**BSC에서 ERC20 토큰 전송:**
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

### 트랜잭션 히스토리

**특정 네트워크에서 트랜잭션 내역 조회:**
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

## 지원하는 모든 API

다음 API들이 모두 `network` 파라미터를 지원합니다:

- **잔액 조회**: `/balance/native`, `/balance/erc20`
- **트랜잭션**: `/transaction/sendNative`, `/transaction/sendErc20`, `/transaction/estimateGas`
- **히스토리**: `/transaction/history`, `/transaction/history/all`, `/transaction/details`
- **이벤트**: `/events/erc20Transfers`

## 팁

1. **네트워크 이름은 자유롭게 설정** 가능합니다. `ethereum`, `bsc`, `my-custom-network` 등 원하는 이름을 사용하세요.

2. **RPC URL은 정확해야 합니다**. 잘못된 URL을 사용하면 API 호출이 실패합니다.

3. **기본 네트워크 변경**은 환경변수 `RPC_ENDPOINT`를 설정하면 됩니다.

4. **네트워크를 지정하지 않으면** 항상 기본 VERY 네트워크가 사용됩니다.

이제 여러 블록체인 네트워크를 자유롭게 사용할 수 있습니다! 🚀
