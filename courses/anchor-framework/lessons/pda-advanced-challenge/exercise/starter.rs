use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_with_bump(seeds: &[&str], bump: u8, program_id: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    for seed in seeds {
        seed.hash(&mut hasher);
    }
    bump.hash(&mut hasher);
    program_id.hash(&mut hasher);
    hasher.finish()
}

fn find_pda(seeds: &[&str], program_id: &str) -> (u64, u8) {
    for bump in (0..=255u8).rev() {
        let hash = hash_with_bump(seeds, bump, program_id);
        if hash % 2 == 0 {
            return (hash, bump);
        }
    }
    panic!("No valid bump found");
}

/// Derive user, stats, and vault PDAs for the given user.
/// Returns (user_hash, stats_hash, vault_hash).
fn derive_user_pdas(user: &str, program_id: &str) -> (u64, u64, u64) {
    // TODO: Derive user PDA with seeds ["user", user]
    // TODO: Derive stats PDA with seeds ["stats", user]
    // TODO: Derive vault PDA with seeds ["vault", user]
    // TODO: Return the three hashes as a tuple
    todo!()
}
