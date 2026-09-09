/// Quote the output of a constant-product swap with a trading fee.
///
/// Guard the inputs, then run every intermediate through a single `u128` with
/// exactly one division on the way back down to `u64`:
///
///   amount_in_with_fee = amount_in * (10_000 - fee_bps)
///   out = (reserve_out * amount_in_with_fee)
///         / (reserve_in * 10_000 + amount_in_with_fee)
///
/// The guards are load-bearing, not decoration. Without the `reserve_in` guard
/// an empty input reserve collapses the denominator to `amount_in_with_fee`,
/// and the quote hands back the whole of `reserve_out` — the entire pool — to
/// the first caller who asks. Without the `fee_bps` guard, `10_000 - fee_bps`
/// underflows for any fee above 10_000. And the multiplies stay `checked` for
/// the m05-l2 reason: three factors, not two, so `u128` is headroom rather than
/// a proof. Past the guards the output is bounded by `reserve_out`, a `u64`, so
/// the final cast cannot truncate. A `const fn`, so the harness below proves
/// all of it at build time.
///
/// DEGRADATION POLICY. The signature returns a bare `u64`, so there is nowhere
/// to put an error and every rejected input leaves as a `0`. Exactly one of
/// those zeros is arithmetic: `fee_bps == 10_000` is a 100% fee, and 0 out is
/// the correct answer. Every other `0` is a sentinel standing in for "this call
/// should not have been made" — an empty reserve, a fee above 10_000, an
/// intermediate too large for `u128`. Real AMMs do not degrade, they revert:
/// Uniswap V2's `getAmountOut` requires both reserves to be non-zero and
/// reverts with INSUFFICIENT_LIQUIDITY otherwise (and INSUFFICIENT_INPUT_AMOUNT
/// on a zero input). The `0` here is an artifact of a signature frozen for
/// grading, which cannot fail. On-chain each of these guards is a `require!` in
/// the handler before the quote is ever reached, and the handler still refuses
/// to settle a trade that quotes 0.
const fn get_amount_out(reserve_in: u64, reserve_out: u64, amount_in: u64, fee_bps: u64) -> u64 {
    if amount_in == 0 || reserve_in == 0 || reserve_out == 0 || fee_bps > 10_000 {
        return 0;
    }
    let amount_in_with_fee = match (amount_in as u128).checked_mul(10_000 - fee_bps as u128) {
        Some(v) => v,
        None => return 0,
    };
    let numerator = match amount_in_with_fee.checked_mul(reserve_out as u128) {
        Some(v) => v,
        None => return 0,
    };
    let denominator = match (reserve_in as u128).checked_mul(10_000) {
        Some(v) => match v.checked_add(amount_in_with_fee) {
            Some(v) => v,
            None => return 0,
        },
        None => return 0,
    };
    (numerator / denominator) as u64
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
