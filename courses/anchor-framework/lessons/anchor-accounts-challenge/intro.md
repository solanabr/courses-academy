# Challenge: Simulate Anchor Account Validation in Rust

Anchor's `#[account]` constraints validate accounts automatically. Here you'll implement the same checks manually — this is exactly what Anchor generates under the hood.

## Your Task

Implement `validate_account(account_owner, expected_owner, is_signer, require_signer, is_writable, require_writable)` that performs Anchor-style validation:

1. Check if `account_owner == expected_owner` → if not, return `Err("IncorrectProgramId")`
2. Check if signer is required but missing → return `Err("MissingRequiredSignature")`
3. Check if writable is required but missing → return `Err("AccountNotWritable")`
4. If all pass, return `Ok("Valid")`

**This mirrors these Anchor constraints:**
- `#[account(owner = expected_owner)]` → ownership check
- `Signer<'info>` → signer check
- `#[account(mut)]` → writable check

**Check order matters** — validate in the order listed above (owner → signer → writable).
