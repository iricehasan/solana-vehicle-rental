use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Car already exists.")]
    CarAlreadyExists,
    #[msg("User already exists.")]
    UserAlreadyExists,
    #[msg("User does not exist.")]
    UserDoesNotExist,
    #[msg("Insufficient balance.")]
    InsufficientBalance,
    #[msg("Car is not available.")]
    CarNotAvailable,
    #[msg("Car is not rented.")]
    CarNotRented,
    #[msg("Overflow error.")]
    Overflow,
    #[msg("Unauthorized.")]
    Unauthorized, // Error when a non-admin tries to end a rent
    #[msg("Amount is Too Big To Withdraw")]
    AmountTooBig,
    #[msg("Amount is Too Small To Deposit")]
    AmountTooSmall,
    #[msg("Could not load price account")]
    PythError,
    #[msg("Failed to serialize price account")]
    TryToSerializePriceAccount,
    #[msg("Invalid argument provided")]
    InvalidArgument,
}
