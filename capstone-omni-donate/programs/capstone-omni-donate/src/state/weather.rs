use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Weather {
    pub id: u64,
    #[max_len(120)]
    pub condition: String,
    pub condition_code: u32,
    pub last_time_checked: i64,
    pub oracle: Pubkey,
    pub bump: u8,
}
