# Challenge: Multi-Wallet Balance Checker

Build a function that checks the SOL balance of multiple wallets and returns formatted results.

## Requirements

- Create an async function that accepts a `Connection` and an array of `PublicKey` objects
- For each wallet, fetch the balance using `connection.getBalance()`
- Convert lamports to SOL (divide by `LAMPORTS_PER_SOL`)
- Return an array of objects with `publicKey` (base58 string) and `balanceInSol` (number, rounded to 4 decimals)
- Use `Promise.all()` to fetch all balances in parallel
