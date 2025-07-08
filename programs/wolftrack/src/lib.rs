use anchor_lang::prelude::*;

declare_id!("9uCnxtqvRhMH4YPhAGKDyBmhHkG4Ef3p1YKXdVR7d6Qc");

#[program]
pub mod wolftrack {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32, // 8 bytes for Anchor discriminator, 32 bytes for name
    )]
    pub my_account: Account<'info, MyAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    pub name: [u8; 32],
}
