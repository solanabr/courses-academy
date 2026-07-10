# Structs and Enums for Solana Programs

Solana programs store data in accounts and process instructions. Rust's `struct` and `enum` types are perfect for modeling both.

## Structs for Account Data

A `struct` groups related data together. In Solana, you'll use structs to define the layout of account data:

```rust
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserAccount {
    pub owner: Pubkey,
    pub balance: u64,
    pub last_login: i64,
    pub is_frozen: bool,
}
```

### Derive Macros

`#[derive(...)]` automatically implements common traits:
- `BorshSerialize`: Converts the struct to bytes for storage
- `BorshDeserialize`: Reconstructs the struct from bytes
- `Debug`: Enables `println!("{:?}", user)` for debugging
- `Clone`, `Copy`, `PartialEq`: Other useful traits

## Enums for Instruction Types

An `enum` represents a value that can be one of several variants. Use enums to define the different instructions your program accepts:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TokenInstruction {
    /// Initialize a new token mint
    InitializeMint { decimals: u8 },
    
    /// Transfer tokens from one account to another
    Transfer { amount: u64 },
    
    /// Burn tokens from an account
    Burn { amount: u64 },
}
```

Each variant can hold different data types, making enums incredibly flexible.

## Pattern Matching

Use `match` to handle different enum variants. Rust's compiler ensures you handle ALL cases:

```rust
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TokenInstruction::try_from_slice(instruction_data)?;
    
    match instruction {
        TokenInstruction::InitializeMint { decimals } => {
            msg!("Initializing mint with {} decimals", decimals);
            process_init_mint(accounts, decimals)
        }
        TokenInstruction::Transfer { amount } => {
            msg!("Transferring {} tokens", amount);
            process_transfer(accounts, amount)
        }
        TokenInstruction::Burn { amount } => {
            msg!("Burning {} tokens", amount);
            process_burn(accounts, amount)
        }
    }
}
```

## Borsh Serialization

Borsh (Binary Object Representation Serializer for Hashing) is Solana's preferred serialization format:
- **Deterministic**: Same data always serializes to the same bytes
- **Compact**: No field names or type tags in the output
- **Fast**: Optimized for blockchain use cases

```rust
let user = UserAccount {
    owner: pubkey,
    balance: 1000,
    last_login: 1640000000,
    is_frozen: false,
};

// Serialize to bytes
let bytes = user.try_to_vec()?;

// Deserialize from bytes
let restored = UserAccount::try_from_slice(&bytes)?;
```

In the next lessons, we'll dive deeper into Borsh and see how to manually serialize data when needed.
