// Patch the withdraw guard: the class the V2 compiler does NOT catch for you.
//
// `Address` is 32 bytes, the shape `pinocchio::address::Address` really has. The
// authority gate is a full-width equality check: comparing a prefix, a single
// byte, or (worse) an ordering would let a near-miss address through, and a
// near-miss address is an attacker's address. In a real handler the check is one
// `caller != authority`; here the gate is a `const fn` so the harness can prove
// it at compile time, and `==` on arrays is a trait call a `const fn` cannot
// make on stable Rust — so the equality is spelled out the way the machine runs
// it anyway: every byte, all 32.
//
// Return convention:
//   >= 0  -> the new balance after a successful withdraw
//     -1  -> rejected: caller is not the authority
//     -2  -> rejected: amount would underflow the balance
type Address = [u8; 32];

/// Gate the withdraw: full 32-byte equality, nothing cheaper.
const fn is_authority(caller: &Address, authority: &Address) -> bool {
    let mut i = 0;
    while i < 32 {
        if caller[i] != authority[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Settle the arithmetic: checked_sub returns None exactly when the withdraw
/// would underflow, and None becomes the -2 sentinel.
const fn checked_remaining(balance: u64, amount: u64) -> i64 {
    match balance.checked_sub(amount) {
        Some(remaining) => remaining as i64,
        None => -2,
    }
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
