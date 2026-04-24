# 💸 Child Support Auto-Payment Contract (Soroban)

## 📌 Project Description

This project is a Soroban smart contract built on the Stellar network that automates recurring child support payments between two parties.

It enables a payer to set up a fixed payment agreement where funds are automatically released to a recipient on a predefined monthly schedule. The contract enforces timing rules on-chain, ensuring consistency, transparency, and reduced reliance on manual processes.

---

## ⚙️ What It Does

- Allows a payer to configure a recurring payment agreement
- Stores payment details including recipient, amount, token, and schedule
- Tracks when the last payment was made
- Enables execution of payments only after the defined time interval
- Transfers tokens securely from payer to recipient when conditions are met

---

## ✨ Features

- 🔁 **Automated Recurring Payments**  
  Supports monthly (or custom interval) child support payments.

- ⏱️ **Time-Based Enforcement**  
  Prevents early withdrawals by enforcing a strict payment schedule.

- 🔐 **Secure Authorization**  
  Only the payer can initialize the contract.

- 💰 **Token Flexibility**  
  Works with any Soroban-compatible token (not limited to XLM).

- 📊 **On-Chain Transparency**  
  Payment history and configuration are stored on-chain.

- ⚡ **Permissionless Execution**  
  Anyone can trigger the payment function once it's due.

---

## 🚀 How It Works

1. The payer initializes the contract with:
   - Recipient address
   - Token address
   - Payment amount
   - Time interval (e.g., 30 days)

2. The contract stores the agreement on-chain.

3. After the interval passes:
   - Anyone can call the `pay()` function
   - The contract verifies the time condition
   - Payment is transferred to the recipient

---

## 🛠️ Example Use Case

A parent wants to ensure child support is paid monthly without relying on reminders or third parties. This contract enforces the agreement automatically and transparently on the blockchain.

---

## ⚠️ Disclaimer

This is a basic reference implementation. For production use, consider:

- Adding cancellation or update mechanisms
- Handling insufficient balance cases gracefully
- Supporting multiple payment agreements
- Conducting a professional smart contract audit

---

## 📦 Tech Stack

- Rust
- Soroban SDK
- Stellar Network

---

wallet address GBBE2Q2YACLFDAE33P2LOKURUHCPIUBLQM7LZIVP6UFOWPTK43TAF7H3

contract address CD5QHQEDUZWH6GZYCVNVG4DVIFPBDCOAUT3L6CREKTRUPXZZVOFN2PDX

<img width="1440" height="900" alt="image" src="https://github.com/user-attachments/assets/d5ccadb2-7872-4977-a663-9a88839a409a" />

