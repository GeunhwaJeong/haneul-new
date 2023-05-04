// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::language_storage::TypeTag;
use haneul_core::test_utils::dummy_transaction_effects;
use haneul_keys::keystore::AccountKeystore;
use haneul_sdk::wallet_context::WalletContext;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::ObjectRef;
use haneul_types::base_types::HaneulAddress;
use haneul_types::committee::Committee;
use haneul_types::crypto::{
    deterministic_random_account_key, AuthorityKeyPair, AuthorityPublicKeyBytes, AuthoritySignInfo,
    KeypairTraits, Signature, Signer,
};
use haneul_types::effects::SignedTransactionEffects;
use haneul_types::messages::CallArg;
use haneul_types::messages::TEST_ONLY_GAS_UNIT_FOR_GENERIC;
use haneul_types::messages::TEST_ONLY_GAS_UNIT_FOR_TRANSFER;
use haneul_types::messages::{
    CertifiedTransaction, ObjectArg, TransactionData, VerifiedCertificate,
    VerifiedSignedTransaction, VerifiedTransaction,
};
use haneul_types::object::{generate_test_gas_objects, Object};
use haneul_types::haneul_system_state::HANEUL_SYSTEM_MODULE_NAME;
use haneul_types::utils::to_sender_signed_transaction;
use haneul_types::{
    HANEUL_FRAMEWORK_OBJECT_ID, HANEUL_SYSTEM_OBJECT_ID, HANEUL_SYSTEM_STATE_OBJECT_ID,
    HANEUL_SYSTEM_STATE_OBJECT_SHARED_VERSION,
};

pub async fn make_staking_transaction_with_wallet_context(
    context: &mut WalletContext,
    validator_address: HaneulAddress,
) -> VerifiedTransaction {
    let accounts_and_objs = context.get_all_accounts_and_gas_objects().await.unwrap();
    let sender = accounts_and_objs[0].0;
    let gas_object = accounts_and_objs[0].1[0];
    let stake_object = accounts_and_objs[0].1[1];
    let gas_price = context.get_reference_gas_price().await.unwrap();

    make_staking_transaction(
        gas_object,
        stake_object,
        validator_address,
        sender,
        context.config.keystore.get_key(&sender).unwrap(),
        gas_price,
    )
}

/// Make a few different test transaction containing the same shared object.
pub fn test_shared_object_transactions(
    shared_object: Option<Object>,
    gas_objects: Option<Vec<Object>>,
    gas_price: u64,
) -> Vec<VerifiedTransaction> {
    // The key pair of the sender of the transaction.
    let (sender, keypair) = deterministic_random_account_key();
    // Make one transaction per gas object (all containing the same shared object).
    let mut transactions = Vec::new();
    let shared_object = shared_object.unwrap_or_else(Object::shared_for_testing);
    let gas_objects = gas_objects.unwrap_or_else(generate_test_gas_objects);
    let shared_object_id = shared_object.id();
    let initial_shared_version = shared_object.version();
    let module = "object_basics";
    let function = "create";

    for gas_object in gas_objects {
        let data = TransactionData::new_move_call(
            sender,
            HANEUL_FRAMEWORK_OBJECT_ID,
            ident_str!(module).to_owned(),
            ident_str!(function).to_owned(),
            /* type_args */ vec![],
            gas_object.compute_object_reference(),
            /* args */
            vec![
                CallArg::Object(ObjectArg::SharedObject {
                    id: shared_object_id,
                    initial_shared_version,
                    mutable: true,
                }),
                CallArg::Pure(16u64.to_le_bytes().to_vec()),
                CallArg::Pure(bcs::to_bytes(&AccountAddress::from(sender)).unwrap()),
            ],
            gas_price * TEST_ONLY_GAS_UNIT_FOR_GENERIC,
            gas_price,
        )
        .unwrap();
        transactions.push(to_sender_signed_transaction(data, &keypair));
    }
    transactions
}

pub fn make_transfer_object_transaction_with_wallet_context(
    object_ref: ObjectRef,
    gas_object: ObjectRef,
    context: &WalletContext,
    sender: HaneulAddress,
    recipient: HaneulAddress,
    gas_price: u64,
) -> VerifiedTransaction {
    let data = TransactionData::new_transfer(
        recipient,
        object_ref,
        sender,
        gas_object,
        TEST_ONLY_GAS_UNIT_FOR_TRANSFER * gas_price,
        gas_price,
    );
    to_sender_signed_transaction(data, context.config.keystore.get_key(&sender).unwrap())
}

