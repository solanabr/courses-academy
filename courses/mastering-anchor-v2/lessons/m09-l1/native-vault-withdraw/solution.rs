// The native quarter-vault's withdraw guard — reference solution.
//
// Every check Anchor V2 would have generated (data validation, checked arithmetic) is
// now hand-written. Zero-amount and over-withdraw are both rejected before any lamport
// moves; the subtraction uses checked_sub so an underflow can never wrap. A `const fn`,
// so the harness below proves the guards at build time.
const fn vault_withdraw(balance: u64, amount: u64) -> i128 {
    if amount == 0 {
        return -2;
    }
    match balance.checked_sub(amount) {
        Some(remaining) => remaining as i128,
        None => -1,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because `vault_withdraw` is a `const fn`, the
// compiler evaluates these while building: an unguarded withdraw does not
// compile at all, and the message names the case it got wrong. The test vectors
// document the same contract; grading is compile-only, so this block is what
// enforces it.
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::vault_withdraw;

    const _: () = assert!(
        vault_withdraw(100, 40) == 60,
        "a normal withdraw returns the remaining balance"
    );
    const _: () = assert!(
        vault_withdraw(100, 0) == -2,
        "a zero-amount withdraw is rejected with -2, not treated as a no-op"
    );
    const _: () = assert!(
        vault_withdraw(100, 102) == -1,
        "an over-withdraw by exactly 2 must be -1: naive subtraction lands on -2, the WRONG sentinel"
    );
    const _: () = assert!(
        vault_withdraw(0, 5) == -1,
        "withdrawing from an empty vault is an over-withdraw (-1), never a zero-amount rejection"
    );
    const _: () = assert!(
        vault_withdraw(50, 50) == 0,
        "draining the vault to exactly zero is allowed"
    );
    const _: () = assert!(
        vault_withdraw(u64::MAX, 1) == (u64::MAX - 1) as i128,
        "a u64::MAX balance survives: the i128 return exists so the full u64 range fits"
    );
}
