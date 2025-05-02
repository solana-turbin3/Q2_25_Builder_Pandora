use anchor_lang::{prelude::*, solana_program::example_mocks::solana_sdk::system_program};

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked},
};

use crate::state::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"Escrow", escrow.maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = maker
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>

}

impl<'info> Take <'info> {

    pub fn take(&mut self, deposit: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info()
        };

        
        let cpi_prog = self.token_program.to_account_info();

        let cpi_context = CpiContext::new(cpi_prog, transfer_accounts);

        transfer_checked(cpi_context, deposit, self.mint_b.decimals)?;

        Ok(())
    }

    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {

        let signer_seeds: &[&[&[u8]]] = &[&[b"Escrow", self.maker.to_account_info().key.as_ref(), &self.escrow.seed.to_le_bytes()[..], &[self.escrow.bump]]];


        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };


        let cpi_prog = self.token_program.to_account_info();

        let cpi_context = CpiContext::new_with_signer(cpi_prog, transfer_accounts, signer_seeds);

        transfer_checked(cpi_context, self.escrow.receive, self.mint_a.decimals)?;

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        }; 

        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);

        close_account(cpi_context)?;

        Ok(())
    }
}