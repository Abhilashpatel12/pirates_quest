use anchor_lang::prelude::*;

declare_id!("7pwQXa7w9hsXJMJCzMFpXJq6vtncnFeUpgfBFXwjmQKY");

#[program]
pub mod programs_fighter_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
