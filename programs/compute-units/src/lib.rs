//! Compute units logs.

mod macros;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log_compute_units;
use vipers::prelude::*;

declare_id!("V1persComputeUn1tsTester1111111111111111111");

#[program]
pub mod compute_units {
    use super::*;

    /// Tests [assert_keys_eq] compute units usage.
    pub fn test_compute_units(
        ctx: Context<TestComputeUnits>,
        expected_dummy_a: Pubkey,
    ) -> ProgramResult {
        msg!("No borrow:");
        sol_log_compute_units();
        assert_keys_eq_no_borrow!(ctx.accounts.dummy_a, expected_dummy_a);
        assert_keys_eq_no_borrow!(ctx.accounts.dummy_a, expected_dummy_a);
        assert_keys_eq_no_borrow!(ctx.accounts.dummy_a, expected_dummy_a);
        sol_log_compute_units();

        msg!("With borrow:");
        sol_log_compute_units();
        assert_keys_eq!(ctx.accounts.dummy_a, expected_dummy_a);
        assert_keys_eq!(ctx.accounts.dummy_a, expected_dummy_a);
        assert_keys_eq!(ctx.accounts.dummy_a, expected_dummy_a);
        sol_log_compute_units();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TestComputeUnits<'info> {
    #[account(zero)]
    pub dummy_a: Account<'info, Dummy>,
}

#[account]
pub struct Dummy {}
