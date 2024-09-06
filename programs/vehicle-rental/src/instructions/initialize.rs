use anchor_lang::prelude::*;

use crate::{AdminAccount, Seq};

// Initializes the sequence account that will keep track of rent sequence and car sequence
// Initializes the admin account to restrict some instructions as admin only
pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
    msg!("Creating a Car Rental Program!!");

    let seq = &mut ctx.accounts.seq;
    seq.rent_seq = 0;
    seq.car_seq = 0;

    let admin_account = &mut ctx.accounts.admin_account;
    admin_account.admin = admin;

    msg!("Current rent_seq is {}", seq.rent_seq);
    msg!("Admin is {}", admin_account.admin);
    msg!("The number of cars in the system is {}", seq.car_seq);
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        init,
        seeds = [authority.key().as_ref()],
        bump,
        payer = authority,
        space = 100
    )]
    seq: Account<'info, Seq>,
    #[account(
        init,
        seeds = [b"admin", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32
    )]
    admin_account: Account<'info, AdminAccount>,
    system_program: Program<'info, System>,
}
