# Challenge: Manual Borsh-Style Serialization in Rust

To truly understand how Borsh works under the hood, you'll manually serialize account data to bytes using Rust's standard library.

## Your Task

Implement `serialize_account(owner, balance, frozen)` that converts account fields to a `Vec<u8>` with this Borsh-like layout:

- **4 bytes**: owner name length as `u32` (little-endian)
- **N bytes**: owner name as UTF-8 bytes
- **8 bytes**: balance as `u64` (little-endian)
- **1 byte**: frozen flag (`0` = false, `1` = true)

**Requirements:**
- Use `u32::to_le_bytes()` for the length prefix
- Use `.as_bytes()` to get UTF-8 bytes from a `&str`
- Use `u64::to_le_bytes()` for the balance
- Push `0u8` or `1u8` for the boolean
- Total size should be `4 + owner.len() + 8 + 1` bytes

**Rust APIs:**
- `Vec::new()`, `Vec::extend_from_slice()`
- `u32::to_le_bytes()`, `u64::to_le_bytes()`
- `str::as_bytes()`, `str::len()`
