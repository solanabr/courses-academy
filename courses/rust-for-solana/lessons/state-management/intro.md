# PDAs and State Management

Program Derived Addresses (PDAs) are one of Solana's most powerful features. They enable programs to "own" accounts and sign transactions programmatically, unlocking patterns impossible in other blockchains.

## What Are PDAs?

A PDA is a public key that:
1. **Has no private key**: It's derived from seeds, not generated randomly
2. **Is off the Ed25519 curve**: Cannot be signed by a regular keypair
3. **Can only be signed by the program** that derives it: Using `invoke_signed`

Think of PDAs as accounts controlled by code, not by a person holding a private key.

## Deriving PDAs

Use `Pubkey::find_program_address()` (or `findProgramAddressSync()` in JS) to derive a PDA:

```rust
let (pda, bump) = Pubkey::find_program_address(
    &[
        b"user",                    // Static seed
        user_pubkey.as_ref(),      // Dynamic seed (user's pubkey)
    ],
    program_id,
);
```

The function returns:
- **`pda`**: The derived public key
- **`bump`**: A single byte (0-255) that ensures the PDA is off-curve

### How It Works

Internally, Solana:
1. Concatenates the seeds + program_id
2. Hashes them with SHA-256
3. Checks if the result is on the Ed25519 curve
4. If yes, decrements the bump (starting at 255) and tries again
5. Returns the first off-curve result (the "canonical bump")

```rust
// Pseudocode
for bump in (0..=255).rev() {
    let hash = sha256(seeds + [bump] + program_id);
    if !is_on_curve(hash) {
        return (PublicKey(hash), bump);
    }
}
```

## Why PDAs?

### 1. Deterministic Account Addresses

You can calculate account addresses before they're created:

```rust
// Client-side: Calculate where the user's token account will be
let (token_account_pda, _) = Pubkey::find_program_address(
    &[b"token", user_pubkey.as_ref(), mint_pubkey.as_ref()],
    &token_program_id,
);

// No need to store this address—just recompute it when needed!
```

### 2. Program-Controlled Signing

Programs can sign transactions on behalf of PDAs using `invoke_signed`:

```rust
// Transfer tokens FROM a PDA (not from the user)
invoke_signed(
    &transfer_instruction,
    &[source_pda, destination, token_program],
    &[&[b"vault", user_pubkey.as_ref(), &[bump]]], // Signer seeds
)?;
```

This enables:
- **Escrow contracts**: Lock tokens until conditions are met
- **Vaults**: Store user funds under program control
- **Proxies**: Programs calling other programs

### 3. One Account Per User (No Collisions)

By including the user's pubkey in the seeds, each user gets a unique PDA:

```rust
let (user_account, _) = Pubkey::find_program_address(
    &[b"account", user_pubkey.as_ref()],
    program_id,
);
```

No two users will ever get the same PDA, even if millions of users exist.

## Canonical Bumps

**Always use the canonical bump** (the first one returned by `find_program_address`). Storing non-canonical bumps enables bump seed attacks where an attacker creates multiple PDAs with different bumps.

```rust
// Good: Store the canonical bump
let (pda, bump) = Pubkey::find_program_address(seeds, program_id);
account.bump = bump; // Save this for later

// Bad: Never recompute with a different bump
let pda = Pubkey::create_program_address(&[seeds, &[wrong_bump]], program_id)?;
```

## Common PDA Patterns

### Pattern 1: Global State (One PDA per Program)

```rust
let (config_pda, _) = Pubkey::find_program_address(
    &[b"config"],
    program_id,
);
```

### Pattern 2: User-Specific Accounts

```rust
let (user_account, _) = Pubkey::find_program_address(
    &[b"user", user_pubkey.as_ref()],
    program_id,
);
```

### Pattern 3: Associated Accounts (Multiple PDAs per User)

```rust
// User's stats PDA
let (stats_pda, _) = Pubkey::find_program_address(
    &[b"stats", user_pubkey.as_ref()],
    program_id,
);

// User's vault PDA
let (vault_pda, _) = Pubkey::find_program_address(
    &[b"vault", user_pubkey.as_ref()],
    program_id,
);
```

### Pattern 4: Indexed PDAs (Lists)

```rust
// 10th post by this user
let (post_pda, _) = Pubkey::find_program_address(
    &[b"post", user_pubkey.as_ref(), &10u64.to_le_bytes()],
    program_id,
);
```

## PDA Validation

Always verify that a passed PDA matches the expected derivation:

```rust
let expected_pda = Pubkey::create_program_address(
    &[b"user", user_pubkey.as_ref(), &[bump]],
    program_id,
)?;

if account.key != &expected_pda {
    return Err(ProgramError::InvalidAccountData);
}
```

This prevents attackers from passing malicious accounts.

## Next Challenge

You'll derive multiple PDAs using different seed patterns and verify they're off-curve using the `PublicKey.isOnCurve()` mock method!
