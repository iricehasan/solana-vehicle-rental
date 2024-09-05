use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{error::ErrorCode::*, AdminAccount, CarAccount, Status};

pub fn add_car_account(
    ctx: Context<AddCarAccount>,
    name: String,
    model: String,
    rent_fee: u64,
    deposit_fee: u64,
    nft_name: String,
    nft_symbol: String,
    nft_uri: String,
) -> Result<()> {
    let admin_account = &ctx.accounts.admin_account;

    if admin_account.admin != ctx.accounts.authority.key() {
        return Err(Unauthorized.into());
    }

    let car_account = &mut ctx.accounts.car_account;
    car_account.car = car_account.key();
    car_account.name = name;
    car_account.model = model;
    car_account.rent_fee = rent_fee;
    car_account.deposit_fee = deposit_fee;
    car_account.car_status = Status::Available;

    msg!("Initialized car account for car {}", car_account.name);

    msg!("Minting Token");
    // Cross Program Invocation (CPI)
    // Invoking the mint_to instruction on the token program
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Creating metadata account");
    // Cross Program Invocation (CPI)
    // Invoking the create_metadata_account_v3 instruction on the token metadata program
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name: nft_name,
            symbol: nft_symbol,
            uri: nft_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, // Is mutable
        true,  // Update authority is signer
        None,  // Collection details
    )?;

    msg!("Creating master edition account");
    // Cross Program Invocation (CPI)
    // Invoking the create_master_edition_v3 instruction on the token metadata program
    create_master_edition_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.edition_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        None, // Max Supply
    )?;

    msg!("NFT minted successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct AddCarAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // The admin who will own this account
    #[account(
        init,
        seeds = [b"car_account"],
        bump,
        payer = authority,
        space = 8 + 32 + 64 + 64 + 8 + 8 + 64 // Adjusted for account size
    )]
    pub car_account: Account<'info, CarAccount>,
    #[account(seeds = [b"admin"], bump)]
    pub admin_account: Account<'info, AdminAccount>, // Admin account to verify authority

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    // Create new mint account, NFTs have 0 decimals
    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority.key(),
        mint::freeze_authority = authority.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint_account,
        associated_token::authority = authority,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
