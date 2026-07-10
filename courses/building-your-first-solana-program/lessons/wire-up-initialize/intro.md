# Challenge: Wire Up Initialize

This challenge is different. The starter code has a **deliberate bug** — it won't compile.

Your job:
1. Click **Build** to see the compiler error
2. Read the error message carefully
3. Fix the code
4. Build again until it compiles

## The Bug

The `Initialize` struct uses `#[account(init, ...)]` to create a `Counter` PDA, but it's missing a required account. When Anchor generates the account creation code, it needs the System Program to execute the `create_account` instruction.

## What You'll Learn

This is the most common Anchor compile error you'll encounter in the wild. Learning to diagnose it now saves you hours of debugging later.

## Requirements

1. Read the compiler error output
2. Add the missing account to `Initialize`
3. Make sure `initialize` sets `counter.count = 0` and `counter.authority = ctx.accounts.user.key()`
