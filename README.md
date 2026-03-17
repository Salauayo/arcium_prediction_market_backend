# Arcium Prediction Market Backend

![Language: Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust\&logoColor=white)
![Framework: Anchor](https://img.shields.io/badge/framework-Anchor-blue?logo=solana\&logoColor=white)
![Status: Ready](https://img.shields.io/badge/status-Ready-brightgreen)

This repository contains the backend implementation of a prediction market built on Solana using Rust and Anchor.

It already have a working Solana backend that creates markets and accepts votes.

##  Overview

This project implements a decentralized prediction market where users can:

* Create prediction markets with a title and multiple options
* Submit votes on existing markets
* Store all market and voting data on-chain

The backend is built using the Anchor framework and Arcium, ensuring **encrypted voting** and structured, scalable Solana programs.

---

##  Features

*  Market creation with multiple outcomes
*  On-chain state management
*  User voting system
*  Voting is encrypted using Arcium, ensuring privacy of all votes on-chain
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

### 0. Update system packages

```bash id="sysupdate"
sudo apt update && sudo apt upgrade -y
```

### 1. Install Solana CLI and test validator

```bash id="solinstall"
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

Reload terminal after installation:

```bash id="solpath"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

Check version:

```bash id="solversion"
solana --version
```

### 2. Navigate to project folder

```bash id="cdproj"
cd ~/arcium_prediction_market_backend
```

### 3. Start local Solana validator

```bash id="validator"
solana-test-validator
```

> ⚠️Note: `solana-test-validator` is part of Solana CLI. You can also create a local helper script if desired:
>
> ```bash
> #!/bin/bash
> solana-test-validator
> ```
>
> Save as `solana-test-validator.sh` and make executable:
>
> ```bash
> chmod +x solana-test-validator.sh
> ./solana-test-validator.sh
> ```

### 4. Build the program

```bash id="anchorbuild"
anchor build
```

### 5. Deploy the program

```bash id="anchordeploy"
anchor deploy
```

### 6. Run tests (optional)

```bash id="anchortest"
anchor test
```

---

##  Integration

This backend is designed to be easily integrated with a frontend using the Anchor IDL.

Frontend clients can interact with the program using:

* `createMarket` → to create new prediction markets
* `vote` → to submit votes (encrypted by Arcium)

---

##  Notes

* Built as part of Arcium RTGS
* Focused on backend logic and on-chain functionality
* Voting is encrypted using Arcium, ensuring privacy of all votes on-chain

---

##  Repository

https://github.com/Salauayo/arcium_prediction_market_backend
