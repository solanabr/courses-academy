# Challenge: Build a Transaction Validator in Rust

Apply your error handling knowledge using Rust's `Result<T, E>` type. You'll validate a transfer by checking multiple conditions and returning typed errors — the same pattern used in real Solana programs.

## Your Task

Implement `validate_transfer(sender, recipient, amount, sender_balance)` that:

1. Returns `Err("INVALID_RECIPIENT")` if recipient is empty
2. Returns `Err("ZERO_AMOUNT")` if amount is 0
3. Returns `Err("INSUFFICIENT_BALANCE")` if sender_balance < amount
4. Returns `Err("SAME_ACCOUNT")` if sender equals recipient
5. Returns `Ok(())` if all validations pass

**Validation Order**: Check in the order listed above (return the first error).

**Rust Concepts Used:**
- `Result<(), String>` return type
- Early returns with `return Err(...)`
- String comparison with `==`
- The `?` operator pattern (for learning)
