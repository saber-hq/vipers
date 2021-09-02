//! Various program IDs.

/// SPL Token program.
pub mod token {
    solana_program::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
}

/// SPL Associated token program.
pub mod associated_token {
    solana_program::declare_id!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
}

/// System program.
pub mod system {
    /// System program ID.
    pub static ID: solana_program::pubkey::Pubkey = solana_program::system_program::ID;
}
