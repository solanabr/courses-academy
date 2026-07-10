# Challenge: Build a Solana Account Struct

Apply what you've learned about Rust ownership, structs, and methods. You'll create a `SolanaAccount` struct that models an on-chain account with balance management.

## Your Task

Complete the `SolanaAccount` struct and its methods, then implement `create_and_withdraw` which:
1. Creates a new `SolanaAccount` using `SolanaAccount::new(owner, initial_balance)`
2. Attempts to withdraw `withdraw_amount`
3. Returns a `String` describing the result

**Requirements:**
- `new()` creates an account that is **not frozen** by default
- `withdraw()` returns `Err("Account is frozen")` if the account is frozen
- `withdraw()` returns `Err("Insufficient balance")` if `amount > balance`
- `withdraw()` subtracts the amount and returns `Ok(remaining_balance)`
- `create_and_withdraw` formats the result as `"OK:<remaining>"` or `"ERR:<message>"`

**Rust Concepts Used:**
- Structs with `impl` blocks
- `Result<T, E>` return type
- Pattern matching with `match`
- String formatting with `format!()`
