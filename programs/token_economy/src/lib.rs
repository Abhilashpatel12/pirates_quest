use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

pub use state::*;
pub use errors::*;
use instructions::*;

declare_id!("3UUL22zcovP754udhGLayo7UCdnbHn63S4yWT1oUzMky");

#[program]
pub mod token_economy {
    use super::*;

    pub fn initialize_pirate(ctx: Context<InitializePirate>) -> Result<()> {
        instructions::initialize_pirate(ctx)
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault(ctx)
    }

    // Mint tokens to a player's vault
    pub fn mint_pirate_tokens(
        ctx: Context<MintPirateTokens>, 
        amount: u64
    ) -> anchor_lang::prelude::Result<()> {
        instructions::mint_pirate_tokens(ctx, amount)
    }

    // Burn tokens from a player's vault
    pub fn burn_pirate_tokens(
        ctx: Context<BurnPirateTokens>, 
        amount: u64
    ) -> anchor_lang::prelude::Result<()> {
        instructions::burn_pirate_tokens(ctx, amount)
    }

    // Transfer tokens between vaults (wallets)
    pub fn transfer_pirate_tokens(
        ctx: Context<TransferPirateTokens>, 
        amount: u64
    ) -> anchor_lang::prelude::Result<()> {
        instructions::transfer_pirate_tokens(ctx, amount)
    }

    // Reward for level completion
    pub fn reward_level_completion(
        ctx: Context<Reward>, 
        level: u8
    ) -> anchor_lang::prelude::Result<()> {
        instructions::reward_level_completion(ctx, level)
    }

    // Reward for treasure finding
    pub fn reward_treasure_found(
        ctx: Context<Reward>, 
        treasure_type: u8
    ) -> anchor_lang::prelude::Result<()> {
        instructions::reward_treasure_found(ctx, treasure_type)
    }

    // Daily login bonus
    pub fn reward_daily_login(ctx: Context<Reward>) -> anchor_lang::prelude::Result<()> {
        instructions::daily_login_bonus(ctx)
    }
}
