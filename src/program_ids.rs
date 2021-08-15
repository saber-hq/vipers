use anchor_lang::{prelude::*, solana_program::system_program};

/// ID of the SPL token program.
pub static TOKEN_PROGRAM_ID: Pubkey = anchor_spl::token::ID;

/// ID of the SPL associated token program.
pub static ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey = spl_associated_token_account::ID;

/// ID of the system program.
pub static SYSTEM_PROGRAM_ID: Pubkey = system_program::ID;
