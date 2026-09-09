// Guard the vault's payout: decide how many lamports may safely leave.
//
// The three failure cases are checked BEFORE computing the safe remainder, and
// the subtraction still goes through `checked_sub` even though the guard above
// already proved it cannot underflow: the proof is one refactor away from being
// wrong, and a bare `-` either aborts the transaction or, on a build with
// overflow-checks off, wraps silently. A `const fn`, so the
// harness below proves the guards at build time.
const fn resolve_withdrawal(balance: u64, rent_exempt_min: u64, requested: u64) -> i64 {
    if requested == 0 {
        return -1; // nothing to withdraw
    }
    if requested > balance {
        return -2; // would underflow the vault
    }
    let Some(remaining) = balance.checked_sub(requested) else {
        return -2; // unreachable while the guard above holds; still not a bare `-`
    };
    if remaining < rent_exempt_min {
        return -3; // would drop the PDA below rent-exempt and risk closing it
    }
    requested as i64
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because `resolve_withdrawal` is a `const fn`, the
// compiler evaluates these while building: an unguarded payout does not compile
// at all, and the message names the case it got wrong. The test vectors
// document the same contract; grading is compile-only, so this block is what
// enforces it.
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::resolve_withdrawal;

    const _: () = assert!(
        resolve_withdrawal(1_000_000, 890_880, 100_000) == 100_000,
        "a withdrawal that leaves the vault rent-exempt returns the requested amount"
    );
    const _: () = assert!(
        resolve_withdrawal(1_000_000, 890_880, 0) == -1,
        "a zero request must be refused with -1"
    );
    const _: () = assert!(
        resolve_withdrawal(500_000, 0, 900_000) == -2,
        "an over-withdraw must be refused with -2, never allowed to underflow"
    );
    const _: () = assert!(
        resolve_withdrawal(1_000_000, 950_000, 100_000) == -3,
        "a remainder below the rent-exempt minimum must be refused with -3"
    );
    const _: () = assert!(
        resolve_withdrawal(1_000_000, 890_880, 109_120) == 109_120,
        "the rent floor is inclusive: leaving EXACTLY rent_exempt_min is allowed"
    );
    const _: () = assert!(
        resolve_withdrawal(1_000_000, 890_880, 109_121) == -3,
        "one lamport past the floor: leaving rent_exempt_min - 1 is refused with -3"
    );
    const _: () = assert!(
        resolve_withdrawal(500_000, 600_000, 900_000) == -2,
        "an over-withdraw that also breaches the rent floor is still -2: balance is checked first"
    );
}
