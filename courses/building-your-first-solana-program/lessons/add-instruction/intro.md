# Challenge: Add an Instruction

Time to modify the program. Add a `greet` instruction that logs a greeting message.

## Requirements

1. Add a new function `greet` inside the `#[program]` module
2. It should take `_ctx: Context<Greet>` as its parameter
3. It should call `msg!("Hello, Solana!")` and return `Ok(())`
4. Add a corresponding `Greet` accounts struct (can be empty, like `Initialize`)

## Tips

- Follow the same pattern as the existing `initialize` function
- The accounts struct name must match the `Context<T>` generic
- Don't forget `#[derive(Accounts)]` on your new struct
