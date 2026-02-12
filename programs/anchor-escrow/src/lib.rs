use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("Dv9XWLt8UfeWwSd9GwtpSQz4EKE5C4UqDxUgdCRQnykE");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn make(ctx: Context<Make>, seed: u64, deposite: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposite)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
