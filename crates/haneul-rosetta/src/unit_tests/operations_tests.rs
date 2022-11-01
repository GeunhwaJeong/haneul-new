// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_types::base_types::{ObjectDigest, ObjectID, SequenceNumber, HaneulAddress};
use haneul_types::messages::TransactionData;

use crate::operations::Operation;
use crate::types::ConstructionMetadata;

#[tokio::test]
async fn test_operation_data_parsing() -> Result<(), anyhow::Error> {
    let gas = (
        ObjectID::random(),
        SequenceNumber::new(),
        ObjectDigest::random(),
    );

    let sender = HaneulAddress::random_for_testing_only();

    let data = TransactionData::new_pay_haneul(
        sender,
        vec![gas],
        vec![HaneulAddress::random_for_testing_only()],
        vec![10000],
        gas,
        1000,
    );

    let ops = Operation::from_data(&data)?;
    let metadata = ConstructionMetadata {
        sender_coins: vec![gas],
    };

    let parsed_data = Operation::create_data(ops, metadata).await.unwrap();
    assert_eq!(data, parsed_data);

    Ok(())
}
