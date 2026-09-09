// Anchor derives every 8-byte discriminator by hashing a NAMESPACED preimage:
// sha256("<namespace>:<Name>")[..8]. Instruction handlers hash from the "global:"
// namespace, not "instruction:", which is the one mapping that trips people up.
//
// `namespace` is a `const fn`, so `match` cannot compare `&str` directly here
// (string equality is a trait call, and trait calls are not `const` on stable);
// matching on the bytes is the const-compatible spelling of the same mapping.
const fn namespace<'a>(item_kind: &'a str) -> &'a str {
    match item_kind.as_bytes() {
        b"account" => "account",
        b"instruction" => "global",
        b"event" => "event",
        _ => item_kind,
    }
}

fn discriminator_preimage(item_kind: &str, name: &str) -> String {
    format!("{}:{name}", namespace(item_kind))
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because `namespace` is a `const fn`, the compiler
// evaluates these while building: an unfixed mapping does not compile at all,
// and the message names the case it got wrong. The test vectors document the
// same contract; grading is compile-only, so this block is what enforces it.
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::namespace;

    const fn str_eq(a: &str, b: &str) -> bool {
        let (a, b) = (a.as_bytes(), b.as_bytes());
        if a.len() != b.len() {
            return false;
        }
        let mut i = 0;
        while i < a.len() {
            if a[i] != b[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    const _: () = assert!(
        str_eq(namespace("account"), "account"),
        "an account struct hashes from the account: namespace"
    );
    const _: () = assert!(
        str_eq(namespace("instruction"), "global"),
        "an instruction handler hashes from global:, not instruction: - this is the trap"
    );
    const _: () = assert!(
        str_eq(namespace("event"), "event"),
        "an event struct hashes from the event: namespace"
    );
}
