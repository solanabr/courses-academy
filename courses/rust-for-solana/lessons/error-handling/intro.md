# Error Handling in Solana Programs

Robust error handling is critical for secure Solana programs. Rust's `Result` type and pattern matching make it impossible to accidentally ignore errors, unlike languages with exceptions.

## The Result Type

Every fallible operation in Rust returns a `Result<T, E>`:
- `Ok(value)`: Success, contains the result
- `Err(error)`: Failure, contains the error

```rust
pub fn divide(a: u64, b: u64) -> Result<u64, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

## Error Propagation with `?`

The `?` operator automatically returns an error if the operation fails, or unwraps the value if it succeeds:

```rust
pub fn process_transfer(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let sender = next_account_info(&mut accounts.iter())?; // Returns early if fails
    let recipient = next_account_info(&mut accounts.iter())?;
    
    if sender.lamports() < amount {
        return Err(ProgramError::InsufficientFunds);
    }
    
    **sender.lamports.borrow_mut() -= amount;
    **recipient.lamports.borrow_mut() += amount;
    
    Ok(())
}
```

This is much cleaner than manual error checking:

```rust
// Without ?
let sender = match next_account_info(&mut accounts.iter()) {
    Ok(acc) => acc,
    Err(e) => return Err(e),
};
```

## ProgramError Enum

Solana provides a standard `ProgramError` enum for common failures:

```rust
pub enum ProgramError {
    InvalidArgument,
    InsufficientFunds,
    IncorrectProgramId,
    MissingRequiredSignature,
    AccountAlreadyInitialized,
    UninitializedAccount,
    // ... and 20+ more variants
}
```

Return these from your `process_instruction` function:

```rust
if !sender.is_signer {
    return Err(ProgramError::MissingRequiredSignature);
}
```

## Custom Error Types

For domain-specific errors, define your own enum:

```rust
use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, FromPrimitive)]
pub enum TokenError {
    #[error("Insufficient token balance")]
    InsufficientBalance,
    
    #[error("Account is frozen")]
    AccountFrozen,
    
    #[error("Invalid mint authority")]
    InvalidMintAuthority,
}

impl From<TokenError> for ProgramError {
    fn from(e: TokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
```

Now you can return custom errors that convert to `ProgramError::Custom`:

```rust
if account.frozen {
    return Err(TokenError::AccountFrozen.into());
}
```

## Validation Patterns

Always validate inputs at the start of your instruction handler:

```rust
pub fn process_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // 1. Validate account count
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    
    let account_iter = &mut accounts.iter();
    let sender = next_account_info(account_iter)?;
    let recipient = next_account_info(account_iter)?;
    
    // 2. Validate ownership
    if sender.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // 3. Validate signer
    if !sender.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // 4. Validate amount
    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }
    
    // 5. Validate balance
    if sender.lamports() < amount {
        return Err(ProgramError::InsufficientFunds);
    }
    
    // Now perform the transfer
    **sender.lamports.borrow_mut() -= amount;
    **recipient.lamports.borrow_mut() += amount;
    
    Ok(())
}
```

## Error Messages

Use the `msg!` macro to log errors for debugging (visible in transaction logs):

```rust
msg!("Transfer failed: insufficient balance ({} < {})", sender.lamports(), amount);
return Err(ProgramError::InsufficientFunds);
```

**Important**: Never log sensitive data like private keys or user PII.

## Key Takeaways

1. **Always use `Result`**: Never use `unwrap()` or `expect()` in production programs
2. **Validate early**: Check all preconditions before mutating state
3. **Use `?` for propagation**: Cleaner than manual error handling
4. **Return specific errors**: Help clients diagnose failures
5. **Log for debugging**: Use `msg!` to provide context in transaction logs
