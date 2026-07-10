# Account Constraints Deep Dive

Anchor's account constraints are the guard rails of Solana development. They validate accounts before your instruction logic runs, preventing entire classes of bugs.

## Essential Constraints

### `#[account(init, payer = X, space = N)]`
Creates a new account. The `payer` funds the rent-exempt deposit. `space` sets the data size in bytes. Can only be used once per account.

### `#[account(mut)]`
Marks an account as mutable. Required for any account whose data or lamport balance changes. Forgetting `mut` gives you:
```
error: A mut constraint was expected but not found.
```

### `Signer<'info>`
The account must have signed the transaction. Used for authorization — proving the caller owns the account. Signers automatically have their signature verified by the runtime.

### `Program<'info, System>`
References a program account. `System` is the System Program (`11111111...`), which handles account creation and SOL transfers.

## Account Types

| Type | Use Case | Mutable Data? |
|------|----------|---------------|
| `Account<'info, T>` | Typed data account | Yes |
| `Signer<'info>` | Must have signed tx | No data |
| `SystemAccount<'info>` | Any system-owned account | No |
| `Program<'info, T>` | A program account | No |
| `UncheckedAccount<'info>` | No validation (dangerous) | Depends |

## Common Errors

### Missing `system_program`
If your instruction creates an account (`init`), you **must** include the System Program:
```rust
pub system_program: Program<'info, System>,
```
Without it, the compiler error is:
```
error[E0277]: the trait bound `Initialize<'_>: anchor_lang::Accounts<'_>` is not satisfied
```

This error is cryptic because Anchor generates the account validation code via macros. The fix is always: add the missing account.

### Missing `mut` on payer
The payer's balance decreases (pays rent), so it must be `#[account(mut)]`:
```rust
#[account(mut)]
pub user: Signer<'info>,
```

### Wrong `space` calculation
If `space` is too small, the account can't hold your data. If too large, the payer overpays for rent. Always calculate: `8 (discriminator) + field sizes`.

## In the Next Challenge

You'll encounter a deliberate compiler error — a program with a missing account. Your job is to read the error output and fix it.
