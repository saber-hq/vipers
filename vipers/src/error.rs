#![allow(missing_docs)]

use std::fmt::Display;

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

/// Conversions into a [VipersWrappedError].
pub trait IntoVipersError {
    /// Converts the value into a [VipersWrappedError].
    fn into_error(self) -> Option<ComparableError>;
}

impl IntoVipersError for anchor_lang::error::Error {
    fn into_error(self) -> Option<ComparableError> {
        Some(ComparableError(self))
    }
}

impl IntoVipersError for Option<anchor_lang::error::Error> {
    fn into_error(self) -> Option<ComparableError> {
        self?.into_error()
    }
}

impl From<anchor_lang::error::Error> for ComparableError {
    fn from(err: anchor_lang::error::Error) -> Self {
        ComparableError(err)
    }
}

/// An error that can be compared (via equality) to other errors.
#[repr(transparent)]
#[derive(Debug)]
pub struct ComparableError(pub anchor_lang::error::Error);

impl PartialEq for ComparableError {
    fn eq(&self, other: &Self) -> bool {
        let (ComparableError(a), ComparableError(b)) = (self, other);
        match (a, b) {
            (Error::AnchorError(err_a), Error::AnchorError(err_b)) => {
                err_a.error_code_number == err_b.error_code_number
            }
            (Error::ProgramError(err_a), Error::ProgramError(err_b)) => {
                err_a.program_error == err_b.program_error
            }
            _ => false,
        }
    }
}

impl Eq for ComparableError {}

impl Display for ComparableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_errors_equal() {
        assert_eq!(
            anchor_lang::error::Error::ProgramError(ProgramError::InvalidArgument.into())
                .into_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::InvalidArgument.into())
                .into_error()
        );
    }

    #[test]
    fn test_program_errors_equal_custom() {
        assert_eq!(
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(10).into()).into_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(10).into()).into_error()
        );
    }

    #[test]
    fn test_program_errors_mismatch() {
        assert_ne!(
            anchor_lang::error::Error::ProgramError(ProgramError::InvalidArgument.into())
                .into_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::AccountAlreadyInitialized.into())
                .into_error()
        );
    }

    #[test]
    fn test_program_errors_mismatch_custom() {
        assert_ne!(
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(10).into()).into_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(11).into()).into_error()
        );
    }

    #[test]
    fn test_program_errors_equal_none() {
        assert_eq!(None.into_error(), None.into_error());
    }

    #[test]
    fn test_program_errors_mismatch_random() {
        assert_ne!(
            None.into_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(11).into()).into_error()
        );
    }
}
