# Challenge: Simulate PDA Derivation in Rust

Understand how Program Derived Addresses work by implementing a simplified PDA derivation algorithm using Rust's standard library hashing.

Real Solana PDAs use SHA-256 and check if the result is on the Ed25519 curve. Here we simulate this concept using `DefaultHasher` and check if the hash is even (our proxy for "off-curve").

## Your Task

Implement `find_program_address(seeds, program_id)` that:

1. Iterates bumps from 255 down to 0
2. For each bump, hashes all seeds + bump + program_id together
3. Returns the first hash that is **even** (simulating "off-curve")
4. Returns `(hash, bump)` as a tuple

**Requirements:**
- Use `std::collections::hash_map::DefaultHasher`
- Hash each seed, then the bump byte, then the program_id
- Return the first even hash (hash % 2 == 0)
- This is deterministic: same seeds always produce the same result

**Rust Concepts Used:**
- `std::hash::{Hash, Hasher}` traits
- Iterating with `for bump in (0..=255u8).rev()`
- Tuples as return types
