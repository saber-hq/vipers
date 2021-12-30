#[macro_export]
macro_rules! assert_keys_eq_no_borrow {
    ($account_a: expr, $account_b: expr $(,)?) => {
        $crate::assert_keys_eq_no_borrow!($account_a, $account_b, vipers::VipersError::KeyMismatch);
    };
    ($account_a: expr, $account_b: expr, $err_code: ident $(,)?) => {
        $crate::assert_keys_eq_no_borrow!($account_a, $account_b, crate::ErrorCode::$err_code);
    };
    ($account_a: expr, $account_b: expr, $msg: literal $(,)?) => {
        $crate::assert_keys_eq_no_borrow!(
            $account_a,
            $account_b,
            $crate::VipersError::KeyMismatch,
            &*format!("Key mismatch: {}", $msg),
        );
    };
    ($account_a: expr, $account_b: expr, $err: expr $(,)?) => {
        $crate::assert_keys_eq_no_borrow!($account_a, $account_b, $err, vipers::format_err!($err));
    };
    ($account_a: expr, $account_b: expr, $err: expr, $msg: expr $(,)?) => {{
        let __account_a = $account_a.key();
        let __account_b = $account_b.key();
        if __account_a != __account_b {
            msg!($msg);
            msg!(stringify!($account_a != $account_b));
            msg!("Left: {}", __account_a);
            msg!("Right: {}", __account_b);
            vipers::throw_err!($err);
        }
    }};
}
