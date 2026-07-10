# From Code to Chain

You've written Anchor programs in the previous courses. Now it's time to compile and deploy them for real.

## The Build Pipeline

When you write an Anchor program, it goes through several stages before it reaches the Solana blockchain:

1. **Rust source code** — what you write in your editor
2. **`cargo-build-sbf`** — the Solana compiler that transforms Rust into Solana Bytecode Format (SBF)
3. **`.so` binary** — the compiled program, an ELF shared object file
4. **`solana program deploy`** — uploads the binary to a Solana cluster

## What is SBF?

Solana Bytecode Format (SBF) is a variant of eBPF (extended Berkeley Packet Filter) adapted for the Solana runtime. It's a register-based instruction set designed for:

- **Safety**: Programs run in a sandboxed VM with no access to the host system
- **Performance**: Near-native execution speed
- **Determinism**: Same inputs always produce the same outputs

## The Build Server

In this course, you won't need a local Rust toolchain. The **Superteam Academy Build Server** handles compilation in the cloud:

1. You write Anchor code in the browser editor
2. Click **Build** to submit your code
3. The build server runs `cargo-build-sbf` on your code
4. You get back either compiler errors or a compiled binary UUID

The build server uses the same Anchor 0.32 and Solana SDK that you'd use locally. Think of it as a cloud-hosted `anchor build`.

## What Gets Compiled

Your `lib.rs` file is placed into a pre-configured Anchor project with:
- `anchor-lang` 0.32.1 (with `init-if-needed` feature)
- `anchor-spl` 0.32.1
- `solana-program` via the Agave 3.0 toolchain

The build server rejects dangerous patterns (`std::process`, `std::fs`, `std::net`) to keep the sandbox secure.

## Ready to Build?

In the next lesson, you'll hit the **Build** button for the first time. Your only job: see the green checkmark.
