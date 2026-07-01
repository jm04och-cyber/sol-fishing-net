# SOL Fishing Net 🎣

A Solana smart contract program for collecting and managing SOL tokens from multiple users.

## Overview

This project implements a **token collector program** on Solana that:
- ✅ Receives SOL deposits from users
- ✅ Aggregates tokens into a contract-owned vault
- ✅ Tracks deposit amounts per user
- ✅ Supports withdrawals and distributions
- ✅ Enables automated SOL harvesting

## Architecture

```
User Wallets
    |
    v
[Deposit Instructions]
    |
    v
[Solana Program (Collector)]
    |
    v
[Vault Account] <-- Aggregated SOL
    |
    v
[Distribution/Staking]
```

## Quick Start

### Prerequisites
- Rust 1.70+
- Solana CLI
- Anchor CLI

### Install Dependencies
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)
npm install -g @project-serum/anchor-cli
```

### Build
```bash
cd programs/sol-collector
anchor build
```

### Test
```bash
anchor test
```

### Deploy
```bash
anchor deploy
```

## Project Structure

```
sol-fishing-net/
├── programs/
│   └── sol-collector/           # Main Solana program
│       ├── src/
│       │   └── lib.rs           # Program entry point
│       └── Cargo.toml
├── tests/                        # Integration tests
│   └── integration.ts
├── Anchor.toml                   # Anchor configuration
├── package.json
└── README.md
```

## Features

### 1. Deposit SOL
Users can deposit SOL into the vault:
```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>
```

### 2. Track Balances
Each user's contribution is recorded in a state account.

### 3. Withdraw SOL
Authorized withdrawals from the vault:
```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>
```

### 4. Admin Controls
- Vault management
- Pause/unpause deposits
- Emergency withdrawals

## Security Considerations

⚠️ **AUDIT REQUIRED BEFORE MAINNET DEPLOYMENT**

- ✅ Signer verification on all write operations
- ✅ Overflow/underflow checks
- ✅ Access control (admin roles)
- ✅ Rent-exempt account handling
- ✅ PDA (Program Derived Address) safety

## Development

### Running Tests
```bash
anchor test
```

### Local Network
```bash
solana-test-validator
```

### Viewing Logs
```bash
solana logs
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

MIT

## Resources

- [Anchor Documentation](https://project-serum.github.io/anchor/)
- [Solana Program Library](https://github.com/solana-labs/solana-program-library)
- [Solana Developer Docs](https://docs.solana.com/)
