use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::*;
use crate::Config;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Donate<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = vault_mint,
        associated_token::authority = donor,
        associated_token::token_program = token_program
    )]
    pub donor_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"campaign", campaign.creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        init_if_needed,
        payer = donor,
        associated_token::mint = vault_mint,
        associated_token::authority = campaign,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account( 
        mint::token_program = token_program 
    )]
    pub vault_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"config", config_account.s.as_ref()],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
    #[account(
        init_if_needed,
        payer = donor,
        associated_token::mint = vault_mint,
        associated_token::authority = config_account,
        associated_token::token_program = token_program
    )]
    pub config_ata: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Donate<'info> {
    pub fn donate_to_campaign(&mut self, id: u64, amount: u64) -> Result<()> {
        require!(!self.campaign.completed, Cases::CAMPAIGNCOMPLETED);
        /*
           Percentage of Fee will be collected from each deposit relative to the deposit amount and Fee Basis Point
        */

        let fee_bps = self.config_account.fee_bps;
        let fee = (amount * fee_bps) / 10_000;
        let net_amount = amount - fee;

        self.collect_fee(fee)?;

        let cpi_accounts = TransferChecked {
            from: self.donor_ata.to_account_info(),
            mint: self.vault_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.donor.to_account_info(),
        };
        let cpi_prog = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_prog, cpi_accounts);

        transfer_checked(cpi_context, net_amount, self.vault_mint.decimals)?;

        self.campaign.raised += net_amount;

        msg!("Donated to Campaign ID: {}", id);
        Ok(())
    }

    pub fn collect_fee(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.donor_ata.to_account_info(),
            mint: self.vault_mint.to_account_info(),
            to: self.config_ata.to_account_info(),
            authority: self.donor.to_account_info(),
        };
        let cpi_prog = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_prog, cpi_accounts);

        transfer_checked(cpi_context, amount, self.vault_mint.decimals)?;

        Ok(())
    }
}
