use crate::{constants::*, error::GameErrorCode, state::GameState};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use switchboard_on_demand::RandomnessAccountData;

#[derive(Accounts)]
pub struct RequestRandomness<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
    mut,
    seeds = [ESCROW_SEED, payer.key().as_ref()],
    bump = game_state.bump
    )]
    pub game_state: Account<'info, GameState>,
    ///CHECK: Account data is manually validated in the handler
    pub randomness_account_data: AccountInfo<'info>,
    #[account(mut, seeds = [VAULT_SEED, game_state.key().as_ref()], bump=game_state.vault_bump)]
    pub player_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RequestRandomness<'info> {
    pub fn request_randomness(&mut self, randomness_account: Pubkey) -> Result<()> {
        let clock = Clock::get().unwrap();
        let randomness_data =
            RandomnessAccountData::parse(self.randomness_account_data.data.borrow()).unwrap();

        if randomness_data.seed_slot != clock.slot - 1 {
            msg!("seed slot: {}", randomness_data.seed_slot);
            msg!("slot: {}", clock.slot);
            return Err(GameErrorCode::RandomnessRevealed.into());
        }

        self.game_state.commit_slot = randomness_data.seed_slot;

        let transfer_instruction = Transfer {
            from: self.payer.to_account_info(),
            to: self.player_vault.to_account_info(),
        };

        transfer(
            CpiContext::new(self.system_program.to_account_info(), transfer_instruction),
            self.game_state.bet,
        )?;

        self.game_state.randomness_account = randomness_account;
        msg!("Randomness requested");
        Ok(())
    }
}
