/// The decision core of a Token-2022 transfer hook's `Execute` handler.
///
/// On every SPROUT transfer, the runtime CPIs into your hook with the transfer
/// `amount` plus the resolved extra accounts. Your hook reads its own state and
/// decides: let the transfer proceed, or return an error so the whole transfer
/// reverts. It CANNOT move funds itself (the transfer accounts arrive read-only,
/// non-signer); its only power is to pass or fail.
///
/// Implement the gate for the Overgrowth harvest hook:
///   - If the hook is paused, reject with code `-2` (pause beats everything).
///   - Otherwise, if the destination is NOT on the treasury allowlist, reject
///     with code `-1`.
///   - Otherwise the transfer is allowed: return the `amount` (as i64).
///
/// Return value convention (so the grader can compare a single value):
///   allowed  -> the transferred `amount` cast to i64
///   rejected -> a negative error code (`-1` allowlist, `-2` paused)
pub fn hook_execute(destination_allowed: bool, is_paused: bool, amount: u64) -> i64 {
    // TODO: enforce the pause gate, then the allowlist gate, then pass the amount.
    amount as i64
}
