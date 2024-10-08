use anchor_lang::prelude::*;

use crate::{UserAccount, USER_ACCOUNT_SEED};

// register a user by providing a user name and user lastname
pub fn register_user_account(
    ctx: Context<RegisterUserAccount>,
    user_name: String,
    user_lastname: String,
) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    user_account.user = ctx.accounts.authority.key();
    user_account.user_name = user_name;
    user_account.user_lastname = user_lastname;
    user_account.balance = 0; // to keep track of balance to be used in deposit and withdraw instructions
    user_account.score = 0;

    msg!("Registered user account for {}", user_account.user_name);

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterUserAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // The user who will own this account
    #[account(
        init,
        seeds = [USER_ACCOUNT_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 68 + 68 + 8 + 8  // Adjusted for account size
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}
