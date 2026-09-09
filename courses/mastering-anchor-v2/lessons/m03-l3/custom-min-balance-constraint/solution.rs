// Reference solution: the `check` hook enforces the floor, exactly as the V2
// `AccountConstraint` trait would when codegen dispatches `quarters::min_balance`.
// The names below are the lesson's scratch names, not Anchor's — `BalanceGate`
// stands in for the real trait, and `meets_floor` is a local `const fn` rather
// than a hook of any kind.
//
// Splitting the condition out is what buys the compile-time proof: a trait
// method cannot be `const` on stable Rust, so the harness at the bottom could
// not evaluate the rule if it lived inside `check`. It costs nothing at run time.

pub trait BalanceGate {
    /// Mirrors the check hook Anchor V2 runs after the account is loaded.
    fn check(&self, balance: u64) -> Result<(), String>;
}

pub struct MinBalanceRule {
    pub min: u64,
}

impl MinBalanceRule {
    /// The floor is inclusive: a balance equal to `self.min` satisfies it.
    const fn meets_floor(&self, balance: u64) -> bool {
        balance >= self.min
    }
}

impl BalanceGate for MinBalanceRule {
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
