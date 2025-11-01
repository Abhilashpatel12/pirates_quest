use anchor_lang::prelude::*;



#[account]
pub struct Listing{
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub is_active: bool,
    pub bump: u8,
}

impl Listing {
    pub const INIT_SPACE: usize = 32 + 32 + 8 + 1 + 1; // 74 bytes
}
