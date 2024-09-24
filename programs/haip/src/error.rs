use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Campaign already proposed")]
    AlreadyProposed,
    #[msg("Campaign has not been proposed")]
    NotProposed,
    #[msg("Campaign proposal is still running")]
    ProposedStillOpen
}
