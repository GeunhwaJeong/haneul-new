// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

use haneul_types::base_types::{ObjectRef, HaneulAddress};
use haneul_types::event::{BalanceChangeType, Event};
use haneul_types::gas_coin::GAS;
use haneul_types::messages::{ExecutionStatus, SingleTransactionKind, TransactionData};
use haneul_types::move_package::disassemble_modules;
use haneul_types::object::Owner;

use crate::types::{
    AccountIdentifier, Amount, CoinAction, CoinChange, CoinIdentifier, ConstructionMetadata,
    IndexCounter, OperationIdentifier, OperationStatus, OperationType,
};
use crate::{Error, ErrorType, HANEUL};

#[cfg(test)]
#[path = "unit_tests/operations_tests.rs"]
mod operations_tests;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Operation {
    pub operation_identifier: OperationIdentifier,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_operations: Vec<OperationIdentifier>,
    #[serde(rename = "type")]
    pub type_: OperationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coin_change: Option<CoinChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl Operation {
    pub fn from_data(data: &TransactionData) -> Result<Vec<Operation>, anyhow::Error> {
        let sender = data.signer();
        let mut counter = IndexCounter::default();
        let mut ops = data
            .kind
            .single_transactions()
            .flat_map(|tx| parse_operations(tx, sender, &mut counter, None, None))
            .flatten()
            .collect::<Vec<_>>();
        let gas = Operation::gas_budget(&mut counter, None, data.gas(), data.gas_budget, sender);
        ops.push(gas);
        Ok(ops)
    }

    pub fn from_data_and_events(
        data: &TransactionData,
        status: &ExecutionStatus,
        events: &Vec<Event>,
    ) -> Result<Vec<Operation>, anyhow::Error> {
        let sender = data.signer();
        let mut counter = IndexCounter::default();
        let status = Some((status).into());
        let mut ops = data
            .kind
            .single_transactions()
            .flat_map(|tx| parse_operations(tx, sender, &mut counter, status, Some(events)))
            .flatten()
            .collect::<Vec<_>>();
        let gas = Operation::gas_budget(&mut counter, status, data.gas(), data.gas_budget, sender);
        ops.push(gas);
        Ok(ops)
    }

    fn get_coin_operation_from_events(
        events: &[Event],
        status: Option<OperationStatus>,
        counter: &mut IndexCounter,
    ) -> Vec<Operation> {
        events
            .iter()
            .flat_map(|event| Self::get_coin_operation_from_event(event, status, counter))
            .collect()
    }

    fn get_coin_operation_from_event(
        event: &Event,
        status: Option<OperationStatus>,
        counter: &mut IndexCounter,
    ) -> Vec<Operation> {
        let mut operations = vec![];
        if let Event::CoinBalanceChange {
            owner: Owner::AddressOwner(owner),
            coin_type,
            amount,
            change_type,
            ..
        } = event
        {
            // We only interested in HANEUL coins and account addresses
            if coin_type == &GAS::type_().to_string() {
                let status = if change_type == &BalanceChangeType::Gas {
                    // We always charge gas
                    Some(OperationStatus::Success)
                } else {
                    status
                };
                operations.push(Operation {
                    operation_identifier: counter.next_idx().into(),
                    related_operations: vec![],
                    type_: OperationType::HaneulBalanceChange,
                    status,
                    account: Some(AccountIdentifier { address: *owner }),
                    amount: Some(Amount {
                        value: (*amount).into(),
                        currency: HANEUL.clone(),
                    }),
                    coin_change: None,
                    metadata: None,
                });
            }
        }
        operations
    }

    /// Parse operation input from rosetta to Haneul transaction
    pub async fn create_data(
        operations: Vec<Operation>,
        metadata: ConstructionMetadata,
    ) -> Result<TransactionData, Error> {
        // Currently only PayHaneul is support,
        // first operation is PayHaneul operation and second operation is the budget operation.
        if operations.len() != 2 || operations[0].type_ != OperationType::PayHaneul {
            return Err(Error::new_with_msg(
                ErrorType::InvalidInput,
                "Malformed operation.",
            ));
        }
        let pay_haneul_op = &operations[0];
        let budget_op = &operations[1];

        let account = pay_haneul_op
            .account
            .as_ref()
            .ok_or_else(|| Error::missing_input("operation.account"))?;
        let address = account.address;
        let pay_haneul = pay_haneul_op
            .metadata
            .clone()
            .ok_or_else(|| Error::missing_input("operation.metadata"))?;
        let pay_haneul: PayHaneulMetadata = serde_json::from_value(pay_haneul)
            .map_err(|e| Error::new_with_cause(ErrorType::MalformedOperationError, e))?;
        let gas = metadata.sender_coins[0];
        let budget_value = budget_op
            .metadata
            .clone()
            .and_then(|v| v.pointer("/budget").cloned())
            .ok_or_else(|| Error::missing_input("gas budget"))?;
        let budget = budget_value
            .as_u64()
            .or_else(|| budget_value.as_str().and_then(|s| u64::from_str(s).ok()))
            .ok_or_else(|| {
                Error::new_with_msg(
                    ErrorType::InvalidInput,
                    format!("Cannot parse gas budget : [{budget_value}]").as_str(),
                )
            })?;

        Ok(TransactionData::new_pay_haneul(
            address,
            metadata.sender_coins,
            pay_haneul.recipients,
            pay_haneul.amounts,
            gas,
            budget,
        ))
    }

    pub fn gas_budget(
        counter: &mut IndexCounter,
        status: Option<OperationStatus>,
        gas: ObjectRef,
        budget: u64,
        sender: HaneulAddress,
    ) -> Self {
        Self {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::GasBudget,
            status,
            account: Some(AccountIdentifier { address: sender }),
            amount: None,
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: gas.into(),
                },
                coin_action: CoinAction::CoinSpent,
            }),
            metadata: Some(json!({ "budget": budget })),
        }
    }
}

