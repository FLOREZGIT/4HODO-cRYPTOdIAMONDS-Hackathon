# API Endpoints - CryptoDiamonds & 4HODO Project  

## Introduction  
This document describes the **API endpoints and interaction mechanisms** available within the smart contracts of **CryptoDiamonds & 4HODO Project**. These endpoints enable **NFT minting, liquidity tracking, reward distribution, and transaction validation**, ensuring seamless integration with external applications.  

---

## 1️⃣ Minting API  

### Endpoint: `mint_classic_diamond`  
**Purpose:** Allows users to mint a Classic CryptoDiamond and assigns the correct valuation based on the mining stage.  
**Contract:** `minting_contract.rs`  

#### Request Format  
```json
{
  "stage": 3,
  "wallet_address": "SOLANA_USER_WALLET"
}{

  "status": "Success",
  "minted_diamond_id": "CD-003",
  "price": "0.96 USD",
  "liquidity_contribution": "0.50 USD",
  "distribution": "0.25 USD",
  "tax": "0.21 USD"
}
{
  "stage": 5,
  "burn_amount": 214000
}
{
  "status": "Success",
  "burned_diamonds": 214000,
  "remaining_supply": 530000,
  "timestamp": "2025-05-12T21:57:00Z"
}
{
  "query": "liquidity_balance",
  "wallet_address": "LIQUIDITY_WALLET"
}
{
  "status": "Success",
  "total_liquidity": "1,200,000 USD",
  "last_update": "2025-05-12T21:57:00Z"
}
{
  "query": "reward_history",
  "wallet_address": "REWARD_POOL_WALLET"
}
{
  "status": "Success",
  "total_rewards_paid": "250,000 USD",
  "eligible_users": 1400,
  "current_cycle": "Stage 4"
}

