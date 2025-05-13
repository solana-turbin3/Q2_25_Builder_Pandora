use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Campaign {
    pub id: u64,
    pub creator: Pubkey,
    #[max_len(120)]
    pub name: String,
    pub date: i64,
    #[max_len(120)]
    pub description: String,
    pub raised: u64,
    pub completed: bool,
    // Campaigns are allowed to have five conditions
    #[max_len(5)]
    pub condition_target: Vec<u32>,
    pub oracle: Pubkey,
    pub beneficiary: Pubkey,
    pub bump: u8,
}
