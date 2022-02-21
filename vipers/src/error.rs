#![allow(missing_docs)]

use anchor_lang::prelude::*;

/// Vipers validation error.
#[error_code(offset = 1100)]
pub enum VipersError {
    #[msg("Keys do not match.")]
    KeyMismatch,
    #[msg("Associated token account does not match.")]
    ATAMismatch,
    #[msg("Program ID does not match.")]
    ProgramIDMismatch,
    #[msg("Integer overflow.")]
    IntegerOverflow,
    #[msg("The provided account is not owned by the specified program.")]
    OwnerMismatch,
    #[msg("The provided token account is not an associated token account.")]
    InvalidATA,
    #[msg("Invariant failed.")]
    InvariantFailed,
    #[msg("Option unwrap failed.")]
    OptionUnwrapFailed,
    #[msg("Keys must not match.")]
    KeysMustNotMatch,
    #[msg("The provided token account is non-zero: amount must be zero, it should not have a delegate, and it should not have a close authority.")]
    TokenAccountIsNonZero,
}

pub trait IntoError {
    fn into_error(self) -> Option<Error>;
}

impl IntoError for anchor_lang::error::Error {
    fn into_error(self) -> Option<Error> {
        Some(self)
    }
}

impl IntoError for Option<anchor_lang::error::Error> {
    fn into_error(self) -> Option<Error> {
        self?.into_error()
    }
}

/// Checks if two [anchor_lang::error::Error]s are equal.
pub fn check_errors_equal<A: IntoError, B: IntoError>(a: A, b: B) -> bool {
    match (a.into_error(), b.into_error()) {
        (Some(Error::AnchorError(err_a)), Some(Error::AnchorError(err_b))) => {
            err_a.error_code_number == err_b.error_code_number
        }
        (Some(Error::ProgramError(err_a)), Some(Error::ProgramError(err_b))) => {
            err_a.program_error == err_b.program_error
        }
        (None, None) => true,
        _ => false,
    }
}
