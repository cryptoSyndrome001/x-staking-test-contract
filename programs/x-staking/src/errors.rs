
use anchor_lang::prelude::*;

#[error_code]
pub enum XError {
    #[msg("Invalid address")]
    InvalidAddress,

    #[msg("not allowed")]
    NotAllowed,
}
