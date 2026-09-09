/// Quote the output of a constant-product swap.
///
/// This is the hot function on the swap instruction you have been profiling.
/// The optimized version must do THREE things the version below does not:
///   1. apply the trading fee (`fee_bps`, in basis points) before quoting,
///   2. compute the intermediate products in `u128` so large reserves cannot
///      silently overflow a `u64` multiply, and
///   3. refuse the inputs the curve is not defined on — an empty reserve, or a
///      `fee_bps` above the 10_000 bps scale — by returning 0, because the
///      frozen `u64` signature has nowhere to put an error.
///
/// Right now it ignores the fee entirely, multiplies in `u64`, and checks
/// nothing: its quotes are wrong whenever `fee_bps > 0`, it panics on a pool
/// deep enough to overflow the multiply, and on an empty `reserve_in` it quotes
/// the entire `reserve_out` — the whole pool — to whoever asks first. Fix it.
///
/// It is a `const fn` (pure integer arithmetic), so the compiler evaluates the
/// assertions at the bottom while it builds — the same device as the m03-l3
/// constraint challenge. A wrong quote fails an assertion with a message naming
/// the case, and the naive `u64` multiply OVERFLOWS in const evaluation on the
/// 1e18 vector: rustc making the lesson's own argument for the promotion.
///
/// Reference formula (Uniswap-style constant product with fee):
///   amount_in_with_fee = amount_in * (10_000 - fee_bps)
///   out = (reserve_out * amount_in_with_fee)
///         / (reserve_in * 10_000 + amount_in_with_fee)
const fn get_amount_out(reserve_in: u64, reserve_out: u64, amount_in: u64, fee_bps: u64) -> u64 {
    let _ = fee_bps; // TODO: the fee is being ignored
    let numerator = reserve_out * amount_in;
    let denominator = reserve_in + amount_in;
    numerator / denominator
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because `get_amount_out` is a `const fn`, the
// compiler evaluates these while building: an unguarded or fee-blind quote does
// not compile at all. The test vectors document the same contract; grading is
// compile-only, so this block is what enforces it.
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::get_amount_out;

    const _: () = assert!(
        get_amount_out(1_000_000, 1_000_000, 10_000, 30) == 9871,
        "30 bps on a balanced 1M/1M pool quotes 9871: the fee-blind version says 9900"
    );
    const _: () = assert!(
        get_amount_out(0, 1_000_000, 10_000, 30) == 0,
        "empty reserve_in must quote 0, never hand the whole pool to the first caller"
    );
    const _: () = assert!(
        get_amount_out(1_000_000, 1_000_000, 10_000, 10_001) == 0,
        "a fee above the 10_000 bps scale has no valid quote: guard it before it underflows"
    );
    const _: () = assert!(
        get_amount_out(1_000_000, 1_000_000, 10_000, 10_000) == 0,
        "fee_bps == 10_000 is the boundary, not an error: a 100% fee eats the whole input"
    );
    const _: () = assert!(
        get_amount_out(
            1_000_000_000_000_000_000,
            1_000_000_000_000_000_000,
            1_000_000_000_000,
            30
        ) == 996_999_005_991,
        "1e18 reserves overflow a u64 multiply: this vector forces the u128 promotion"
    );
}
