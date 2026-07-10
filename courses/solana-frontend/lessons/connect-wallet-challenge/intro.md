# Challenge: Wallet Connection Flow

Simulate a wallet connection flow by generating a keypair (representing a wallet) and extracting key information.

## Requirements

- Create a function that simulates connecting to a wallet
- Generate a new keypair to represent the wallet
- Validate that the public key is on the ed25519 curve
- Return an object with `publicKey` (base58 string), `connected` (boolean), and `isValid` (boolean)
