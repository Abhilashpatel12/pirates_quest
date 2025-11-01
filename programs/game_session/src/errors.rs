// errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum GameSessionError {
    #[msg("Session has already ended")]
    SessionAlreadyEnded,
    #[msg("Session is not active")]
    SessionNotActive,
    #[msg("Unauthorized action")]
    Unauthorized,
}
