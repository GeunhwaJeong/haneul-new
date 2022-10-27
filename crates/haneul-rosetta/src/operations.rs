// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

use haneul_types::base_types::{ObjectID, ObjectRef, HaneulAddress};
use haneul_types::coin::{PAY_JOIN_FUNC_NAME, PAY_MODULE_NAME, PAY_SPLIT_VEC_FUNC_NAME};
use haneul_types::event::Event;
use haneul_types::gas_coin::GAS;
use haneul_types::messages::{
    CallArg, ExecutionStatus, MoveCall, ObjectArg, Pay, PayAllHaneul, PayHaneul, SingleTransactionKind,
    TransactionData, TransferObject,
};
use haneul_types::move_package::disassemble_modules;
use haneul_types::object::Owner;
use haneul_types::{parse_haneul_struct_tag, HANEUL_FRAMEWORK_OBJECT_ID};

use crate::types::{
    AccountIdentifier, Amount, CoinAction, CoinChange, CoinIdentifier, ConstructionMetadata,
    IndexCounter, OperationIdentifier, OperationStatus, OperationType,
};
use crate::ErrorType::UnsupportedOperation;
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
        let budget = data.gas_budget;
        let gas = data.gas();
        let sender = data.signer();
        Ok(data
            .kind
            .single_transactions()
            .flat_map(|tx| {
                parse_operations(
                    tx,
                    budget,
                    gas,
                    sender,
                    &mut IndexCounter::default(),
                    None,
                    None,
                )
            })
            .flatten()
            .collect::<Vec<_>>())
    }

    pub fn from_data_and_events(
        data: &TransactionData,
        status: &ExecutionStatus,
        events: &Vec<Event>,
    ) -> Result<Vec<Operation>, anyhow::Error> {
        let budget = data.gas_budget;
        let gas = data.gas();
        let sender = data.signer();
        let mut counter = IndexCounter::default();
        let status = Some((status).into());
        Ok(data
            .kind
            .single_transactions()
            .flat_map(|tx| {
                parse_operations(tx, budget, gas, sender, &mut counter, status, Some(events))
            })
            .flatten()
            .collect::<Vec<_>>())
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
            ..
        } = event
        {
            // We only interested in HANEUL coins and account addresses
            if coin_type == &GAS::type_().to_string() {
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

    pub async fn parse_transaction_data(
        operations: Vec<Operation>,
        metadata: ConstructionMetadata,
    ) -> Result<TransactionData, Error> {
        let action: HaneulAction = operations.try_into()?;
        action.try_into_data(metadata).await
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
    budget: u64,
    gas: ObjectRef,
    sender: HaneulAddress,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
    events: Option<&Vec<Event>>,
) -> Result<Vec<Operation>, anyhow::Error> {
    let mut operations = match tx {
        SingleTransactionKind::TransferHaneul(tx) => transfer_haneul_operations(
            budget,
            gas,
            sender,
            tx.recipient,
            tx.amount,
            counter,
            status,
        ),
        SingleTransactionKind::TransferObject(tx) => transfer_object_operations(
            budget,
            tx.object_ref,
            gas,
            sender,
            tx.recipient,
            counter,
            status,
        ),
        SingleTransactionKind::Call(c) => {
            move_call_operations(sender, gas, budget, c, counter, status)
        }
        SingleTransactionKind::Publish(p) => {
            let disassembled = disassemble_modules(p.modules.iter())?;
            vec![Operation {
                operation_identifier: counter.next_idx().into(),
                related_operations: vec![],
                type_: OperationType::Publish,
                status,
                account: Some(AccountIdentifier { address: sender }),
                amount: None,
                coin_change: None,
                metadata: Some(json!(disassembled)),
            }]
        }
        SingleTransactionKind::ChangeEpoch(change) => vec![Operation {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::EpochChange,
            status,
            account: None,
            amount: None,
            coin_change: None,
            metadata: Some(json!(change)),
        }],
        SingleTransactionKind::Pay(pay) => parse_pay(sender, gas, budget, pay, counter, status),
        SingleTransactionKind::PayHaneul(pay_haneul) => {
            parse_pay_haneul(sender, gas, budget, pay_haneul, counter, status)
        }
        SingleTransactionKind::PayAllHaneul(pay_all_haneul) => {
            parse_pay_all_haneul(sender, gas, budget, pay_all_haneul, counter, status)
        }
    };
    if let Some(events) = events {
        let coin_change_operations =
            Operation::get_coin_operation_from_events(events, status, counter);
        operations.extend(coin_change_operations);
    }
    Ok(operations)
}

fn transfer_haneul_operations(
    budget: u64,
    coin: ObjectRef,
    sender: HaneulAddress,
    recipient: HaneulAddress,
    amount: Option<u64>,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
) -> Vec<Operation> {
    let transfer_haneul = TransferHaneulMetadata { recipient, amount };
    vec![
        Operation {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::TransferHANEUL,
            status,
            account: Some(AccountIdentifier { address: sender }),
            amount: None,
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: coin.into(),
                },
                coin_action: CoinAction::CoinSpent,
            }),
            metadata: Some(json!(transfer_haneul)),
        },
        Operation::gas_budget(counter, status, coin, budget, sender),
    ]
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct TransferHaneulMetadata {
    pub recipient: HaneulAddress,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub amount: Option<u64>,
}

