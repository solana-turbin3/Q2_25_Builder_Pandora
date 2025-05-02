use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenInterface};

use crate::state::marketplace::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes().as_ref()],
        bump,
        space = Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
        mint::token_program = token_program
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> Initialize <'info> {
    pub fn init(&mut self, name: String, fees: u16, bump: InitializeBumps) -> Result<()> {
        
        self.marketplace.set_inner(Marketplace { admin: *self.admin.key, fee: fees, bump: bump.marketplace, treasury_bump: bump.treasury, rewards_bump: bump.rewards_mint, name: name });

        Ok(())
    }
}