fn parse_operations(
    tx: &SingleTransactionKind,
    sender: HaneulAddress,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
    events: Option<&Vec<Event>>,
) -> Result<Vec<Operation>, anyhow::Error> {
    let (type_, metadata) = match tx {
        SingleTransactionKind::TransferObject(tx) => (OperationType::TransferObject, json!(tx)),
        SingleTransactionKind::Publish(tx) => {
            let disassembled = disassemble_modules(tx.modules.iter())?;
            (OperationType::Publish, json!(disassembled))
        }
        SingleTransactionKind::Call(tx) => (OperationType::MoveCall, json!(tx)),
        SingleTransactionKind::TransferHaneul(tx) => (OperationType::TransferHANEUL, json!(tx)),
        SingleTransactionKind::Pay(tx) => (OperationType::Pay, json!(tx)),
        SingleTransactionKind::PayHaneul(tx) => {
            let pay_haneul = PayHaneulMetadata {
                recipients: tx.recipients.clone(),
                amounts: tx.amounts.clone(),
            };
            (OperationType::PayHaneul, json!(pay_haneul))
        }
        SingleTransactionKind::PayAllHaneul(tx) => (OperationType::PayAllHaneul, json!(tx)),
        SingleTransactionKind::ChangeEpoch(tx) => (OperationType::EpochChange, json!(tx)),
    };

    let mut operations = vec![Operation {
        operation_identifier: counter.next_idx().into(),
        related_operations: vec![],
        type_,
        status,
        account: Some(AccountIdentifier { address: sender }),
        amount: None,
        coin_change: None,
        metadata: Some(metadata),
    }];

    // Extract coin change operations from events
    if let Some(events) = events {
        let coin_change_operations =
            Operation::get_coin_operation_from_events(events, status, counter);
        operations.extend(coin_change_operations);
    }
    Ok(operations)
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct PayHaneulMetadata {
    pub recipients: Vec<HaneulAddress>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub amounts: Vec<u64>,
}
