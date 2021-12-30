//! Specifies a trait for borrowing keys of accounts.

use anchor_lang::{prelude::*, solana_program, ZeroCopy};

/// Trait which allows borrowing the key of accounts.
pub trait BorrowKey {
    /// Borrows the [Pubkey] of the value.
    fn borrow_key(&self) -> &Pubkey;
}

impl<'info> BorrowKey for Pubkey {
    fn borrow_key(&self) -> &Pubkey {
        self
    }
}

impl<'info> BorrowKey for UncheckedAccount<'info> {
    fn borrow_key(&self) -> &Pubkey {
        self.key
    }
}

impl<'info> BorrowKey for AccountInfo<'info> {
    fn borrow_key(&self) -> &Pubkey {
        self.key
    }
}

macro_rules! impl_borrow_key {
    ($name: ident, $($tt:tt)*) => {
        #[allow(deprecated)]
        impl<'info, T: $($tt)*> BorrowKey for $name<'info, T> {
            fn borrow_key(&self) -> &Pubkey {
                self.as_ref().key
            }
        }
    };
    ($name: ident) => {
        impl<'info> BorrowKey for $name<'info> {
            fn borrow_key(&self) -> &Pubkey {
                self.as_ref().key
            }
        }
    };
}

impl_borrow_key!(
    Account,
    AccountSerialize + AccountDeserialize + Owner + Clone
);
impl_borrow_key!(Sysvar, solana_program::sysvar::Sysvar);
impl_borrow_key!(SystemAccount);

impl_borrow_key!(Signer);
impl_borrow_key!(AccountLoader, ZeroCopy + Owner);

impl_borrow_key!(Loader, ZeroCopy);
impl_borrow_key!(CpiAccount, AccountDeserialize + Clone);
impl_borrow_key!(CpiState, AccountSerialize + AccountDeserialize + Clone);
impl_borrow_key!(
    ProgramAccount,
    AccountSerialize + AccountDeserialize + Clone
);
impl_borrow_key!(ProgramState, AccountSerialize + AccountDeserialize + Clone);
