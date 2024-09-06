use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode::*, UserAccount, FEED_ID, MAXIMUM_AGE, USER_ACCOUNT_SEED,
};

use solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction}; // to send lamports

use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2}; // To get SOL/USD price using the Price Feed and PriceUpdateV2

pub fn deposit(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
    // If the user tries to deposit less than 20 dollars, throw error
    if amount < 20 {
        return Err(error!(AmountTooSmall));
    };

    // fetch the latest SOL/USD price
    let price_update = &mut ctx.accounts.price_update;
    let price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(FEED_ID)?,
    )?;

    // deposit amount in lamports
    let amount_in_lamports = LAMPORTS_PER_SOL
        .checked_mul(10_u64.pow(price.exponent.abs().try_into().unwrap()))
        .unwrap()
        .checked_mul(amount)
        .unwrap()
        .checked_div(price.price.try_into().unwrap())
        .unwrap();

    let from_account = &mut ctx.accounts.holder;
    let to_account = &mut ctx.accounts.user_account;

    // Create the transfer instruction
    let transfer_instruction =
        system_instruction::transfer(&from_account.key(), &to_account.key(), amount_in_lamports);

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            to_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    to_account.balance += amount; // Increase the balance by the amount deposited

    Ok(())
}

pub fn withdraw(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
    let from_account = &mut ctx.accounts.user_account;
    let to_account = &mut ctx.accounts.holder;

    // If user tries to withdraw more than the user account balance, throw error
    if amount > from_account.balance {
        return Err(error!(AmountTooBig));
    };

    // fetch the latest SOL/USD price
    let price_update = &mut ctx.accounts.price_update;
    let price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(FEED_ID)?,
    )?;

    // withdraw amount in lamports
    let amount_in_lamports = LAMPORTS_PER_SOL
        .checked_mul(10_u64.pow(price.exponent.abs().try_into().unwrap()))
        .unwrap()
        .checked_mul(amount)
        .unwrap()
        .checked_div(price.price.try_into().unwrap())
        .unwrap();

    // Create the transfer instruction
    let transfer_instruction =
        system_instruction::transfer(&from_account.key(), &to_account.key(), amount_in_lamports);

        let seeds = &[
            b"user_account".as_ref(),
            &to_account.key.as_ref(),
        ];

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            to_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[&seeds[..]],
    )?;

    from_account.balance -= amount; // Decrease the balance by the amount withdrawn

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateBalance<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(mut, seeds = [USER_ACCOUNT_SEED, holder.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>, // user account that is deposited to or withdrawn from

    pub price_update: Account<'info, PriceUpdateV2>, // To fetch SOL/USD price

    pub system_program: Program<'info, System>, // To send lamports
}
