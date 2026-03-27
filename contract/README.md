# 🧠 DAO Treasury (Soroban Smart Contract)

## 📌 Project Description
DAO Treasury is a simple decentralized treasury management smart contract built using Soroban on the Stellar blockchain. It provides a foundational structure for managing shared funds in a decentralized organization.

This contract is designed as a minimal prototype to demonstrate how DAOs can securely manage deposits and withdrawals using on-chain logic.

---

## ⚙️ What it does
- Initializes a DAO treasury with an admin address
- Allows users to deposit funds into the treasury
- Allows only the admin to withdraw funds
- Keeps track of the total treasury balance on-chain

---

## ✨ Features
- 🔐 Admin-controlled withdrawals
- 💰 Deposit tracking system
- 📊 On-chain balance storage
- ⚡ Lightweight and gas-efficient design
- 🧱 Built with Soroban (Stellar smart contracts)

---

## 🚀 Future Improvements
- Multi-signature governance
- Proposal & voting system
- Token-based DAO membership
- Timelocked withdrawals
- Event logging

---

## 🛠 Tech Stack
- Rust
- Soroban SDK
- Stellar Blockchain

---

## 📦 How to Use
1. Deploy contract to Soroban network
2. Call `init(admin_address)`
3. Users call `deposit()`
4. Admin can call `withdraw()`
5. Query balance using `get_balance()`

---

## 📜 License
MIT License
