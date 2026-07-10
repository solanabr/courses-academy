# Deploy to Devnet

You've built the complete counter program. Now compile it one final time and learn how to deploy it.

## Final Build

The starter code is your complete counter program. Hit **Build** to compile it. When it succeeds, you'll see a **Build UUID** in the output — this is the identifier for your compiled `.so` binary.

## What Happens After Build

The build server compiled your Rust code into a Solana program binary (`.so` file). To deploy it to Devnet, you'd run:

```bash
# Download the compiled binary
curl -H 'X-API-Key: YOUR_KEY' \
  https://build-server-url/deploy/YOUR_UUID \
  -o program.so

# Deploy to Devnet
solana program deploy program.so --url devnet
```

The `solana program deploy` command:
1. Uploads the binary to a buffer account
2. Creates the program account
3. Sets the program as executable
4. Returns the **Program ID** — the on-chain address of your program

## Interacting with Your Program

Once deployed, anyone can call your program's instructions:

```typescript
// TypeScript client using Anchor
const program = new Program(idl, programId, provider);

// Initialize a counter
await program.methods
  .initialize()
  .accounts({ counter: counterKeypair.publicKey })
  .signers([counterKeypair])
  .rpc();

// Increment
await program.methods
  .increment()
  .accounts({ counter: counterKeypair.publicKey })
  .rpc();
```

## What You've Accomplished

You wrote a Solana program from scratch:
- **4 instructions**: initialize, greet, increment, decrement
- **Account management**: init with payer, mutable state, space calculation
- **Error handling**: Custom error enum with underflow protection
- **Compilation**: cargo-build-sbf via the build server

This counter program follows the same patterns used by every production Anchor program on Solana. The complexity scales, but the fundamentals are the same.

## Next Steps

- **DeFi on Solana** course — build a token vault with PDAs and CPIs
- **Deploy locally** — install the Solana CLI and Anchor to build on your machine
- **Read the Anchor Book** — [book.anchor-lang.com](https://book.anchor-lang.com)
