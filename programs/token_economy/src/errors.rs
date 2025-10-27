use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Calculation overflow")]
    Overflow,
    #[msg("Not enough tokens to complete action")]
    InsufficientBalance,
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
  #[msg("Invalid reward amount")]
    InvalidReward,
    #[msg("Invalid level number")]
    InvalidLevel,
    #[msg("Invalid treasure type")]
    InvalidTreasure,
}
