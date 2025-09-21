# Solana CPI with PDA Creation

This repository demonstrates **Cross Program Invocation (CPI)** in Solana for creating **Program Derived Accounts (PDAs)**.  
It includes a Rust program and TypeScript tests using [LiteSVM](https://github.com/anza-xyz/litesvm), so you don’t need to run a validator or deploy to devnet to run the tests.  

---

## 📂 Project Structure

```text
.
├── contract/      # Solana program (creates PDAs using CPI + seeds + bump)
└── test/          # Bun + LiteSVM client tests for the contract
```
### 🚀 Setup & Usage
#### 0. Clone the repository
```bash
git clone https://github.com/yashwankhade5/solana-cpi-pda.git
cd solana-cpi-pda


```


#### 1. Prerequisites

Make sure you have the following installed:

- Rust (with Solana SBF tools)

- Solana CLI

- Bun (for running tests)

#### 2. Build the program
   Compile the contract to generate its `.so` file:

```bash
cd contract
cargo build-sbf --release
```

This will produce:
```bash
 target/deploy/contract.so
 ```

#### 3. Run tests

Tests are written in TypeScript using Bun and LiteSVM.
```bash
cd test
bun install
bun test
```

This will spin up a local LiteSVM instance and run tests to verify that the PDA accounts are created successfully by the contract.

## 🛠️ Tech Stack

- Rust — Solana on-chain program

- Bun — Test runner

- LiteSVM — Local Solana runtime for fast testing

- TypeScript — Client + tests

##📖 Concepts Covered

- Cross Program Invocation (CPI)

- PDA creation with seeds + bump

- Using invoke_signed for PDA signing

- Writing local tests with Bun + LiteSVM