//! Allows getting the reference of the key of an account or [Pubkey].

use anchor_lang::prelude::*;
use anchor_lang::ZeroCopy;

/// Defines the Pubkey of an account, fetching it as a reference.
pub trait AsKeyRef {
    /// Returns the [Pubkey] of the account or key as a reference.
    fn as_key_ref(&self) -> &Pubkey;
}

impl AsKeyRef for Pubkey {
    fn as_key_ref(&self) -> &Pubkey {
        self
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Owner + Clone> AsKeyRef
    for Box<Account<'info, T>>
{
    fn as_key_ref(&self) -> &Pubkey {
        self.as_ref().as_key_ref()
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Owner + Clone> AsKeyRef
    for Account<'info, T>
{
    fn as_key_ref(&self) -> &Pubkey {
        let my_ref: &AccountInfo<'info> = self.as_ref();
        my_ref.key
    }
}

impl<'info> AsKeyRef for AccountInfo<'info> {
    fn as_key_ref(&self) -> &Pubkey {
        self.key
    }
}

impl<'info, T: ZeroCopy + Owner> AsKeyRef for AccountLoader<'info, T> {
    fn as_key_ref(&self) -> &Pubkey {
        self.as_ref().key
    }
}

impl<'info> AsKeyRef for Signer<'info> {
    fn as_key_ref(&self) -> &Pubkey {
        self.as_ref().key
    }
}

impl<'info> AsKeyRef for SystemAccount<'info> {
    fn as_key_ref(&self) -> &Pubkey {
        self.as_ref().key
    }
}

impl<'info, T: anchor_lang::solana_program::sysvar::Sysvar> AsKeyRef for Sysvar<'info, T> {
    fn as_key_ref(&self) -> &Pubkey {
        self.as_ref().key
    }
}

impl<'info> AsKeyRef for UncheckedAccount<'info> {
    fn as_key_ref(&self) -> &Pubkey {
        self.as_ref().key
    }
}
