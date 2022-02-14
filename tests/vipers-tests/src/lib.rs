//! Vipers tests for Anchor 0.20.1.
#![cfg(test)]

#[cfg(feature = "anchor-0_21_0")]
extern crate anchor_lang_0_21_0 as anchor_lang;
#[cfg(feature = "anchor-0_21_0")]
extern crate anchor_spl_0_21_0 as anchor_spl;

#[cfg(feature = "anchor-0_20_1")]
extern crate anchor_lang_0_20_1 as anchor_lang;
#[cfg(feature = "anchor-0_20_1")]
extern crate anchor_spl_0_20_1 as anchor_spl;

#[cfg(feature = "anchor-0_19_0")]
extern crate anchor_lang_0_19_0 as anchor_lang;
#[cfg(feature = "anchor-0_19_0")]
extern crate anchor_spl_0_19_0 as anchor_spl;

#[cfg(feature = "anchor-0_18_2")]
extern crate anchor_lang_0_18_2 as anchor_lang;
#[cfg(feature = "anchor-0_18_2")]
extern crate anchor_spl_0_18_2 as anchor_spl;

use anchor_lang::prelude::*;

declare_id!("VipersTest111111111111111111111111111111111");

use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{self},
};
use vipers::*;

#[error]
pub enum ErrorCode {
    MyError,
}

#[account]
#[derive(Default)]
struct TestData {
    pub byte: u8,
}

#[test]
#[allow(deprecated)]
pub fn test_compiles_deprecated() -> ProgramResult {
    assert_keys!(token::ID, token::ID, "token program");

    Ok(())
}

#[test]
#[allow(deprecated)]
pub fn test_compiles() -> ProgramResult {
    let ata = get_associated_token_address(&token::ID, &token::ID);
    assert_ata!(ata, token::ID, token::ID, "ATA");

    let weird_math: Option<i32> = (1_i32).checked_add(2);
    let _result = unwrap_int!(weird_math);
    unwrap_opt!(weird_math, "aaa");

    Ok(())
}

#[test]
#[allow(deprecated)]
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

#[test]
fn test_unwrap_checked() -> ProgramResult {
    assert_throws!(
        {
            unwrap_checked!({
                let one: u64 = 1;
                let four = one.checked_add(u64::MAX)?;
                four.checked_add(3)
            });
        },
        VipersError::IntegerOverflow
    );
    Ok(())
}

#[test]
fn test_unwrap_opt_block() {
    assert_throws!(
        {
            unwrap_opt_block!(
                {
                    let one: u64 = 1;
                    one.checked_add(u64::MAX)
                },
                ErrorCode::MyError
            );
        },
        ErrorCode::MyError
    );
}

#[test]
#[allow(clippy::eq_op)]
fn test_invariant() {
    assert_does_not_throw!({
        invariant!(1 == 1, ErrorCode::MyError);
    });
    assert_throws!(
        {
            invariant!(1 == 2);
        },
        VipersError::InvariantFailed
    );
    assert_throws!(
        {
            invariant!(1 == 2, "this is stupid");
        },
        VipersError::InvariantFailed
    );
    assert_throws!(
        {
            invariant!(1 == 2, ErrorCode::MyError);
        },
        ErrorCode::MyError
    );
    assert_throws!(
        {
            invariant!(1 == 2, ErrorCode::MyError);
        },
        ErrorCode::MyError
    );
    assert_throws!(
        {
            invariant!(1 == 2, ErrorCode::MyError, "this is wack");
        },
        ErrorCode::MyError
    );
}

#[test]
fn test_assert_keys_eq_pass() {
    assert_does_not_throw!({
        let default = Pubkey::default();
        assert_keys_eq!(
            default,
            anchor_lang::solana_program::system_program::ID,
            ErrorCode::MyError,
            "this is wack"
        );
    });
}

use crate::anchor_lang::solana_program::program_pack::Pack;

#[test]
fn test_assert_keys_eq_boxed() {
    let key = Pubkey::new_unique();
    let lamports = &mut 0;

    let token_account_data = spl_token::state::Account {
        state: spl_token::state::AccountState::Initialized,
        ..Default::default()
    };

    let mut out = [0; 165];
    token_account_data.pack_into_slice(&mut out);
    let account_a = AccountInfo::new(&key, false, false, lamports, &mut out, &token::ID, false, 0);
    let box_a: Box<Account<anchor_spl::token::TokenAccount>> =
        Box::new(Account::try_from_unchecked(&account_a).unwrap());

    assert_does_not_throw!({
        assert_keys_eq!(key, box_a);
    });

    assert_throws!(
        {
            assert_keys_eq!(
                box_a,
                anchor_lang::solana_program::system_program::ID,
                ErrorCode::MyError,
            )
        },
        ErrorCode::MyError
    );
}

#[test]
fn test_assert_keys_eq_no_match() {
    assert_throws!(
        {
            let default = Pubkey::default();
            assert_keys_eq!(
                default,
                anchor_lang::solana_program::sysvar::rent::ID,
                ErrorCode::MyError,
                "this is wack"
            )
        },
        ErrorCode::MyError
    );
}

#[test]
fn test_assert_keys_neq_pass() {
    assert_does_not_throw!({
        let default = Pubkey::default();
        assert_keys_neq!(
            default,
            anchor_lang::solana_program::sysvar::rent::ID,
            ErrorCode::MyError,
            "this is wack"
        );
    });
}

#[test]
fn test_assert_keys_neq_no_match() {
    assert_throws!(
        {
            let default = Pubkey::default();
            assert_keys_neq!(
                default,
                anchor_lang::solana_program::system_program::ID,
                ErrorCode::MyError,
                "this is wack"
            )
        },
        ErrorCode::MyError
    );
}
