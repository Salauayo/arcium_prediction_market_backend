# Arcium Prediction Market Backend

This repository contains the backend implementation of a prediction market built on Solana using Rust and Anchor.

It already have a working Solana backend that creates markets and accepts votes.

##  Overview

This project implements a decentralized prediction market where users can:

* Create prediction markets with a title and multiple options
* Submit votes on existing markets
* Store all market and voting data on-chain

The backend is built using the Anchor framework, ensuring a structured and scalable Solana program.

---

##  Features

*  Market creation with multiple outcomes
*  On-chain state management
*  User voting system
*  Anchor-based architecture (IDL compatible)
*  Ready for frontend integration

---

##  Project Structure

programs/arcium_prediction_market/src/lib.rs → Main smart contract (Rust + Anchor)
tests/market_tests.rs → Basic test setup
Anchor.toml → Anchor configuration
Cargo.toml → Rust dependencies

---

##  Getting Started

### 1. Start local validator

```bash
solana-test-validator
```

### 2. Build the program

```bash
anchor build
```

### 3. Deploy the program

```bash
anchor deploy
```

### 4. Run tests (optional)

```bash
anchor test
```

---

##  Integration

This backend is designed to be easily integrated with a frontend using the Anchor IDL.

Frontend clients can interact with the program using:

* `createMarket` → to create new prediction markets
* `vote` → to submit votes

---

##  Notes

* Built as part of Arcium RTGS
* Focused on backend logic and on-chain functionality
* Can be extended with privacy layers (e.g., encrypted voting via Arcium)

---

##  Repository

https://github.com/Salauayo/arcium_prediction_market_backend
