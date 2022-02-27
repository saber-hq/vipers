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
    #[msg("Bump not found.")]
    UnknownBump,
}

/// Conversions into a [CmpError].
pub trait IntoCmpError {
    /// Converts the value into a [CmpError].
    fn into_cmp_error(self) -> Option<CmpError>;
}

impl<T> IntoCmpError for Result<T> {
    fn into_cmp_error(self) -> Option<CmpError> {
        self.err()?.into_cmp_error()
    }
}

impl IntoCmpError for anchor_lang::error::Error {
    fn into_cmp_error(self) -> Option<CmpError> {
        Some(CmpError(self))
    }
}

impl IntoCmpError for Option<anchor_lang::error::Error> {
    fn into_cmp_error(self) -> Option<CmpError> {
        self?.into_cmp_error()
    }
}

impl From<anchor_lang::error::Error> for CmpError {
    fn from(err: anchor_lang::error::Error) -> Self {
        CmpError(err)
    }
}

/// A comparable error: an error that can be compared (via equality) to other errors.
#[repr(transparent)]
#[derive(Debug)]
pub struct CmpError(pub anchor_lang::error::Error);

impl PartialEq for CmpError {
    fn eq(&self, other: &Self) -> bool {
        let (CmpError(a), CmpError(b)) = (self, other);
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

impl Eq for CmpError {}

impl Display for CmpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_program_errors_equal() {
        assert_eq!(
            anchor_lang::error::Error::ProgramError(ProgramError::InvalidArgument.into())
                .into_cmp_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::InvalidArgument.into())
                .into_cmp_error()
        );
    }

    #[test]
    fn test_program_errors_equal_custom() {
        assert_eq!(
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(10).into())
                .into_cmp_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(10).into())
                .into_cmp_error()
        );
    }

    #[test]
    fn test_program_errors_mismatch() {
        assert_ne!(
            anchor_lang::error::Error::ProgramError(ProgramError::InvalidArgument.into())
                .into_cmp_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::AccountAlreadyInitialized.into())
                .into_cmp_error()
        );
    }

    #[test]
    fn test_program_errors_mismatch_custom() {
        assert_ne!(
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(10).into())
                .into_cmp_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(11).into())
                .into_cmp_error()
        );
    }

    #[test]
    fn test_program_errors_equal_none() {
        assert_eq!(None.into_cmp_error(), None.into_cmp_error());
    }

    #[test]
    fn test_program_errors_mismatch_random() {
        assert_ne!(
            None.into_cmp_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(11).into())
                .into_cmp_error()
        );
    }

    #[test]
    fn test_program_errors_mismatch_anchor_program() {
        assert_ne!(
            error!(ErrorCode::MyError).into_cmp_error(),
            anchor_lang::error::Error::ProgramError(ProgramError::Custom(11).into())
                .into_cmp_error()
        );
    }

    #[test]
    fn test_display_anchor_error() {
        let anchor_error = error!(ErrorCode::MyError);
        assert_eq!(
            format!("{}", anchor_error),
            format!("{}", anchor_error.into_cmp_error().unwrap())
        );
    }

    #[error_code]
    pub enum ErrorCode {
        MyError,
        MyOtherError,
    }

    #[test]
    fn test_anchor_errors_eq() {
        assert_eq!(
            error!(ErrorCode::MyError).into_cmp_error(),
            error!(ErrorCode::MyError).into_cmp_error(),
        );
    }

    #[test]
    fn test_anchor_errors_eq_result() {
        assert_eq!(
            (err!(ErrorCode::MyError) as Result<()>).into_cmp_error(),
            (err!(ErrorCode::MyError) as Result<()>).into_cmp_error(),
        );
    }

    #[test]
    fn test_anchor_errors_ne_result() {
        assert_ne!(
            (err!(ErrorCode::MyError) as Result<()>).into_cmp_error(),
            (err!(ErrorCode::MyOtherError) as Result<()>).into_cmp_error(),
        );
    }

    #[test]
    fn test_from_anchor_error() {
        let error_a: CmpError = (error!(ErrorCode::MyError)).into();
        let error_b: CmpError = (error!(ErrorCode::MyError)).into();
        assert_eq!(error_a, error_b);
    }
}
