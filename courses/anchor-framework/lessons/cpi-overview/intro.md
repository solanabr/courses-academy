# Cross-Program Invocations (CPIs)

Cross-Program Invocations allow one Solana program to call another program. This is what makes Solana programs **composable** — you can build on top of existing programs like building blocks.

## Why CPIs Matter

CPIs enable:
- **Token transfers** via the Token Program
- **NFT minting** via Metaplex programs
- **Swaps** through DEX programs (Jupiter, Raydium)
- **Lending** through DeFi protocols (Solend, MarginFi)

Without CPIs, every program would need to reimplement basic functionality. With CPIs, you compose existing programs.

## How CPIs Work

When Program A calls Program B:

```
User Transaction
  |
  v
Program A (your program)
  |
  v (CPI)
Program B (Token Program, System Program, etc.)
```

The Solana runtime tracks the **call stack** and ensures:
- Account ownership rules are enforced
- Signers are propagated correctly
- Programs can't exceed the call depth limit (4 levels)

## invoke vs invoke_signed

### invoke - Regular CPI

```rust
use solana_program::program::invoke;

invoke(
    &instruction,
    &[
        source_account.clone(),
        destination_account.clone(),
        authority.clone(),
    ],
)?;
```

Use `invoke` when the required signers are already in your instruction's signer list.

### invoke_signed - CPI with PDA Signer

```rust
use solana_program::program::invoke_signed;

invoke_signed(
    &instruction,
    &[
        pda_account.clone(),
        destination_account.clone(),
        system_program.clone(),
    ],
    &[&[b"vault", &[bump]]], // PDA seeds
)?;
```

Use `invoke_signed` when a **PDA** needs to sign. The program derives the PDA and "signs" on its behalf.

## CPI in Anchor

Anchor provides a cleaner API for CPIs:

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
```

### CPI with PDA Signer in Anchor

```rust
pub fn transfer_from_pda(ctx: Context<TransferFromPDA>, amount: u64) -> Result<()> {
    let seeds = &[
        b"vault",
        ctx.accounts.authority.key().as_ref(),
        &[ctx.accounts.vault.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(
        cpi_program,
        cpi_accounts,
        signer_seeds,
    );

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
```

## Common CPI Patterns

### 1. Token Transfer

```rust
// Transfer SPL tokens from user to program vault
token::transfer(
    CpiContext::new(token_program, transfer_accounts),
    amount,
)?;
```

### 2. Minting Tokens

```rust
// Mint new tokens (requires mint authority)
token::mint_to(
    CpiContext::new_with_signer(token_program, mint_accounts, signer_seeds),
    amount,
)?;
```

### 3. Creating Accounts

```rust
// Create a new account (CPI to System Program)
system_program::create_account(
    CpiContext::new(system_program, create_accounts),
    lamports,
    space,
    program_id,
)?;
```

## CPI Security Considerations

### 1. Account Validation

Always validate accounts before CPI:

```rust
require_keys_eq!(
    ctx.accounts.token_program.key(),
    token::ID,
    ErrorCode::InvalidTokenProgram
);
```

### 2. Signer Propagation

Signers in your instruction are automatically propagated to CPIs. But be careful:

```rust
// BAD: Anyone can call this and spend vault tokens!
pub fn dangerous_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    token::transfer(
        CpiContext::new_with_signer(...),
        amount,
    )
}

// GOOD: Check authority first
pub fn safe_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.authority.key(),
        ctx.accounts.vault.authority,
        ErrorCode::Unauthorized
    );
    token::transfer(...)
}
```

### 3. Reentrancy

Solana programs are **not reentrant** by default (a program can't call itself). This prevents classic reentrancy attacks.

## Real-World CPI Examples

**Escrow program**:
1. User deposits tokens → CPI to Token Program to transfer tokens to escrow PDA
2. Trade executes → CPI to Token Program to transfer from escrow PDA to recipient

**Staking program**:
1. User stakes tokens → CPI to Token Program to transfer to staking vault
2. User claims rewards → CPI to Token Program to mint reward tokens
3. User unstakes → CPI to Token Program to transfer staked tokens back

**NFT marketplace**:
1. User lists NFT → CPI to Token Program to transfer NFT to marketplace PDA
2. Buyer purchases → CPI to System Program (SOL payment) + CPI to Token Program (NFT transfer)

## Next Challenge

You'll construct a CPI instruction to transfer SOL from a PDA-owned account.
