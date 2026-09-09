// token<->ticket swap: quote the output amount from the pool's own reserves.
//
// Uniswap-v2 constant-product invariant with a 0.3% fee (997/1000). A u128
// intermediate keeps the multiplication from overflowing on u64 reserves, and
// every step is checked so an overflow degrades to a 0 quote rather than panicking
// inside an on-chain instruction. A `const fn`, so the harness below proves the
// quote at build time.
//
//   amount_in_with_fee = amount_in * 997
//   out = (amount_in_with_fee * reserve_out) / (reserve_in * 1000 + amount_in_with_fee)
const fn swap_out(reserve_in: u64, reserve_out: u64, amount_in: u64) -> u64 {
    if amount_in == 0 || reserve_in == 0 || reserve_out == 0 {
        return 0;
    }
    let fee_adjusted = match (amount_in as u128).checked_mul(997) {
        Some(v) => v,
        None => return 0,
    };
    let numerator = match fee_adjusted.checked_mul(reserve_out as u128) {
        Some(v) => v,
        None => return 0,
    };
    let denominator = match (reserve_in as u128).checked_mul(1000) {
        Some(v) => match v.checked_add(fee_adjusted) {
            Some(v) => v,
            None => return 0,
        },
        None => return 0,
    };
    (numerator / denominator) as u64
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
