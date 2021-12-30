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
    ///
    /// Savings of 53 compute units.
    ///
    /// ```
    /// Program log: Instruction: TestComputeUnits
    /// Program log: No borrow:
    /// Program consumption: 198528 units remaining
    /// Program consumption: 198294 units remaining
    /// Program log: With borrow:
    /// Program consumption: 198278 units remaining
    /// Program consumption: 198097 units remaining
    /// ```
    pub fn bench_assert_keys_eq(
        ctx: Context<BenchAssertKeysEq>,
        expected_dummy_a: Pubkey,
    ) -> ProgramResult {
        msg!("=== Compare against Pubkey ===");
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

        // This also saves about 52 CU's for 3 checks.
        msg!("=== Compare two accounts ===");
        let dummy_b = ctx.accounts.dummy_a.clone();
        msg!("No borrow:");
        sol_log_compute_units();
        assert_keys_eq_no_borrow!(ctx.accounts.dummy_a, dummy_b);
        assert_keys_eq_no_borrow!(ctx.accounts.dummy_a, dummy_b);
        assert_keys_eq_no_borrow!(ctx.accounts.dummy_a, dummy_b);
        sol_log_compute_units();

        msg!("With borrow:");
        sol_log_compute_units();
        assert_keys_eq!(ctx.accounts.dummy_a, dummy_b);
        assert_keys_eq!(ctx.accounts.dummy_a, dummy_b);
        assert_keys_eq!(ctx.accounts.dummy_a, dummy_b);
        sol_log_compute_units();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BenchAssertKeysEq<'info> {
    #[account(zero)]
    pub dummy_a: Account<'info, Dummy>,
}

#[account]
pub struct Dummy {}
