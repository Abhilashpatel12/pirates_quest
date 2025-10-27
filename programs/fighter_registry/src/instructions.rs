use anchor_lang::prelude::*;

use crate::state::Fighter;
use crate::errors::GameError;





pub fn initializefighter(ctx: Context<InitializeFighter>, name: String) -> Result<()> {
    // Validation with custom errors
    require!(!name.is_empty(), GameError::EmptyName);
    require!(name.len() <= 32, GameError::NameTooLong);

    let fighter = &mut ctx.accounts.fighter;
    fighter.authority = *ctx.accounts.user.key;
    fighter.name = name;
    fighter.health = 100;
    fighter.stamina = 100;
    fighter.experience = 0;
    fighter.level = 1;
    fighter.tokens = 0;
    fighter.bump = ctx.bumps.fighter;
    Ok(())
}

pub fn updatefighter(ctx: Context<UpdateFighter>, health: u16, stamina: u16, experience: u32, level: u8, tokens: u64) -> Result<()> {
    // Custom sanity checks with your errors
    require!(health <= 1000, GameError::InvalidHealth);
    require!(level > 0 && level <= 100, GameError::InvalidLevel);
    require!(stamina <= 200, GameError::InvalidStamina);

    let fighter = &mut ctx.accounts.fighter;
    fighter.health = health;
    fighter.stamina = stamina;
    fighter.experience = experience;
    fighter.level = level;
    fighter.tokens = tokens;
    Ok(())
}

pub fn deletefighter(_ctx: Context<DeleteFighter>) -> Result<()> {
    // No-op: Anchor will close the account & refund rent automatically
    Ok(())
}













#[derive(Accounts)]
pub struct InitializeFighter<'info> {
    #[account(init, payer = user, space = 8 + Fighter::INIT_SPACE,seeds = [b"fighter", user.key().as_ref()], bump)]
    pub fighter: Account<'info, Fighter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdateFighter<'info> {
    #[account(mut, has_one = authority, seeds = [b"fighter", authority.key().as_ref()], bump = fighter.bump)]
    pub fighter: Account<'info, Fighter>,
    pub authority: Signer<'info>,

}

#[derive(Accounts)]
pub struct DeleteFighter<'info> {
    #[account(mut, close = user, has_one = authority, seeds = [b"fighter", authority.key().as_ref()], bump = fighter.bump)]
    pub fighter: Account<'info, Fighter>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}
