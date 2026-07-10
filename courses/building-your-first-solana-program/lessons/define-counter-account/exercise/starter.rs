use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod academy_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Superteam Academy default program");
        Ok(())
    }

    pub fn greet(_ctx: Context<Greet>) -> Result<()> {
        msg!("Hello, Solana!");
        Ok(())
    }
}

// TODO: Update Initialize to create a Counter PDA account
// Use seeds = [b"counter", user.key().as_ref()] and bump
#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Greet {}

// TODO: Add a Counter account struct with count: u64 and authority: Pubkey
