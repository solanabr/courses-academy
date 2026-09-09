use anchor_lang::prelude::*;

declare_id!("2fLbW1PG2CeyAgR5krLF9okkqCXRmqy1o3srBh4E26WT");

#[program]
pub mod counter_v2_twin {
    use super::*;

    pub fn initialize(ctx: &mut Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = *ctx.accounts.payer.address();
        counter.count = PodU64::from(0);
        Ok(())
    }

    pub fn increment(ctx: &mut Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        let next = counter
            .count
            .get()
            .checked_add(1)
            .ok_or(CounterError::Overflow)?;
        counter.count = PodU64::from(next);
        Ok(())
    }
}

#[account]
#[repr(C)]
pub struct Counter {
    pub authority: Address, // 32 bytes
    pub count: PodU64,      //  8 bytes
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(mut)]
    pub payer: Signer,
    #[account(init, payer = payer)]
    pub counter: Account<Counter>,
    pub system_program: Program<System>,
}

#[derive(Accounts)]
pub struct Increment {
    // has_one is deprecated in V2; address = parent.field is the replacement.
    #[account(address = counter.authority)]
    pub authority: Signer,
    #[account(mut)]
    pub counter: Account<Counter>,
}

#[error_code]
pub enum CounterError {
    #[msg("play counter overflowed")]
    Overflow,
}
