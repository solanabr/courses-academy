# Anchor Account Constraints

Anchor's constraint system is one of its most powerful features. Constraints allow you to declaratively specify account validation rules, and Anchor enforces them automatically.

## The #[account] Attribute

The `#[account(...)]` attribute is used to apply constraints to accounts in your instruction context:

```rust
#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = user.data_version < 10 @ ErrorCode::VersionTooHigh
    )]
    pub user: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}
```

## Common Constraints

### 1. init - Initialize a New Account

```rust
#[account(
    init,
    payer = user,
    space = 8 + 32 + 8
)]
pub my_account: Account<'info, MyAccount>,
```

- Allocates space for the account
- Transfers lamports from payer
- Assigns account to the program
- Sets account discriminator

**Space calculation**: `8 (discriminator) + account_data_size`

### 2. mut - Mark Account as Mutable

```rust
#[account(mut)]
pub user: Account<'info, UserAccount>,
```

Requires the account to be writable. Anchor will reject the transaction if the account isn't marked as writable.

### 3. has_one - Verify Account Relationship

```rust
#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub counter: u64,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub user: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}
```

`has_one = authority` checks that `user.authority == authority.key()`. This prevents unauthorized updates.

### 4. seeds & bump - PDA Derivation

```rust
#[account(
    init,
    payer = user,
    space = 8 + 32 + 8,
    seeds = [b"user", user.key().as_ref()],
    bump
)]
pub user_account: Account<'info, UserAccount>,
```

- `seeds`: The seeds used to derive the PDA
- `bump`: Anchor finds the canonical bump automatically

You can also use a saved bump:

```rust
#[account(
    mut,
    seeds = [b"user", authority.key().as_ref()],
    bump = user_account.bump
)]
pub user_account: Account<'info, UserAccount>,
```

### 5. constraint - Custom Constraints

```rust
#[account(
    mut,
    constraint = user.balance >= 100 @ ErrorCode::InsufficientFunds,
    constraint = user.is_active @ ErrorCode::AccountInactive
)]
pub user: Account<'info, UserAccount>,
```

Custom constraints let you enforce arbitrary rules. The `@ ErrorCode::...` syntax specifies which error to return if the constraint fails.

## Space Calculation

When using `init`, you must specify the account size:

```rust
#[account]
pub struct UserAccount {
    pub authority: Pubkey,    // 32 bytes
    pub counter: u64,         // 8 bytes
    pub name: String,         // 4 + length (max 32) = 36 bytes
}

// Space = 8 (discriminator) + 32 + 8 + 36 = 84 bytes
```

**Common sizes**:
- Pubkey: 32 bytes
- u64/i64: 8 bytes
- u32/i32: 4 bytes
- bool: 1 byte
- String: 4 (length prefix) + max_length
- Vec<T>: 4 (length prefix) + (item_size * max_count)

## Constraint Execution Order

Anchor executes constraints in this order:
1. `init` / `init_if_needed`
2. `mut`
3. `seeds` + `bump`
4. `has_one`
5. Custom `constraint`s

This ensures accounts are properly initialized and validated before your instruction logic runs.

## Next Challenge

You'll implement account validation logic that mimics Anchor's constraint system.
