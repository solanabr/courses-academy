# On-Chain State

So far, our program doesn't store any data. Every instruction just logs a message and exits. To build anything useful, you need **on-chain state** — data that persists between transactions.

## Accounts = Storage

On Solana, all state lives in **accounts**. An account is a buffer of bytes owned by a program. Think of it like a row in a database, but on-chain.

Every account has:
- **Address** (public key) — how you find it
- **Owner** (program ID) — which program can modify it
- **Data** (byte array) — the actual state
- **Lamports** (balance) — rent deposit to keep it alive

## The Discriminator

Anchor adds an 8-byte **discriminator** at the start of every account's data. This is a hash of the account type name, used to prevent account type confusion attacks.

```
[8 bytes discriminator][...your data...]
```

You never read or write the discriminator directly — Anchor handles it.

## Space Calculation

When you create an account, you must specify its size upfront (accounts can't be resized after creation). The formula:

```
space = 8 (discriminator) + sum of field sizes
```

Common field sizes:
- `u8` / `i8` / `bool` = 1 byte
- `u16` / `i16` = 2 bytes
- `u32` / `i32` = 4 bytes
- `u64` / `i64` = 8 bytes
- `u128` / `i128` = 16 bytes
- `Pubkey` = 32 bytes
- `String` = 4 (length prefix) + content bytes
- `Vec<T>` = 4 (length prefix) + (count * size_of::<T>())

## Example: A Counter

A simple counter needs one `u64` field:

```rust
#[account]
pub struct Counter {
    pub count: u64,  // 8 bytes
}
// Total space = 8 (discriminator) + 8 (u64) = 16 bytes
```

The `#[account]` attribute tells Anchor to:
1. Add serialization/deserialization (Borsh)
2. Add the 8-byte discriminator
3. Implement account validation traits

In the next challenge, you'll define this Counter account and wire it into your program.
