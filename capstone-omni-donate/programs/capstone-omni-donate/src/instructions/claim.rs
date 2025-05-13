use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{
        Mint, TokenAccount,
        TokenInterface,
    },
};

use crate::state::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Claim<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    #[account(
        init_if_needed,
        payer = beneficiary,
        associated_token::mint = vault_mint,
        associated_token::authority = beneficiary,
        associated_token::token_program = token_program
    )]
    pub beneficiary_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        has_one = beneficiary,
        seeds = [b"campaign", campaign.creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        seeds = [b"weather", campaign.creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = weather.bump,
    )]
    pub weather: Account<'info, Weather>,
    #[account(
        mut,
        associated_token::mint = vault_mint,
        associated_token::authority = campaign,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account( 
        mint::token_program = token_program 
    )]
    pub vault_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self, id: u64) -> Result<()> {
        let is_valid = self
            .campaign
            .condition_target
            .iter()
            .any(|x| self.weather.condition_code == *x);

        require!(is_valid, Cases::WEATHERCONDITIONNOTMET);
        require!(self.campaign.raised > 0, Cases::INSUFFICIENTFUNDS);

        let binding = self.campaign.creator.key();
        let amount = self.campaign.raised;

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"campaign",
            binding.as_ref(),
            &id.to_le_bytes()[..],
            &[self.campaign.bump],
        ]];

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.vault_mint.to_account_info(),
            to: self.beneficiary_ata.to_account_info(),
            authority: self.campaign.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount, self.vault_mint.decimals)?;

        self.campaign.completed = true;
        self.campaign.raised = 0;

        Ok(())
    }

}
