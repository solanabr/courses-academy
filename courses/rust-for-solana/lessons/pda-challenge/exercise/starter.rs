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

/// Find a "program address" by trying bumps from 255 down to 0.
/// Returns the first (hash, bump) where hash is even (off-curve).
fn find_program_address(seeds: &[&str], program_id: &str) -> (u64, u8) {
    // TODO: Iterate bumps from 255 down to 0
    // TODO: For each bump, call hash_with_bump(seeds, bump, program_id)
    // TODO: If hash % 2 == 0, return (hash, bump)
    // TODO: If no bump works, panic!("No valid bump found")
    todo!()
}
