//! Library for writing safer Solana programs.

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

#[error(offset = 1100)]
pub enum VipersError {
    KeyMismatch,
    ATAMismatch,
    ProgramIDMismatch,
    IntegerOverflow,
}
