use anchor_lang::prelude::*;

declare_id!("9jpNFgneTzqwfZxV3uC1YvxpGRU39mXuSQxke4DrsCN7");

#[program]
pub mod programs_token_economy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
