# Ownership and Borrowing in Rust

Rust's ownership system is its most distinctive feature. Understanding it is essential for writing Solana programs, as it directly maps to how accounts are passed to your program.

## The Three Rules of Ownership

1. **Each value has a single owner**: Only one variable owns a piece of data at any time
2. **When the owner goes out of scope, the value is dropped**: Automatic cleanup, no memory leaks
3. **Ownership can be transferred (moved)**: Passing a value to a function or assigning it to another variable moves ownership

```rust
let account_data = vec![1, 2, 3, 4];
let moved_data = account_data; // Ownership transferred
// println!(":{?}", account_data); // Error: value moved
```

## Borrowing: Using Data Without Taking Ownership

Borrowing lets you reference data without taking ownership. There are two types:

### Immutable Borrowing (`&T`)

You can have **multiple** immutable borrows simultaneously:

```rust
fn calculate_balance(accounts: &[AccountInfo]) -> u64 {
    accounts.iter().map(|a| a.lamports).sum()
}

let accounts = vec![/* ... */];
let total = calculate_balance(&accounts);
// accounts still valid here
```

### Mutable Borrowing (`&mut T`)

You can have **only ONE** mutable borrow at a time, and no immutable borrows can exist simultaneously:

```rust
fn debit_account(account: &mut AccountInfo, amount: u64) {
    account.lamports -= amount;
}

let mut account = /* ... */;
debit_account(&mut account);
```

## How This Maps to Solana

When you write a Solana program, accounts are passed as `&[AccountInfo]` (immutable borrow) or extracted as `&mut AccountInfo` (mutable borrow):

```rust
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo], // immutable borrow of the slice
    instruction_data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let sender = next_account_info(account_iter)?; // immutable borrow
    let recipient = next_account_info(account_iter)?;
    
    // To modify, you need mutable access:
    **sender.lamports.borrow_mut() -= amount;
    **recipient.lamports.borrow_mut() += amount;
    Ok(())
}
```

## Key Insight

Solana's runtime enforces similar rules:
- Multiple programs can read an account simultaneously (immutable borrows)
- Only ONE program can write to an account in a transaction (mutable borrow)
- The runtime prevents data races at the transaction level, and Rust prevents them at compile time

This alignment makes Rust the perfect fit for Solana development.
