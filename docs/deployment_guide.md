# Deployment Guide - CryptoDiamonds & 4HODO Project  

## Introduction  
This document provides a step-by-step guide for deploying the smart contracts of **CryptoDiamonds & 4HODO Project** on **Solana Mainnet**. The process ensures correct contract execution, financial sustainability, and transparent interaction with the blockchain ecosystem.  

---

## 1️⃣ Setup Environment  

### Install Solana CLI  
Before deploying, ensure the Solana CLI is installed on your machine.  
```sh
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
solana --version
solana config set --url https://api.mainnet-beta.solana.com
solana-keygen new --outfile mainnet_wallet.json
solana airdrop 2 $(solana-keygen pubkey mainnet_wallet.json)
solana program deploy target/deploy/minting_contract.so
solana program deploy target/deploy/burning_contract.so
solana program deploy target/deploy/liquidity_contract.so
solana program deploy target/deploy/rewards_contract.so
solana program show
solana program invoke mint_classic_diamond --arg "1" --payer mainnet_wallet.json
solana program invoke burn_crypto_diamonds --arg "3" --payer mainnet_wallet.json
solana account-info liquidity_pool.json


