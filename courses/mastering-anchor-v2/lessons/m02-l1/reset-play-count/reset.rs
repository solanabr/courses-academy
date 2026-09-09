// Reference solution for the m02-l1 challenge — the program half.
// Read this only after you have a green run of your own.
//
// The handler goes inside `#[program] pub mod cabinet_counter`, below
// `increment`; the accounts struct goes at the bottom of `lib.rs`, next to
// `Increment`. Nothing else in the program changes.

// The operator clears the shift's play tally. ONLY play_count moves:
// high_score is the all-time marquee number, and it survives the reset.
// One field assigned, one field deliberately left alone — the test in
// cabinet_reset.rs exists to prove the second half of that sentence.
pub fn reset(ctx: &mut Context<Reset>) -> Result<()> {
    ctx.accounts.cabinet.play_count = PodU64::from(0);
    Ok(())
}

// Same PDA-validated cabinet as Increment, mutable: the seeds re-derive the
// one cabinet this player owns, so a player cannot reset someone else's.
#[derive(Accounts)]
pub struct Reset {
    #[account(
        mut,
        seeds = [b"cabinet", player.address().as_ref()],
        bump
    )]
    pub cabinet: Account<Cabinet>,
    pub player: Signer,
}
