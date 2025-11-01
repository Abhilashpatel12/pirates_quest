use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::states::*;
use crate::errors::*;
use std::str::FromStr;

// Paste your PIRATE token mint address here
pub const PIRATE_TOKEN_MINT: &str = "euGobpfkxFW7tqgkTvet5qT5qLDEYcECCddaT7DXUuk";

pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    listing.nft_mint = ctx.accounts.nft_mint.key();
    listing.seller = ctx.accounts.seller.key();
    listing.price = price;
    listing.is_active = true;
    listing.bump = ctx.bumps.listing;
    Ok(())


}

pub fn update_listing(ctx: Context<UpdateListing>, new_price: u64, is_active: bool) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    require_keys_eq!(listing.seller, ctx.accounts.seller.key(), MarketplaceError::Unauthorized);
    listing.price = new_price;
    listing.is_active = is_active;
    Ok(())
}


pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    require_keys_eq!(listing.seller, ctx.accounts.seller.key(), MarketplaceError::Unauthorized);
    listing.is_active = false;
    Ok(())
}


pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    require!(listing.is_active, MarketplaceError::ListingNotActive);
    require_keys_eq!(ctx.accounts.pirate_token_mint.key(), Pubkey::from_str(PIRATE_TOKEN_MINT).unwrap(), MarketplaceError::InvalidPirateMint);
    
    
    let cpi_accounts = Transfer {
        from: ctx.accounts.buyer_pirate_token.to_account_info(),
        to: ctx.accounts.seller_pirate_token.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, listing.price)?;

    let nft_cpi_accounts = Transfer {
        from: ctx.accounts.seller_nft_token.to_account_info(),
        to: ctx.accounts.buyer_nft_token.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    let cpi_ctx_nft = CpiContext::new(ctx.accounts.token_program.to_account_info(), nft_cpi_accounts);
    token::transfer(cpi_ctx_nft, 1)?; // Always 1 for NFT

   
    listing.is_active = false;
    Ok(())
}



#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(init, payer = seller, space = 8 + Listing::INIT_SPACE, seeds = [b"listing", nft_mint.key().as_ref()], bump)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    /// CHECK: Only used for address and seed
    pub nft_mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateListing<'info> {
    #[account(mut, seeds = [b"listing", listing.nft_mint.as_ref()], bump = listing.bump, has_one = seller)]
    pub listing: Account<'info, Listing>,
    pub seller: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut, seeds = [b"listing", listing.nft_mint.as_ref()], bump = listing.bump, has_one = seller)]
    pub listing: Account<'info, Listing>,
    pub seller: Signer<'info>,
}

#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(mut, seeds = [b"listing", listing.nft_mint.as_ref()], bump = listing.bump)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_pirate_token: Account<'info, TokenAccount>,
    /// CHECK: Seller's address, validated by the listing account
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    #[account(mut)]
    pub seller_pirate_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_nft_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_nft_token: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    pub pirate_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}
