# Solana RPC Methods

The Solana JSON-RPC API is how your frontend communicates with the blockchain. Understanding RPC methods is essential for reading on-chain data.

## What is JSON-RPC?

JSON-RPC is a lightweight remote procedure call protocol. You send a JSON request to a Solana validator node and get a JSON response:

```javascript
// Request
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBalance",
  "params": ["7C4jsPZpht42Tw6MjXWF56Q5RQUocjBBmciEjDa8HRtp"]
}

// Response
{
  "jsonrpc": "2.0",
  "result": { "value": 1000000000 },
  "id": 1
}
```

The `@solana/web3.js` library wraps these RPC calls in convenient JavaScript methods.

## The Connection Object

```typescript
import { Connection, clusterApiUrl } from '@solana/web3.js';

const connection = new Connection(
  clusterApiUrl('devnet'),
  'confirmed' // commitment level
);
```

**RPC Endpoints**:
- **Devnet**: `https://api.devnet.solana.com`
- **Mainnet**: `https://api.mainnet-beta.solana.com`
- **Custom**: Helius, QuickNode, Alchemy (faster, more reliable)

## Essential RPC Methods

### getBalance()

Get SOL balance in lamports:

```typescript
const balance = await connection.getBalance(publicKey);
console.log(`Balance: ${balance / LAMPORTS_PER_SOL} SOL`);
```

### getAccountInfo()

Get full account details:

```typescript
const accountInfo = await connection.getAccountInfo(publicKey);

if (accountInfo) {
  console.log('Owner:', accountInfo.owner.toBase58());
  console.log('Lamports:', accountInfo.lamports);
  console.log('Data:', accountInfo.data);
  console.log('Executable:', accountInfo.executable);
}
```

### getTransaction()

Fetch a transaction by signature:

```typescript
const tx = await connection.getTransaction(signature, {
  maxSupportedTransactionVersion: 0,
});

if (tx) {
  console.log('Slot:', tx.slot);
  console.log('Block Time:', new Date(tx.blockTime! * 1000));
  console.log('Fee:', tx.meta?.fee);
}
```

### getLatestBlockhash()

Required for constructing transactions:

```typescript
const { blockhash } = await connection.getLatestBlockhash();
transaction.recentBlockhash = blockhash;
```

## Commitment Levels

Commitment levels define how finalized a transaction must be:

| Level | Finalized? | Use Case |
|-------|-----------|----------|
| **processed** | No | Real-time updates (may be rolled back) |
| **confirmed** | Partially | Most dApp reads (balance checks, etc.) |
| **finalized** | Yes | High-value transactions, withdrawals |

```typescript
const balance = await connection.getBalance(
  publicKey,
  'confirmed' // commitment level
);
```

**Best Practice**: Use `'confirmed'` for UI updates, `'finalized'` for critical operations.

## Advanced: getProgramAccounts()

Fetch all accounts owned by a program:

```typescript
const TOKEN_PROGRAM_ID = new PublicKey(
  'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
);

const tokenAccounts = await connection.getProgramAccounts(
  TOKEN_PROGRAM_ID,
  {
    filters: [
      { dataSize: 165 }, // Token account size
      {
        memcmp: {
          offset: 32, // Owner field offset
          bytes: userPublicKey.toBase58(),
        },
      },
    ],
  }
);

console.log(`Found ${tokenAccounts.length} token accounts`);
```

## Rate Limits and Best Practices

Public RPC endpoints have strict rate limits:
- **Devnet**: ~100 requests/10 seconds
- **Mainnet**: ~40 requests/10 seconds

**Solutions**:
1. Use premium RPC providers (Helius, QuickNode)
2. Implement request caching (React Query, SWR)
3. Batch requests where possible
4. Use WebSocket subscriptions for real-time data

## WebSocket Subscriptions

For real-time updates, use subscriptions instead of polling:

```typescript
const subscriptionId = connection.onAccountChange(
  publicKey,
  (accountInfo) => {
    console.log('Account changed!', accountInfo);
  },
  'confirmed'
);

// Cleanup
connection.removeAccountChangeListener(subscriptionId);
```

## Next Steps

In the next challenge, you'll use `getBalance()` to build a multi-wallet balance checker.
