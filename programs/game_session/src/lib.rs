use anchor_lang::prelude::*;

mod states;
mod instructions;
mod errors;

pub use states::*;
pub use instructions::*;
pub use errors::*;

// Replace this with your deployed program ID
declare_id!("4CWu3JYtBZtqPYsEnPheYt8kipe26gqygUAYiaBicQpD");

#[program]
pub mod game_session {
    use super::*;

    pub fn start_session(
        ctx: Context<StartSession>,
        session_id: u64,
        player_b: Pubkey,
        session_type: SessionType,
    ) -> Result<()> {
        instructions::start_session(ctx, session_id, player_b, session_type)
    }

    pub fn end_session(
        ctx: Context<EndSession>,
        result: SessionResult,
    ) -> Result<()> {
        instructions::end_session(ctx, result)
    }
}
