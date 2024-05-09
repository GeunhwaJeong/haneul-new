// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use std::sync::Arc;
use haneul_bridge::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use haneul_bridge::eth_transaction_builder::build_eth_transaction;
use haneul_bridge::haneul_client::HaneulClient;
use haneul_bridge::haneul_transaction_builder::build_haneul_transaction;
use haneul_bridge::tools::{
    make_action, select_contract_address, Args, BridgeCliConfig, BridgeValidatorCommand,
};
use haneul_bridge::utils::{
    generate_bridge_authority_key_and_write_to_file, generate_bridge_client_key_and_write_to_file,
    generate_bridge_node_config_and_write_to_file,
};
use haneul_config::Config;
use haneul_sdk::HaneulClient as HaneulSdkClient;
use haneul_types::bridge::BridgeChainId;
use haneul_types::crypto::Signature;
use haneul_types::transaction::Transaction;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init logging
    let (_guard, _filter_handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();
    let args = Args::parse();

    match args.command {
        BridgeValidatorCommand::CreateBridgeValidatorKey { path } => {
            generate_bridge_authority_key_and_write_to_file(&path)?;
            println!("Bridge validator key generated at {}", path.display());
        }
        BridgeValidatorCommand::CreateBridgeClientKey { path, use_ecdsa } => {
            generate_bridge_client_key_and_write_to_file(&path, use_ecdsa)?;
            println!("Bridge client key generated at {}", path.display());
        }
        BridgeValidatorCommand::CreateBridgeNodeConfigTemplate { path, run_client } => {
            generate_bridge_node_config_and_write_to_file(&path, run_client)?;
            println!(
                "Bridge node config template generated at {}",
                path.display()
            );
        }

        BridgeValidatorCommand::GovernanceClient {
            config_path,
            chain_id,
            cmd,
        } => {
            let chain_id = BridgeChainId::try_from(chain_id).expect("Invalid chain id");
            println!("Chain ID: {:?}", chain_id);
            let config = BridgeCliConfig::load(config_path).expect("Couldn't load BridgeCliConfig");
            let haneul_client = HaneulClient::<HaneulSdkClient>::new(&config.haneul_rpc_url).await?;

            let (haneul_key, haneul_address, gas_object_ref) = config
                .get_haneul_account_info()
                .await
                .expect("Failed to get haneul account info");
            let bridge_summary = haneul_client
                .get_bridge_summary()
                .await
                .expect("Failed to get bridge summary");
            let bridge_committee = Arc::new(
                haneul_client
                    .get_bridge_committee()
                    .await
                    .expect("Failed to get bridge committee"),
            );
            let agg = BridgeAuthorityAggregator::new(bridge_committee);

            // Handle Haneul Side
            if chain_id.is_haneul_chain() {
                let haneul_chain_id = BridgeChainId::try_from(bridge_summary.chain_id).unwrap();
                assert_eq!(
                    haneul_chain_id, chain_id,
                    "Chain ID mismatch, expected: {:?}, got from url: {:?}",
                    chain_id, haneul_chain_id
                );
                // Create BridgeAction
                let haneul_action = make_action(haneul_chain_id, &cmd);
                println!("Action to execute on Haneul: {:?}", haneul_action);
                let threshold = haneul_action.approval_threshold();
                let certified_action = agg
                    .request_committee_signatures(haneul_action, threshold)
                    .await
                    .expect("Failed to request committee signatures");
                let bridge_arg = haneul_client
                    .get_mutable_bridge_object_arg_must_succeed()
                    .await;
                let id_token_map = haneul_client.get_token_id_map().await.unwrap();
                let tx = build_haneul_transaction(
                    haneul_address,
                    &gas_object_ref,
                    certified_action,
                    bridge_arg,
                    &id_token_map,
                )
                .expect("Failed to build haneul transaction");
                let haneul_sig = Signature::new_secure(
                    &IntentMessage::new(Intent::haneul_transaction(), tx.clone()),
                    &haneul_key,
                );
                let tx = Transaction::from_data(tx, vec![haneul_sig]);
                let resp = haneul_client
                    .execute_transaction_block_with_effects(tx)
                    .await
                    .expect("Failed to execute transaction block with effects");
                if resp.status_ok().unwrap() {
                    println!("Haneul Transaction succeeded: {:?}", resp.digest);
                } else {
                    println!(
                        "Haneul Transaction failed: {:?}. Effects: {:?}",
                        resp.digest, resp.effects
                    );
                }
                return Ok(());
            }

            // Handle eth side
            // TODO assert chain id returned from rpc matches chain_id
            let eth_signer_client = config
                .get_eth_signer_client()
                .await
                .expect("Failed to get eth signer client");
            println!("Using Eth address: {:?}", eth_signer_client.address());
            // Create BridgeAction
            let eth_action = make_action(chain_id, &cmd);
            println!("Action to execute on Eth: {:?}", eth_action);
            // Create Eth Signer Client
            let threshold = eth_action.approval_threshold();
            let certified_action = agg
                .request_committee_signatures(eth_action, threshold)
                .await
                .expect("Failed to request committee signatures");
            let contract_address = select_contract_address(&config, &cmd);
            let tx = build_eth_transaction(contract_address, eth_signer_client, certified_action)
                .await
                .expect("Failed to build eth transaction");
            println!("sending Eth tx: {:?}", tx);
            match tx.send().await {
                Ok(tx_hash) => {
                    println!("Transaction sent with hash: {:?}", tx_hash);
                }
                Err(err) => {
                    let revert = err.as_revert();
                    println!("Transaction reverted: {:?}", revert);
                }
            };

            return Ok(());
        }
    }

    Ok(())
}
