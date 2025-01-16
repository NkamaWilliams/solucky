use crate::{
  constants::*,
  state::GlobalState,
};
use anchor_lang::{
  prelude::*,
  system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct InitializeGlobal<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = DISCRIMINATOR_SIZE + GlobalState::INIT_SPACE,
        seeds = [GLOBAL_ESCROW_SEED, owner.key().as_ref()],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [GLOBAL_VAULT_SEED, global_state.key().as_ref()], 
        bump
    )]
    pub global_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeGlobal<'info> {
    pub fn initialize_global(&mut self, bump: u8, vault_bump: u8) -> Result<()> {
        self.global_state.owner = self.owner.key();
        self.global_state.bump = bump;
        self.global_state.vault_bump = vault_bump;

        let transfer_accounts = Transfer {
            from: self.owner.to_account_info(),
            to: self.global_vault.to_account_info(),
        };

        transfer(
            CpiContext::new(self.system_program.to_account_info(), transfer_accounts),
            2 * 10u64.pow(9),
        )?;

        Ok(())
    }
}