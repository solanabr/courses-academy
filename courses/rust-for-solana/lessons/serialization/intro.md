# Borsh Serialization for Solana

Solana programs store data in accounts as raw bytes. To work with structured data (structs, enums), you need a serialization format. Solana uses **Borsh** (Binary Object Representation Serializer for Hashing).

## Why Borsh?

Borsh was designed specifically for blockchain use cases and offers several advantages over alternatives like JSON or MessagePack:

### 1. Deterministic Serialization

The same data **always** produces the same byte sequence. This is critical for:
- **Consensus**: All validators must agree on account state
- **Hashing**: Content-addressed storage requires consistent hashes
- **Merkle proofs**: Deterministic encoding enables efficient proofs

```rust
// JSON is NOT deterministic (field order can vary):
{"balance": 100, "owner": "abc"} vs {"owner": "abc", "balance": 100}

// Borsh IS deterministic:
struct { balance: 100, owner: "abc" } → always [100, 0, 0, 0, 3, 0, 0, 0, 97, 98, 99]
```

### 2. Compact Encoding

Borsh doesn't include field names or type metadata in the output:

```json
// JSON: 42 bytes
{"balance":1000000,"frozen":false}

// Borsh: 9 bytes (4 for u64 + 4 for length prefix + 1 for bool)
```

Solana charges rent based on account size, so smaller serialization = lower costs.

### 3. Fixed vs Variable-Size Data

**Fixed-size types** (primitives) have a known byte length:
- `u8`: 1 byte
- `u32`, `i32`: 4 bytes
- `u64`, `i64`: 8 bytes
- `bool`: 1 byte (0 or 1)
- `[u8; 32]`: 32 bytes (fixed array)

**Variable-size types** include a 4-byte length prefix:
- `String`: 4-byte length + UTF-8 bytes
- `Vec<T>`: 4-byte length + serialized elements
- `Option<T>`: 1-byte discriminant (0 or 1) + value if Some

```rust
// Example encoding:
let name = "Alice"; // Borsh: [5, 0, 0, 0, 65, 108, 105, 99, 101]
                    //         ^^^^len=5  ^^^^UTF-8 bytes

let numbers = vec![1u32, 2, 3]; // [3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]
                                //  ^^^^len=3  ^^^^^^^^^^ three u32s
```

## How Solana Uses Borsh

### Account Data

Programs deserialize account data at the start of `process_instruction`:

```rust
let user_account = UserAccount::try_from_slice(&account.data.borrow())?;
```

After modifying the struct, serialize it back:

```rust
user_account.balance += amount;
user_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
```

### Instruction Data

Clients serialize instructions before sending transactions:

```rust
let instruction = TokenInstruction::Transfer { amount: 1000 };
let data = instruction.try_to_vec()?; // Serialize to bytes
```

The program deserializes it to determine which function to call:

```rust
let instruction = TokenInstruction::try_from_slice(instruction_data)?;
match instruction { /* ... */ }
```

## Comparison with JSON

| Feature | Borsh | JSON |
|---------|-------|------|
| Deterministic | ✅ Yes | ❌ No (field order) |
| Size | Small (no field names) | Large (verbose) |
| Speed | Fast (binary) | Slow (text parsing) |
| Human-readable | ❌ No | ✅ Yes |
| Schema required | ✅ Yes (Rust structs) | ❌ No |

JSON is great for APIs and config files, but Borsh is the clear winner for on-chain data.

## Next Steps

In the next challenge, you'll manually serialize and deserialize data using JavaScript's `DataView` and `TextEncoder` to understand how Borsh encoding works under the hood.
