use anchor_lang::prelude::*;

// Data structures
#[account]
pub struct RentSeq {
    pub rent_seq: u64,
}

#[account]
#[derive(Default)]
pub struct AdminAccount {
    pub admin: Pubkey, // Stores the admin's public key
}

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub user: Pubkey,
    pub user_name: String,
    pub user_lastname: String,
    pub balance: u64,
    pub score: u64,
}

#[account]
#[derive(Default)]
pub struct CarAccount {
    pub car: Pubkey,
    pub name: String,
    pub model: String,
    pub rent_fee: u64,
    pub deposit_fee: u64,
    //pub metadata_uri: String,
    pub car_status: Status,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Available,
    InUse,
}
