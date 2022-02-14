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

#[allow(deprecated)]
mod deprecated {
    use super::*;
    use anchor_lang::accounts::{
        cpi_account::CpiAccount, cpi_state::CpiState, loader::Loader,
        program_account::ProgramAccount, state::ProgramState,
    };

    #[allow(deprecated)]
    impl<'info, T: AccountDeserialize + Clone> AsKeyRef for CpiAccount<'info, T> {
        fn as_key_ref(&self) -> &Pubkey {
            self.as_ref().key
        }
    }

    #[allow(deprecated)]
    impl<'info, T: AccountSerialize + AccountDeserialize + Clone> AsKeyRef for CpiState<'info, T> {
        fn as_key_ref(&self) -> &Pubkey {
            self.as_ref().key
        }
    }

    #[allow(deprecated)]
    impl<'info, T: ZeroCopy> AsKeyRef for Loader<'info, T> {
        fn as_key_ref(&self) -> &Pubkey {
            self.as_ref().key
        }
    }

    #[allow(deprecated)]
    impl<'info, T: AccountSerialize + AccountDeserialize + Clone> AsKeyRef
        for ProgramAccount<'info, T>
    {
        fn as_key_ref(&self) -> &Pubkey {
            self.as_ref().key
        }
    }

    #[allow(deprecated)]
    impl<'info, T: AccountSerialize + AccountDeserialize + Clone> AsKeyRef for ProgramState<'info, T> {
        fn as_key_ref(&self) -> &Pubkey {
            self.as_ref().key
        }
    }
}
