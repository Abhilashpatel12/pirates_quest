use anchor_lang::prelude::*;
mod states;
mod instructions;
mod errors;

pub use states::*;
pub use instructions::*;
pub use errors::*;


declare_id!("BjuQiWyhrmQd3JeWVLbkscM3mYZorti2Y9bDBtLPz4TU");

#[program]
pub mod programs_marketplace {
    use super::*;

   pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        instructions::list_nft(ctx, price)
    }
    pub fn update_listing(ctx: Context<UpdateListing>, new_price: u64, is_active: bool) -> Result<()> {
        instructions::update_listing(ctx, new_price, is_active)
    }
    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        instructions::cancel_listing(ctx)
    }
    pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
        instructions::buy_nft(ctx)
    }
}
