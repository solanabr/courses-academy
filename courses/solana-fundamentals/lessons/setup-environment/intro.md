# Setting Up Your Development Environment

Before you can start building on Solana, you need to set up your development environment.

## Prerequisites

- **Node.js** (v18 or later)
- **Rust** (latest stable)
- **Git**

## Install the Solana CLI

```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

After installation, verify:

```bash
solana --version
```

## Configure Solana CLI for Devnet

```bash
solana config set --url devnet
```

## Create a Keypair

```bash
solana-keygen new --outfile ~/.config/solana/devnet.json
```

This creates a new keypair that you will use for development on devnet.

## Get Devnet SOL

```bash
solana airdrop 2
```

You can request up to 2 SOL per airdrop on devnet.

## Install Anchor Framework

Anchor is the most popular framework for Solana development:

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install latest
avm use latest
```

Verify the installation:

```bash
anchor --version
```

## Recommended IDE Setup

- **VS Code** with the following extensions:
  - rust-analyzer
  - Solana (by Solana Labs)
  - Better TOML
