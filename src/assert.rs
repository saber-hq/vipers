//! Various assertions.

/// Asserts that two accounts share the same key.
#[macro_export]
macro_rules! assert_keys {
    ($account_a: expr, $account_b: expr $(,)?) => {
        assert_keys!($account_a, $account_b, "key mismatch")
    };
    ($account_a: expr, $account_b: expr, $msg: expr $(,)?) => {
        let __account_a = anchor_lang::Key::key(&$account_a);
        let __account_b = anchor_lang::Key::key(&$account_b);
        if __account_a != __account_b {
            msg!(
                "Key mismatch: {}: {} (left) != {} (right)",
                $msg,
                __account_a,
                __account_b
            );
            return Err($crate::VipersError::KeyMismatch.into());
        }
    };
}

/// Asserts that the ATA is the one of the given owner/mint.
#[macro_export]
macro_rules! assert_ata {
    ($ata: expr, $owner: expr, $mint: expr $(,)?) => {
        assert_ata!($ata, $owner, $mint, "ata mismatch")
    };
    ($ata: expr, $owner: expr, $mint: expr, $msg: expr $(,)?) => {
        let __owner = anchor_lang::Key::key(&$owner);
        let __mint = anchor_lang::Key::key(&$mint);
        let __ata = anchor_lang::Key::key(&$ata);
        let __real_ata =
            spl_associated_token_account::get_associated_token_address(&__owner, &__mint);
        if __real_ata != __ata {
            msg!(
                "ATA mismatch: {}: {} (left) != {} (right)",
                $msg,
                __ata,
                __real_ata
            );
            msg!("Owner: {}", __owner);
            msg!("Mint: {}", __mint);
            return Err($crate::VipersError::ATAMismatch.into());
        }
    };
}

/// Asserts that the given account matches the program id.
#[macro_export]
macro_rules! assert_program {
    ($account: expr, $program: ident $(,)?) => {
        let __account = anchor_lang::Key::key(&$account);
        let __program_id: Pubkey = $crate::program_ids::$program::ID;
        if __account != __program_id {
            msg!(
                "Incorrect program ID for program '{}': expected {}, found {}",
                stringify!($program),
                __program_id,
                __account
            );
            return Err($crate::VipersError::ProgramIDMismatch.into());
        }
    };
}

/// Asserts that an account is owned by the given program.
#[macro_export]
macro_rules! assert_owner {
    ($program_account: expr, $owner: expr $(,)?) => {
        assert_owner!($program_account, $owner, "owner mismatch")
    };
    ($program_account: expr, $owner: expr, $msg: expr $(,)?) => {
        let __program_account =
            *anchor_lang::ToAccountInfo::to_account_info(&$program_account).owner;
        let __owner = anchor_lang::Key::key(&$owner);
        if __program_account != __owner {
            msg!(
                "Owner mismatch: {}: expected {}, got {}",
                $msg,
                __program_account,
                __owner
            );
            return Err($crate::VipersError::OwnerMismatch.into());
        }
    };
}

/// Ensures an [Option] can be unwrapped, otherwise returns the error
#[macro_export]
macro_rules! unwrap_or_err {
    ($option:expr, $error:tt $(,)?) => {
        $option.ok_or_else(|| -> ProgramError { crate::ErrorCode::$error.into() })?
    };
}

/// Unwraps the result of a checked integer operation.
#[macro_export]
macro_rules! unwrap_int {
    ($option:expr $(,)?) => {
        $option.ok_or_else(|| -> ProgramError { $crate::VipersError::IntegerOverflow.into() })?
    };
}

/// Tries to unwrap the [Result], otherwise returns the error
#[macro_export]
macro_rules! try_or_err {
    ($result:expr, $error:tt $(,)?) => {
        $result.map_err(|_| -> ProgramError { crate::ErrorCode::$error.into() })?
    };
}

/// Returns the given error as a program error.
#[macro_export]
macro_rules! program_err {
    ($error:tt $(,)?) => {
        Err(crate::ErrorCode::$error.into())
    };
}

/// Require or return a [solana_program::program_error::ProgramError], logging the string representation to the program log.
#[macro_export]
macro_rules! prog_require {
    ($invariant:expr, $err:expr $(,)?) => {
        if !($invariant) {
            msg!("Invariant failed: {:?}", $err);
            return Err($err.into());
        }
    };
}

#[cfg(test)]
mod tests {
    use anchor_lang::prelude::*;
    use anchor_spl::token;
    use spl_associated_token_account::get_associated_token_address;

    #[account]
    #[derive(Default)]
    struct TestData {
        pub byte: u8,
    }

    #[test]
    fn test_compiles() -> ProgramResult {
        assert_keys!(token::ID, token::ID, "token program");

        assert_ata!(
            get_associated_token_address(&token::ID, &token::ID),
            token::ID,
            token::ID,
            "ATA"
        );

        assert_program!(token::ID, token);

        let weird_math: Option<i32> = (1_i32).checked_add(2);
        let _result = unwrap_int!(weird_math);

        Ok(())
    }

    #[test]
    fn test_assert_owner() -> ProgramResult {
        let key = Pubkey::new_unique();
        let mut lamports: u64 = 8 + (TestData::default().try_to_vec().unwrap().len() as u64);

        let mut buffer: [u8; 16] = [0; 16];
        let mut buf: &mut [u8] = &mut buffer;
        TestData::default().try_serialize(&mut buf)?;

        let info: CpiAccount<TestData> = CpiAccount::try_from(&AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut buffer,
            &key,
            false,
            0,
        ))?;
        assert_owner!(info, key);

        Ok(())
    }
}
