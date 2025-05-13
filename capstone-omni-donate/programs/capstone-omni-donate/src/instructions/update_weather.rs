#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(_id: u64)]
pub struct UpdateWeather<'info> {
    pub oracle: Signer<'info>,
    #[account(
        seeds = [b"campaign", campaign.creator.key().as_ref(), _id.to_le_bytes().as_ref()],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        mut,
        constraint = weather.oracle.key() == oracle.key() @ Cases::NOTORACLE,
        seeds = [b"weather", campaign.creator.key().as_ref(), _id.to_le_bytes().as_ref()],
        bump = weather.bump,
    )]
    pub weather: Account<'info, Weather>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateWeather<'info> {
    pub fn update_weather(
        &mut self,
        _id: u64,
        condition_code: u32,
        condition: String,
    ) -> Result<()> {
        // Require 1hr cool down before updating Weather condition
        require!(
            self.clock.unix_timestamp > self.weather.last_time_checked + 3600000,
            Cases::TIMENOTPASSED
        );
        require!(!self.campaign.completed, Cases::CAMPAIGNCOMPLETED);

        self.weather.last_time_checked = self.clock.unix_timestamp;
        self.weather.condition_code = condition_code;
        self.weather.condition = condition;

        Ok(())
    }
}
