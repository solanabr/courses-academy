# Understanding Solana's Account Model

Solana's account model is fundamentally different from Ethereum's contract-based approach. Understanding this model is crucial for building efficient Solana programs.

## Everything is an Account

In Solana, **everything is an account**. Unlike Ethereum where contracts hold state, Solana separates code (programs) from data (accounts):

- **Programs**: Executable code (stateless)
- **Accounts**: Store data and SOL balance

```typescript
interface Account {
  publicKey: PublicKey;     // Account address
  owner: PublicKey;         // Program that owns this account
  lamports: number;         // SOL balance (1 SOL = 1B lamports)
  data: Uint8Array;         // Raw account data
  executable: boolean;      // Is this a program?
  rentEpoch: number;        // Rent collection tracking
}
```

## The Owner-Account Relationship

Every account has an **owner** (a program). Only the owner program can:
- Modify the account's data
- Deduct lamports from the account

For example:
- Your wallet is owned by the **System Program**
- Token accounts are owned by the **Token Program**
- Your custom program owns its data accounts

```typescript
const accountInfo = await connection.getAccountInfo(publicKey);
console.log('Owner:', accountInfo.owner.toBase58());
// System Program: 11111111111111111111111111111111
// Token Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
```

## Rent and Account Lifetime

Solana accounts must maintain a minimum SOL balance to remain active (called **rent exemption**). The required balance depends on the account's data size:

```typescript
const rentExemption = await connection.getMinimumBalanceForRentExemption(
  dataSize // bytes
);
```

**Best Practice**: Always create accounts with enough SOL to be rent-exempt. Rent-exempt accounts never expire.

## System Accounts vs Program Accounts

### System Accounts
- Owned by the System Program (`11111111111111111111111111111111`)
- Hold SOL but no custom data
- Your wallet is a system account

### Program Accounts
- Owned by custom programs
- Store structured data (game state, user profiles, DeFi positions, etc.)
- Created via CPI (Cross-Program Invocation) to the System Program

## Real-World Example: Jupiter Aggregator

Jupiter, the leading Solana DEX aggregator, uses accounts to store:
- User token accounts (owned by Token Program)
- Swap routing data (owned by Jupiter Program)
- Price feed data (owned by Pyth oracle program)

Each account has a specific owner and purpose, creating a composable ecosystem.

## Key Takeaways

1. Programs are stateless; accounts hold all data
2. Only the owner program can modify an account's data
3. Accounts must be rent-exempt to persist
4. The account model enables Solana's parallel processing
