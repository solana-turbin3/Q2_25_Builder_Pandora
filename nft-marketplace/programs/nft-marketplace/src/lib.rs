use anchor_lang::prelude::*;

declare_id!("7tvYQBuPR3sMerzFb6UkuUmXy8uE2egP6S9g2iWKrJCV");


pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::*;

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, ctx.bumps)?;
        Ok(())
    }
}
