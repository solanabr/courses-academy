// token<->ticket swap: quote the output amount from the pool's own reserves.
//
// The arcade pool holds `reserve_in` of the token you pay with and `reserve_out`
// of the token you want. Given `amount_in`, return how many output tokens the
// trader receives, applying a 0.3% fee (multiply the input by 997/1000) under the
// constant-product invariant (x * y = k).
//
// It is a `const fn` (pure integer arithmetic), so the compiler evaluates the
// assertions at the bottom while it builds -- the same device as the m03-l3
// constraint challenge. Two kinds of failure surface there: a wrong quote fails
// an assertion with a message naming the case, and the naive u64 multiply
// OVERFLOWS in const evaluation on the 1e18 vector, which is rustc telling you
// the same thing the lesson does: promote to u128 and keep it checked.
//
// TODO: replace the naive body below. This version just scales by the *current*
// price ratio: it ignores the 0.3% fee AND the fact that adding `amount_in` shifts
// the reserves, so it over-quotes and lets a trader drain the pool.
const fn swap_out(reserve_in: u64, reserve_out: u64, amount_in: u64) -> u64 {
    if reserve_in == 0 {
        return 0;
    }
    amount_in * reserve_out / reserve_in
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because `swap_out` is a `const fn`, the compiler
// evaluates these while building: a naive quote does not compile at all. The
// test vectors document the same contract; grading is compile-only, so this
// block is what enforces it.
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::swap_out;

    const _: () = assert!(
        swap_out(1000, 1000, 100) == 90,
        "0.3% fee plus the reserve shift on a shallow pool: the naive ratio says 100, the curve says 90"
    );
    const _: () = assert!(
        swap_out(1_000_000, 1_000_000, 10_000) == 9871,
        "the lesson's worked example: 10k into a balanced 1M/1M pool quotes 9871"
    );
    const _: () = assert!(
        swap_out(0, 1_000_000, 10_000) == 0,
        "empty reserve_in is not on the curve: quote 0, never the whole pool"
    );
    const _: () = assert!(
        swap_out(1_000_000_000_000_000_000, 1_000_000_000_000_000_000, 1_000_000_000_000)
            == 996_999_005_991,
        "1e18 reserves overflow a u64 multiply: this vector forces the u128 promotion"
    );
    const _: () = assert!(
        swap_out(u64::MAX, u64::MAX, u64::MAX) == 0,
        "u64::MAX everywhere runs past u128 too: the CHECKED multiply must degrade to 0"
    );
}
