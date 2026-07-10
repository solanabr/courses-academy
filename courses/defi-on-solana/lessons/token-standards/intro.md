# Solana Token Standards

Solana has two token programs: the original **SPL Token** and the newer **Token-2022** with advanced extensions.

## SPL Token Program

The original token standard (still used by 99% of tokens):

```
Program ID: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
```

### Account Structure

**Mint Account** (defines the token):
```
- Mint Authority: Can mint new tokens
- Supply: Total tokens in circulation
- Decimals: Number of decimal places (9 for SOL-like)
- Freeze Authority: Can freeze token accounts
```

**Token Account** (holds tokens for an owner):
```
- Mint: Which token this account holds
- Owner: Who owns these tokens
- Amount: Token balance
- Delegate: Optional account that can spend on behalf of owner
- State: Normal, Frozen, or Uninitialized
```

### Associated Token Account (ATA)

A deterministic token account address:

```typescript
import { getAssociatedTokenAddress } from '@solana/spl-token';

const ata = await getAssociatedTokenAddress(
  mint,         // Token mint address
  owner,        // Wallet address
);

// Derivation: PDA with seeds [owner, TOKEN_PROGRAM_ID, mint]
```

ATAs simplify token transfers (one account per mint per wallet).

## Token-2022 (Token Extensions)

The new token program with powerful extensions:

```
Program ID: TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

### Why Token-2022?

- **Backwards compatible** with SPL Token
- **Extensions** add features without changing core program
- **Future-proof** (can add new extensions)

### Available Extensions

#### 1. Transfer Hooks

Execute custom logic on every transfer:

```rust
// Example: Charge a fee on every transfer
pub fn transfer_hook(
    ctx: Context<TransferHook>,
    amount: u64,
) -> Result<()> {
    let fee = amount / 100;  // 1% fee
    // Deduct fee, send to treasury
    Ok(())
}
```

Use cases:
- Transaction fees
- Blacklist/whitelist enforcement
- Royalties on token transfers

#### 2. Confidential Transfers

Privacy via zero-knowledge proofs:

```
Normal transfer: "Alice sent 100 USDC to Bob" (public)
Confidential transfer: "Alice sent ??? USDC to Bob" (amount hidden)
```

Balances are encrypted, but the network verifies the math is correct.

#### 3. Transfer Fees

Built-in transfer fee without hooks:

```
Transfer 1000 tokens with 1% fee:
Recipient receives: 990 tokens
Fee account receives: 10 tokens
```

#### 4. Interest-Bearing Tokens

Tokens that accrue interest:

```
Rate: 5% APY
You hold: 1000 tokens
After 1 year: Balance shows 1050 tokens (automatically)
```

Implemented via an exchange rate that updates over time.

#### 5. Non-Transferable Tokens

Soulbound tokens (can't be transferred):

```
Use cases:
- Credentials (diplomas, certificates)
- Achievements (non-tradeable NFTs)
- Identity tokens
```

#### 6. Permanent Delegate

A delegate that can't be revoked:

```
Use case: Staking program that needs permanent control
```

#### 7. Metadata Pointer

Store metadata on-chain (no external JSON):

```json
{
  "name": "My Token",
  "symbol": "MTK",
  "uri": "https://example.com/metadata.json"
}
```

Metaplex Token Metadata is still more common, but metadata pointer is simpler.

### Creating a Token-2022 Mint

```typescript
import { createMint } from '@solana/spl-token';
import { TOKEN_2022_PROGRAM_ID } from '@solana/spl-token';

const mint = await createMint(
  connection,
  payer,
  mintAuthority,
  freezeAuthority,
  decimals,
  undefined,  // keypair (optional)
  undefined,  // confirm options
  TOKEN_2022_PROGRAM_ID  // Use Token-2022 instead of SPL Token
);
```

### Adding Extensions

```typescript
import {
  createInitializeTransferFeeConfigInstruction,
  createInitializeMintInstruction,
} from '@solana/spl-token';

// 1. Calculate space needed
const extensions = [ExtensionType.TransferFeeConfig];
const mintLen = getMintLen(extensions);

// 2. Create account
const createAccountIx = SystemProgram.createAccount({
  fromPubkey: payer.publicKey,
  newAccountPubkey: mint.publicKey,
  space: mintLen,
  lamports: await connection.getMinimumBalanceForRentExemption(mintLen),
  programId: TOKEN_2022_PROGRAM_ID,
});

// 3. Initialize extension
const initTransferFeeIx = createInitializeTransferFeeConfigInstruction(
  mint.publicKey,
  feeAuthority.publicKey,
  withdrawAuthority.publicKey,
  feeBasisPoints,  // 100 = 1%
  maxFee,
  TOKEN_2022_PROGRAM_ID
);

// 4. Initialize mint
const initMintIx = createInitializeMintInstruction(
  mint.publicKey,
  decimals,
  mintAuthority.publicKey,
  freezeAuthority.publicKey,
  TOKEN_2022_PROGRAM_ID
);

// Send transaction with all 3 instructions
```

## Token Metadata (Metaplex)

Metaplex Token Metadata is the standard for NFT metadata:

```
Metadata Account (PDA):
- Name: "Solana Monkey #1234"
- Symbol: "SMB"
- URI: "https://arweave.net/..."
- Creators: [{ address, verified, share }]
- Seller Fee Basis Points: 500 (5% royalty)
- Primary Sale Happened: true/false
```

The URI points to JSON:

```json
{
  "name": "Solana Monkey #1234",
  "description": "A cool monkey",
  "image": "https://arweave.net/image.png",
  "attributes": [
    { "trait_type": "Background", "value": "Blue" },
    { "trait_type": "Hat", "value": "Crown" }
  ]
}
```

## Choosing the Right Standard

| Feature | SPL Token | Token-2022 |
|---------|-----------|------------|
| Mature ecosystem | ✅ | ❌ (newer) |
| DEX support | ✅ All DEXs | ⚠️ Limited |
| Transfer fees | ❌ | ✅ |
| Transfer hooks | ❌ | ✅ |
| Confidential transfers | ❌ | ✅ |
| Interest-bearing | ❌ | ✅ |

**Use SPL Token** for:
- General purpose tokens
- Maximum compatibility
- Production DeFi

**Use Token-2022** for:
- Compliance requirements (transfer hooks for KYC)
- Privacy (confidential transfers)
- Novel token mechanics (interest-bearing, non-transferable)

## Token Account Rent

Token accounts require rent (~0.002 SOL):

```
Create 100 token accounts = 0.2 SOL locked
```

You can close accounts to reclaim rent:

```typescript
import { closeAccount } from '@solana/spl-token';

await closeAccount(
  connection,
  payer,
  tokenAccount,
  destination,  // Where to send reclaimed SOL
  owner
);
```

## Best Practices

1. **Use ATAs** for user-facing wallets (simplifies UX)
2. **Close unused accounts** to reclaim rent
3. **Validate token mint** before accepting transfers (prevent spam)
4. **Check program ID** (SPL Token vs Token-2022 vs fake programs)
5. **Handle decimals** correctly (9 decimals = 1.0 = 1_000_000_000 base units)

## Next Challenge

You'll implement token math functions for handling decimals, price calculations, and LP token formulas.
