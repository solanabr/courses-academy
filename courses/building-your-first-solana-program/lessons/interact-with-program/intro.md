# Interact with Your Program

Your counter program is live on Solana Devnet. Now let's call its instructions and watch the on-chain state change in real time.

## What You'll Do

1. **Initialize** a counter account — creates a new account owned by your program
2. **Increment** the counter — modifies the on-chain state
3. **Decrement** the counter — see your custom error handling in action

## Understanding the Data

The Program Explorer below shows two views of your counter account:

### Parsed Data
The human-readable version: `count: 3`

### Raw Data (Hex)
What's actually stored on-chain:
```
[a8 fc 0b 1f 05 77 32 45 | 03 00 00 00 00 00 00 00]
 ^^^^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^
 discriminator (8 bytes)    count: 3 (u64 LE)
```

The **discriminator** is Anchor's way of identifying account types. It's the first 8 bytes of `sha256("account:Counter")`. Every Anchor account starts with this — it's how the runtime knows it's looking at a `Counter` struct and not random data.

The **count** is stored as a `u64` in **little-endian** byte order. The value `3` is `03 00 00 00 00 00 00 00` because the least significant byte comes first.

## Try This

1. Click **Initialize** — watch the discriminator bytes appear
2. Click **Increment** 3 times — watch `count` change from `00` to `03`
3. Click **Decrement** down to 0, then try decrementing again
4. See your `#[error_code] Underflow` in action!

> **Teaching moment**: When you decrement below zero, your custom error `Cannot decrement below zero` is enforced by the Solana runtime. This is your code running on a real blockchain, enforcing rules you defined.
