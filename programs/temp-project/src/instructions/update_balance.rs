/* 
use anchor_lang::prelude::*;

use crate::{error::ErrorCode::*, UserAccount, USER_ACCOUNT_SEED};

use solana_program::system_instruction;

pub fn deposit(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
    // Smaller than 0.0001 Sol
    if amount < 100000 {
        return Err(error!(AmountTooSmall));
    };

    let from_account = &mut ctx.accounts.holder;
    let to_account = &mut ctx.accounts.user_account;

    // Create the transfer instruction
    let transfer_instruction =
        system_instruction::transfer(from_account.key, &to_account.user, amount);
    
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

    to_account.balance += amount;

    Ok(())
}

pub fn withdraw(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
    let from_account = &mut ctx.accounts.user_account;
    let to_account = &mut ctx.accounts.holder;

    if amount > from_account.balance {
        return Err(error!(AmountTooBig));
    };

    // Create the transfer instruction
    let transfer_instruction =
        system_instruction::transfer(&from_account.user, to_account.key, amount);

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

    from_account.balance -= amount;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateBalance<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(mut, seeds = [USER_ACCOUNT_SEED, holder.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}
*/


use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode::*, UserAccount, FEED_ID, MAXIMUM_AGE, USER_ACCOUNT_SEED,
};

use solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction};

use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub fn deposit(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
    // Smaller than 20 USD
    if amount < 20 {
        return Err(error!(AmountTooSmall));
    };

    // Fetch latest price
    let price_update = &mut ctx.accounts.price_update;
    let price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(FEED_ID)?,
    )?;

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
        system_instruction::transfer(from_account.key, &to_account.user, amount_in_lamports);

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

    to_account.balance += amount;

    Ok(())
}

pub fn withdraw(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
    let from_account = &mut ctx.accounts.user_account;
    let to_account = &mut ctx.accounts.holder;

    if amount > from_account.balance {
        return Err(error!(AmountTooBig));
    };

    let price_update = &mut ctx.accounts.price_update;
    let price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(FEED_ID)?,
    )?;

    let amount_in_lamports = LAMPORTS_PER_SOL
        .checked_mul(10_u64.pow(price.exponent.abs().try_into().unwrap()))
        .unwrap()
        .checked_mul(amount)
        .unwrap()
        .checked_div(price.price.try_into().unwrap())
        .unwrap();

    // Create the transfer instruction
    let transfer_instruction =
        system_instruction::transfer(&from_account.user, to_account.key, amount_in_lamports);

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

    from_account.balance -= amount;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateBalance<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(mut, seeds = [USER_ACCOUNT_SEED, holder.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub system_program: Program<'info, System>,
}
