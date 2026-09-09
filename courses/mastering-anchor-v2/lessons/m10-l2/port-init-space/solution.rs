/// Port a v1 account-space calculation to the Anchor V2 rule.
///
///   space = T::DISCRIMINATOR.len() + T::INIT_SPACE
///
/// DISCRIMINATOR.len() is 8 (sha256 default, unchanged in V2); INIT_SPACE is the
/// sum of the field sizes and never includes the discriminator, so it is added
/// back exactly once here.
///
/// The restored 8 goes back where the migration took it from -- inside
/// `with_discriminator`, the chain's last link, checked like every other step.
/// Outside the chain, an overflowed count would return 8: a number that looks
/// like an empty account instead of the refusal it is.
///
/// The field sizes below are the `#[account(borsh)]` tier's: back-to-back
/// fields, no alignment padding, no length prefixes on fixed-size types.

/// The chain's last link: from the INIT_SPACE half (field bytes only) to the
/// full on-chain data length. A `const fn`, so the harness below proves the
/// restored link at build time.
const fn with_discriminator(init_space: u64) -> Option<u64> {
    const DISCRIMINATOR_LEN: u64 = 8;
    init_space.checked_add(DISCRIMINATOR_LEN)
}

fn account_len(address_fields: u64, u64_fields: u64, bool_fields: u64) -> u64 {
    address_fields
        .checked_mul(32)
        .and_then(|bytes| bytes.checked_add(u64_fields.checked_mul(8)?))
        .and_then(|bytes| bytes.checked_add(bool_fields))
        .and_then(with_discriminator)
        .unwrap_or(0)
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because `with_discriminator` is a `const fn`, the
// compiler evaluates these while building: a link that under-counts does not
// compile at all, and the message names the case it got wrong. The test vectors
// document the same contract on `account_len`; grading is compile-only, so this
// block is what enforces the restored link.
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::with_discriminator;

    const _: () = assert!(
        matches!(with_discriminator(41), Some(49)),
        "the full on-chain length is INIT_SPACE plus the 8-byte discriminator"
    );
    const _: () = assert!(
        matches!(with_discriminator(0), Some(8)),
        "an empty struct still carries the 8-byte discriminator"
    );
    const _: () = assert!(
        matches!(with_discriminator(u64::MAX), None),
        "the restored link is checked: an overflowing count is a refusal, never a wrap"
    );
}
