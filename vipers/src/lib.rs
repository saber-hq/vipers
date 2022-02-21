//! Library for writing safer Solana programs.
#![deny(missing_docs)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

pub mod assert;
mod error;
mod keyref;
pub mod validate;

use anchor_lang::prelude::*;
pub use error::*;
pub use keyref::AsKeyRef;
#[cfg(feature = "spl-associated-token-account")]
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
/// assert!(!validate_derived_address(
///   &solana_program::system_program::ID, &vipers::ID, seeds
/// ));
/// ```
pub fn validate_derived_address(
    derived_address: &Pubkey,
    program_id: &Pubkey,
    seeds: &[&[u8]],
) -> bool {
    match Pubkey::create_program_address(seeds, program_id) {
        Ok(ref key) => derived_address == key,
        _ => false,
    }
}

pub mod prelude {
    //! The prelude contains all commonly used components of the crate. All programs should include it via `use vipers::prelude::*;`.

    pub use super::{
        assert_is_zero_token_account, assert_keys_eq, assert_keys_neq, invariant, try_or_err,
        unwrap_checked, unwrap_int, unwrap_opt, unwrap_opt_block, unwrap_or_err, AsKeyRef,
        IntoVipersError, Validate, VipersError,
    };
}
