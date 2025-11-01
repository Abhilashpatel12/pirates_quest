use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::GameSessionError;

// CREATE a new game session
pub fn start_session(
    ctx: Context<StartSession>,
    session_id: u64,
    player_b: Pubkey,
    session_type: SessionType,
) -> Result<()> {
    let session = &mut ctx.accounts.game_session;
    session.session_id = session_id;
    session.creator = ctx.accounts.creator.key();
    session.player_a = ctx.accounts.creator.key(); // creator is player_a
    session.player_b = player_b;
    session.session_type = session_type;
    session.start_time = Clock::get()?.unix_timestamp;
    session.end_time = None;
    session.result = SessionResult::Ongoing;
    session.is_active = true;
    session.bump = ctx.bumps.game_session;
    Ok(())
}

// END an existing game session
pub fn end_session(
    ctx: Context<EndSession>,
    result: SessionResult,
) -> Result<()> {
    let session = &mut ctx.accounts.game_session;
    require!(session.is_active, GameSessionError::SessionAlreadyEnded);
    session.end_time = Some(Clock::get()?.unix_timestamp);
    session.result = result;
    session.is_active = false;
    Ok(())
}

// ------- Anchor Context Structs -------

#[derive(Accounts)]
#[instruction(session_id: u64)]
pub struct StartSession<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + GameSession::INIT_SPACE,
        seeds = [b"game_session", &session_id.to_le_bytes() as &[u8]],
        bump,
    )]
    pub game_session: Account<'info, GameSession>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndSession<'info> {
    #[account(
        mut,
        seeds = [b"game_session", &game_session.session_id.to_le_bytes()],
        bump = game_session.bump,
        has_one = creator,
    )]
    pub game_session: Account<'info, GameSession>,
    pub creator: Signer<'info>,
}
