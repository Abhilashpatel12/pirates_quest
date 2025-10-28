use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("HtHrPWT8w5tKk4Ex2xjprB9b1bmm96GfcXxpq4aD4oLT");

#[program]
pub mod token_economy {
    use super::*;

    // Mint tokens to a player's vault
    pub fn mint_pirate_tokens(
        ctx: Context<MintPirateTokens>, 
        amount: u64
    ) -> Result<()> {
        instructions::mint_pirate_tokens(ctx, amount)
    }

    // Burn tokens from a player's vault
    pub fn burn_pirate_tokens(
        ctx: Context<BurnPirateTokens>, 
        amount: u64
    ) -> Result<()> {
        instructions::burn_pirate_tokens(ctx, amount)
    }

    // Transfer tokens between vaults (wallets)
    pub fn transfer_pirate_tokens(
        ctx: Context<TransferPirateTokens>, 
        amount: u64
    ) -> Result<()> {
        instructions::transfer_pirate_tokens(ctx, amount)
    }

    // Reward for level completion
    pub fn reward_level_completion(
        ctx: Context<Reward>, 
        level: u8
    ) -> Result<()> {
        instructions::reward_level_completion(ctx, level)
    }

    // Reward for treasure finding
    pub fn reward_treasure_found(
        ctx: Context<Reward>, 
        treasure_type: u8
    ) -> Result<()> {
        instructions::reward_treasure_found(ctx, treasure_type)
    }

    // Daily login bonus
    pub fn reward_daily_login(ctx: Context<Reward>) -> Result<()> {
        instructions::daily_login_bonus(ctx)
    }
}
