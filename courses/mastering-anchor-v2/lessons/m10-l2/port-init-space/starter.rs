/// Port a v1 account-space calculation to the Anchor V2 rule.
///
/// In Anchor v1 you sized an account by hand as `8 + <field bytes>`, where the
/// leading `8` was the account discriminator. V2 drops the magic number: the
/// idiom is `T::DISCRIMINATOR.len() + T::INIT_SPACE`, and INIT_SPACE is the sum
/// of the field sizes ONLY -- it never includes the discriminator.
///
/// The v1 helper below was already defensive: field counts reach it from a
/// caller, so every step is a `checked_*` and a bad count degrades to 0 rather
/// than wrapping into a plausible-looking length. Its last link,
/// `with_discriminator`, used to hold a `.checked_add(8)`. Then a migration
/// swept the file for the additive `8` of the v1 `space = 8 + ...` idiom and
/// gutted that link, leaving it handing its input straight through. The `* 8`
/// sizing a u64 field below survived the same sweep, correctly -- it is a field
/// width, not a magic number -- which is how a chain that still looks careful
/// ends up under-counting every account by exactly 8 bytes.
///
/// Name the tier, because the arithmetic below only holds for one of them: these
/// are the `#[account(borsh)]` sizes, fields written back to back with no
/// alignment padding and, since all three types are fixed-size, no length
/// prefixes either.
///   Address field = 32 bytes, u64 field = 8 bytes, bool field = 1 byte.
/// Under the Pod default these same three fields never reach a length at all --
/// see `error[E0080]` in the lesson text for why.
///
/// TODO: put the discriminator back. `account_len` must return the FULL on-chain
/// data length, and the 8 belongs INSIDE `with_discriminator`, checked like
/// every other link. It is deliberately not named `init_space`: INIT_SPACE is
/// the half that excludes the discriminator, and that name is how the bug got
/// written.

/// The chain's last link: from the INIT_SPACE half (field bytes only) to the
/// full on-chain data length. A `const fn`, so the compiler can prove it while
/// it builds (the m03-l3 device).
const fn with_discriminator(init_space: u64) -> Option<u64> {
    // TODO: the 8 goes here -- checked, so an overflowing count stays a
    // refusal. Right now this link passes its input through unchanged, and
    // every account is 8 bytes short.
    Some(init_space)
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
