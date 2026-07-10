# Challenge: Derive Multiple Related PDAs in Rust

Real Anchor programs often derive multiple PDAs per user — profile, stats, vault, etc. Each uses different seed prefixes but the same user key.

Using the simulated PDA derivation from earlier, derive 3 related PDAs and return their hashes as a tuple.

## Your Task

Implement `derive_user_pdas(user, program_id)` that:

1. Derives a **user PDA** with seeds `["user", user]`
2. Derives a **stats PDA** with seeds `["stats", user]`
3. Derives a **vault PDA** with seeds `["vault", user]`
4. Returns `(user_hash, stats_hash, vault_hash)` as a `(u64, u64, u64)` tuple

A helper `find_pda` is provided — use it to derive each PDA.

**Requirements:**
- All three hashes must be off-curve (even)
- All three must be different from each other
- Same inputs must always produce same outputs (deterministic)
