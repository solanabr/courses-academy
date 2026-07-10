# What You've Built

Congratulations! You've completed the full Solana developer lifecycle:

## Your Journey

1. **Write Code** — You wrote a counter program in Rust using the Anchor framework
2. **Compile** — The Superteam Academy Build Server compiled it to a Solana program binary
3. **Deploy** — You deployed it to Solana Devnet using the BPF Loader Upgradeable
4. **Interact** — You called instructions and watched on-chain state change in real time

## What You Learned

### Anchor Framework
- `#[program]` — defines your instruction handlers
- `#[derive(Accounts)]` — specifies account constraints
- `#[account]` — defines on-chain data structures
- `#[error_code]` — custom errors enforced by the runtime

### On-Chain Concepts
- **Accounts** are Solana's storage primitive (not key-value pairs)
- **Discriminators** identify account types (first 8 bytes)
- **Rent** prevents spam (accounts must maintain minimum balance)
- **Transactions** are batches of instructions signed by wallets

### Deployment Protocol
- Programs are uploaded in **~1000-byte chunks** via the BPF Loader
- A **buffer account** holds the binary during upload
- The **program account** is a pointer to the executable data
- Upgradeable programs can be updated by the upgrade authority

## What's Next?

- **DeFi on Solana** — Build a token vault with PDAs and CPIs
- **Anchor Framework Mastery** — Deep dive into testing, PDAs, and CPIs
- **Install locally** — Set up Anchor CLI on your machine for faster iteration

Your deployed program will continue to live on Devnet. You can interact with it anytime using the Solana CLI or any Anchor client.
