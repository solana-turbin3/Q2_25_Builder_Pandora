use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Beneficiary {
    pub beneficiary_address: Pubkey,
    pub delegate_address: Pubkey,
    pub bump: u8,
}
