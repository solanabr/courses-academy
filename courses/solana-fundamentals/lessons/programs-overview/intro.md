# Introduction to Solana Programs

Solana programs (often called "smart contracts" on other blockchains) are the executable code that runs on the Solana network. Understanding how they work is essential for building decentralized applications.

## Programs are Stateless

Unlike Ethereum smart contracts that store state internally, **Solana programs are completely stateless**. They only contain executable code. All state is stored in separate accounts.

```
┌─────────────────┐         ┌──────────────┐
│  Your Program   │ ────>   │ Account Data │
│  (Stateless)    │         │  (Stateful)  │
└─────────────────┘         └──────────────┘
```

This separation enables:
- **Parallel execution**: Multiple transactions can run simultaneously
- **Composability**: Programs can easily interact with each other
- **Upgradability**: Update code without migrating state

## Instruction Anatomy

Programs are invoked via **instructions**. Each instruction contains:

```typescript
interface Instruction {
  programId: PublicKey;           // Which program to call
  keys: AccountMeta[];            // Which accounts to pass
  data: Uint8Array;               // Instruction-specific data
}

interface AccountMeta {
  pubkey: PublicKey;              // Account address
  isSigner: boolean;              // Must sign transaction?
  isWritable: boolean;            // Program will modify?
}
```

### Example: Transfer Instruction

```typescript
import { SystemProgram, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';

const instruction = SystemProgram.transfer({
  fromPubkey: sender,             // Signer, writable
  toPubkey: recipient,            // Writable
  lamports: 0.5 * LAMPORTS_PER_SOL
});

console.log('Program ID:', instruction.programId.toBase58());
// 11111111111111111111111111111111 (System Program)
console.log('Keys:', instruction.keys);
// [{ pubkey: sender, isSigner: true, isWritable: true },
//  { pubkey: recipient, isSigner: false, isWritable: true }]
```

## Built-in Programs vs Custom Programs

### System Program
- **ID**: `11111111111111111111111111111111`
- **Purpose**: Account creation, SOL transfers, account assignment
- **Used by**: Every Solana transaction

### Token Program
- **ID**: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- **Purpose**: SPL token minting, transfers, account management
- **Used by**: All DeFi protocols, NFT platforms, token projects

### Custom Programs
- Deployed by developers
- Written in Rust, C, or C++ (compiled to BPF bytecode)
- Examples: Raydium AMM, Magic Eden marketplace, Jupiter aggregator

## The BPF Runtime (Sealevel)

Solana programs run in the **BPF (Berkeley Packet Filter)** virtual machine called **Sealevel**. Key features:

- **Deterministic**: Same input always produces same output
- **Metered**: Compute units prevent infinite loops
- **Sandboxed**: Programs cannot access system resources
- **Parallel**: Thousands of programs run simultaneously

```typescript
// Programs have a compute budget
const COMPUTE_UNITS_PER_INSTRUCTION = 200_000;
const MAX_COMPUTE_UNITS_PER_TX = 1_400_000;
```

## Cross-Program Invocation (CPI)

Programs can call other programs via **CPI**. This is how Solana achieves composability:

```rust
// Your program calling the Token Program (Rust example)
invoke(
    &transfer_instruction,
    &[
        token_account.clone(),
        destination.clone(),
        authority.clone(),
    ],
)?;
```

Real example: Jupiter aggregates multiple DEXs by making CPIs to Raydium, Orca, and other AMM programs in a single transaction.

## Next Steps

In the next challenge, you'll construct a custom instruction from scratch, understanding how programs receive and process data.
