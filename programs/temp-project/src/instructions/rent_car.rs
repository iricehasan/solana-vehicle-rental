use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode::*, CarAccount, RentAccount, RentSeq, Status, UserAccount, USER_ACCOUNT_SEED,
};

pub fn rent_car(ctx: Context<RentCar>, start_date: u64, end_date: u64) -> Result<()> {
    msg!("Adding 1 to the rent_seq!!");

    let car = &mut ctx.accounts.car_account;

    if car.car_status != Status::Available {
        return Err(CarNotAvailable.into());
    }

    car.car_status = Status::InUse;
    // rent_amount = 1;

    let rent = &mut ctx.accounts.rent_account;
    rent.id = rent.key();
    rent.user = ctx.accounts.user_account.key();
    rent.user_name = ctx.accounts.user_account.user_name.clone();
    rent.user_lastname = ctx.accounts.user_account.user_lastname.clone();
    rent.car_id = car.car;
    rent.start_date = start_date;
    rent.end_date = end_date;

    // Updating the counter must be here
    let rent_seq = &mut ctx.accounts.rent_seq;
    rent_seq.rent_seq += 1;

    msg!("Current rent_seq is {}", rent_seq.rent_seq);

    Ok(())
}

#[derive(Accounts)]
pub struct RentCar<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // The signer of the transaction
    #[account(mut, seeds = [authority.key().as_ref()], bump)]
    pub rent_seq: Account<'info, RentSeq>, // Sequence tracker
    #[account(
        init,
        seeds = [b"rent_account"],
        bump,
        payer = authority,
        space = 200 // Adjust space as needed
    )]
    pub rent_account: Account<'info, RentAccount>,
    #[account(mut, seeds = [b"car_account"], bump)]
    pub car_account: Account<'info, CarAccount>, // The car being rented
    #[account(mut, seeds = [USER_ACCOUNT_SEED, authority.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>, // The user renting the car
    pub system_program: Program<'info, System>,
}
