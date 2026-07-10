# Challenge: Define a Counter Account (PDA)

Add a `Counter` account struct and update the `Initialize` accounts to create it as a **Program Derived Address** (PDA).

## What is a PDA?

A PDA is an account address derived deterministically from **seeds** and the **program ID**. Unlike keypair accounts, PDAs don't have a private key — the program itself controls them. The same seeds always produce the same address.

For our counter: `seeds = ["counter", user_wallet_pubkey]` means each wallet gets exactly one counter per program.

## Requirements

1. Add a `Counter` struct with `#[account]` attribute containing `pub count: u64` and `pub authority: Pubkey`
2. Add lifetime `<'info>` to the `Initialize` struct
3. Add a `counter` field with `#[account(init, payer = user, space = 8 + 8 + 32, seeds = [b"counter", user.key().as_ref()], bump)]`
4. Add a `user` field as `Signer<'info>` with `#[account(mut)]`
5. Add `system_program` as `Program<'info, System>`

## What Each Constraint Does

- `init` — creates the account (allocates space, assigns owner)
- `payer = user` — the `user` account pays the rent deposit
- `space = 8 + 8 + 32` — 8 bytes discriminator + 8 bytes `u64` + 32 bytes `Pubkey`
- `seeds` — the byte arrays used to derive the PDA address
- `bump` — Anchor finds the valid bump seed automatically
- `mut` on `user` — required because their lamport balance changes (pays rent)

## Tips

- The `Counter` struct goes outside the `#[program]` module
- The `Initialize` struct needs a lifetime parameter: `pub struct Initialize<'info>`
- Make sure `counter` has type `Account<'info, Counter>`
- The `authority` field records who created the counter (useful for access control later)
