use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameState {
    pub player: Pubkey,
    pub randomness_account: Pubkey,
    pub random_num: u64,
    pub bet: u64,
    pub commit_slot: u64,
    pub bump: u8,
    pub vault_bump: u8,
    pub randomness_consumed: bool,
}

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub owner: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
}
