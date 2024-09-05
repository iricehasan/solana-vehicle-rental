use anchor_lang::prelude::*;

use crate::{AdminAccount, RentSeq};

pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
    msg!("Creating a Car Rental Program!!");

    let rent_seq = &mut ctx.accounts.rent_seq;
    rent_seq.rent_seq = 0;

    let admin_account = &mut ctx.accounts.admin_account;
    admin_account.admin = admin;

    msg!("Current rent_seq is {}", rent_seq.rent_seq);
    msg!("Admin is {}", admin_account.admin);

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
    rent_seq: Account<'info, RentSeq>,
    #[account(
        init,
        seeds = [b"admin"],
        bump,
        payer = authority,
        space = 8 + 32 // Space for storing a Pubkey
    )]
    admin_account: Account<'info, AdminAccount>,
    system_program: Program<'info, System>,
}
