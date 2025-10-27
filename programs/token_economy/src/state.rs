use anchor_lang::prelude::*;




#[account]
pub struct Pirate {
    pub mint : Pubkey,
    pub authority : Pubkey,
    pub decimals : u8,
    pub total_supply : u64,
    pub bump :u8,
}


#[account]
pub struct Vault {
    pub owner : Pubkey,
    pub balance : u64,
}