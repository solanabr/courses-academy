# Challenge: Build a Custom Instruction

Construct a custom Solana instruction that could be used to interact with a program. You'll define the program ID, the accounts involved, and the instruction data.

## Requirements

- Create a function that accepts `programId` (PublicKey), `payer` (PublicKey), `dataAccount` (PublicKey), and `amount` (number)
- Return an instruction object with:
  - `programId`: the program to call
  - `keys`: an array of AccountMeta objects (payer as signer + writable, dataAccount as writable)
  - `data`: a Uint8Array containing the amount encoded as 8 bytes (little-endian u64)
- Use `DataView` to encode the amount as a little-endian 64-bit unsigned integer
