use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod academy_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Superteam Academy default program");
        Ok(())
    }

    // TODO: Add a greet instruction here
}

#[derive(Accounts)]
pub struct Initialize {}

// TODO: Add a Greet accounts struct here
