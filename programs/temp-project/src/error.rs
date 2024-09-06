use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient amount.")]
    InsufficientAmount, // error when renting the car by sending insufficient amount
    #[msg("Car is not available.")]
    CarNotAvailable, // car is already rented and in use.
    #[msg("Rent is Not Ended Yet.")]
    RentNotEndedYet, // error when admin tries to end the rent before the end_date
    #[msg("Unauthorized.")]
    Unauthorized, // Error when a non-admin tries to execute transactiont
    #[msg("Amount is Too Big To Withdraw")] 
    AmountTooBig, // Error when requested to withdraw more than user balance
    #[msg("Amount is Too Small To Deposit")] 
    AmountTooSmall, // Error when deposited less than 20 dollars
    #[msg("Car Type is Not Supported")] 
    CarTypeNotSupported, // Error when adding the car to the program with invalid type
}
