use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameSession {
    pub session_id: u64,
    pub creator: Pubkey,
    pub player_a: Pubkey,
    pub player_b: Pubkey,
    pub session_type: SessionType,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub result: SessionResult,
    pub is_active: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum SessionType {
    Pve,
    Pvp,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum SessionResult {
    Ongoing,
    PlayerAWon,
    PlayerBWon,
    Draw,
}
