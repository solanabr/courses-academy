# Custom Errors in Anchor

Properly handling errors is essential for debugging and security. Anchor provides a clean way to define and use custom errors.

## Defining Custom Errors

```rust
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for this operation")]
    InsufficientFunds,

    #[msg("User is not authorized to perform this action")]
    Unauthorized,

    #[msg("The provided amount exceeds the maximum allowed")]
    AmountTooLarge,

    #[msg("Account has already been initialized")]
    AlreadyInitialized,
}
```

Each error gets:
- A unique error code (auto-assigned)
- A human-readable message
- Integration with Anchor's error handling

## Using the require! Macro

The `require!` macro is the cleanest way to enforce conditions:

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user;

    // Check balance
    require!(
        user_account.balance >= amount,
        ErrorCode::InsufficientFunds
    );

    // Check authorization
    require_keys_eq!(
        ctx.accounts.authority.key(),
        user_account.authority,
        ErrorCode::Unauthorized
    );

    // Check amount limit
    require!(
        amount <= 1_000_000,
        ErrorCode::AmountTooLarge
    );

    user_account.balance -= amount;
    Ok(())
}
```

## Anchor's Built-in require! Variants

### require_keys_eq!

```rust
require_keys_eq!(
    ctx.accounts.user.authority,
    ctx.accounts.signer.key(),
    ErrorCode::Unauthorized
);
```

Compares two public keys.

### require_keys_neq!

```rust
require_keys_neq!(
    ctx.accounts.from.key(),
    ctx.accounts.to.key(),
    ErrorCode::CannotTransferToSelf
);
```

Ensures two keys are different.

### require_gt! / require_gte!

```rust
require_gte!(
    user.balance,
    MIN_BALANCE,
    ErrorCode::BalanceTooLow
);
```

Numeric comparisons (gt = greater than, gte = greater than or equal).

## Constraint Errors

Constraints in `#[account(...)]` automatically return errors:

```rust
#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(
        mut,
        has_one = authority @ ErrorCode::Unauthorized,
        constraint = user.is_active @ ErrorCode::AccountInactive
    )]
    pub user: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}
```

If `has_one` or `constraint` fails, Anchor returns your custom error.

## Error Handling in TypeScript Tests

Anchor errors are automatically parsed in TypeScript:

```typescript
try {
  await program.methods
    .transfer(new anchor.BN(10000))
    .accounts({ user: userPDA })
    .rpc();

  throw new Error('Should have failed');
} catch (err) {
  // Anchor error structure
  expect(err.error.errorCode.code).to.equal('InsufficientFunds');
  expect(err.error.errorCode.number).to.equal(6000); // First custom error
  expect(err.error.errorMessage).to.include('Insufficient funds');
}
```

## Error Codes

Anchor assigns error codes starting at **6000**:

```rust
#[error_code]
pub enum ErrorCode {
    InsufficientFunds,    // 6000
    Unauthorized,         // 6001
    AmountTooLarge,       // 6002
}
```

You can also manually assign codes:

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds = 7000,
}
```

## Anchor's Built-in Errors

Anchor provides common errors automatically:

- `ErrorCode::ConstraintMut` - Account not marked as mutable
- `ErrorCode::ConstraintHasOne` - has_one constraint failed
- `ErrorCode::ConstraintSigner` - Account not a signer
- `ErrorCode::ConstraintRaw` - Generic constraint failed
- `ErrorCode::AccountNotInitialized` - Account data is empty
- `ErrorCode::AccountOwnedByWrongProgram` - Account owned by wrong program

## Best Practices

### 1. Be Specific

```rust
// BAD: Vague error
#[msg("Invalid input")]
InvalidInput,

// GOOD: Specific error
#[msg("Amount must be between 1 and 1,000,000 lamports")]
AmountOutOfRange,
```

### 2. Use Constraints Where Possible

Instead of:

```rust
pub fn update(ctx: Context<Update>) -> Result<()> {
    require!(
        ctx.accounts.user.authority == ctx.accounts.signer.key(),
        ErrorCode::Unauthorized
    );
    // ...
}
```

Use:

```rust
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        has_one = authority @ ErrorCode::Unauthorized
    )]
    pub user: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}
```

### 3. Log Context for Debugging

```rust
require!(
    amount > 0,
    ErrorCode::AmountMustBePositive
);

msg!("Transferring {} lamports", amount);
```

Use `msg!()` to log values for debugging.

## Matching Errors in Integration Tests

```typescript
const expectError = async (fn, errorCode) => {
  try {
    await fn();
    throw new Error(`Expected ${errorCode} error`);
  } catch (err) {
    expect(err.error.errorCode.code).to.equal(errorCode);
  }
};

await expectError(
  () => program.methods.transfer(tooMuch).rpc(),
  'InsufficientFunds'
);
```

## Next Lesson

You'll learn how to deploy Anchor programs to devnet and mainnet.
