# Challenge: Simulate a Transfer Test in Rust

Test-driven development is critical for Solana programs. You'll write a function that simulates a complete transfer test scenario — the same logic you'd verify with `anchor test`.

## Your Task

Implement `simulate_transfer_test(sender_initial, transfer_amount, fee)` that:

1. Checks if sender can afford `transfer_amount + fee`
2. If yes, computes final balances and returns `"PASS:sender=<final>,receiver=<final>"`
3. If no, returns `"FAIL:insufficient_funds"`

**Test constants:**
- Receiver starts with 0 balance
- Fee is paid by sender and burned (not added to receiver)
- The function simulates what an Anchor test's `assert` checks would verify

**This mirrors the Anchor test pattern:**
```typescript
it('transfers SOL correctly', async () => {
  const balanceBefore = await connection.getBalance(sender);
  await program.methods.transfer(amount).rpc();
  const balanceAfter = await connection.getBalance(sender);
  expect(balanceAfter).to.equal(balanceBefore - amount - fee);
});
```
