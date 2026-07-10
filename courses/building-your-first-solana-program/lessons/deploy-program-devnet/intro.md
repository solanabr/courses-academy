# Deploy Your Program to Devnet

This is it — you're about to deploy your counter program to the Solana blockchain.

## The Deployment Process

Deploying a Solana program involves 4 steps:

1. **Compile**: Your Rust code → `.so` binary (already done by the build server)
2. **Create Buffer**: A temporary on-chain account to hold the binary during upload
3. **Upload Chunks**: The binary is uploaded in ~1000-byte chunks (200+ transactions for a typical program)
4. **Finalize**: Link the buffer to a new program account, making it executable

This follows the **BPF Loader Upgradeable** protocol — the standard way all Anchor programs are deployed.

## What You'll See

- **1 wallet popup** to create the buffer account
- **Batch signing** every ~30 chunks (to avoid blockhash expiry)
- **A real-time transaction log** showing each chunk being confirmed
- **Your Program ID** when deployment completes

## Instructions

1. Click **Build** to compile your counter program
2. When the build succeeds, click **Deploy to Devnet**
3. Approve the transactions in your wallet
4. Watch the deployment progress in real time!

> **If deployment fails**: Don't worry! The system saves your progress. Click **Resume** to pick up where you left off — you won't re-upload chunks that already succeeded.
