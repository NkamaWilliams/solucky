use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq, Eq)]
pub enum GameErrorCode {
    #[msg("Unauthorized access attempt")]
    Unauthorized,
    InsufficientFunds,
    InsufficientFundsAfterRent,
    RandomnessRevealed,
    RandomnessExpired,
    RandomnessNotResolved,
    RandomValueUsed,
    UnauthorizedWithdrawal,
}
