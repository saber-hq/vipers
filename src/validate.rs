//! Validations for accounts.

use anchor_lang::prelude::*;

/// Validator for [Accounts] structs.
pub trait Validate<'info>: Accounts<'info> {
    /// Validates the account struct.
    fn validate(&self) -> ProgramResult;
}
