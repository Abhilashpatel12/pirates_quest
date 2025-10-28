use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Collection {
    pub authority: Pubkey,
    pub total_minted: u64,
    #[max_len(50)]
    pub name: String,
    #[max_len(100)]
    pub uri: String,

    
}


#[account]
#[derive(InitSpace)]
pub struct GameItem {
    pub asset : Pubkey,
    pub owner : Pubkey,
    pub item_type: ItemType,
    pub rarity: u8,
    pub level: u8,
    pub stats : ItemStats,
    pub experience: u32,
    pub is_equipped: bool,
    pub is_listed : bool,
    pub created_at : i64,
    pub boss_proof: Option<BossProof>,
     pub treasury_proof: Option<TreasuryProof>,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub seller: Pubkey,
    pub asset: Pubkey,
    pub price: u64,
    pub game_item : Pubkey,
    pub listed_at : i64,
    pub bump: u8,


}

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace)]
pub struct ItemStats {
    pub attack_power: u16,
    pub  defense: u16,
    pub  speed_boost: u16,
    pub special_ability: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace)]
pub struct BossProof {
    pub boss_id :u8,
    pub defeat_timestamp: i64,
    pub player : Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace, Debug)]
pub enum ItemType {
   Weapon,
    Ship,
    Tool,
    Artifact,
    Cosmetic,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct TreasuryProof {
    pub claim_timestamp: i64,
    pub player: Pubkey,
    pub all_islands_conquered: bool,  // Must have beaten all 7 bosses
    pub final_battle_score: u32,      // PvP ranking
}