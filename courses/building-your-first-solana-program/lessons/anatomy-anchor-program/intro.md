# Anatomy of an Anchor Program

Before we start adding features, let's break down every line of the program you just compiled.

## The Import

```rust
use anchor_lang::prelude::*;
```

This single import brings in everything Anchor needs: the `#[program]` macro, `Context`, `Account`, `Signer`, `Result`, error types, and more.

## Program ID

```rust
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
```

Every Solana program has a unique address (public key). The `declare_id!` macro hardcodes this address into the binary. When you deploy, Solana verifies the binary matches this ID.

For development, you can use any valid base58 public key. For production, you'd generate one with `solana-keygen grind`.

## The Program Module

```rust
#[program]
pub mod academy_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Superteam Academy default program");
        Ok(())
    }
}
```

The `#[program]` attribute tells Anchor this module contains your instruction handlers. Each `pub fn` inside becomes a callable instruction on-chain.

**Key rules:**
- First parameter is always `Context<T>` where `T` is an accounts struct
- Return type is always `Result<()>`
- `msg!()` logs to the transaction log (visible in explorers)

## Accounts Structs

```rust
#[derive(Accounts)]
pub struct Initialize {}
```

Every instruction needs an accounts struct that defines which accounts the instruction reads/writes. This empty struct means `initialize` doesn't need any accounts.

In the next lesson, you'll add your first real instruction with a non-empty accounts struct.

## What the Compiler Checks

When you hit Build, `cargo-build-sbf` validates:
- Rust syntax and type safety
- Anchor macro expansion (accounts, constraints, errors)
- SBF compatibility (no system calls, no heap abuse)

If any of these fail, you get a compiler error with the exact line number.
