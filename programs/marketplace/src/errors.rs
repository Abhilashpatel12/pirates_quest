use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Listing is not active")]
    ListingNotActive,
    #[msg("Only the seller can update or cancel listing")]
    Unauthorized,
    #[msg("Invalid PIRATE mint")]
    InvalidPirateMint,
}
