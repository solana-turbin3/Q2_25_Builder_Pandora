#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

mod instructions;
use instructions::make::*;
use instructions::take::*;
pub mod state;
pub use state::*;

declare_id!("3iLXQ6CkBEzHNsyje2Vcwh1juWHVwKcxjbgD2zy5CQwA");

#[program]
pub mod escrow_turbin {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>, deposit: u64) -> Result<()> {
        ctx.accounts.take(deposit)?;
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
