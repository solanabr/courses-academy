# Challenge: Build a Transfer Function

Create a function that builds a SOL transfer transaction. The function should take a sender public key, recipient public key, and amount in SOL, then return the transaction object ready to be signed.

## Requirements

- Accept sender (PublicKey), recipient (PublicKey), and amount (number, in SOL)
- Convert SOL amount to lamports
- Create and return a Transaction with a SystemProgram.transfer instruction
