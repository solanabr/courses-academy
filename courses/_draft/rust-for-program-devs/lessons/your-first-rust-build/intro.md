# Your First Rust Build

> **Version stamp — checked 2026-07-26.** `anchor-lang` **1.1.2** (published 2026-06-26; the newest release on crates.io on the date this lesson was written) · Rust **edition 2021** · Agave **≥ 3.1.10** · borsh **1.5.7 declared / 1.8.0 resolved** · the build server compiles with `cargo-build-sbf --tools-version v1.54`, whose pinned `rustc` reports 1.89.0-dev. That last number is not printed anywhere in your build log, which is why it is written here — this stamp is the only place you can read it. Lesson 4 depends on `overflow-checks = true`, which is set in the build crate's `[profile.release]`.

In Course 1 your inspector took an address, pulled the account back off devnet, and turned the bytes into fields with names — `owner`, `balance`, `bump`.

Those bytes had no names on the chain. They had names because somebody wrote a `struct` in Rust, and the memory layout of that struct **is** the account. Your decoder was reading a shape it had been told about. This course is about being the person who defines the shape.

Fourteen lessons from now you will hold a file called `vault_core.rs`: a `VaultState`, a `deposit`, a `withdraw` that cannot silently wrap a `u64`, and an error enum with four named variants. Course 3 wraps a program around it and puts it on devnet.

Right now, none of that. Right now: a green build.

## Nothing to install

You are not going to install Rust. There is no `rustup`, no `cargo`, no toolchain to keep in sync with a tutorial, and no `.so` file to upload from your machine.

The editor below sends one file to a build server that already has the Solana platform toolchain on it. It compiles the file for the SBF target — the bytecode format Solana validators execute — and sends back the result. That is the whole loop, and it is the loop for every code exercise in this course and the next.

That promise is this course's defining seam — including against our own catalog, not just against the video courses that make you install things. The elective **Rust & TypeScript Fundamentals for Web3** (`rust-ts-fundamentals`) teaches general Rust the other way on purpose: rustup and cargo on your own machine, thiserror/anyhow, binaries and containers. This course is the zero-install rung of the path: build-server graded, anchor-lang dialect, one artifact — `vault_core.rs` — that Course 3 consumes. Same language, opposite toolchain promise; pick by which promise you need.

### Finished Rust & TypeScript Fundamentals already?

Then its m04–m05 covered what Modules 1–3 teach here — ownership, borrows, checked math, enums, `Result` and derives — in the general-Rust dialect. Take the Module 2 checkpoint quiz (`ownership-checkpoint`) as your placement test and start at Module 3 or Module 4; the anchor-lang layer (`#[account]`, `InitSpace`, `#[error_code]`) will still be new to you.

This first exercise exists to prove that loop works for you, on your network, before any Rust matters. Nothing in the file needs changing.

## The file, line by line

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub fn vault_core_banner() -> Result<()> {
    msg!("vault core reporting in from {}", ID);
    Ok(())
}
```

Four things, and each one is worth a sentence.

**`use anchor_lang::prelude::*;`** — Rust has no ambient globals. Nothing is in scope that you did not bring into scope. A `prelude` module is a crate's answer to that: a curated bundle of the names you will want in almost every file, imported in one line. `Result`, `msg!` and `declare_id!` all arrive through this import.

**`declare_id!("Fg6Pa…")`** — the address the program lives at, as a compile-time constant named `ID`. It is a placeholder here; Course 3 replaces it with the id of the program you actually deploy. It is not optional decoration. Anchor's account macros generate code that refers to `crate::ID`, so a file that uses them and forgets this line does not compile — and the error appears at the macro, not here, which is why it is worth recognising now.

**`pub fn vault_core_banner() -> Result<()>`** — `fn` declares a function. `pub` makes it visible outside this module. `-> Result<()>` is the return type: *either* success carrying nothing, *or* an error. Anchor's `Result<()>` is a program's standard return shape, and one of the reasons Solana programs written in Rust do not throw. There is no `throw`. Failure is a value you return.

**`Ok(())`** — the success value. `()` is the unit type, Rust's "no meaningful value" — the closest thing it has to `void`, except that it is a real type with exactly one value, so it can be put inside `Ok(...)` like anything else. Note the absence of a semicolon: this is the function's **tail expression**, and it is what the function evaluates to. Lesson 3 is about how much that missing semicolon matters.

The keen-eyed will notice this file has no `#[program]` and no `#[derive(Accounts)]` — no instruction handlers, no account lists. That is deliberate, and it stays that way for all fourteen lessons. Those two macros are Course 3's job. Your file is the logic they wrap around.

## Press Build

Expect **30 to 90 seconds** the first time. The server is compiling Anchor and its dependency tree, not just your eight lines. Subsequent builds reuse that work and come back in seconds.

This one will pass, which means you get one line back and no log. Read the next section anyway — it is about what arrives when a build *fails*, which is the interesting case and the one you will be in for the next thirteen lessons. For the rest of this course the compiler is not an obstacle between you and a passing grade. It is the fastest reviewer you will ever have, and it works for free.
