use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode::*, CarAccount, RentAccount, Seq, Status, UserAccount, USER_ACCOUNT_SEED, CarType, FEED_ID, MAXIMUM_AGE, 
};

use solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction}; // to send lamports

use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2}; // To get SOL/USD price using the Price Feed and PriceUpdateV2

pub fn rent_car(ctx: Context<RentCar>, rent_time_in_days: u64, amount: u64) -> Result<()> {
    msg!("Adding 1 to the rent_seq!!");

    let car = &mut ctx.accounts.car_account;

    // If car is not available then throw error
    if car.car_status != Status::Available {
        return Err(CarNotAvailable.into());
    }

    car.car_status = Status::InUse; // Change the status as InUse

    let rent = &mut ctx.accounts.rent_account;

    // Use Solana's Clock to get the current timestamp as start_date
    let clock = Clock::get()?;
    let start_date = clock.unix_timestamp as u64; // current UNIX timestamp in seconds
    rent.start_date = start_date;

    // Calculate end_date based on rent_time_in_days
    let end_date = start_date + rent_time_in_days * 24 * 3600; // Convert days to seconds
    rent.end_date = end_date;

    // Match the rent rate using the car type
    let daily_rate_in_usd = match car.car_type {
        CarType::Small => 20,       // $20/day
        CarType::Medium => 32,      // $32/day
        CarType::Large => 55,       // $55/day
        CarType::Luxury => 80,      // $80/day
        CarType::Suv => 39,         // $39/day
        CarType::Commercial => 65,  // $65/day
    };

  
    // Fetch latest SOL/USD price
     let price_update = &mut ctx.accounts.price_update;
     let price = price_update.get_price_no_older_than(
         &Clock::get()?,
         MAXIMUM_AGE,
         &get_feed_id_from_hex(FEED_ID)?,
     )?;

    // total rent price in dollars
    let rent_price_usd = daily_rate_in_usd * rent_time_in_days as u64;

     // Throw an error if user sends less than the rent price
    if amount < rent_price_usd {
        return Err(InsufficientAmount.into());
    }

     // Calculate the amount that will be paid from the user
    let amount_in_lamports = LAMPORTS_PER_SOL
    .checked_mul(10_u64.pow(price.exponent.abs().try_into().unwrap()))
    .unwrap()
    .checked_mul(rent_price_usd)
    .unwrap()
    .checked_div(price.price.try_into().unwrap())
    .unwrap();

    let from_account = &mut ctx.accounts.authority;

    // Create the transfer instruction
    let transfer_instruction =
        system_instruction::transfer(from_account.key, &rent.key(), amount_in_lamports);

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            rent.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    // return the amount back if user sent more than rent_price
    if amount - rent_price_usd > 0 {

        let extra = amount - rent_price_usd;

        let amount_to_return_in_lamports = LAMPORTS_PER_SOL
        .checked_mul(10_u64.pow(price.exponent.abs().try_into().unwrap()))
        .unwrap()
        .checked_mul(extra)
        .unwrap()
        .checked_div(price.price.try_into().unwrap())
        .unwrap();
    
    
        // Create the transfer instruction
        let return_transfer_instruction =
            system_instruction::transfer(&rent.key(), &from_account.key(), amount_to_return_in_lamports);
    
            let seeds = &[
                b"rent_account".as_ref(),
                &car.car.as_ref(),
            ];

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &return_transfer_instruction,
            &[
                rent.to_account_info(),
                from_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&seeds[..]],
        )?;
    }

    
    rent.id = rent.key();
    rent.user = ctx.accounts.user_account.key();
    rent.user_name = ctx.accounts.user_account.user_name.clone();
    rent.user_lastname = ctx.accounts.user_account.user_lastname.clone();
    rent.car_id = car.car;
    rent.start_date = start_date;
    rent.end_date = end_date;

    // Updating the rent seq by increasing it by 1
    let seq = &mut ctx.accounts.seq;
    seq.rent_seq += 1;

    msg!("Current rent_seq is {}", seq.rent_seq);

    Ok(())
}

#[derive(Accounts)]
pub struct RentCar<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // The signer of the transaction
    #[account(mut, seeds = [authority.key().as_ref()], bump)]
    pub seq: Account<'info, Seq>, // Sequence tracker
    #[account(
        init,
        seeds = [b"rent_account", car_account.car.key().as_ref()],
        bump,
        payer = authority,
        space = 200 
    )]
    pub rent_account: Account<'info, RentAccount>,
    #[account(mut, seeds = [b"car_account", seq.car_seq.to_string().as_bytes()], bump)]
    pub car_account: Account<'info, CarAccount>, // The car being rented
    #[account(mut, seeds = [USER_ACCOUNT_SEED, authority.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>, // The user renting the car
    pub price_update: Account<'info, PriceUpdateV2>, // PriceUpdate account to fetch the SOL/USD price
    pub system_program: Program<'info, System>, // To send lamports
}
