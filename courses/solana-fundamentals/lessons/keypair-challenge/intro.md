# Challenge: Generate a Keypair

Now it is your turn! Write a function that generates a new Solana keypair and returns an object with the public key (as a base58 string) and a boolean indicating whether the keypair is valid.

## Requirements

- Import `Keypair` from `@solana/web3.js`
- Generate a new keypair
- Return an object with `publicKey` (base58 string) and `isValid` (boolean)
- A keypair is valid if the public key is on the ed25519 curve
