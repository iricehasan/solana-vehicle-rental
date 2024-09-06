use anchor_lang::prelude::*;

use instructions::*;
pub mod instructions;

pub mod state;
use state::*;

pub mod error;

mod constants;
pub use constants::*;

declare_id!("HEo7YjjFz4zNRk6VC7chR2EPFyKF8nsjQ9qdEu1NNjse");

#[program]
pub mod vehicle_rental {
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
        model: String,
        car_type: String,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        add_car_account::add_car_account(
            ctx,
            model,
            car_type,
            nft_name,
            nft_symbol,
            nft_uri,
        )
    }

    pub fn rent_car(ctx: Context<RentCar>, rent_time_in_days: u64, amount: u64) -> Result<()> {
        rent_car::rent_car(ctx, rent_time_in_days, amount)
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

}
