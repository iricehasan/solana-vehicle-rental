use anchor_lang::prelude::*;

// Data structures

// Seq Account to keep track of rent sequence and car sequence
#[account]
pub struct Seq {
    pub rent_seq: u64,
    pub car_seq: u64,
}

// Admnin Account to restrict some instructions as admin only
#[account]
#[derive(Default)]
pub struct AdminAccount {
    pub admin: Pubkey, // Stores the admin's public key
}

//User Account that is a PDA when a User Registers
#[account]
#[derive(Default)]
pub struct UserAccount {
    pub user: Pubkey,
    pub user_name: String,
    pub user_lastname: String,
    pub balance: u64,
    pub score: u64,
}

// Car Account PDA that when A new car is added to the system it is derived from the car_seq seed
#[account]
#[derive(Default)]
pub struct CarAccount {
    pub car: Pubkey, 
    pub model: String,
    pub car_status: Status, // Available or InUse to check
    pub car_type: CarType, // Different car types has different rent rates
}

#[account]
#[derive(Default)]
pub struct RentAccount {
    pub id: Pubkey,
    pub user: Pubkey,
    pub user_name: String,
    pub user_lastname: String,
    pub car_id: Pubkey,
    pub start_date: u64,
    pub end_date: u64,
}

// Status of the car to check when renting the car
// Car can be rented only if it is available
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Available,
    InUse,
}

// Car types which has different rent rates in the rent_car instruction
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default)]
pub enum CarType {
    #[default]
    Small,
    Medium,
    Large,
    Luxury,
    Suv,
    Commercial,
}