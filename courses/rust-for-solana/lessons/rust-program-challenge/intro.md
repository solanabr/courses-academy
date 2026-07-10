# Challenge: Process a Transfer Instruction

Simulate Solana's `process_instruction` pattern in pure Rust. You'll process a transfer by validating balances, deducting fees, and returning updated state.

## Your Task

Implement `process_transfer(sender_balance, recipient_balance, amount, fee)` that:

1. Checks the sender can afford `amount + fee`
2. If not, returns `Err("INSUFFICIENT_BALANCE")`
3. If yes, computes new balances and returns `Ok((new_sender, new_recipient))`

**Requirements:**
- Sender pays both the transfer amount AND the fee
- Recipient receives only the amount (not the fee)
- The fee is burned (deducted from sender, not added anywhere)
- Use checked arithmetic to prevent overflow

**This mirrors how Solana programs work:**
- `process_instruction` receives account balances
- Validates preconditions
- Mutates balances
- Returns success or error
