// Guard the vault's payout: decide how many lamports may safely leave.
//
// A program-owned vault PDA can sign its own withdrawal. Before the CPI fires,
// though, the program must decide whether the withdrawal is even legal.
// Implement `resolve_withdrawal` so it:
//   * returns -1 if `requested` is 0               (nothing to withdraw)
//   * returns -2 if `requested` exceeds `balance`  (would underflow the vault)
//   * returns -3 if the remainder would fall below `rent_exempt_min`
//                                                  (would risk closing the PDA)
//   * otherwise returns `requested` as i64         (safe to sign for)
//
// It is a `const fn` (pure integer decisions, nothing stops it), so the
// compiler proves the guards at build time via the assertions at the bottom --
// the same device as the m03-l3 constraint challenge. The starter below
// ignores every guard and just hands back what was asked, so the build fails
// on the zero-request case first.
const fn resolve_withdrawal(balance: u64, rent_exempt_min: u64, requested: u64) -> i64 {
    // TODO: check the three failure cases before returning the requested amount.
    let _ = (balance, rent_exempt_min);
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
