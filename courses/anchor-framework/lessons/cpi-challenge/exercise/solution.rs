use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_with_bump(seeds: &[&str], bump: u8, program_id: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    for seed in seeds { seed.hash(&mut hasher); }
    bump.hash(&mut hasher);
    program_id.hash(&mut hasher);
    hasher.finish()
}

fn find_pda(seeds: &[&str], program_id: &str) -> (u64, u8) {
    for bump in (0..=255u8).rev() {
        let hash = hash_with_bump(seeds, bump, program_id);
        if hash % 2 == 0 { return (hash, bump); }
    }
    panic!("No valid bump found");
}

fn build_vault_transfer(
    user: &str,
    vault_balance: u64,
    recipient_balance: u64,
    amount: u64,
) -> Result<(u64, u64, u8), String> {
    let (_, bump) = find_pda(&["vault", user], "system");
    if vault_balance < amount {
        return Err("INSUFFICIENT_VAULT_BALANCE".to_string());
    }
    Ok((vault_balance - amount, recipient_balance + amount, bump))
}
