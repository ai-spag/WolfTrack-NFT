use anchor_lang::prelude::*;

declare_id!("YourProgramId1111111111111111111111111111111111");

#[program]
pub mod wolftrack {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
