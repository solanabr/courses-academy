// Anchor V2 lets you ship your OWN constraint namespace by implementing the
// `AccountConstraint` trait's hooks (init / check / update / exit). Downstream
// crates then write `#[account(quarters::min_balance = N)]` with no framework
// change. This is the scratch file from the top of the lesson: the same two
// names, distilled to plain Rust so you can prove the rule without the macro.
//
// Nothing here is Anchor's API. `BalanceGate` and `MinBalanceRule` stand in for
// the real trait and marker type, which have a different shape (static methods,
// an associated Value, a program error). `meets_floor` is not a hook at all --
// it is a local `const fn` this file owns, and it exists because the compiler
// can evaluate a `const fn` while it builds, so the assertions at the bottom
// prove your rule at COMPILE time as well as against the test vectors. A trait
// method cannot be `const` on stable Rust, which is the whole reason the
// condition sits beside the impl rather than inside it. `check` owns the hook's
// shape; `meets_floor` owns its rule.
//
// TODO: implement `MinBalanceRule::meets_floor` so it is false when `balance` is
// below `self.min` and true otherwise. `check` and `run_constraint` are wired.

pub trait BalanceGate {
    /// Mirrors the check hook Anchor V2 runs after the account is loaded.
    fn check(&self, balance: u64) -> Result<(), String>;
}

pub struct MinBalanceRule {
    pub min: u64,
}

impl MinBalanceRule {
    /// The floor itself, and the whole exercise. The floor is inclusive.
    const fn meets_floor(&self, balance: u64) -> bool {
        // TODO: compare `balance` against `self.min`. Right now the constraint
        // never rejects anything, so every vault sails through.
        let _ = balance;
        true
    }
}

impl BalanceGate for MinBalanceRule {
    /// Given: the hook forwards to the condition above and turns a `false` into
    /// the error codegen would surface from the constraint.
    fn check(&self, balance: u64) -> Result<(), String> {
        if self.meets_floor(balance) {
            Ok(())
        } else {
            Err(format!(
                "quarters::min_balance violated: {} < {}",
                balance, self.min
            ))
        }
    }
}

fn run_constraint(balance: u64, min: u64) -> bool {
    MinBalanceRule { min }.check(balance).is_ok()
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because `meets_floor` is a `const fn`, the compiler
// evaluates these while building: an unfixed hook does not compile at all, and
// the message names the case it got wrong. The test vectors document the same
// contract but never execute — grading is compile-only — so this block is the
// gate, and it pins what the vectors alone could not force anyway: that the
// rule reads `self.min` rather than a hardcoded constant, and that the floor
// is inclusive rather than off by one. (Deleting the block compiles, but the
// answer key is public; the assertions exist to measure you, not to hide it.)
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::MinBalanceRule;

    const _: () = assert!(
        MinBalanceRule { min: 100 }.meets_floor(500),
        "a balance above the floor must pass"
    );
    const _: () = assert!(
        MinBalanceRule { min: 100 }.meets_floor(100),
        "the floor is inclusive: a balance exactly at the floor must pass"
    );
    const _: () = assert!(
        !MinBalanceRule { min: 100 }.meets_floor(99),
        "one lamport below the floor must be rejected"
    );
    const _: () = assert!(
        MinBalanceRule { min: 0 }.meets_floor(0),
        "a zero floor admits an empty account"
    );
    const _: () = assert!(
        !MinBalanceRule { min: 200 }.meets_floor(100),
        "the same balance that passed a 100 floor is rejected by a 200 floor: the threshold comes from self.min"
    );
}
