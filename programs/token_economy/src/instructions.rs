use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use crate::errors::ErrorCode;
use crate::state::*;

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
    from_vault.balance = from_vault.balance.checked_sub(reward_amount).ok_or(ErrorCode::Overflow)?;
    msg!("Player rewarded {} PIRATE tokens for completing level {}", reward_amount, level);
    Ok(())
}

pub fn reward_treasure_found(ctx: Context<Reward>, treasure_type: u8) -> Result<()> {
    let pirate = &mut ctx.accounts.pirate;
    let from_vault = &mut ctx.accounts.from_vault;
    let reward_amount = calculate_treasure_reward(treasure_type)?;
    require!(reward_amount > 0, ErrorCode::InvalidReward);
    pirate.total_supply = pirate.total_supply.checked_add(reward_amount).ok_or(ErrorCode::Overflow)?;
    from_vault.balance = from_vault.balance.checked_sub(reward_amount).ok_or(ErrorCode::Overflow)?;
    msg!("Player rewarded {} PIRATE tokens for finding treasure", reward_amount);
    Ok(())
}

pub fn daily_login_bonus(ctx: Context<Reward>) -> Result<()> {
    let pirate = &mut ctx.accounts.pirate;
    let from_vault = &mut ctx.accounts.from_vault;
    let bonus_amount = 55_u64;
    pirate.total_supply = pirate.total_supply.checked_add(bonus_amount).ok_or(ErrorCode::Overflow)?;
    from_vault.balance = from_vault.balance.checked_sub(bonus_amount).ok_or(ErrorCode::Overflow)?;
    msg!("Player received a daily login bonus of {} PIRATE tokens", bonus_amount);
    Ok(())
}

fn calculate_level_reward(level: u8) -> Result<u64> {
    let base_reward = match level {
        1..=5 => 100,     // Easy levels: 100 tokens
        6..=10 => 250,    // Medium levels: 250 tokens
        11..=15 => 500,   // Hard levels: 500 tokens
        16..=20 => 1000,  // Boss levels: 1000 tokens
        _ => return Err(ErrorCode::InvalidLevel.into()),
    };
    Ok(base_reward)
}

fn calculate_treasure_reward(treasure_type: u8) -> Result<u64> {
    let reward = match treasure_type {
        1 => 150,   // Common treasure
        2 => 350,   // Rare treasure
        3 => 750,   // Epic treasure
        4 => 1500,  // Legendary treasure
        _ => return Err(ErrorCode::InvalidTreasure.into()),
    };
    Ok(reward)
}

#[derive(Accounts)]
pub struct MintPirateTokens<'info> {
    #[account(
        mut,
        seeds = [b"pirate", mint.key().as_ref()],
        bump
    )]
    pub pirate: Account<'info, Pirate>,

    #[account(mut)]
    pub to_vault: Account<'info, Vault>,

    pub authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnPirateTokens<'info> {
    #[account(
        mut,
        seeds = [b"pirate", mint.key().as_ref()],
        bump
    )]
    pub pirate: Account<'info, Pirate>,

    #[account(mut)]
    pub from_vault: Account<'info, Vault>,

    pub authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferPirateTokens<'info> {
    #[account(
        mut,
        seeds = [b"pirate", mint.key().as_ref()],
        bump
    )]
    pub pirate: Account<'info, Pirate>,

    #[account(mut)]
    pub from_vault: Account<'info, Vault>,
    #[account(mut)]
    pub to_vault: Account<'info, Vault>,

    pub authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Reward<'info> {
    #[account(
        mut,
        seeds = [b"pirate", mint.key().as_ref()],
        bump
    )]
    pub pirate: Account<'info, Pirate>,
    #[account(mut)]
    pub from_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint : AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}