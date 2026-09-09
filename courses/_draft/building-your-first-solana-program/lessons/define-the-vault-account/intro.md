# Define the Vault Account

> **Version stamp — checked 2026-07-26.** `anchor-lang 1.1.2` (published 2026-06-26) · `cargo-build-sbf` with platform-tools v1.54 (rustc 1.89) · `borsh` resolves to **1.8.0** · edition 2021. Every compiler message quoted below was produced by compiling this lesson's file on that toolchain. Scope of the harness: it pins the field names, their types and the `bump` constraint. It cannot see whether your `space =` expression is the right number — see "The one thing the build cannot check".

You have derived the size. You have derived the address. What you have not done is write the whole
accounts struct yourself, in one piece, and that is this lesson.

Every line you need is in the file, commented out and out of order. Two of them belong nowhere. Your
job is to pick, order and uncomment — not to invent.

## What an accounts struct is for

`#[derive(Accounts)]` is not a parameter list. It is a **validation contract**, checked by generated
code before your handler body runs. Each field says "an account of this type, satisfying these
constraints, or the instruction fails."

`InitializeVault` needs three accounts, each for a different reason:

```
vault           the account being created. Program-owned, PDA-addressed, 49 bytes
user            who is paying, and who the vault will belong to
system_program  the program that actually allocates the bytes
```

## The five constraints on the vault

```rust
init                                          // create it
payer = user                                  // this account funds the rent deposit
space = 8 + 32 + 8 + 1                        // 49 bytes, derived not copied
seeds = [b"vault", user.key().as_ref()]       // one vault per wallet
bump                                          // at the canonical address, checked
```

`init` is the one that does the work, and it does more than it looks like:

- allocates `space` bytes,
- assigns the account's **owner** to your program id,
- moves the rent-exempt deposit from `payer`,
- writes `VaultState`'s 8-byte discriminator into the front,
- and does all of that by **calling the System Program** — which is why `system_program` has to be in
  the struct.

That last point is not a detail. Anchor does not inject accounts. If `init` needs the System Program,
you declare the System Program.

## Why `user` needs both `mut` and `Signer`

Two separate facts about the same account, enforced separately.

`Signer<'info>` says the transaction carries this account's signature. `#[account(mut)]` says this
instruction changes the account — and it does, because `payer = user` debits about 0.00123 SOL of rent
deposit from it.

Miss the `mut` and Anchor rejects the instruction rather than silently spending someone's lamports. The
runtime's rule is the broader one: **any account whose lamports or data change must be declared
writable**, all the way up at the transaction level. `mut` is how you declare it here.

## The two lines that belong nowhere

One pool entry is `rent = rent`. That constraint was **removed in Anchor 0.31** — Anchor reads the rent
parameters itself and derives the deposit from `space`. Use it and the build stops.

The other is `mut,` sitting among the vault's constraints. It is the line people reach for out of
reflex, because the vault is obviously about to be written to. But the vault does not exist yet, and
`mut` means "this existing account will be modified." `init` is the request to bring an account into
being, and it implies the write. Ask for both, or for `mut` alone with `space` and `payer` beside it,
and Anchor tells you the combination is wrong.

Both traps compile-fail, which is the mercy of this rung. The one that does *not* compile-fail is
`space`.

## The one thing the build cannot check

`space = 8 + 32 + 8 + 1` and `space = 8 + 999` compile identically. Nothing in Rust, nothing in Anchor
and nothing in the grader compares your `space` to your struct. An account sized 8 bytes short deploys
fine and fails on first write; an account sized 1024 bytes deploys fine and quietly locks twenty times
the rent deposit, per vault, for as long as it exists.

That is why lesson 1 walked the arithmetic instead of handing you the number, and why
`8 + VaultState::INIT_SPACE` is what you would reach for in production. Here you write the sum, because
here the point is knowing what it is made of. The literal has one more graduation ahead of it, too: in
Anchor V2 the magic `8` goes as well, and the idiom becomes
`VaultState::DISCRIMINATOR.len() + VaultState::INIT_SPACE` — every piece of the sum named. That is
where this number goes next, and it is exactly what **Master Anchor V2** teaches (its migration module
grades the off-by-8 port bug this line prevents).

## Requirements

1. Replace `pub struct InitializeVault {}` with pool line (1). Note what is different about it, and why
   an empty struct did not need it.
2. Give the `vault` field its five constraints, inside one `#[account( ... )]`.
3. Add the `user` field, as a mutable `Signer`.
4. Add the `system_program` field.
5. Leave the two dead pool lines alone.

The verification harness at the bottom of the file names every field and type a passing answer must
have. Read it — it is the acceptance criteria, written in Rust.
