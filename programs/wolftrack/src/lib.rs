#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32, // 8 for discriminator, 32 for name
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
