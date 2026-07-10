# Adding Instructions

A program with just `initialize` isn't very useful. Real programs have multiple instructions that modify state.

## The Pattern

Every new instruction follows the same pattern:

1. Add a `pub fn` inside `#[program]`
2. Create a corresponding accounts struct with `#[derive(Accounts)]`
3. Define which accounts the instruction needs and their constraints

## Increment: Modifying State

To increment a counter, we need:

```rust
pub fn increment(ctx: Context<Increment>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.count += 1;
    Ok(())
}
```

The `Increment` accounts struct needs the counter PDA and the user (to derive the PDA seeds):

```rust
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}
```

Note: `mut` is required because we're modifying `count`. The `seeds` and `bump` tell Anchor to verify the PDA address matches. The `user` provides the wallet pubkey used in the seeds.

## Decrement with Safety

What happens if someone tries to decrement a counter that's already at 0? With `u64`, subtraction would underflow and panic.

Anchor provides `require!` for safe checks:

```rust
pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    require!(counter.count > 0, ErrorCode::Underflow);
    counter.count -= 1;
    Ok(())
}
```

## Custom Errors

The `ErrorCode::Underflow` needs to be defined:

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Cannot decrement below zero")]
    Underflow,
}
```

`#[error_code]` generates Anchor-compatible error types. The `#[msg]` attribute provides a human-readable error message visible in transaction logs.

## Coming Up

You'll build the increment instruction first, then combine everything into the complete counter program with error handling.
