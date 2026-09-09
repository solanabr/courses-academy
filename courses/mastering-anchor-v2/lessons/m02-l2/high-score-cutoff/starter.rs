/// Admit a new score to a fixed-capacity cabinet high-score board and return
/// the LOWEST score still on the board afterwards, the leaderboard "cutoff".
///
/// The board is a bounded, fixed-size list: exactly the discipline a Pod
/// `Slab<Header, TailItem>` enforces on-chain, where you cannot heap-grow a
/// `Vec` inside an account. Rules:
///   * while the board has fewer than `cap` entries, always admit the score;
///   * once the board is full, admit the score ONLY if it strictly beats the
///     current cutoff (ties do not evict);
///   * keep the board sorted highest-first and never let it exceed `cap`,
///     including when the board handed to you already exceeds it.
///
/// Return the cutoff (the minimum retained score), or 0 for an empty board.
///
/// The full-board admission RULE lives in `beats_cutoff` below, a `const fn`
/// the compiler can evaluate while it builds (a device this course leans on
/// again in the m03-l3 constraint challenge): route the full-board branch of
/// `admit` through it.

/// Does `score` evict on a full board? Strictly beating the cutoff gets in;
/// a tie does not.
const fn beats_cutoff(score: u64, cutoff: u64) -> bool {
    // TODO: strictly beat the cutoff -- ties do not evict. Right now nothing
    // ever evicts, so a real high score never makes a full board.
    let _ = (score, cutoff);
    false
}

fn admit(board: Vec<u64>, score: u64, cap: usize) -> u64 {
    // TODO: actually admit `score` under the `cap` limit (through
    // `beats_cutoff` when the board is full) and keep only the top `cap`
    // entries. For now this ignores the new score and the cap entirely.
    let _ = (score, cap);
    *board.iter().min().unwrap_or(&0)
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION HARNESS — DO NOT EDIT ANYTHING BELOW THIS LINE.
// Compile-time assertions. Because `beats_cutoff` is a `const fn`, the compiler
// evaluates these while building: an unfixed rule does not compile at all, and
// the message names the case it got wrong. The test vectors document the wider
// contract on `admit`; grading is compile-only, so this block is what enforces
// the admission rule.
// ─────────────────────────────────────────────────────────────────────────────
#[doc(hidden)]
#[allow(dead_code)]
mod verify {
    use super::beats_cutoff;

    const _: () = assert!(
        beats_cutoff(75, 50),
        "a score strictly above the cutoff must evict the cutoff"
    );
    const _: () = assert!(
        !beats_cutoff(30, 30),
        "a tie with the cutoff does not evict"
    );
    const _: () = assert!(
        !beats_cutoff(10, 20),
        "a score below the cutoff never gets onto a full board"
    );
}
