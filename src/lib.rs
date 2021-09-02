//! Library for writing safer Solana programs.
#![deny(missing_docs)]

pub mod assert;
pub mod program_ids;
pub mod validate;

use anchor_lang::prelude::*;

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
}
