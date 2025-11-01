use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    instructions::{CreateCollectionV2CpiBuilder, CreateV2CpiBuilder},
    types::{Attribute, Attributes, Plugin, PluginAuthorityPair},
};
use crate::state::*;
use crate::errors::*;

// Initialize a Collection Asset (Metaplex Core collection)

 pub fn initialize_collection(
        ctx: Context<InitializeCollection>,
        name: String,
        uri: String,
    ) -> 
Result<()> {
        CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .collection(&ctx.accounts.collection_mint.to_account_info())
            .payer(&ctx.accounts.authority.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(name.clone())
            .uri(uri.clone())
            .invoke()?;
        
        let collection = &mut ctx.accounts.collection;
        collection.authority = ctx.accounts.authority.key();
        collection.total_minted = 0;
        collection.name = name;
        collection.uri = uri;
        Ok(())
    }

    pub fn mint_game_item(
        ctx: Context<MintGameItem>,
        name: String,
        uri: String,
        item_type: ItemType,
        rarity: u8,
        base_stats: ItemStats,
    ) -> Result<()> {
        require!(rarity >= 1 && rarity <= 5, GameError::InvalidRarity);
        require!(base_stats.attack_power >= 0, GameError::StatValueNegative);
        require!(base_stats.defense >= 0, GameError::StatValueNegative);
        require!(base_stats.speed_boost >= 0, GameError::StatValueNegative);

        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection_mint.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .owner(Some(&ctx.accounts.owner.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(name.clone())
            .uri(uri.clone())
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute { key: "item_type".into(), value: format!("{:?}", item_type) },
                        Attribute { key: "rarity".into(), value: rarity.to_string() },
                        Attribute { key: "attack".into(), value: base_stats.attack_power.to_string() },
                        Attribute { key: "defense".into(), value: base_stats.defense.to_string() },
                        Attribute { key: "speed".into(), value: base_stats.speed_boost.to_string() },
                        Attribute { key: "special_ability".into(), value: base_stats.special_ability.to_string() },
                    ]
                }),
                authority: None,
            }])
            .invoke()?;

        let collection = &mut ctx.accounts.collection;
        let game_item = &mut ctx.accounts.game_item;
        collection.total_minted += 1;
        game_item.asset = ctx.accounts.asset.key();
        game_item.owner = ctx.accounts.owner.key();
        game_item.item_type = item_type;
        game_item.rarity = rarity;
        game_item.stats = base_stats;
        game_item.level = 1;
        game_item.experience = 0;
        game_item.is_equipped = false;
        game_item.is_listed = false;
        game_item.boss_proof = None;
        game_item.treasury_proof = None;
        game_item.created_at = Clock::get()?.unix_timestamp;
        Ok(())
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
        require!(rarity == 5, GameError::BossDropNotLegendary);
        require!(boss_proof.is_some(), GameError::MissingBossProof);

        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection_mint.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .owner(Some(&ctx.accounts.owner.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(name.clone())
            .uri(uri.clone())
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute { key: "item_type".into(), value: format!("{:?}", item_type) },
                        Attribute { key: "rarity".into(), value: rarity.to_string() },
                        Attribute { key: "attack".into(), value: base_stats.attack_power.to_string() },
                        Attribute { key: "defense".into(), value: base_stats.defense.to_string() },
                        Attribute { key: "speed".into(), value: base_stats.speed_boost.to_string() },
                        Attribute { key: "special_ability".into(), value: base_stats.special_ability.to_string() },
                        Attribute { key: "boss".into(), value: boss_proof.as_ref().map_or("".to_string(), |bp| format!("#{} @ {}", bp.boss_id, bp.defeat_timestamp)) },
                    ]
                }),
                authority: None,
            }])
            .invoke()?;

        let collection = &mut ctx.accounts.collection;
        let game_item = &mut ctx.accounts.game_item;
        collection.total_minted += 1;
        game_item.asset = ctx.accounts.asset.key();
        game_item.owner = ctx.accounts.owner.key();
        game_item.item_type = item_type;
        game_item.rarity = rarity;
        game_item.stats = base_stats;
        game_item.level = 1;
        game_item.experience = 0;
        game_item.is_equipped = false;
        game_item.is_listed = false;
        game_item.boss_proof = boss_proof;
        game_item.treasury_proof = None;
        game_item.created_at = Clock::get()?.unix_timestamp;
        Ok(())
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
        require!(rarity == 5, GameError::TreasuryDropNotLegendary);
        require!(treasury_proof.is_some(), GameError::MissingTreasuryProof);

        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection_mint.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .owner(Some(&ctx.accounts.owner.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(name.clone())
            .uri(uri.clone())
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute { key: "item_type".into(), value: format!("{:?}", item_type) },
                        Attribute { key: "rarity".into(), value: rarity.to_string() },
                        Attribute { key: "attack".into(), value: base_stats.attack_power.to_string() },
                        Attribute { key: "defense".into(), value: base_stats.defense.to_string() },
                        Attribute { key: "speed".into(), value: base_stats.speed_boost.to_string() },
                        Attribute { key: "special_ability".into(), value: base_stats.special_ability.to_string() },
                        Attribute { key: "treasury".into(), value: treasury_proof.as_ref().map_or("".to_string(), |tp| format!("@ {}", tp.claim_timestamp)) },
                    ]
                }),
                authority: None,
            }])
            .invoke()?;

        let collection = &mut ctx.accounts.collection;
        let game_item = &mut ctx.accounts.game_item;
        collection.total_minted += 1;
        game_item.asset = ctx.accounts.asset.key();
        game_item.owner = ctx.accounts.owner.key();
        game_item.item_type = item_type;
        game_item.rarity = rarity;
        game_item.stats = base_stats;
        game_item.level = 1;
        game_item.experience = 0;
        game_item.is_equipped = false;
        game_item.is_listed = false;
        game_item.boss_proof = None;
        game_item.treasury_proof = treasury_proof;
        game_item.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }


