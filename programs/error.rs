use anchor_lang::prelude::*;

#[error_code]
pub enum FetchError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Not Allowed.")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already marked.")]
    AlreadyMarked,
}
