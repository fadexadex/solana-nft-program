# NFT Staking Program

## Overview
This project demonstrates a **Non-Fungible Token (NFT) staking program** built on the **Solana blockchain** using the **Anchor framework**. The program enables users to stake their NFTs without transferring them out of their wallets (**non-custodial staking**). Users can earn rewards and interact with staking pools while retaining full custody of their assets.

---

## Description
The program provides a flexible and secure mechanism for users to stake NFTs and earn rewards. Unlike traditional custodial staking programs, this solution uses **delegated authority** to interact with NFTs, ensuring they remain in users' wallets during the staking process.

Key supported features include:
- **Token Minting**: Setting up a token mint for reward distribution.
- **Staking Pools**: Initializing and managing staking pools.
- **Airdrops**: Distributing tokens as rewards.
- **Staking and Unstaking**: Securely staking and unstaking NFTs.

The implementation leverages the **Solana Program Library (SPL)** and the **Anchor framework** for a streamlined development process.

---

## Getting Started

### Prerequisites
Ensure you have the following installed:
- **Rust** and the **Solana toolchain**
- **Node.js** and **npm/yarn**
- **Anchor Framework**
- A local or remote **Solana validator** (devnet or mainnet-beta)

---

### Installing
1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/nft-staking-program.git
   cd nft-staking-program ```

2. Build the program:
    ```bash
    anchor build
    ```
    
3. Deploy the program to your Solana cluster:
    ```bash
    anchor deploy
    ```
    
### Executing the Program
```bash
anchor test 
```
    
### Help
For common issues:
    Ensure the Solana validator is running locally or is accessible via the network.