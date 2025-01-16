use crate::{constants::*, state::GameState};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
      init,
      payer = payer,
      seeds = [ESCROW_SEED, payer.key().as_ref()],
      space = DISCRIMINATOR_SIZE + GameState::INIT_SPACE,
      bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(seeds = [VAULT_SEED, game_state.key().as_ref()], bump)]
    pub player_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, bump: u8, vault_bump: u8) -> Result<()> {
        self.game_state.player = self.payer.key();
        self.game_state.bump = bump;
        self.game_state.randomness_account = Pubkey::default();
        self.game_state.random_num = 0;
        self.game_state.bet = 1 * 10 ^ 6;
        self.game_state.vault_bump = vault_bump;
        self.game_state.randomness_consumed = false;
        Ok(())
    }
}
