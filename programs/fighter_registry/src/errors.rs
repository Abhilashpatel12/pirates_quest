
use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Fighter name cannot be empty")]
    EmptyName,

    #[msg("Fighter name too long (max 32 chars)")]
    NameTooLong,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Amount too large")]
    AmountTooLarge,

    #[msg("Invalid island ID (1-4)")]
    InvalidIsland,

    #[msg("Invalid rarity (1-4)")]
    InvalidRarity,

    #[msg("Invalid item ID")]
    InvalidItemId,

    #[msg("You do not own this item")]
    NotItemOwner,

    #[msg("Item already equipped")]
    AlreadyEquipped,

    #[msg("Invalid health amount")]
    InvalidHealth,

    #[msg("Invalid level")]
    InvalidLevel,

    #[msg("Invalid stamina amount")]
    InvalidStamina,
}