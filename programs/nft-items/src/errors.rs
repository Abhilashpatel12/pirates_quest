use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Rarity must be between 1 and 5")]
    InvalidRarity,
    #[msg("Level must be at least 1")]
    LevelTooLow,
    #[msg("You are not the owner of this NFT")]
    UnauthorizedOwner,
    #[msg("Boss drop must be legendary rarity")]
    BossDropNotLegendary,
    #[msg("Treasury drop must be legendary rarity")]
    TreasuryDropNotLegendary,
    #[msg("Stat values cannot be negative")]
    StatValueNegative,
    #[msg("Item is already equipped")]
    AlreadyEquipped,
    #[msg("Item is already listed")]
    AlreadyListed,
    #[msg("Missing boss proof data")]
    MissingBossProof,
    #[msg("Missing treasury proof data")]
    MissingTreasuryProof
}
