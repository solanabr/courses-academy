# Capstone: Complete Counter Program

This is the capstone challenge. Build the complete counter program with:
- `initialize` — create the counter PDA, set to 0
- `increment` — add 1
- `decrement` — subtract 1, with underflow protection

## Requirements

1. Add a `decrement` instruction that subtracts 1 from `counter.count`
2. Use `require!(counter.count > 0, ErrorCode::Underflow)` to prevent underflow
3. Add a `Decrement` accounts struct (same shape as `Increment` — PDA counter + user Signer)
4. Define a custom `ErrorCode` enum with an `Underflow` variant

## Custom Error Enum

Anchor uses `#[error_code]` to define program-specific errors:

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Cannot decrement below zero")]
    Underflow,
}
```

The `#[msg]` attribute provides the human-readable error message.

## This Is It

When this compiles, you've built a real Solana program from scratch using PDAs — the production-standard pattern. The next lesson covers deployment.
