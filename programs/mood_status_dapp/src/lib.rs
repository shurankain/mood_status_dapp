use anchor_lang::prelude::*;

declare_id!("5eWpbFtEPydGTGaMEhLBoogRBU7sd7cU4AVfwCC8U1sA");

#[program]
pub mod mood_status_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
      let base_account = &mut ctx.accounts.base_account;
      base_account.mood = String::from("Happy");
      Ok(())
  }

  pub fn update(ctx: Context<Update>, mood: String) -> Result<()> {
      let base_account = &mut ctx.accounts.base_account;
      base_account.mood = mood;
      Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 50)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub mood: String,
}
