// Patch the withdraw guard: the class the V2 compiler does NOT catch for you.
//
// This is the escrow/swap payout logic, distilled to a pure function so it grades
// deterministically. Two vulnerability classes from this lesson live here, one
// per helper:
//   1. Access control (`is_authority`): only the vault authority may withdraw.
//   2. Arithmetic (`checked_remaining`): an over-withdraw must be rejected,
//      never wrapped or panicked.
//
// `Address` is the real shape of an on-chain address: 32 bytes, which is exactly
// what `pinocchio::address::Address` is. Two addresses are the same only if all
// 32 bytes match, and an address has no meaningful ordering — the only comparison
// an access-control check is allowed to make is equality. In a real handler that
// is one `!=`; here the gate is a `const fn` so the compiler can prove it while
// it builds (the m03-l3 device), and `==` on arrays is a trait call a `const fn`
// cannot make on stable Rust — so prove equality the way the machine does, byte
// by byte, all 32 of them.
//
// Return convention (so the grader can value-compare):
//   >= 0  -> the new balance after a successful withdraw
//     -1  -> rejected: caller is not the authority
//     -2  -> rejected: amount would underflow the balance
//
// The starter ships the vulnerable version: a gate that admits everyone, and a
// raw subtraction. `settle_withdraw` is wired; the two guards are the exercise.
type Address = [u8; 32];

/// Gate the withdraw: is `caller` the vault authority?
const fn is_authority(caller: &Address, authority: &Address) -> bool {
    // TODO: true only when ALL 32 bytes match. A prefix, a single byte, or an
    // ordering comparison is the vulnerability, not the gate. Right now every
    // caller passes.
    let _ = (caller, authority);
    true
}

/// Settle the arithmetic: the balance left after the withdraw, or -2.
const fn checked_remaining(balance: u64, amount: u64) -> i64 {
    // TODO: use checked arithmetic so an over-withdraw returns -2 instead of
    // underflowing. This raw `-` is the drain.
    (balance - amount) as i64
}

fn settle_withdraw(balance: u64, amount: u64, caller: Address, authority: Address) -> i64 {
    if !is_authority(&caller, &authority) {
        return -1;
    }
    checked_remaining(balance, amount)
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because both guards are `const fn`s, the compiler
// evaluates these while building: an open gate fails with a message naming the
// caller it let through, and the raw subtraction overflows in const evaluation
// on the over-withdraw case. The test vectors document the same contract;
// grading is compile-only, so this block is what enforces it.
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::{checked_remaining, is_authority};

    const _: () = assert!(
        is_authority(&[7u8; 32], &[7u8; 32]),
        "the authority must pass its own gate"
    );
    const _: () = assert!(
        !is_authority(&[9u8; 32], &[7u8; 32]),
        "a non-authority caller must be rejected: the gate is not written yet"
    );
    const _: () = assert!(
        !is_authority(&[3u8; 32], &[7u8; 32]),
        "a caller that sorts BELOW the authority is still not the authority: no ordering"
    );
    const _: () = {
        let mut last_byte_off = [7u8; 32];
        last_byte_off[31] = 8;
        assert!(
            !is_authority(&last_byte_off, &[7u8; 32]),
            "near-miss in the LAST byte: a prefix comparison lets this address drain the vault"
        );
    };
    const _: () = {
        let mut first_byte_off = [7u8; 32];
        first_byte_off[0] = 8;
        assert!(
            !is_authority(&first_byte_off, &[7u8; 32]),
            "near-miss in the FIRST byte: only a full 32-byte equality passes both near-misses"
        );
    };
    const _: () = assert!(
        checked_remaining(100, 30) == 70,
        "a withdraw within balance returns the new balance"
    );
    const _: () = assert!(
        checked_remaining(50, 50) == 0,
        "an exact-balance withdraw returns zero"
    );
    const _: () = assert!(
        checked_remaining(30, 100) == -2,
        "an over-withdraw must be the -2 sentinel: a raw `-` underflows right here"
    );
}
