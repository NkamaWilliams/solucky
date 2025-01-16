use crate::{
    constants::*,
    error::GameErrorCode,
    state::{GameState, GlobalState},
};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct ConfirmGuess<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [ESCROW_SEED, payer.key().as_ref()], bump = game_state.bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut, seeds = [GLOBAL_ESCROW_SEED, global_state.owner.as_ref()], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut, seeds = [VAULT_SEED, game_state.key().as_ref()], bump = game_state.vault_bump)]
    pub player_vault: SystemAccount<'info>,
    #[account(mut, seeds = [GLOBAL_VAULT_SEED, global_state.key().as_ref()], bump = global_state.vault_bump)]
    pub global_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ConfirmGuess<'info> {
    pub fn confirm_guess(&mut self, guess: u64) -> Result<()> {
        require!(
            !self.game_state.randomness_consumed,
            GameErrorCode::RandomValueUsed
        );
        require!(
            self.player_vault.lamports() >= self.game_state.bet,
            GameErrorCode::InsufficientFunds
        );

        if guess == self.game_state.random_num {
            msg!("You win!!");
            let accounts = Transfer {
                from: self.global_vault.to_account_info(),
                to: self.player_vault.to_account_info(),
            };
            transfer(
                CpiContext::new(self.system_program.to_account_info(), accounts),
                self.game_state.bet,
            )?;
        } else {
            msg!("You lose!!");
            let accounts = Transfer {
                from: self.player_vault.to_account_info(),
                to: self.global_vault.to_account_info(),
            };
            transfer(
                CpiContext::new(self.system_program.to_account_info(), accounts),
                self.game_state.bet,
            )?;
        }

        self.game_state.randomness_consumed = true;
        Ok(())
    }
}