pub fn make_staking_transaction(
    gas_object: ObjectRef,
    coin: ObjectRef,
    validator: HaneulAddress,
    sender: HaneulAddress,
    keypair: &dyn Signer<Signature>,
    gas_price: u64,
) -> VerifiedTransaction {
    let data = TransactionData::new_move_call(
        sender,
        HANEUL_SYSTEM_OBJECT_ID,
        HANEUL_SYSTEM_MODULE_NAME.to_owned(),
        "request_add_stake".parse().unwrap(),
        vec![],
        gas_object,
        vec![
            CallArg::Object(ObjectArg::SharedObject {
                id: HANEUL_SYSTEM_STATE_OBJECT_ID,
                initial_shared_version: HANEUL_SYSTEM_STATE_OBJECT_SHARED_VERSION,
                mutable: true,
            }),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(coin)),
            CallArg::Pure(bcs::to_bytes(&validator).unwrap()),
        ],
        gas_price * TEST_ONLY_GAS_UNIT_FOR_GENERIC,
        gas_price,
    )
    .unwrap();
    to_sender_signed_transaction(data, keypair)
}

/// Make a transaction calling a specific move module & function.
pub fn move_transaction(
    gas_object: Object,
    module: &'static str,
    function: &'static str,
    package_id: ObjectID,
    arguments: Vec<CallArg>,
    gas_budget: u64,
    gas_price: u64,
) -> VerifiedTransaction {
    move_transaction_with_type_tags(
        gas_object,
        module,
        function,
        package_id,
        &[],
        arguments,
        gas_budget,
        gas_price,
    )
}

/// Make a transaction calling a specific move module & function, with specific type tags
pub fn move_transaction_with_type_tags(
    gas_object: Object,
    module: &'static str,
    function: &'static str,
    package_id: ObjectID,
    type_args: &[TypeTag],
    arguments: Vec<CallArg>,
    gas_budget: u64,
    gas_price: u64,
) -> VerifiedTransaction {
    let (sender, keypair) = deterministic_random_account_key();

    // Make the transaction.
    let data = TransactionData::new_move_call(
        sender,
        package_id,
        ident_str!(module).to_owned(),
        ident_str!(function).to_owned(),
        type_args.to_vec(),
        gas_object.compute_object_reference(),
        arguments,
        gas_budget,
        gas_price,
    )
    .unwrap();
    to_sender_signed_transaction(data, &keypair)
}

/// Make a test certificates for each input transaction.
pub fn make_tx_certs_and_signed_effects(
    transactions: Vec<VerifiedTransaction>,
) -> (Vec<VerifiedCertificate>, Vec<SignedTransactionEffects>) {
    let (committee, key_pairs) = Committee::new_simple_test_committee();
    make_tx_certs_and_signed_effects_with_committee(transactions, &committee, &key_pairs)
}

/// Make a test certificates for each input transaction.
pub fn make_tx_certs_and_signed_effects_with_committee(
    transactions: Vec<VerifiedTransaction>,
    committee: &Committee,
    key_pairs: &[AuthorityKeyPair],
) -> (Vec<VerifiedCertificate>, Vec<SignedTransactionEffects>) {
    let mut tx_certs = Vec::new();
    let mut effect_sigs = Vec::new();
    for tx in transactions {
        let mut sigs: Vec<AuthoritySignInfo> = Vec::new();
        for key in key_pairs {
            let vote = VerifiedSignedTransaction::new(
                committee.epoch,
                tx.clone(),
                key.public().into(),
                key,
            );
            sigs.push(vote.auth_sig().clone());
            if let Ok(tx_cert) =
                CertifiedTransaction::new(vote.into_inner().into_data(), sigs.clone(), committee)
            {
                tx_certs.push(tx_cert.verify(committee).unwrap());
                let effects = dummy_transaction_effects(&tx);
                let signed_effects = SignedTransactionEffects::new(
                    committee.epoch(),
                    effects,
                    key,
                    AuthorityPublicKeyBytes::from(key.public()),
                );
                effect_sigs.push(signed_effects);
                break;
            };
        }
    }
    (tx_certs, effect_sigs)
}
