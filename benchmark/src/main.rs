//! Vipers compute unit benchmarks.

use anchor_client::solana_client::client_error::ClientErrorKind;
use anchor_client::solana_client::rpc_config::RpcTransactionConfig;
use anchor_client::solana_client::rpc_request::{RpcError, RpcResponseErrorData};
use anchor_client::solana_sdk::signature::{Signature, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{solana_client, ClientError};
use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair,
    },
    Client, Cluster,
};
use anyhow::Result;

trait RpcClientWrapper {
    fn confirm_and_print_logs(&self, sig: &Signature) -> Result<()>;
}

impl RpcClientWrapper for RpcClient {
    fn confirm_and_print_logs(&self, sig: &Signature) -> Result<()> {
        let recent_hash = self.get_latest_blockhash()?;
        self.confirm_transaction_with_spinner(sig, &recent_hash, CommitmentConfig::confirmed())?;
        let tx_result = self.get_transaction_with_config(
            sig,
            RpcTransactionConfig {
                encoding: None,
                commitment: Some(CommitmentConfig::confirmed()),
            },
        )?;
        for line in tx_result
            .transaction
            .meta
            .and_then(|meta| meta.log_messages)
            .unwrap_or_default()
        {
            println!("{}", line);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let payer = Keypair::new();
    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );

    let commitment = CommitmentConfig::confirmed();
    let native_client = RpcClient::new_with_commitment(url.url().to_string(), commitment);
    let airdrop_sig = native_client.request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)?;
    let recent_hash = native_client.get_latest_blockhash()?;
    native_client.confirm_transaction_with_spinner(&airdrop_sig, &recent_hash, commitment)?;

    let client = Client::new_with_options(url, payer, commitment);
    let program = client.program(compute_units::ID);

    let dummy_kp = Keypair::new();
    let sig = program
        .request()
        .instruction(system_instruction::create_account(
            &program.payer(),
            &dummy_kp.pubkey(),
            program.rpc().get_minimum_balance_for_rent_exemption(500)?,
            500,
            &program.id(),
        ))
        .signer(&dummy_kp)
        .accounts(compute_units::accounts::BenchAssertKeysEq {
            dummy_a: dummy_kp.pubkey(),
        })
        .args(compute_units::instruction::BenchAssertKeysEq {
            expected_dummy_a: dummy_kp.pubkey(),
        })
        .send()
        .map_err(|err| {
            if let ClientError::SolanaClientError(solana_client::client_error::ClientError {
                request: _,
                kind:
                    ClientErrorKind::RpcError(RpcError::RpcResponseError {
                        code: _,
                        message: _,
                        data: RpcResponseErrorData::SendTransactionPreflightFailure(sim_result),
                    }),
            }) = &err
            {
                println!("Transaction error! Logs below:");
                for line in sim_result.clone().logs.unwrap_or_default() {
                    println!("{}", line);
                }
            } else {
                println!("WE ERRORED {:?}", &err);
            }
            err
        })?;

    native_client.confirm_and_print_logs(&sig)?;

    Ok(())
}
