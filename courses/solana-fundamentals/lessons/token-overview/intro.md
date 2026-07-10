# Understanding SPL Tokens

SPL Tokens are the token standard on Solana, similar to ERC-20 on Ethereum but with key differences.

## How SPL Tokens Work

On Solana, tokens are managed by the **Token Program**:

1. **Mint Account**: Defines the token (supply, decimals, mint authority)
2. **Token Account**: Holds tokens for a specific owner
3. **Associated Token Account (ATA)**: A deterministically derived token account for a given wallet and mint

```
Mint Account (Token Definition)
  ├── Token Account (User A's balance)
  ├── Token Account (User B's balance)
  └── Token Account (User C's balance)
```

## Key Differences from Ethereum

| Feature | Solana (SPL) | Ethereum (ERC-20) |
|---------|-------------|-------------------|
| Token balances | Separate accounts | Single contract |
| Creating tokens | Token Program | Deploy new contract |
| Account model | Account-based | Contract storage |
| Cost | ~0.002 SOL per account | Gas fees vary |

## The Token Program

```typescript
import { createMint } from '@solana/spl-token';

const mint = await createMint(
  connection,      // Connection
  payer,           // Fee payer
  mintAuthority,   // Who can mint new tokens
  freezeAuthority, // Who can freeze accounts (null = no one)
  9                // Decimals (9 = standard for SOL-like tokens)
);

console.log('Mint address:', mint.toBase58());
```

## Creating Token Accounts

```typescript
import { getOrCreateAssociatedTokenAccount } from '@solana/spl-token';

const tokenAccount = await getOrCreateAssociatedTokenAccount(
  connection,
  payer,
  mint,
  owner.publicKey
);
```

## Next Steps

In the next lesson, you will create your own SPL token on devnet!
