use anchor_lang::prelude::*;
use crate::state::Fighter;
mod state;





#[derive(Accounts)]
pub struct InitializeFighter<'info> {
    #[account(init, payer = user, space = 8 + Fighter::INIT_SPACE)]
    pub fighter: Account<'info, Fighter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdateFighter<'info> {
    #[account(mut, has_one = authority)]
    pub fighter: Account<'info, Fighter>,
    pub authority: Signer<'info>,

}

#[derive(Accounts)]
pub struct DeleteFighter<'info> {
    #[account(mut, close = user, has_one = authority)]
    pub fighter: Account<'info, Fighter>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}
