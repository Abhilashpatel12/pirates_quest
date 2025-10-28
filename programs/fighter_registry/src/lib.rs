use anchor_lang::prelude::*;

pub mod instructions;
use crate::instructions::*;

pub mod state;

pub mod errors;

declare_id!("BkvZpft92ThTiVG7XNDWpdha1432vG2LzdjWijGdb8KQ");

#[program]
pub mod fighter_registry {
    use super::*;

    pub fn initializefighter(ctx: Context<InitializeFighter>, name: String) -> Result<()> {
        instructions::initializefighter(ctx, name)
    }

    pub fn updatefighter(
        ctx: Context<UpdateFighter>,
        health: u16,
        stamina: u16,
        experience: u32,
        level: u8,
        tokens: u64,
    ) -> Result<()> {
        instructions::updatefighter(ctx, health, stamina, experience, level, tokens)
    }

    pub fn deletefighter(ctx: Context<DeleteFighter>) -> Result<()> {
        instructions::deletefighter(ctx)
    }
}
