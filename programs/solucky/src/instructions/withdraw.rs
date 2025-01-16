use crate::{constants::*, error::GameErrorCode, state::GameState};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[ESCROW_SEED, payer.key().as_ref()], bump=game_state.bump)]
    pub game_state: Account<'info, GameState>,
    #[account(seeds=[VAULT_SEED, game_state.key().as_ref()], bump=game_state.vault_bump)]
    pub player_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        require!(
            self.player_vault.lamports() > 0,
            GameErrorCode::InsufficientFunds
        );
        require!(
            self.payer.key() == self.game_state.player,
            GameErrorCode::UnauthorizedWithdrawal
        );

        let current_balance = self.player_vault.lamports();
        //Get rent exemption minimum balance
        let rent_exempt_minimum = Rent::get()?.minimum_balance(self.player_vault.data_len());

        // Calculate the withdrawable amount (subtract rent-exempt balance)
        let withdrawable_amount = current_balance.saturating_sub(rent_exempt_minimum);
        let accounts = Transfer {
            from: self.player_vault.to_account_info(),
            to: self.payer.to_account_info(),
        };

        require!(
            withdrawable_amount > 0,
            GameErrorCode::InsufficientFundsAfterRent
        );
        transfer(
            CpiContext::new(self.system_program.to_account_info(), accounts),
            withdrawable_amount,
        )?;
        msg!(
            "Withdrawal successful! Transfered {} lamports to {}",
            withdrawable_amount,
            self.payer.key()
        );
        Ok(())
    }
}
