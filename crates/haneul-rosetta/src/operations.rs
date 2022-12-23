// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::ops::Neg;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use haneul_sdk::rpc_types::{HaneulEvent, HaneulExecutionStatus, HaneulTransactionData, HaneulTransactionKind};

use haneul_types::base_types::{ObjectRef, HaneulAddress};
use haneul_types::event::BalanceChangeType;
use haneul_types::gas_coin::GAS;
use haneul_types::messages::TransactionData;
use haneul_types::object::Owner;

use crate::types::{
    AccountIdentifier, Amount, CoinAction, CoinChange, CoinIdentifier, ConstructionMetadata,
    IndexCounter, OperationIdentifier, OperationStatus, OperationType, SignedValue,
};
use crate::Error;

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
    pub fn from_data(data: &HaneulTransactionData) -> Result<Vec<Operation>, anyhow::Error> {
        let sender = data.sender;
        let mut counter = IndexCounter::default();
        let mut ops = data
            .transactions
            .iter()
            .flat_map(|tx| parse_operations(tx, sender, &mut counter, None))
            .flatten()
            .collect::<Vec<_>>();
        let gas = Operation::gas_budget(
            &mut counter,
            None,
            data.gas_payment.to_object_ref(),
            data.gas_budget,
            sender,
        );
        ops.push(gas);
        Ok(ops)
    }

    pub fn from_data_and_events(
        data: &HaneulTransactionData,
        status: &HaneulExecutionStatus,
        events: &[HaneulEvent],
    ) -> Result<Vec<Operation>, anyhow::Error> {
        let sender = data.sender;
        let mut counter = IndexCounter::default();
        let status = Some((status).into());
        let mut ops = data
            .transactions
            .iter()
            .flat_map(|tx| parse_operations(tx, sender, &mut counter, status))
            .flatten()
            .collect::<Vec<_>>();
        let gas = Operation::gas_budget(
            &mut counter,
            status,
            data.gas_payment.to_object_ref(),
            data.gas_budget,
            sender,
        );
        ops.push(gas);

        // We will need to subtract the PayHaneul operation amounts from the actual balance
        // change amount extracted from event to prevent double counting.
        let mut pay_haneul_balance_to_subtract = HashMap::new();

        let pay_haneul_ops = ops
            .iter()
            .filter_map(|op| match (op.type_, &op.account, &op.amount) {
                (OperationType::PayHaneul, Some(acc), Some(amount)) => {
                    let amount = if amount.value.is_negative() {
                        // Safe to downcast, total supply of HANEUL is way less then i128::MAX
                        amount.value.abs() as i128
                    } else {
                        (amount.value.abs() as i128).neg()
                    };
                    Some((acc.address, amount))
                }
                _ => None,
            });

        for (addr, amount) in pay_haneul_ops {
            *pay_haneul_balance_to_subtract.entry(addr).or_default() += amount
        }

        // Extract coin change operations from events
        let coin_change_operations = Operation::get_coin_operation_from_events(
            events,
            status,
            pay_haneul_balance_to_subtract,
            &mut counter,
        );
        ops.extend(coin_change_operations);

        Ok(ops)
    }

    pub fn get_coin_operation_from_events(
        events: &[HaneulEvent],
        status: Option<OperationStatus>,
        balance_to_subtract: HashMap<HaneulAddress, i128>,
        counter: &mut IndexCounter,
    ) -> Vec<Operation> {
        // Aggregate balance changes by address, rosetta don't care about coins.
        let mut balance_change = balance_to_subtract;
        let mut gas: HashMap<HaneulAddress, i128> = HashMap::new();
        for (type_, address, amount) in events.iter().flat_map(Self::get_balance_change_from_event)
        {
            if type_ == OperationType::HaneulBalanceChange {
                let sum = balance_change.entry(address).or_default();
                *sum += amount;
            } else if type_ == OperationType::GasSpent {
                let sum = gas.entry(address).or_default();
                *sum += amount;
            }
        }

        let mut ops = balance_change
            .into_iter()
            .filter_map(|(addr, amount)| {
                if amount != 0 {
                    Some(Operation {
                        operation_identifier: counter.next_idx().into(),
                        related_operations: vec![],
                        type_: OperationType::HaneulBalanceChange,
                        status,
                        account: Some(addr.into()),
                        amount: Some(Amount::new(amount.into())),
                        coin_change: None,
                        metadata: None,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        ops.extend(
            gas.into_iter()
                .map(|(addr, amount)| Operation {
                    operation_identifier: counter.next_idx().into(),
                    related_operations: vec![],
                    type_: OperationType::GasSpent,
                    status: Some(OperationStatus::Success),
                    account: Some(addr.into()),
                    amount: Some(Amount::new(amount.into())),
                    coin_change: None,
                    metadata: None,
                })
                .collect::<Vec<_>>(),
        );
        ops
    }

    fn get_balance_change_from_event(
        event: &HaneulEvent,
    ) -> Option<(OperationType, HaneulAddress, i128)> {
        if let HaneulEvent::CoinBalanceChange {
            owner: Owner::AddressOwner(owner),
            coin_type,
            amount,
            change_type,
            ..
        } = event
        {
            // We only interested in HANEUL coins and account addresses
            if coin_type == &GAS::type_().to_string() {
                let type_ = if change_type == &BalanceChangeType::Gas {
                    // We always charge gas
                    OperationType::GasSpent
                } else {
                    OperationType::HaneulBalanceChange
                };
                return Some((type_, *owner, *amount));
            }
        }
        None
    }

    /// Parse operation input from rosetta to Haneul transaction
    pub async fn create_data(
        operations: Vec<Operation>,
        metadata: ConstructionMetadata,
    ) -> Result<TransactionData, Error> {
        let mut type_ = None;
        let mut recipients = vec![];
        let mut amounts = vec![];
        let mut sender = None;
        let mut budget = None;
        for op in operations {
            // Currently only PayHaneul is support,
            if op.type_ != OperationType::PayHaneul && op.type_ != OperationType::GasBudget {
                return Err(Error::UnsupportedOperation(op.type_));
            }
            if type_.is_none() && op.type_ != OperationType::GasBudget {
                type_ = Some(op.type_)
            }
            if op.type_ == OperationType::GasBudget {
                let budget_value = op
                    .metadata
                    .clone()
                    .and_then(|v| v.pointer("/budget").cloned())
                    .ok_or_else(|| Error::MissingInput("gas budget".to_string()))?;
                budget = Some(
                    budget_value
                        .as_u64()
                        .or_else(|| budget_value.as_str().and_then(|s| u64::from_str(s).ok()))
                        .ok_or_else(|| Error::InvalidInput(format!("{budget_value}")))?,
                );
            } else if op.type_ == OperationType::PayHaneul {
                if let (Some(amount), Some(account)) = (op.amount, op.account) {
                    if amount.value.is_negative() {
                        sender = Some(account.address)
                    } else {
                        recipients.push(account.address);
                        let amount = amount.value.abs();
                        if amount > u64::MAX as u128 {
                            return Err(Error::InvalidInput(
                                "Input amount exceed u64::MAX".to_string(),
                            ));
                        }
                        amounts.push(amount as u64)
                    }
                }
            }
        }

        let address = sender.ok_or_else(|| Error::MissingInput("Sender address".to_string()))?;
        let gas = metadata.sender_coins[0];
        let budget = budget.ok_or_else(|| Error::MissingInput("gas budget".to_string()))?;

        Ok(TransactionData::new_pay_haneul(
            address,
            metadata.sender_coins,
            recipients,
            amounts,
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
    tx: &HaneulTransactionKind,
    sender: HaneulAddress,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
) -> Result<Vec<Operation>, anyhow::Error> {
    let operations = if let HaneulTransactionKind::PayHaneul(tx) = tx {
        let recipients = tx.recipients.iter().zip(&tx.amounts);
        let mut aggregated_recipients: HashMap<HaneulAddress, u64> = HashMap::new();

        for (recipient, amount) in recipients {
            *aggregated_recipients.entry(*recipient).or_default() += *amount
        }

        let mut pay_operations = aggregated_recipients
            .into_iter()
            .map(|(recipient, amount)| Operation {
                operation_identifier: counter.next_idx().into(),
                related_operations: vec![],
                type_: OperationType::PayHaneul,
                status,
                account: Some(recipient.into()),
                amount: Some(Amount::new(amount.into())),
                coin_change: None,
                metadata: None,
            })
            .collect::<Vec<_>>();
        let total_paid = tx.amounts.iter().sum::<u64>();
        pay_operations.push(Operation {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::PayHaneul,
            status,
            account: Some(sender.into()),
            amount: Some(Amount::new(SignedValue::neg(total_paid as u128))),
            coin_change: None,
            metadata: None,
        });
        pay_operations
    } else {
        let (type_, metadata) = match tx {
            HaneulTransactionKind::TransferObject(tx) => (OperationType::TransferObject, json!(tx)),
            HaneulTransactionKind::Publish(tx) => (OperationType::Publish, json!(tx.disassembled)),
            HaneulTransactionKind::Call(tx) => (OperationType::MoveCall, json!(tx)),
            HaneulTransactionKind::TransferHaneul(tx) => (OperationType::TransferHANEUL, json!(tx)),
            HaneulTransactionKind::Pay(tx) => (OperationType::Pay, json!(tx)),
            HaneulTransactionKind::PayAllHaneul(tx) => (OperationType::PayAllHaneul, json!(tx)),
            HaneulTransactionKind::ChangeEpoch(tx) => (OperationType::EpochChange, json!(tx)),
            HaneulTransactionKind::Genesis(tx) => (OperationType::Genesis, json!(tx)),
            HaneulTransactionKind::PayHaneul(_) => unreachable!(),
        };
        generic_operation(counter, type_, status, sender, metadata)
    };
    Ok(operations)
}

fn generic_operation(
    counter: &mut IndexCounter,
    type_: OperationType,
    status: Option<OperationStatus>,
    sender: HaneulAddress,
    metadata: Value,
) -> Vec<Operation> {
    vec![Operation {
        operation_identifier: counter.next_idx().into(),
        related_operations: vec![],
        type_,
        status,
        account: Some(AccountIdentifier { address: sender }),
        amount: None,
        coin_change: None,
        metadata: Some(metadata),
    }]
}
