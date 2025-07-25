# EVM Wallet API Handlers

## 1. Health Check

**핸들러**: `health_check`  
**호출 설명**: 서버 상태 확인  
**호출 예시**:
```bash
GET /health
```
**응답 예시**:
```
EVM Wallet API is running!
```

## 2. Address From Private Key

**핸들러**: `address_from_private_key`  
**호출 설명**: 프라이빗 키로부터 주소 추출 (지갑주소 확인시에 사용하시면 됩니다)
**호출 예시**:
```bash
POST /wallet/getAddress
Content-Type: application/json

{
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
}
```
**응답 예시**:
```json
{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa"
}
```

## 3. Generate Mnemonic

**핸들러**: `generate_mnemonic`  
**호출 설명**: 24단어 니모닉 생성  (BIP-39기반 표준 니모닉 생성)
**호출 예시**:
```bash
POST /wallet/generateMnemonic
```
**응답 예시**:
```json
{
  "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident account accuse achieve acid acoustic acquire across act action actor actress actual"
}
```

## 4. Generate Mnemonic Custom

**핸들러**: `generate_mnemonic_with_words`  
**호출 설명**: 커스텀 단어 수로 니모닉 생성 (BIP-39기반 표준 니모닉생성 단어수를 조절가능합니다. 기본은 24자리)
**호출 예시**:
```bash
POST /wallet/generateMnemonicCustom
Content-Type: application/json

{
  "word_count": 12
}
```
**응답 예시**:
```json
{
  "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident"
}
```

## 5. Create Wallet From Mnemonic

**핸들러**: `create_wallet_from_mnemonic`  
**호출 설명**: 니모닉으로부터 지갑 생성  (생성된 니모닉으로부터 지갑을 생성하는로직입니다 니모닉 > 지갑)
**호출 예시**:
```bash
POST /wallet/fromMnemonic
Content-Type: application/json

{
  "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident account accuse achieve acid acoustic acquire across act action actor actress actual"
}
```
**응답 예시**:
```json
{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "public_key": "0x04abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident account accuse achieve acid acoustic acquire across act action actor actress actual"
}
```

## 6. Send Native Coin

**핸들러**: `send_native_coin`  
**호출 설명**: VERY 코인 전송  
**호출 예시**:
```bash
POST /transaction/sendNative
Content-Type: application/json

{
  "to": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "amount": "1",
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
}
```
**응답 예시**:
```json
{
  "hash": "0xabc123def456789abc123def456789abc123def456789abc123def456789abc123",
  "status": "confirmed"
}
```

## 7. Send ERC20 Token

**핸들러**: `send_erc20_token`  
**호출 설명**: ERC20 토큰 전송  (VERY에서 발행한 토큰을 전송하는 로직)
**호출 예시**:
```bash
POST /transaction/sendErc20
Content-Type: application/json

{
  "to": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "amount": "100",
  "token_address": "0xA0b86a33E6441f8C7f9d51e6B8ff0C6a2e4E5F2c",
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
}
```
**응답 예시**:
```json
{
  "hash": "0xdef456789abc123def456789abc123def456789abc123def456789abc123def456",
  "status": "confirmed"
}
```

## 8. Estimate Gas

**핸들러**: `estimate_gas`  
**호출 설명**: 가스 추정 및 비용 계산 (최종적으로는 total_fee를 확인하시면 되며 이건 "예측비용이기때문에" 실제로는 +/- 가 일부발생할 수 있습니다) 
**호출 예시**:
```bash
POST /transaction/estimateGas
Content-Type: application/json

{
  "to": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "amount": "1",
  "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
}
```
**응답 예시**:
```json
{
  "gas_limit": 21000,
  "gas_price": "1200000000",
  "total_fee": "25200000000000"
}
```

## 9. Get Transaction Details

**핸들러**: `get_transaction_details`  
**호출 설명**: 거래 상세 정보 조회  (트랙잭션이 정상실행되었는지 확인하는 방법)
**호출 예시**:
```bash
POST /transaction/details
Content-Type: application/json

{
  "tx_hash": "0xabc123def456789abc123def456789abc123def456789abc123def456789abc123"
}
```
**응답 예시**:
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

**핸들러**: `get_native_balance`  
**호출 설명**: VERY 코인 잔액 조회
**호출 예시**:
```bash
POST /balance/native
Content-Type: application/json

{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa"
}
```
**응답 예시**:
```json
{
  "balance": "98.991502669199115334"
}
```

## 11. Get ERC20 Balance

**핸들러**: `get_erc20_balance`  
**호출 설명**: ERC20 토큰 잔고 조회
**호출 예시**:
```bash
POST /balance/erc20
Content-Type: application/json

{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "token_address": "0xA0b86a33E6441f8C7f9d51e6B8ff0C6a2e4E5F2c"
}
```
**응답 예시**:
```json
{
  "balance": "1000.0"
}
```

## 12. Get Native Transaction History

**핸들러**: `get_native_transaction_history`  
**호출 설명**:  VERY 거래 내역 조회   
**호출 예시**:
```bash
POST /transaction/history
Content-Type: application/json

{
  "address": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa",
  "from_block": 12345600,
  "to_block": 12345700
}
```
**응답 예시**:
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

**핸들러**: `get_erc20_events`  
**호출 설명**: ERC20 전송 이벤트 조회  
**호출 예시**:
```bash
POST /events/erc20Transfers
Content-Type: application/json

{
  "token_address": "0xA0b86a33E6441f8C7f9d51e6B8ff0C6a2e4E5F2c",
  "from_block": 12345600,
  "to_block": 12345700,
  "address_filter": "0x742d35Cc6634C0532925a3b8C17F21E71d45aa"
}
```
**응답 예시**:
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

**핸들러**: `get_all_native_transaction_history`  
**호출 설명**: 전체 네이티브 거래 내역 조회 (from_block은 필수, to_block은 선택사항 - 값이 없으면 현재 최신 블록으로 설정)
**호출 예시**:
```bash
POST /transaction/history/all
Content-Type: application/json

{
  "from_block": 12345600,
  "to_block": 12345700
}
```
**응답 예시**:
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

**핸들러**: `get_current_block`  
**호출 설명**: 현재 최신 블록 번호 조회
**호출 예시**:
```bash
GET /block/current
```
**응답 예시**:
```json
{
  "current_block": 12345678
}
```
``` 