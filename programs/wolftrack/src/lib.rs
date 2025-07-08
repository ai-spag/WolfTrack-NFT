use anchor_lang::prelude::*;

declare_id!("9uCnxtqvRhMH4YPhAGKDyBmhHkG4Ef3p1YKXdVR7d6Qc");

#[program]
pub mod wolftrack {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        let input_name = name.as_bytes();
        let my_account = &mut ctx.accounts.my_account;

        if input_name.len() > 32 {
            return Err(ErrorCode::NameTooLong.into());
        }

        my_account.name[..input_name.len()].copy_from_slice(input_name);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32,
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

#[error_code]
pub enum ErrorCode {
    #[msg("The provided name is too long.")]
    NameTooLong,
}
