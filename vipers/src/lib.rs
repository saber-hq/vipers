//! Library for writing safer Solana programs.
#![deny(missing_docs)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

pub mod assert;
pub mod validate;

use anchor_lang::prelude::*;
pub use spl_associated_token_account as ata;

pub use validate::Validate;

declare_id!("VipersTest111111111111111111111111111111111");

/// Validates a derived program address.
///
/// # Example
///
/// ```
/// use vipers::validate_derived_address;
/// use anchor_lang::solana_program;
/// let random = solana_program::system_program::ID;
/// let seeds: &[&[u8]] = &["test".as_ref() as &[u8], &random.to_bytes()];
/// let expected = static_pubkey::static_pubkey!("HjTCk2QYVrDPH1emJyrKBjtnooGqTvHfxa8ResZg3Kb4");
/// assert!(validate_derived_address(
///   &expected, &vipers::ID, seeds
/// ));
/// ```
pub fn validate_derived_address(
    derived_address: &Pubkey,
    program_id: &Pubkey,
    seeds: &[&[u8]],
) -> bool {
    println!("test");
    match Pubkey::create_program_address(seeds, program_id) {
        Ok(ref key) => derived_address == key,
        _ => false,
    }
}

pub mod prelude {
    //! The prelude contains all commonly used components of the crate. All programs should include it via `use vipers::prelude::*;`.

    pub use super::{
        assert_is_zero_token_account, assert_keys_eq, assert_keys_neq, invariant, try_or_err,
        unwrap_int, unwrap_opt, unwrap_opt_block, unwrap_or_err, Validate,
    };
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
    #[msg("The provided token account is non-zero: amount must be zero, it should not have a delegate, and it should not have a close authority.")]
    TokenAccountIsNonZero,
}