fn transfer_object_operations(
    budget: u64,
    object_ref: ObjectRef,
    gas: ObjectRef,
    sender: HaneulAddress,
    recipient: HaneulAddress,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
) -> Vec<Operation> {
    let transfer_object = TransferObject {
        recipient,
        object_ref,
    };
    vec![
        Operation {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::TransferObject,
            status,
            account: Some(AccountIdentifier { address: sender }),
            amount: None,
            coin_change: None,
            metadata: Some(json!(transfer_object)),
        },
        Operation::gas_budget(counter, status, gas, budget, sender),
    ]
}

fn move_call_operations(
    sender: HaneulAddress,
    gas: ObjectRef,
    budget: u64,
    call: &MoveCall,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
) -> Vec<Operation> {
    vec![
        Operation {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::MoveCall,
            status,
            account: Some(AccountIdentifier { address: sender }),
            amount: None,
            coin_change: None,
            metadata: Some(json!(call)),
        },
        Operation::gas_budget(counter, status, gas, budget, sender),
    ]
}

fn parse_pay(
    sender: HaneulAddress,
    gas: ObjectRef,
    budget: u64,
    pay: &Pay,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
) -> Vec<Operation> {
    vec![
        Operation {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::Pay,
            status,
            account: Some(AccountIdentifier { address: sender }),
            amount: None,
            coin_change: None,
            metadata: Some(json!(pay)),
        },
        Operation::gas_budget(counter, status, gas, budget, sender),
    ]
}

fn parse_pay_haneul(
    sender: HaneulAddress,
    gas: ObjectRef,
    budget: u64,
    pay_haneul: &PayHaneul,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
) -> Vec<Operation> {
    vec![
        Operation {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::PayHaneul,
            status,
            account: Some(AccountIdentifier { address: sender }),
            amount: None,
            coin_change: None,
            metadata: Some(json!(pay_haneul)),
        },
        Operation::gas_budget(counter, status, gas, budget, sender),
    ]
}

fn parse_pay_all_haneul(
    sender: HaneulAddress,
    gas: ObjectRef,
    budget: u64,
    pay_all_haneul: &PayAllHaneul,
    counter: &mut IndexCounter,
    status: Option<OperationStatus>,
) -> Vec<Operation> {
    vec![
        Operation {
            operation_identifier: counter.next_idx().into(),
            related_operations: vec![],
            type_: OperationType::PayAllHaneul,
            status,
            account: Some(AccountIdentifier { address: sender }),
            amount: None,
            coin_change: None,
            metadata: Some(json!(pay_all_haneul)),
        },
        Operation::gas_budget(counter, status, gas, budget, sender),
    ]
}

