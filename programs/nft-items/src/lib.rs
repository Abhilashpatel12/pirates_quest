use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

pub use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod nft_items {
    use super::*;

    pub fn initialize_collection(
        ctx: Context<InitializeCollection>,
        name: String,
        uri: String,
    ) -> Result<()> {
        instructions::initialize_collection(ctx, name, uri)
    }

    pub fn mint_game_item(
        ctx: Context<MintGameItem>,
        name: String,
        uri: String,
        item_type: ItemType,
        rarity: u8,
        base_stats: ItemStats,
    ) -> Result<()> {
        instructions::mint_game_item(ctx, name, uri, item_type, rarity, base_stats)
    }

    pub fn mint_boss_drop(
        ctx: Context<MintBossDrop>,
        name: String,
        uri: String,
        item_type: ItemType,
        rarity: u8,
        base_stats: ItemStats,
        boss_proof: Option<BossProof>,
    ) -> Result<()> {
        instructions::mint_boss_drop(ctx, name, uri, item_type, rarity, base_stats, boss_proof)
    }

    pub fn mint_treasury_drop(
        ctx:Context<MintTreasuryDrop>,
        name: String,
        uri: String,
        item_type: ItemType,
        rarity: u8,
        base_stats: ItemStats,
        treasury_proof: Option<TreasuryProof>,
    ) -> Result<()> {
        instructions::mint_treasury_drop(ctx, name, uri, item_type, rarity, base_stats, treasury_proof)
    }
}