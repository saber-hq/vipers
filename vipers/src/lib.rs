//! Library for writing safer Solana programs.
#![deny(missing_docs)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

pub mod assert;
pub mod borrow_key;
pub mod validate;

use anchor_lang::prelude::*;
pub use spl_associated_token_account as ata;

/// The prelude contains all commonly used components of the crate.
/// All Vipers programs should include it via `vipers::prelude::*;`.
pub mod prelude {
    pub use super::{
        assert_ata, assert_is_ata, assert_keys_eq, assert_keys_neq, assert_owner, invariant,
        program_err, throw_err, try_or_err, unwrap_int, unwrap_opt, unwrap_or_err,
    };
    pub use super::{borrow_key::BorrowKey, validate::Validate};
}

declare_id!("VipersTest111111111111111111111111111111111");

/// Validates a derived program address.
pub fn validate_derived_address(
    derived_address: &Pubkey,
    program_id: &Pubkey,
    seeds: &[&[u8]],
) -> bool {
    match Pubkey::create_program_address(seeds, program_id) {
        Ok(key) => *derived_address == key,
        _ => false,
    }
}

/// Vipers validation error.
#[allow(missing_docs)]
#[error(offset = 1100)]
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
}
