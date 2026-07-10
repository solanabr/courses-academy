# Airdrop & Fund Your Wallet

Before deploying your program, you need devnet SOL to pay for the on-chain storage (called "rent").

## How Much SOL Do You Need?

Deploying an Anchor program requires approximately **2 SOL** on devnet:
- **~1.4 SOL** for the buffer account (temporary storage during upload)
- **~0.1 SOL** for the program account
- **~0.3 SOL** for transaction fees (200+ transactions)

The good news: most of this rent is **reclaimable** after deployment.

## The Devnet Faucet

Solana provides a free faucet on devnet. You can request **2 SOL per airdrop**.

Use the widget below to fund your wallet. You'll need at least 4-5 SOL to comfortably deploy.

> **Tip**: The faucet can be slow during peak hours. If you see "faucet is busy," just wait 30-60 seconds and try again. This is normal — devnet is a shared resource.

## What is Rent?

On Solana, every account must maintain a minimum balance called **rent**. This prevents spam accounts from filling up validator storage.

The rent amount depends on the account's data size:

```
rent = base_rent + (data_size * rent_per_byte)
```

For a 200KB program binary, the buffer account rent is ~1.4 SOL.

Accounts that maintain the minimum balance are **rent-exempt** — they won't be garbage collected.
