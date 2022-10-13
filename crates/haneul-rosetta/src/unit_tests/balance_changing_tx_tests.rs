// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::identifier::Identifier;
use std::str::FromStr;
use haneul_types::base_types::{
    ObjectDigest, ObjectID, SequenceNumber, HaneulAddress, TransactionDigest,
};
use haneul_types::event::Event::TransferObject;
use haneul_types::event::TransferType;
use haneul_types::gas::GasCostSummary;
use haneul_types::messages::{ExecutionStatus, TransactionData, TransactionEffects};
use haneul_types::object::Owner;
use haneul_types::HANEUL_FRAMEWORK_OBJECT_ID;

use crate::operations::Operation;
use crate::state::extract_balance_changes_from_ops;
use crate::types::SignedValue;

#[test]
fn test_transfer_haneul_null_amount() {
    let sender = HaneulAddress::random_for_testing_only();
    let recipient = HaneulAddress::random_for_testing_only();
    let gas = (
        ObjectID::random(),
        SequenceNumber::new(),
        ObjectDigest::random(),
    );
    let data = TransactionData::new_transfer_haneul(recipient, sender, None, gas, 1000);

    let effect = TransactionEffects {
        status: ExecutionStatus::Success,
        gas_used: GasCostSummary {
            computation_cost: 100,
            storage_cost: 100,
            storage_rebate: 50,
        },
        shared_objects: vec![],
        transaction_digest: TransactionDigest::random(),
        created: vec![],
        mutated: vec![],
        unwrapped: vec![],
        deleted: vec![],
        wrapped: vec![],
        gas_object: (gas, Owner::AddressOwner(sender)),
        events: vec![TransferObject {
            package_id: HANEUL_FRAMEWORK_OBJECT_ID,
            transaction_module: Identifier::from_str("test").unwrap(),
            sender,
            recipient: Owner::AddressOwner(recipient),
            object_id: ObjectID::random(),
            version: Default::default(),
            type_: TransferType::Coin,
            amount: Some(10000),
        }],
        dependencies: vec![],
    };
    let ops = Operation::from_data_and_effect(&data, &effect, &[]).unwrap();
    let balances = extract_balance_changes_from_ops(ops).unwrap();

    assert_eq!(SignedValue::neg(10150), balances[&sender]);
    assert_eq!(SignedValue::from(10000u64), balances[&recipient]);
}
