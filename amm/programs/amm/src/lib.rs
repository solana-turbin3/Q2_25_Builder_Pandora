#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("HDpU8pAxKCq4b872fZweXc4VwVra4Wc12P7CffTt5Jv8");

pub mod instructions;
pub mod state;

use instructions::*;


#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)?;
        Ok(())
    }
}