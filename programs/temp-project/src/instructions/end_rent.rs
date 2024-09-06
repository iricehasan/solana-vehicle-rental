use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode::*, AdminAccount, CarAccount, RentAccount, Seq, Status, UserAccount,
    USER_ACCOUNT_SEED,
};

// Admin can close the rent account if the end_date of the rent has passed
pub fn end_rent(ctx: Context<EndRent>) -> Result<()> {
    msg!("Close the Rent Account and return Lamports to the Admin");

    let admin_account = &mut ctx.accounts.admin_account;
    let rent_account = &mut ctx.accounts.rent_account;
    let user_account = &mut ctx.accounts.user_account;
    let car_account = &mut ctx.accounts.car_account;

    // Only the admin can end a rent
    if admin_account.admin != ctx.accounts.authority.key() {
        return Err(Unauthorized.into());
    }

    // Check: end_date before ending the rent account
    if rent_account.end_date > Clock::get()?.unix_timestamp as u64 {
        return Err(RentNotEndedYet.into());
    }

    // Change the car status to available again
    car_account.car_status = Status::Available; 

    msg!("Increased the user score by 5 points");
    user_account.score += 5;

    Ok(())
}

#[derive(Accounts)]
pub struct EndRent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // The admin who will own this account
    #[account(mut, seeds = [authority.key().as_ref()], bump)]
    pub seq: Account<'info, Seq>, // Sequence tracker
    #[account(
        mut,
        seeds = [b"rent_account", car_account.car.key().as_ref()],
        bump,
        close = authority, // close account and return lamportsxw
    )]
    pub rent_account: Account<'info, RentAccount>, // Rent Account PDA that holds the rent info
    #[account(mut, seeds = [b"car_account", seq.car_seq.to_string().as_bytes()], bump)]
    pub car_account: Account<'info, CarAccount>, // The car being rented
    #[account(mut, seeds = [USER_ACCOUNT_SEED, authority.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>, // The user renting the car
    #[account(seeds = [b"admin", authority.key().as_ref()], bump)]
    pub admin_account: Account<'info, AdminAccount>, // Admin account to verify authority
    pub system_program: Program<'info, System>,
}
