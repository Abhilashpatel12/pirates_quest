use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::errors::ErrorCode;
use crate::state::{Pirate, Vault};

#[derive(Accounts)]
pub struct InitializePirate<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1 + 8 + 1,
        seeds = [b"pirate", mint.key().as_ref()],
        bump
    )]
    pub pirate: Account<'info, Pirate>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintPirateTokens<'info> {
    #[account(mut)]
    pub pirate: Account<'info, Pirate>,
    #[account(mut)]
    pub to_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct BurnPirateTokens<'info> {
    #[account(mut)]
    pub pirate: Account<'info, Pirate>,
    #[account(mut)]
    pub from_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferPirateTokens<'info> {
    #[account(mut)]
    pub from_vault: Account<'info, Vault>,
    #[account(mut)]
    pub to_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reward<'info> {
    #[account(mut)]
    pub pirate: Account<'info, Pirate>,
    #[account(mut)]
    pub from_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_pirate(ctx: Context<InitializePirate>) -> Result<()> {
    ctx.accounts.pirate.mint = ctx.accounts.mint.key();
    ctx.accounts.pirate.authority = ctx.accounts.authority.key();
    ctx.accounts.pirate.decimals = ctx.accounts.mint.decimals;
    ctx.accounts.pirate.total_supply = 0;
    ctx.accounts.pirate.bump = ctx.bumps.pirate;
    Ok(())
}

pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
    ctx.accounts.vault.owner = ctx.accounts.owner.key();
    ctx.accounts.vault.balance = 0;
    Ok(())
}

// Mints new Pirate tokens by increasing total_supply and user vault balance.
pub fn mint_pirate_tokens(ctx: Context<MintPirateTokens>, amount: u64) -> Result<()> {
    let pirate = &mut ctx.accounts.pirate;
    let to_vault = &mut ctx.accounts.to_vault;
    pirate.total_supply = pirate.total_supply.checked_add(amount).ok_or(ErrorCode::Overflow)?;
    to_vault.balance = to_vault.balance.checked_add(amount).ok_or(ErrorCode::Overflow)?;
    Ok(())
}

// Burns Pirate tokens, reducing supply and vault balance.
pub fn burn_pirate_tokens(ctx: Context<BurnPirateTokens>, amount: u64) -> Result<()> {
    let pirate = &mut ctx.accounts.pirate;
    let from_vault = &mut ctx.accounts.from_vault;
    require!(from_vault.balance >= amount, ErrorCode::InsufficientBalance);
    pirate.total_supply = pirate.total_supply.checked_sub(amount).ok_or(ErrorCode::Overflow)?;
    from_vault.balance = from_vault.balance.checked_sub(amount).ok_or(ErrorCode::Overflow)?;
    Ok(())
}

// Transfers Pirate tokens between users' SPL vaults.
pub fn transfer_pirate_tokens(ctx: Context<TransferPirateTokens>, amount: u64) -> Result<()> {
    let from_vault = &mut ctx.accounts.from_vault;
    let to_vault = &mut ctx.accounts.to_vault;
    require!(from_vault.balance >= amount, ErrorCode::InsufficientBalance);
    from_vault.balance = from_vault.balance.checked_sub(amount).ok_or(ErrorCode::Overflow)?;
    to_vault.balance = to_vault.balance.checked_add(amount).ok_or(ErrorCode::Overflow)?;
    Ok(())
}

pub fn reward_level_completion(ctx: Context<Reward>, level: u8) -> Result<()> {
    let pirate = &mut ctx.accounts.pirate;
    let from_vault = &mut ctx.accounts.from_vault;

    let reward_amount = calculate_level_reward(level)?;
    require!(reward_amount > 0, ErrorCode::InvalidReward);
    pirate.total_supply = pirate.total_supply.checked_add(reward_amount).ok_or(ErrorCode::Overflow)?;
    from_vault.balance = from_vault.balance.checked_add(reward_amount).ok_or(ErrorCode::Overflow)?;
    msg!("Player rewarded {} PIRATE tokens for completing level {}", reward_amount, level);
    Ok(())
}

pub fn reward_treasure_found(ctx: Context<Reward>, treasure_type: u8) -> Result<()> {
    let pirate = &mut ctx.accounts.pirate;
    let from_vault = &mut ctx.accounts.from_vault;
    let reward_amount = calculate_treasure_reward(treasure_type)?;
    require!(reward_amount > 0, ErrorCode::InvalidReward);
    pirate.total_supply = pirate.total_supply.checked_add(reward_amount).ok_or(ErrorCode::Overflow)?;
    from_vault.balance = from_vault.balance.checked_add(reward_amount).ok_or(ErrorCode::Overflow)?;
    msg!("Player rewarded {} PIRATE tokens for finding treasure", reward_amount);
    Ok(())
}

pub fn daily_login_bonus(ctx: Context<Reward>) -> Result<()> {
    let pirate = &mut ctx.accounts.pirate;
    let from_vault = &mut ctx.accounts.from_vault;
    let bonus_amount = 55_u64;
    pirate.total_supply = pirate.total_supply.checked_add(bonus_amount).ok_or(ErrorCode::Overflow)?;
    from_vault.balance = from_vault.balance.checked_add(bonus_amount).ok_or(ErrorCode::Overflow)?;
    Ok(())
}

// Helper functions
fn calculate_level_reward(level: u8) -> Result<u64> {
    Ok(level as u64 * 10)
}

fn calculate_treasure_reward(_treasure_type: u8) -> Result<u64> {
    Ok(100)
}