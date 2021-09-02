//! Validations for accounts.

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::assert_owner;

/// Validates the contents of a variable. Generally used for [Accounts] structs and struct members.
pub trait Validate<'info> {
    /// Validates the account struct.
    fn validate(&self) -> ProgramResult;
}

/// Represents an account owned by a program.
pub trait ProgramOwned: AccountDeserialize + Clone {
    /// Gets the expected owner of the program owned account.
    fn expected_owner(&self) -> &'static Pubkey;
}

impl<'info, T: ProgramOwned> Validate<'info> for CpiAccount<'info, T> {
    fn validate(&self) -> ProgramResult {
        assert_owner!(*self, *self.expected_owner());
        Ok(())
    }
}

impl ProgramOwned for TokenAccount {
    fn expected_owner(&self) -> &'static Pubkey {
        &crate::program_ids::token::ID
    }
}

impl ProgramOwned for Mint {
    fn expected_owner(&self) -> &'static Pubkey {
        &crate::program_ids::token::ID
    }
}
