use anchor_lang::prelude::*;

declare_id!("9zT7oFjMiCKuz3pvGKtHtG6MFda9LepV8PUr3UUNoezU");

#[program]
pub mod wolftrack {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("🐺 WolfTrack NFT Program Initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
