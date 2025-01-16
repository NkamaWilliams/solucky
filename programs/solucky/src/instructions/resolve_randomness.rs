use crate::{constants::*, error::GameErrorCode, state::GameState};
use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

#[derive(Accounts)]
pub struct ResolveRandomness<'info> {
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
}

impl<'info> ResolveRandomness<'info> {
    pub fn resolve_randomness(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let randomness_data =
            RandomnessAccountData::parse(self.randomness_account_data.data.borrow()).unwrap();
        if randomness_data.seed_slot != self.game_state.commit_slot {
            return Err(GameErrorCode::RandomnessExpired.into());
        }

        let revealed_random_value = randomness_data
            .get_value(&clock)
            .map_err(|_| GameErrorCode::RandomnessNotResolved)?;
        self.game_state.random_num = revealed_random_value[0] as u64 % 6 + 1;
        self.game_state.randomness_consumed = false;

        Ok(())
    }
}
