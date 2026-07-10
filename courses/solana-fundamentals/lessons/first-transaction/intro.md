# Your First Transaction

Let's send your first transaction on Solana devnet!

## Understanding Transactions

A Solana transaction consists of:
- **Instructions**: The operations to perform
- **Signers**: The accounts that must sign the transaction
- **Recent Blockhash**: Prevents duplicate transactions

## Sending SOL

```typescript
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';

async function sendSol() {
  // Connect to devnet
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  // Create sender keypair (in production, use wallet adapter)
  const sender = Keypair.generate();

  // Airdrop SOL to sender
  const airdropSig = await connection.requestAirdrop(
    sender.publicKey,
    2 * LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(airdropSig);

  // Create recipient
  const recipient = Keypair.generate();

  // Create transfer instruction
  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: sender.publicKey,
      toPubkey: recipient.publicKey,
      lamports: 0.5 * LAMPORTS_PER_SOL,
    })
  );

  // Send and confirm
  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [sender]
  );

  console.log('Transaction signature:', signature);
}
```

## Key Concepts

- **Lamports**: The smallest unit of SOL (1 SOL = 1,000,000,000 lamports)
- **Connection**: Connects your code to a Solana cluster
- **SystemProgram**: Built-in program for basic operations like transfers

## Viewing Your Transaction

After sending, view your transaction on [Solana Explorer](https://explorer.solana.com/?cluster=devnet).
