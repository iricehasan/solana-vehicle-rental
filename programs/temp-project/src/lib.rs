use anchor_lang::prelude::*;

use instructions::*;
pub mod instructions;

pub mod state;
use state::*;

pub mod error;

mod constants;
pub use constants::*;

declare_id!("HJCKrdL1VmqHqqqbc8unVatfdrTpvX5gvmWE4YLf6zCJ");

#[program]
pub mod temp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
        initialize::initialize(ctx, admin)
    }

    pub fn register_user_account(
        ctx: Context<RegisterUserAccount>,
        user_name: String,
        user_lastname: String,
    ) -> Result<()> {
        register_user_account::register_user_account(ctx, user_name, user_lastname)
    }

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
        add_car_account::add_car_account(
            ctx,
            name,
            model,
            rent_fee,
            deposit_fee,
            nft_name,
            nft_symbol,
            nft_uri,
        )
    }

    pub fn rent_car(ctx: Context<RentCar>, start_date: u64, end_date: u64) -> Result<()> {
        rent_car::rent_car(ctx, start_date, end_date)
    }
    
    pub fn deposit(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
        update_balance::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
        update_balance::withdraw(ctx, amount)
    }
    
    pub fn end_rent(ctx: Context<EndRent>) -> Result<()> {
        end_rent::end_rent(ctx)
    }

    pub fn read_price(ctx: Context<ReadPrice>) -> Result<()> {
        read_price::read_price(ctx)
    }
}
