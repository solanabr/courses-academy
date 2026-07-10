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

fn find_program_address(seeds: &[&str], program_id: &str) -> (u64, u8) {
    for bump in (0..=255u8).rev() {
        let hash = hash_with_bump(seeds, bump, program_id);
        if hash % 2 == 0 {
            return (hash, bump);
        }
    }
    panic!("No valid bump found");
}
