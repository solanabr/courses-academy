// Anchor derives every 8-byte discriminator by hashing a NAMESPACED preimage:
// sha256("<namespace>:<Name>")[..8]. The bytes come later; the preimage STRING is
// the part you have to get right, and one of the three namespaces is a classic trap.
//
// `namespace` maps an item kind to the namespace Anchor actually hashes from:
//   - an account struct       -> "account"
//   - an instruction handler  -> "global"     <-- NOT "instruction"
//   - an event struct         -> "event"
//
// It is a `const fn` so the compiler can evaluate it while it builds: the
// assertions at the bottom prove the mapping at COMPILE time -- a device this
// course leans on again in the m03-l3 constraint challenge. The starter below
// just echoes the item kind, so the preimage comes out "instruction:increment"
// instead of "global:increment" and the build fails on that exact case.
const fn namespace<'a>(item_kind: &'a str) -> &'a str {
    // TODO: map each item_kind to its real Anchor namespace. Only one of the
    // three differs from its item kind -- that one is the whole exercise.
    item_kind
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