#[derive(Accounts)]
pub struct InitializeCollection<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Collection::INIT_SPACE,
        seeds = [b"collection"],
        bump
    )]
    pub collection: Account<'info, Collection>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Collection mint created by Metaplex Core
    #[account(mut, signer)]
    pub collection_mint: UncheckedAccount<'info>,
    
    /// CHECK: Metaplex Core Program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintGameItem<'info> {
    #[account(
        mut,
        seeds = [b"collection"],
        bump
    )]
    pub collection: Account<'info, Collection>,
    
    /// CHECK: Collection mint
    #[account(mut)]
    pub collection_mint: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + GameItem::INIT_SPACE,
        seeds = [b"game_item", asset.key().as_ref()],
        bump
    )]
    pub game_item: Account<'info, GameItem>,
    
    /// CHECK: Core NFT asset
    #[account(mut, signer)]
    pub asset: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Owner
    pub owner: UncheckedAccount<'info>,
    
    /// CHECK: Metaplex Core
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintBossDrop<'info> {
    #[account(
        mut,
        seeds = [b"collection"],
        bump
    )]
    pub collection: Account<'info, Collection>,
    
    /// CHECK: Collection mint
    #[account(mut)]
    pub collection_mint: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + GameItem::INIT_SPACE,
        seeds = [b"game_item", asset.key().as_ref()],
        bump
    )]
    pub game_item: Account<'info, GameItem>,
    
    /// CHECK: Asset
    #[account(mut, signer)]
    pub asset: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Owner
    pub owner: UncheckedAccount<'info>,
    
    /// CHECK: MPL Core
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct MintTreasuryDrop<'info> {
    #[account(
        mut,
        seeds = [b"collection"],
        bump
    )]
    pub collection: Account<'info, Collection>,

    /// CHECK: Collection mint
    #[account(mut)]
    pub collection_mint: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + GameItem::INIT_SPACE,
        seeds = [b"game_item", asset.key().as_ref()],
        bump
    )]
    pub game_item: Account<'info, GameItem>,
    /// CHECK: Asset
    #[account(mut, signer)]
    pub asset: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Owner
    pub owner: UncheckedAccount<'info>,

    /// CHECK: The Metaplex Core program ID, required for CPIs.
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,

}
