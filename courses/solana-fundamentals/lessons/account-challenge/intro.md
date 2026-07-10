# Challenge: Explore the Account Model

Create a function that builds a mock Solana account structure. This will help you understand the key properties every account has.

## Requirements

- Create a function that accepts `owner` (PublicKey), `lamports` (number), `data` (Uint8Array), and `executable` (boolean)
- Return an object representing an account with these properties
- The object should also include a `publicKey` (generate a new one) and `rentEpoch` (set to 0 for this mock)
