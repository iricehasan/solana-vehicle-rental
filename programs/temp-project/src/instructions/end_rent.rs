use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode::*, AdminAccount, CarAccount, RentAccount, RentSeq, Status, UserAccount,
    USER_ACCOUNT_SEED,
};

pub fn end_rent(ctx: Context<EndRent>) -> Result<()> {
    let admin_account = &mut ctx.accounts.admin_account;
    if admin_account.admin != ctx.accounts.authority.key() {
        return Err(Unauthorized.into());
    }

    let car_account = &mut ctx.accounts.car_account;
    car_account.car_status = Status::Available;

    Ok(())
}

#[derive(Accounts)]
pub struct EndRent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // The admin who will own this account
    #[account(mut, seeds = [authority.key().as_ref()], bump)]
    pub rent_seq: Account<'info, RentSeq>, // Sequence tracker
    #[account(
        mut,
        seeds = [b"rent_account"],
        bump,
    )]
    pub rent_account: Account<'info, RentAccount>,
    #[account(mut, seeds = [b"car_account"], bump)]
    pub car_account: Account<'info, CarAccount>, // The car being rented
    #[account(mut, seeds = [USER_ACCOUNT_SEED, authority.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>, // The user renting the car
    #[account(seeds = [b"admin"], bump)]
    pub admin_account: Account<'info, AdminAccount>, // Admin account to verify authority
    pub system_program: Program<'info, System>,
}
