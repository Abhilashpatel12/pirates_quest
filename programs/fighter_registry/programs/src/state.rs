use anchor_lang::prelude::*;








#[account]
#[derive(InitSpace)]
pub struct Fighter {
    pub authority: Pubkey,       
    #[max_len(32)]
    pub name: String,            
    pub health: u16,             
    pub stamina: u16,            
    pub experience: u32,         
    pub level: u8,               
}