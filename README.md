## Overview
This repository contains the backend for a prediction market built on Solana using Rust + Anchor. It allows:

- Creating prediction markets with a title and multiple options
- Submitting votes on existing markets
- Storing market and vote data on-chain

## Structure

programs/arcium_prediction_market/src/lib.rs
tests/market_tests.rs
Anchor.toml
Cargo.toml


## Instructions
1. Start a local Solana validator:
```bash
solana-test-validator
