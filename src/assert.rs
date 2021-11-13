//! Various assertions.

/// Formats an error as a `&str`.
#[macro_export]
macro_rules! format_err {
    ($err: expr) => {
        &*format!("{:?}: {}", $err, $err)
    };
}

/// Returns the given error as a program error.
///
/// # Example
///
/// ```
/// # use anchor_lang::prelude::*;
/// # impl From<ErrorCode> for ProgramError { fn from(code: ErrorCode) -> Self { ProgramError::Custom(10) } }
/// # pub enum ErrorCode { MyError }
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// let fail = false;
/// if fail {
///     return program_err!(MyError);
/// }
/// Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! program_err {
    ($error:tt $(,)?) => {
        Err(crate::ErrorCode::$error.into())
    };
}

/// Logs where in the code the macro was invoked.
#[macro_export]
macro_rules! log_code_location {
    () => {
        msg!("Error thrown at {}:{}", file!(), line!());
    };
}

/// Throws an error.
///
/// # Example
///
/// ```
/// # use anchor_lang::prelude::*;
/// # impl From<ErrorCode> for ProgramError { fn from(code: ErrorCode) -> Self { ProgramError::Custom(10) } }
/// # pub enum ErrorCode { MyError }
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// let fail = false;
/// if fail {
///     throw_err!(MyError);
/// }
/// Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! throw_err {
    ($error:ident $(,)?) => {
        $crate::throw_err!(crate::ErrorCode::$error);
    };
    ($error:expr $(,)?) => {
        $crate::log_code_location!();
        return Err($error.into());
    };
}