#[derive(Debug)]
pub enum HaneulAction {
    TransferHaneul {
        budget: u64,
        coin: ObjectID,
        sender: HaneulAddress,
        recipient: HaneulAddress,
        amount: Option<u64>,
    },

    Transfer {
        budget: u64,
        coin: ObjectID,
        gas: ObjectID,
        sender: HaneulAddress,
        recipient: HaneulAddress,
    },

    MergeCoin {
        budget: u64,
        primary_coin: ObjectID,
        coin_to_merge: ObjectID,
        gas: ObjectID,
        sender: HaneulAddress,
    },
    SplitCoin {
        budget: u64,
        coin_to_split: ObjectID,
        split_amounts: Vec<u64>,
        gas: ObjectID,
        sender: HaneulAddress,
    },
}

impl HaneulAction {
    pub async fn try_into_data(
        self,
        metadata: ConstructionMetadata,
    ) -> Result<TransactionData, Error> {
        Ok(match self {
            HaneulAction::TransferHaneul {
                budget,
                coin,
                sender,
                recipient,
                amount,
            } => {
                let gas = metadata.try_get_info(&coin)?;
                TransactionData::new_transfer_haneul(recipient, sender, amount, gas.into(), budget)
            }
            HaneulAction::Transfer {
                budget,
                coin,
                gas,
                sender,
                recipient,
            } => {
                let gas = metadata.try_get_info(&gas)?;
                let coin = metadata.try_get_info(&coin)?;
                TransactionData::new_transfer(recipient, coin.into(), sender, gas.into(), budget)
            }
            HaneulAction::MergeCoin {
                budget,
                primary_coin,
                coin_to_merge,
                gas,
                sender,
            } => {
                let gas = metadata.try_get_info(&gas)?;
                let primary_coin = metadata.try_get_info(&primary_coin)?;
                let coin_to_merge = metadata.try_get_info(&coin_to_merge)?;
                let type_args = parse_haneul_struct_tag(&primary_coin.type_)?.type_params;

                TransactionData::new_move_call(
                    sender,
                    metadata.try_get_info(&HANEUL_FRAMEWORK_OBJECT_ID)?.into(),
                    PAY_MODULE_NAME.to_owned(),
                    PAY_JOIN_FUNC_NAME.to_owned(),
                    type_args,
                    gas.into(),
                    vec![
                        CallArg::Object(ObjectArg::ImmOrOwnedObject(primary_coin.into())),
                        CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_to_merge.into())),
                    ],
                    budget,
                )
            }
            HaneulAction::SplitCoin {
                budget,
                coin_to_split,
                split_amounts,
                gas,
                sender,
            } => {
                let gas = metadata.try_get_info(&gas)?;
                let coin_to_split = metadata.try_get_info(&coin_to_split)?;
                let type_args = parse_haneul_struct_tag(&coin_to_split.type_)?.type_params;
                TransactionData::new_move_call(
                    sender,
                    metadata.try_get_info(&HANEUL_FRAMEWORK_OBJECT_ID)?.into(),
                    PAY_MODULE_NAME.to_owned(),
                    PAY_SPLIT_VEC_FUNC_NAME.to_owned(),
                    type_args,
                    gas.into(),
                    vec![
                        CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_to_split.into())),
                        CallArg::Pure(bcs::to_bytes(&split_amounts)?),
                    ],
                    budget,
                )
            }
        })
    }

    pub fn input_objects(&self) -> Vec<ObjectID> {
        match self {
            HaneulAction::TransferHaneul { coin, .. } => {
                vec![*coin]
            }
            HaneulAction::Transfer { coin, gas, .. } => vec![*coin, *gas],
            HaneulAction::MergeCoin {
                primary_coin,
                coin_to_merge,
                gas,
                ..
            } => vec![HANEUL_FRAMEWORK_OBJECT_ID, *primary_coin, *coin_to_merge, *gas],
            HaneulAction::SplitCoin {
                coin_to_split, gas, ..
            } => vec![HANEUL_FRAMEWORK_OBJECT_ID, *coin_to_split, *gas],
        }
    }

    pub fn signer(&self) -> HaneulAddress {
        *match self {
            HaneulAction::TransferHaneul { sender, .. }
            | HaneulAction::Transfer { sender, .. }
            | HaneulAction::MergeCoin { sender, .. }
            | HaneulAction::SplitCoin { sender, .. } => sender,
        }
    }
}

