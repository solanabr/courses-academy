# Program Derived Addresses (PDAs) Deep Dive

Program Derived Addresses are one of Solana's most powerful and unique features. They enable programs to "own" accounts and sign transactions without needing a private key.

## What Are PDAs?

A PDA is a public key that:
1. Is derived deterministically from seeds and a program ID
2. Is **not** on the Ed25519 elliptic curve (no corresponding private key exists)
3. Can only be "signed for" by the program that derived it

```rust
// In Anchor
let (pda, bump) = Pubkey::find_program_address(
    &[b"user", user.key().as_ref()],
    program_id
);
```

## Why PDAs Are Off-Curve

Solana uses Ed25519 for signatures. Ed25519 keys lie on an elliptic curve. PDAs are specifically designed to fall **off** this curve, which means:
- No private key can exist for a PDA
- Only the deriving program can sign for the PDA (via CPI with `invoke_signed`)

This creates **program-controlled accounts** that can't be compromised by key theft.

## PDA Derivation Process

```
seeds = [b"user", user_pubkey.as_ref()]
program_id = your_program_id

for bump in 255..=0 {
    candidate = sha256(seeds + [bump] + program_id)
    if !is_on_curve(candidate) {
        return (candidate, bump)  // This is your PDA
    }
}
```

The **canonical bump** is the first bump (starting from 255) that produces an off-curve address.

## Seeds Design Patterns

### Pattern 1: User-Scoped Account

```rust
seeds = [b"user", user.key().as_ref()]
```

One account per user. Used for user profiles, settings, etc.

### Pattern 2: User + Resource

```rust
seeds = [b"escrow", user.key().as_ref(), trade_id.to_le_bytes().as_ref()]
```

Multiple accounts per user, one per resource (escrow, order, etc.).

### Pattern 3: Global Singleton

```rust
seeds = [b"config"]
```

One global account for the program (config, authority, etc.).

### Pattern 4: Nested PDAs

```rust
// User stats PDA
seeds_stats = [b"stats", user.key().as_ref()]

// User vault PDA (derived from stats PDA)
seeds_vault = [b"vault", stats_pda.key().as_ref()]
```

PDAs can be derived from other PDAs for complex hierarchies.

## Storing the Bump

Always store the canonical bump in your account data:

```rust
#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub bump: u8,  // Store this!
    pub data: u64,
}
```

Why? Because recalculating the bump on every instruction is expensive (requires up to 256 hash operations). Storing it makes subsequent operations cheaper.

## PDAs as Signers

PDAs can sign CPIs using `invoke_signed`:

```rust
invoke_signed(
    &transfer_instruction,
    &[
        pda_account.to_account_info(),
        recipient.to_account_info(),
        system_program.to_account_info(),
    ],
    &[&[b"user", user.key().as_ref(), &[bump]]], // Signer seeds
)?;
```

The program derives the PDA, verifies it matches, and "signs" the CPI.

## Security Considerations

### 1. Seed Collision

Be careful with seed design:

```rust
// BAD: Can collide if strings contain delimiters
seeds = [user_name.as_bytes()]  // "alice" and "ali" + "ce" collide!

// GOOD: Use fixed-size data or well-defined separators
seeds = [b"user", user.key().as_ref()]
```

### 2. Bump Validation

Always use the canonical bump:

```rust
// In Anchor, this is automatic:
#[account(
    seeds = [b"user", authority.key().as_ref()],
    bump = user.bump  // Validates stored bump is canonical
)]
```

## PDA vs Regular Account

| Feature | Regular Account | PDA |
|---------|----------------|-----|
| Has private key | Yes | No |
| Can self-sign | Yes | No (requires program) |
| Can hold SOL | Yes | Yes |
| Can own other accounts | Yes | Yes |
| Deterministic address | No | Yes |
| On Ed25519 curve | Yes | No |

## Real-World Examples

**Token vaults** (escrow programs):
```rust
seeds = [b"vault", escrow.key().as_ref()]
```

**Associated Token Accounts**:
```rust
seeds = [wallet.key(), token_program.key(), mint.key()]
```

**Governance proposal accounts**:
```rust
seeds = [b"proposal", governance.key(), proposal_id.to_le_bytes()]
```

## Next Challenge

You'll derive multiple related PDAs from a common base to understand PDA hierarchies.
