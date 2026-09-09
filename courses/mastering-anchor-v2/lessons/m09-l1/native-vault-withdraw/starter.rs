// The native quarter-vault's withdraw guard.
//
// In the framework version, Anchor generated the account checks for you. Stripping
// the framework, YOU write the validation by hand. This is the load-bearing branch of
// the native withdraw instruction: given the vault's current lamport `balance` and a
// requested `amount`, decide the outcome.
//
// Contract (return an i128 so we can signal failure without a Result in the harness):
//   - a zero-amount withdraw is invalid          -> return -2
//   - an over-withdraw (amount > balance)        -> return -1   (use checked_sub, no naive `-`)
//   - otherwise                                  -> return the remaining balance
//
// It is a `const fn` (pure integer decisions), so the compiler proves the guards
// at build time via the assertions at the bottom -- the same device as the
// m03-l3 constraint challenge. The starter below skips BOTH guards and subtracts
// naively. It happens to return the right number for a normal withdraw, but it is
// wrong (and unsafe) for the reject cases, and the build fails on the first one.
const fn vault_withdraw(balance: u64, amount: u64) -> i128 {
    // TODO: reject a zero-amount withdraw with -2
    // TODO: reject an over-withdraw with -1 using balance.checked_sub(amount)
    (balance as i128) - (amount as i128)
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
