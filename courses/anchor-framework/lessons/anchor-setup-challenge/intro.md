# Challenge: Model Anchor Program Structure in Rust

Anchor programs follow a specific pattern: instructions contain account metadata and serialized data. You'll model this structure using Rust structs and enums.

## Your Task

Implement `build_instruction(program_id, user_key, instruction_name)` that constructs an Anchor-style instruction:

1. Encode the instruction name as bytes and compute its length
2. Build account metadata for two accounts:
   - User account: signer=true, writable=true
   - System program (`"11111111111111111111111111111111"`): signer=false, writable=false
3. Return a formatted string: `"program:<id>,accounts:<count>,data_len:<len>"`

**This mirrors Anchor's `#[derive(Accounts)]` pattern** where each instruction declares which accounts it needs and how they should be validated.

**Rust Concepts Used:**
- Structs with boolean fields
- `Vec` collection
- String formatting with `format!()`
- Byte conversion with `.as_bytes()`
