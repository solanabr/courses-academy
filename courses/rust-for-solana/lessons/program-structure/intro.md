# Solana Program Structure

Solana programs (smart contracts) follow a specific structure. Understanding this architecture is essential before writing your first program.

## The Entrypoint Macro

Every Solana program starts with the `entrypoint!` macro, which defines the entry function the runtime calls:

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Your program logic here
    Ok(())
}
```

## The Three Parameters

### 1. `program_id: &Pubkey`

The public key (address) of your deployed program. Use this to:
- Verify that accounts are owned by your program
- Derive PDAs (Program Derived Addresses)
- Check that the correct program was called

```rust
if account.owner != program_id {
    return Err(ProgramError::IncorrectProgramId);
}
```

### 2. `accounts: &[AccountInfo]`

A slice of all accounts passed to the instruction. Each `AccountInfo` contains:

```rust
pub struct AccountInfo<'a> {
    pub key: &'a Pubkey,           // Account's public key
    pub lamports: Rc<RefCell<&'a mut u64>>, // Balance in lamports
    pub data: Rc<RefCell<&'a mut [u8]>>,    // Raw account data
    pub owner: &'a Pubkey,         // Program that owns this account
    pub rent_epoch: u64,           // When rent is next due
    pub is_signer: bool,           // True if account signed the transaction
    pub is_writable: bool,         // True if account is mutable
    pub executable: bool,          // True if account is a program
}
```

Access accounts using an iterator:

```rust
let account_iter = &mut accounts.iter();
let sender = next_account_info(account_iter)?;
let recipient = next_account_info(account_iter)?;
let system_program = next_account_info(account_iter)?;
```

### 3. `instruction_data: &[u8]`

Raw bytes representing the instruction. Deserialize to determine what action to perform:

```rust
let instruction = MyInstruction::try_from_slice(instruction_data)?;

match instruction {
    MyInstruction::Initialize { amount } => process_initialize(accounts, amount),
    MyInstruction::Transfer { amount } => process_transfer(accounts, amount),
    MyInstruction::Close => process_close(accounts),
}
```

## Account Validation

**Critical**: Always validate accounts before using them. Attackers can pass malicious accounts to exploit your program.

```rust
pub fn process_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let sender = next_account_info(account_iter)?;
    let recipient = next_account_info(account_iter)?;
    
    // 1. Verify sender signed the transaction
    if !sender.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // 2. Verify sender is writable (we'll modify balance)
    if !sender.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // 3. Verify sender is owned by this program
    if sender.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // 4. Verify recipient is writable
    if !recipient.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Now safe to proceed
    // ...
    Ok(())
}
```

## Common Validation Checks

| Check | Prevents |
|-------|----------|
| `!is_signer` | Unauthorized access |
| `!is_writable` | Mutating read-only accounts |
| `owner != program_id` | Cross-program account hijacking |
| `accounts.len() < N` | Index out of bounds |
| `data.len() < expected` | Uninitialized accounts |
| `key != expected_key` | Account substitution attacks |

## Program Flow

```rust
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 1. Parse instruction
    let instruction = MyInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    
    // 2. Route to handler based on instruction type
    match instruction {
        MyInstruction::Initialize { amount } => {
            msg!("Instruction: Initialize");
            process_initialize(program_id, accounts, amount)
        }
        MyInstruction::Transfer { amount } => {
            msg!("Instruction: Transfer");
            process_transfer(program_id, accounts, amount)
        }
    }
}

fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // 3. Validate accounts
    // 4. Deserialize account data
    // 5. Perform business logic
    // 6. Serialize account data back
    Ok(())
}
```

## Key Principles

1. **Validate everything**: Never trust input data or accounts
2. **Fail early**: Return errors before mutating state
3. **Log actions**: Use `msg!()` to help debug transactions
4. **Minimize compute**: Solana charges for compute units used
5. **Test thoroughly**: Write unit tests for all code paths

In the next challenge, you'll build a complete instruction handler that validates accounts and processes a transfer!
