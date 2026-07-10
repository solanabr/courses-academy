# Setting Up a Solana Wallet

A wallet is your gateway to the Solana ecosystem. It stores your private keys and lets you sign transactions.

## Phantom Wallet (Recommended)

1. Visit [phantom.app](https://phantom.app) and install the browser extension
2. Click "Create a new wallet"
3. **Write down your recovery phrase** and store it safely
4. Set a password for your wallet

## Understanding Keypairs

In Solana, a **keypair** consists of:
- **Public Key**: Your wallet address (safe to share)
- **Private Key**: Used to sign transactions (NEVER share this)

```typescript
import { Keypair } from '@solana/web3.js';

// Generate a new keypair
const keypair = Keypair.generate();

console.log('Public Key:', keypair.publicKey.toBase58());
// Example: 7C4jsPZpht42Tw6MjXWF56Q5RQUocjBBmciEjDa8HRtp
```

## Connecting to Devnet

Switch your Phantom wallet to Devnet:
1. Open Phantom settings
2. Go to "Developer Settings"
3. Change network to "Devnet"

## Getting Devnet SOL

You can get free devnet SOL from:
- Solana CLI: `solana airdrop 2`
- [Sol Faucet](https://faucet.solana.com)

Devnet SOL has no real value and is only for testing.
