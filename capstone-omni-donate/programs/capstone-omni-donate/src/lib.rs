#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("J7mBvVKt1XyqtwdtHpxGPTHQhua2rBRkqYkdEopT3bhs");

pub mod instructions;
pub mod state;

use crate::instructions::*;
use crate::state::*;



#[program]
pub mod capstone_omni_donate {

    use super::*;

    pub fn create_campaign(
        ctx: Context<InitializeCampaign>,
        id: u64,
        name: String,
        description: String,
        condition_target: Vec<u32>,
        oracle: Pubkey,
        beneficiary: Pubkey,
    ) -> Result<()> {
        ctx.accounts.init_campaign(
            id,
            name,
            description,
            condition_target,
            oracle,
            beneficiary,
            &ctx.bumps,
        )?;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, id: u64, amount: u64) -> Result<()> {
        ctx.accounts.donate_to_campaign(id, amount)?;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, id: u64) -> Result<()> {
        ctx.accounts.claim(id)?;
        Ok(())
    }

    pub fn update_weather(
        ctx: Context<UpdateWeather>,
        _id: u64,
        condition_code: u32,
        condition: String,
    ) -> Result<()> {
        ctx.accounts.update_weather(_id, condition_code, condition)?;
        Ok(())
    }

    pub fn init_config(
       ctx: Context<CollectFees>,
       protocol_address: Pubkey,
       fee_bps: u64,
       fee_collector: Pubkey,
    ) -> Result<()> {
        
        ctx.accounts.init_config(protocol_address, fee_bps, fee_collector, &ctx.bumps)?;
        Ok(())
    }

    pub fn update_fee_bps(
        ctx: Context<UpdateConfig>,
        fee_bps: u64
    ) -> Result<()> {
        ctx.accounts.update_fee_bps(fee_bps)?;
        Ok(())
    }

    pub fn update_address(
        ctx: Context<UpdateConfig>,
        address: Pubkey,
        is_protocol: bool
    ) -> Result<()> {
        ctx.accounts.update_address(address, is_protocol)?;
        Ok(())
    }

}

// Protocol Fee Configuration
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub protocol_address: Pubkey,
    pub fee_bps: u64,
    pub fee_collector: Pubkey,
    pub s: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CollectFees<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"config", signer.key().as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE
    )]
    pub config_account: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> CollectFees<'info> {
    pub fn init_config(
        &mut self,
        protocol_address: Pubkey,
        fee_bps: u64,
        fee_collector: Pubkey,
        bump: &CollectFeesBumps,
    ) -> Result<()> {
        self.config_account.set_inner(Config {
            protocol_address,
            fee_bps,
            fee_collector,
            s: self.signer.key(),
            bump: bump.config_account,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        constraint = signer.key() == config_account.s @ Cases::NOTADMIN,
        seeds = [b"config", signer.key().as_ref()],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateConfig <'info> {
    pub fn update_fee_bps(&mut self, fee_bps: u64) -> Result<()> {
        require!(1000 < fee_bps && fee_bps > 0, Cases::INVALIDFEEBPS);

        self.config_account.fee_bps = fee_bps;
        Ok(())
    }

    pub fn update_address(&mut self, address: Pubkey, is_protocol: bool) -> Result<()> {
        match is_protocol {
            true => self.config_account.protocol_address = address,
            false => self.config_account.fee_collector = address,
        }

        Ok(())
    }
}
