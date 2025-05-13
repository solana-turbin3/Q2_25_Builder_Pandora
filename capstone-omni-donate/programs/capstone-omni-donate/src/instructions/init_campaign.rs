#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::*;
use crate::state::constant::*;


#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitializeCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        seeds = [b"campaign", creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump,
        space = 8 + Campaign::INIT_SPACE
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        init, 
        payer = creator,
        seeds = [b"weather", creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump,
        space = 8 + Weather::INIT_SPACE
    )]
    pub weather: Account<'info, Weather>,
    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = vault_mint,
        associated_token::authority = campaign,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account( 
        mint::token_program = token_program 
    )]
    pub vault_mint: InterfaceAccount<'info, Mint>,
    pub clock: Sysvar<'info, Clock>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeCampaign<'info> {

    pub fn init_campaign(
        &mut self,
        id: u64,
        name: String,
        description: String,
        condition_target: Vec<u32>,
        oracle: Pubkey,
        beneficiary: Pubkey,
        bump: &InitializeCampaignBumps,
    ) -> Result<()> {
        let current_time = self.clock.unix_timestamp;

        require!(condition_target.len() as u8 <= MAX_CAMPAIGN_CONDITIONS, Cases::REACHEDMAXCONDITIONS);

        // Create a New Campaign
        self.campaign.set_inner(Campaign {
            id,
            creator: self.creator.key(),
            name: name.clone(),
            date: current_time,
            description,
            raised: 0,
            completed: false,
            condition_target,
            oracle,
            beneficiary,
            bump: bump.campaign,
        });

        // Initialize the Weather Account for the Associated Campaign
        self.weather.set_inner(Weather {
            id,
            condition: String::new(),
            condition_code: 0,
            last_time_checked: current_time,
            oracle,
            bump: bump.weather,
        });

        msg!("Campaign Created For : {}", name.clone());

        Ok(())
    }
}