impl TryInto<HaneulAction> for Vec<Operation> {
    type Error = Error;

    fn try_into(self) -> Result<HaneulAction, Self::Error> {
        let mut builder = HaneulActionBuilder::default();

        for op in self {
            match op.type_ {
                OperationType::TransferHANEUL => {
                    let account = op
                        .account
                        .as_ref()
                        .ok_or_else(|| Error::missing_input("operation.account"))?;
                    let address = account.address;
                    builder.operation_type = Some(op.type_);
                    let transfer_haneul = op
                        .metadata
                        .ok_or_else(|| Error::missing_input("operation.metadata"))?;
                    let transfer_haneul: TransferHaneulMetadata = serde_json::from_value(transfer_haneul)
                        .map_err(|e| {
                            Error::new_with_cause(ErrorType::MalformedOperationError, e)
                        })?;
                    builder.coin = op
                        .coin_change
                        .map(|coin| coin.coin_identifier.identifier.id);
                    builder.sender = Some(address);
                    builder.recipient = Some(transfer_haneul.recipient);
                    builder.send_amount = transfer_haneul.amount;
                }
                OperationType::GasBudget => {
                    if let Some(coin) = op.coin_change.as_ref() {
                        builder.gas = Some(coin.coin_identifier.identifier.id);
                    }
                    let budget_value = op
                        .metadata
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
                    builder.gas_budget = Some(budget);
                }
                OperationType::TransferObject
                | OperationType::HaneulBalanceChange
                | OperationType::Pay
                | OperationType::PayHaneul
                | OperationType::PayAllHaneul
                | OperationType::GasSpent
                | OperationType::Genesis
                | OperationType::MoveCall
                | OperationType::Publish
                | OperationType::EpochChange => return Err(Error::unsupported_operation(op.type_)),
            }
        }
        builder.build()
    }
}

#[derive(Default)]
struct HaneulActionBuilder {
    sender: Option<HaneulAddress>,
    recipient: Option<HaneulAddress>,
    gas: Option<ObjectID>,
    coin: Option<ObjectID>,
    send_amount: Option<u64>,
    gas_budget: Option<u64>,
    operation_type: Option<OperationType>,
}

impl HaneulActionBuilder {
    fn build(self) -> Result<HaneulAction, Error> {
        let type_ = self
            .operation_type
            .ok_or_else(|| Error::missing_input("operation_type"))?;
        match type_ {
            OperationType::TransferHANEUL => {
                let sender = self.sender.ok_or_else(|| Error::missing_input("sender"))?;
                let recipient = self
                    .recipient
                    .ok_or_else(|| Error::missing_input("recipient"))?;
                let gas = self.gas.ok_or_else(|| Error::missing_input("gas"))?;
                let budget = self
                    .gas_budget
                    .ok_or_else(|| Error::missing_input("gas_budget"))?;
                Ok(HaneulAction::TransferHaneul {
                    budget,
                    coin: gas,
                    sender,
                    recipient,
                    amount: self.send_amount,
                })
            }
            _ => Err(Error::new_with_msg(
                UnsupportedOperation,
                format!("Unsupported operation [{type_:?}]").as_str(),
            )),
        }
    }
}