/// Asserts that the ATA is the one of the given owner/mint.
///
/// Warning: this uses a lot of compute units due to the need to generate a PDA.
/// It is recommended to cache this value.
#[macro_export]
macro_rules! assert_ata {
    ($ata: expr, $owner: expr, $mint: expr $(,)?) => {
        assert_ata!($ata, $owner, $mint, "ata mismatch")
    };
    ($ata: expr, $owner: expr, $mint: expr, $msg: expr $(,)?) => {
        let __owner = anchor_lang::Key::key(&$owner);
        let __mint = anchor_lang::Key::key(&$mint);
        let __ata = anchor_lang::Key::key(&$ata);
        let __real_ata = $crate::ata::get_associated_token_address(&__owner, &__mint);
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

/// Asserts that the given [anchor_spl::token::TokenAccount] is an associated token account.
///
/// Warning: this uses a lot of compute units due to the need to generate a PDA.
/// Use this macro sparingly.
#[macro_export]
macro_rules! assert_is_ata {
    ($ata: expr $(,)?) => {
        $crate::assert_ata!($ata, "invalid ata")
    };
    ($ata: expr, $msg: expr $(,)?) => {
        $crate::assert_owner!($ata, token, "ATA not owned by token program");
        let __owner = $ata.owner;
        let __mint = $ata.mint;
        let __ata = anchor_lang::Key::key(&$ata);
        let __real_ata =
            $crate::spl_associated_token_account::get_associated_token_address(&__owner, &__mint);
        if __real_ata != __ata {
            msg!(
                "Invalid ATA: {}: {} (left) != {} (right)",
                $msg,
                __ata,
                __real_ata
            );
            msg!("Owner: {}", __owner);
            msg!("Mint: {}", __mint);
            return Err($crate::VipersError::InvalidATA.into());
        }
    };
}

/// Asserts that an account is owned by the given program.
///
/// As of Anchor 0.15, Anchor handles this for you automatically.
/// You should not need to use this.
#[macro_export]
macro_rules! assert_owner {
    ($program_account: expr, $owner: expr $(,)?) => {
        $crate::assert_owner!($program_account, $owner, "owner mismatch")
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
    ($program_account: expr, $owner: ident $(,)?) => {
        $crate::assert_owner!($program_account, $owner);
    };
    ($program_account: expr, $owner: ident, $msg: expr $(,)?) => {
        let __program_id = $crate::program_ids::$owner::ID;
        $crate::assert_owner!($program_account, $owner, $msg);
    };
}

/// Asserts that two accounts share the same key.
///
/// # Example
///
/// ```should_panic
/// # use anchor_lang::prelude::*;
/// # impl From<ErrorCode> for ProgramError { fn from(code: ErrorCode) -> Self { ProgramError::Custom(10) } }
/// # pub enum ErrorCode { MyError }
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// let one = anchor_lang::solana_program::sysvar::clock::ID;
/// let two = anchor_lang::solana_program::system_program::ID;
/// assert_keys_eq!(one, two); // throws an error
/// Ok(())
/// # }
/// ```
///
/// ```should_panic
/// # use anchor_lang::prelude::*;
/// # impl From<ErrorCode> for ProgramError { fn from(code: ErrorCode) -> Self { ProgramError::Custom(10) } }
/// # pub enum ErrorCode { MyError }
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// let one = anchor_lang::solana_program::sysvar::clock::ID;
/// let two = anchor_lang::solana_program::system_program::ID;
/// assert_keys_eq!(one, two, "invalid"); // throws an error
/// Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! assert_keys_eq {
    ($account_a: expr, $account_b: expr $(,)?) => {
        $crate::assert_keys_eq!($account_a, $account_b, $crate::VipersError::KeyMismatch);
    };
    ($account_a: expr, $account_b: expr, $err_code: ident $(,)?) => {
        $crate::assert_keys_eq!($account_a, $account_b, crate::ErrorCode::$err_code);
    };
    ($account_a: expr, $account_b: expr, $msg: literal $(,)?) => {
        $crate::assert_keys_eq!(
            $account_a,
            $account_b,
            $crate::VipersError::KeyMismatch,
            &*format!("Key mismatch: {}", $msg),
        );
    };
    ($account_a: expr, $account_b: expr, $err: expr $(,)?) => {
        $crate::assert_keys_eq!($account_a, $account_b, $err, $crate::format_err!($err));
    };
    ($account_a: expr, $account_b: expr, $err: expr, $msg: expr $(,)?) => {
        let __account_a = $account_a.key();
        let __account_b = $account_b.key();
        if __account_a != __account_b {
            msg!($msg);
            msg!(stringify!($account_a != $account_b));
            msg!("Left: {}", __account_a);
            msg!("Right: {}", __account_b);
            $crate::throw_err!($err);
        }
    };
}

/// Asserts that two accounts do not share the same key.
///
/// # Example
///
/// ```should_panic
/// # use anchor_lang::prelude::*;
/// # impl From<ErrorCode> for ProgramError { fn from(code: ErrorCode) -> Self { ProgramError::Custom(10) } }
/// # pub enum ErrorCode { MyError }
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// let one = Pubkey::default();
/// let two = Pubkey::default();
/// assert_keys_neq!(one, two); // throws an error
/// Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! assert_keys_neq {
    ($account_a: expr, $account_b: expr $(,)?) => {
        $crate::assert_keys_neq!(
            $account_a,
            $account_b,
            $crate::VipersError::KeysMustNotMatch
        );
    };
    ($account_a: expr, $account_b: expr, $err_code: ident $(,)?) => {
        $crate::assert_keys_neq!($account_a, $account_b, crate::ErrorCode::$err_code);
    };
    ($account_a: expr, $account_b: expr, $msg: literal $(,)?) => {
        $crate::assert_keys_neq!(
            $account_a,
            $account_b,
            $crate::VipersError::KeysMustNotMatch,
            &*format!("Keys must not match: {}", $msg),
        );
    };
    ($account_a: expr, $account_b: expr, $err: expr $(,)?) => {
        $crate::assert_keys_neq!($account_a, $account_b, $err, $crate::format_err!($err));
    };
    ($account_a: expr, $account_b: expr, $err: expr, $msg: expr $(,)?) => {
        let __account_a = $account_a.key();
        let __account_b = $account_b.key();
        if __account_a == __account_b {
            msg!($msg);
            msg!(stringify!($account_a == $account_b));
            msg!("Left: {}", __account_a);
            msg!("Right: {}", __account_b);
            $crate::throw_err!($err);
        }
    };
}

/// Ensures an [Option] can be unwrapped, otherwise returns the error.
///
/// # Example
///
/// ```should_panic
/// # use anchor_lang::prelude::*;
/// # impl From<ErrorCode> for ProgramError { fn from(code: ErrorCode) -> Self { ProgramError::Custom(10) } }
/// # pub enum ErrorCode { MyError }
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// let one = 1_u64;
/// let two = 2_u64;
/// let my_value = unwrap_or_err!(one.checked_sub(2), MyError); // throws an error
/// Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! unwrap_or_err {
    ($option:expr, $error:tt $(,)?) => {
        $option.ok_or_else(|| -> ProgramError { crate::ErrorCode::$error.into() })?
    };
}

/// Unwraps the result of a checked integer operation.
///
/// # Example
///
/// ```should_panic
/// # use anchor_lang::prelude::*;
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// let one = 1_u64;
/// let two = 2_u64;
/// let my_value = unwrap_int!(one.checked_sub(2)); // returns an error
/// Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! unwrap_int {
    ($option:expr $(,)?) => {
        $crate::unwrap_opt!($option, $crate::VipersError::IntegerOverflow)
    };
}

