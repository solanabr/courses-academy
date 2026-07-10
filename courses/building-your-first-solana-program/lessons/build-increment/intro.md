# Challenge: Build the Increment

Add an `increment` instruction to the counter program.

## Requirements

1. Add a `pub fn increment` inside the `#[program]` module
2. It should take `ctx: Context<Increment>` and return `Result<()>`
3. Get a mutable reference to `ctx.accounts.counter` and add 1 to `count`
4. Add an `Increment` accounts struct with `counter` (PDA, `mut`, same seeds as Initialize) and `user` (`Signer`)

## Why `user` is needed in Increment

The counter is a PDA derived from `[b"counter", user.key()]`. To verify the PDA matches, Anchor needs the `user` account to re-derive the seeds. This also acts as authorization — only the wallet that matches the seeds can modify this counter.

## Tips

- The `Increment` struct needs the `<'info>` lifetime, just like `Initialize`
- The counter field needs `seeds` and `bump` (same seeds as Initialize, but no `init`)
- The `user` field is `Signer<'info>` (not mut — they don't pay anything here)
- Don't change the existing `initialize` or `greet` instructions
