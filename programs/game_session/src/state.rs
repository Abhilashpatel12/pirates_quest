use anchor_lang::prelude::*;

#[account]
pub struct GameSession {
    pub session_id: u64,             // Unique session number for reference
    pub creator: Pubkey,             // Player/team who started the session
    pub player_a: Pubkey,            // Main player or team
    pub player_b: Pubkey,            // Opponent or Pubkey::default() for PvE
    pub session_type: SessionType,   // PvP, PvE
    pub start_time: i64,             // Unix timestamp for when session began
    pub end_time: Option<i64>,       // Set when match is over
    pub result: SessionResult,       // What happened in this session
    pub is_active: bool,             // True = ongoing
    pub bump: u8,                    // PDA bump for Anchor
}

// Game session mode/type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SessionType {
    Pve,
    Pvp,
}

// Result at the end of game session
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SessionResult {
    Ongoing,
    PlayerAWon,
    PlayerBWon,
    Draw,
}

impl GameSession {
    pub const INIT_SPACE: usize = 8 + 8 + 32 + 32 + 32 + 1 + 8 + 9 + 1 + 1 + 1;
}