/// Tries to unwrap the [Result], otherwise returns the error
///
/// # Example
///
/// ```should_panic
/// # use anchor_lang::prelude::*;
/// # impl From<ErrorCode> for ProgramError { fn from(code: ErrorCode) -> Self { ProgramError::Custom(10) } }
/// # pub enum ErrorCode { MyError }
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// fn function_returning_result() -> Result<u64, u64> {
///     Err(123)
/// }
///
/// let my_value = try_or_err!(function_returning_result(), MyError);
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! try_or_err {
    ($result:expr, $error:tt $(,)?) => {
        $result.map_err(|_| -> ProgramError { crate::ErrorCode::$error.into() })?
    };
}

/// Asserts that an invariant holds, otherwise logs the given message.
/// This is a drop-in replacement for `require!`.
///
/// # Example
///
/// ```should_panic
/// # use anchor_lang::prelude::*;
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// invariant!(1 == 2, "incorrect");
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! invariant {
    ($invariant: expr, $err_code: ident $(,)?) => {
        $crate::invariant!($invariant, crate::ErrorCode::$err_code);
    };
    ($invariant: expr, $msg: literal $(,)?) => {
        $crate::invariant!($invariant, $crate::VipersError::InvariantFailed, $msg);
    };
    ($invariant:expr, $err:expr $(,)?) => {
        $crate::invariant!($invariant, $err, $crate::format_err!($err));
    };
    ($invariant:expr, $err:expr, $msg: expr $(,)?) => {
        if !($invariant) {
            msg!("Invariant failed: {:?}", $err);
            $crate::throw_err!($crate::VipersError::InvariantFailed);
        }
    };
}

/// Attempts to unwrap an [Option], and if it fails, prints an error.
///
/// # Example
///
/// ```should_panic
/// # use anchor_lang::prelude::*;
/// # #[macro_use] extern crate vipers; fn main() -> ProgramResult {
/// let one = 1_u64;
/// let two = 2_u64;
/// let my_value = unwrap_opt!(one.checked_sub(2), "cannot do this"); // returns an error
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! unwrap_opt {
    ($option: expr, $err_code: ident $(,)?) => {
        $crate::unwrap_opt!($option, crate::ErrorCode::$err_code)
    };
    ($option: expr, $msg: literal $(,)?) => {
        $crate::unwrap_opt!($option, $crate::VipersError::OptionUnwrapFailed, $msg)
    };
    ($option:expr, $err:expr $(,)?) => {
        $crate::unwrap_opt!($option, $err, $crate::format_err!($err))
    };
    ($option:expr, $err:expr, $msg: expr $(,)?) => {
        $option.ok_or_else(|| -> ProgramError {
            msg!("Option unwrap failed: {:?}", $err);
            $crate::log_code_location!();
            $crate::VipersError::OptionUnwrapFailed.into()
        })?
    };
}

/// Asserts that two accounts share the same key.
///
/// Deprecated in favor of [assert_keys_eq].
#[deprecated]
#[macro_export]
macro_rules! assert_keys {
    ($account_a: expr, $account_b: expr $(,)?) => {
        $crate::assert_keys_eq!($account_a, $account_b, "key mismatch")
    };
    ($account_a: expr, $account_b: expr, $msg: expr $(,)?) => {
        $crate::assert_keys_eq!($account_a, $account_b, $msg)
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

    #[allow(deprecated)]
    #[test]
    fn test_compiles_deprecated() -> ProgramResult {
        assert_keys!(token::ID, token::ID, "token program");

        Ok(())
    }

    #[test]
    fn test_compiles() -> ProgramResult {
        assert_ata!(
            get_associated_token_address(&token::ID, &token::ID),
            token::ID,
            token::ID,
            "ATA"
        );

        let weird_math: Option<i32> = (1_i32).checked_add(2);
        let _result = unwrap_int!(weird_math);
        unwrap_opt!(weird_math, "aaa");

        Ok(())
    }

    #[test]
    fn test_assert_owner() -> ProgramResult {
        let mut lamports: u64 = 8 + (TestData::default().try_to_vec().unwrap().len() as u64);

        let mut buffer: [u8; 16] = [0; 16];
        let mut buf: &mut [u8] = &mut buffer;
        TestData::default().try_serialize(&mut buf)?;

        let info: Account<TestData> = Account::try_from(&AccountInfo::new(
            &crate::ID,
            false,
            false,
            &mut lamports,
            &mut buffer,
            &crate::ID,
            false,
            0,
        ))?;
        assert_owner!(info, crate::ID);

        Ok(())
    }
}
