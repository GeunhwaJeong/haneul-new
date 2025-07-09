// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::workloads::Gas;
use crate::ValidatorProxy;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use haneul_keys::keystore::{AccountKeystore, FileBasedKeystore};
use haneul_test_transaction_builder::TestTransactionBuilder;
use haneul_types::base_types::ObjectRef;
use haneul_types::crypto::{AccountKeyPair, KeypairTraits};
use haneul_types::object::Owner;
use haneul_types::transaction::{Transaction, TransactionData, TEST_ONLY_GAS_UNIT_FOR_TRANSFER};
use haneul_types::utils::to_sender_signed_transaction;
use haneul_types::{base_types::HaneulAddress, crypto::HaneulKeyPair};

// This is the maximum gas we will transfer from primary coin into any gas coin
// for running the benchmark

pub type UpdatedAndNewlyMintedGasCoins = Vec<Gas>;

pub fn get_ed25519_keypair_from_keystore(
    keystore_path: PathBuf,
    requested_address: &HaneulAddress,
) -> Result<AccountKeyPair> {
    let keystore = FileBasedKeystore::new(&keystore_path)?;
    match keystore.get_key(requested_address) {
        Ok(HaneulKeyPair::Ed25519(kp)) => Ok(kp.copy()),
        other => Err(anyhow::anyhow!("Invalid key type: {:?}", other)),
    }
}

pub fn make_pay_tx(
    input_coins: Vec<ObjectRef>,
    sender: HaneulAddress,
    addresses: Vec<HaneulAddress>,
    split_amounts: Vec<u64>,
    gas: ObjectRef,
    keypair: &AccountKeyPair,
    gas_price: u64,
) -> Result<Transaction> {
    let pay = TransactionData::new_pay(
        sender,
        input_coins,
        addresses,
        split_amounts,
        gas,
        TEST_ONLY_GAS_UNIT_FOR_TRANSFER * gas_price,
        gas_price,
    )?;
    Ok(to_sender_signed_transaction(pay, keypair))
}

pub async fn publish_basics_package(
    gas: ObjectRef,
    proxy: Arc<dyn ValidatorProxy + Sync + Send>,
    sender: HaneulAddress,
    keypair: &AccountKeyPair,
    gas_price: u64,
) -> ObjectRef {
    let transaction = TestTransactionBuilder::new(sender, gas, gas_price)
        .publish_examples("basics")
        .build_and_sign(keypair);
    let (_, execution_result) = proxy.execute_transaction_block(transaction).await;
    let effects = execution_result.unwrap();
    effects
        .created()
        .iter()
        .find(|(_, owner)| matches!(owner, Owner::Immutable))
        .map(|(reference, _)| *reference)
        .unwrap()
}
