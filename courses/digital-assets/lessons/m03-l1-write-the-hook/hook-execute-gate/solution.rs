/// The decision core of a Token-2022 transfer hook's `Execute` handler.
///
/// On every SPROUT transfer, the runtime CPIs into your hook with the transfer
/// `amount` plus the resolved extra accounts. Your hook reads its own state and
/// decides: let the transfer proceed, or return an error so the whole transfer
/// reverts. It CANNOT move funds itself (the transfer accounts arrive read-only,
/// non-signer); its only power is to pass or fail.
///
/// Return value convention (so the grader can compare a single value):
///   allowed  -> the transferred `amount` cast to i64
///   rejected -> a negative error code (`-1` allowlist, `-2` paused)
pub fn hook_execute(destination_allowed: bool, is_paused: bool, amount: u64) -> i64 {
    // Pause is the global kill-switch: it beats the allowlist.
    if is_paused {
        return -2;
    }
    // The hook's only real power on the transfer path: refuse.
    if !destination_allowed {
        return -1;
    }
    amount as i64
}
