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

/// Asserts that two accounts share the same key.
#[macro_export]
macro_rules! assert_program {
    ($account_a: expr, $program_id: tt $(,)?) => {
        let __account_a = anchor_lang::Key::key(&$account_a);
        let __program_id: Pubkey = $crate::program_ids::$program_id;
        if __account_a != __program_id {
            msg!(
                "Program ID mismatch: expected {}, found {}",
                __program_id,
                __account_a
            );
            return Err($crate::VipersError::ProgramIDMismatch.into());
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

/// Require or return a [ProgramError], logging the string representation to the program log.
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

    #[test]
    fn test_compiles() -> ProgramResult {
        assert_keys!(token::ID, token::ID, "token program");

        assert_ata!(
            get_associated_token_address(&token::ID, &token::ID),
            token::ID,
            token::ID,
            "ATA"
        );

        assert_program!(token::ID, TOKEN_PROGRAM_ID);

        let weird_math: Option<i32> = (1_i32).checked_add(2);
        let _result = unwrap_int!(weird_math);

        Ok(())
    }
}
