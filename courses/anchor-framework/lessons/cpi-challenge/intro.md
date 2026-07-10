# Challenge: Simulate a CPI Vault Transfer in Rust

Cross-Program Invocations (CPIs) let programs transfer SOL from PDA-controlled vaults. You'll simulate this pattern: derive a vault PDA, verify it has funds, and compute the transfer.

## Your Task

Implement `build_vault_transfer(user, vault_balance, recipient_balance, amount)` that:

1. Derives the vault PDA using `find_pda(&["vault", user], "system")` to get the bump
2. Checks the vault has enough balance for the transfer
3. Returns `Ok((new_vault_balance, new_recipient_balance, bump))` on success
4. Returns `Err("INSUFFICIENT_VAULT_BALANCE")` if vault can't cover the amount

**This mirrors the real Anchor pattern:**
```rust
invoke_signed(
    &transfer_instruction,
    &[vault_pda, recipient, system_program],
    &[&[b"vault", user.key().as_ref(), &[bump]]],
)?;
```

The bump is needed for `invoke_signed` — the program proves it controls the PDA.
