use anchor_lang::prelude::*;

declare_id!("6iuP8j1RfwRyHCDUdtrSv3GAtZa9ZKkePMVVbNRXMoH8");

#[program]
pub mod mood_status_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
