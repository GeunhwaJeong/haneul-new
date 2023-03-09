// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use std::collections::HashMap;
use std::vec;
use haneul_json_rpc_types::HaneulArgument;
use haneul_json_rpc_types::HaneulCommand;
use haneul_json_rpc_types::HaneulProgrammableMoveCall;
use haneul_json_rpc_types::HaneulProgrammableTransaction;
use haneul_sdk::json::HaneulJsonValue;

use serde::Deserialize;
use serde::Serialize;
use haneul_sdk::rpc_types::{
    HaneulEvent, HaneulTransactionData, HaneulTransactionDataAPI, HaneulTransactionEffectsAPI,
    HaneulTransactionKind, HaneulTransactionResponse,
};

use haneul_types::base_types::{SequenceNumber, HaneulAddress};
use haneul_types::committee::EpochId;
use haneul_types::event::BalanceChangeType;
use haneul_types::gas_coin::{GasCoin, GAS};
use haneul_types::governance::{
    ADD_DELEGATION_LOCKED_COIN_FUN_NAME, ADD_DELEGATION_MUL_COIN_FUN_NAME,
};
use haneul_types::messages::TransactionData;
use haneul_types::object::Owner;
use haneul_types::haneul_system_state::HANEUL_SYSTEM_MODULE_NAME;
use haneul_types::HANEUL_FRAMEWORK_OBJECT_ID;

use crate::types::{
    AccountIdentifier, Amount, CoinAction, CoinChange, CoinID, CoinIdentifier, InternalOperation,
    OperationIdentifier, OperationStatus, OperationType, PreprocessMetadata,
};
use crate::Error;

#[cfg(test)]
#[path = "unit_tests/operations_tests.rs"]
mod operations_tests;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Operations(Vec<Operation>);

impl FromIterator<Operation> for Operations {
    fn from_iter<T: IntoIterator<Item = Operation>>(iter: T) -> Self {
        Operations::new(iter.into_iter().collect())
    }
}

impl FromIterator<Vec<Operation>> for Operations {
    fn from_iter<T: IntoIterator<Item = Vec<Operation>>>(iter: T) -> Self {
        iter.into_iter().flatten().collect()
    }
}

impl IntoIterator for Operations {
    type Item = Operation;
    type IntoIter = vec::IntoIter<Operation>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Operations {
    pub fn new(mut ops: Vec<Operation>) -> Self {
        for (index, mut op) in ops.iter_mut().enumerate() {
            op.operation_identifier = (index as u64).into()
        }
        Self(ops)
    }

    pub fn set_status(mut self, status: Option<OperationStatus>) -> Self {
        for op in &mut self.0 {
            op.status = status
        }
        self
    }

    pub fn type_(&self) -> Option<OperationType> {
        self.0.first().map(|op| op.type_)
    }

    /// Parse operation input from rosetta to Haneul transaction
    pub fn into_internal(
        self,
        metadata: Option<PreprocessMetadata>,
    ) -> Result<InternalOperation, Error> {
        match (
            self.type_()
                .ok_or_else(|| Error::MissingInput("Operation type".into()))?,
            metadata,
        ) {
            (OperationType::PayHaneul, _) => self.pay_haneul_ops_to_internal(),
            (
                OperationType::Delegation,
                Some(PreprocessMetadata::Delegation { locked_until_epoch }),
            ) => self.delegation_ops_to_internal(locked_until_epoch),
            (OperationType::Delegation, _) => self.delegation_ops_to_internal(None),
            (op, _) => Err(Error::UnsupportedOperation(op)),
        }
    }

    fn pay_haneul_ops_to_internal(self) -> Result<InternalOperation, Error> {
        let mut recipients = vec![];
        let mut amounts = vec![];
        let mut sender = None;
        for op in self {
            if let (Some(amount), Some(account)) = (op.amount.clone(), op.account.clone()) {
                if amount.value.is_negative() {
                    sender = Some(account.address)
                } else {
                    recipients.push(account.address);
                    let amount = amount.value.abs();
                    if amount > u64::MAX as i128 {
                        return Err(Error::InvalidInput(
                            "Input amount exceed u64::MAX".to_string(),
                        ));
                    }
                    amounts.push(amount as u64)
                }
            }
        }
        let sender = sender.ok_or_else(|| Error::MissingInput("Sender address".to_string()))?;
        Ok(InternalOperation::PayHaneul {
            sender,
            recipients,
            amounts,
        })
    }

    fn delegation_ops_to_internal(
        self,
        locked_until_epoch: Option<EpochId>,
    ) -> Result<InternalOperation, Error> {
        let mut ops = self
            .0
            .into_iter()
            .filter(|op| op.type_ == OperationType::Delegation)
            .collect::<Vec<_>>();
        if ops.len() != 1 {
            return Err(Error::MalformedOperationError(
                "Delegation should only have one operation.".into(),
            ));
        }
        // Checked above, safe to unwrap.
        let op = ops.pop().unwrap();
        let sender = op
            .account
            .ok_or_else(|| Error::MissingInput("Sender address".to_string()))?
            .address;
        let metadata = op
            .metadata
            .ok_or_else(|| Error::MissingInput("Delegation metadata".to_string()))?;

        let amount = op
            .amount
            .ok_or_else(|| Error::MissingInput("Amount".to_string()))?
            .value
            .unsigned_abs();

        let OperationMetadata::Delegation {  validator } = metadata else {
            return Err(Error::InvalidInput("Cannot find delegation info from metadata.".into()))
        };

        Ok(InternalOperation::Delegation {
            sender,
            validator,
            amount,
            locked_until_epoch,
        })
    }

    fn from_transaction(
        tx: HaneulTransactionKind,
        sender: HaneulAddress,
        status: Option<OperationStatus>,
    ) -> Result<Vec<Operation>, Error> {
        Ok(match tx {
            HaneulTransactionKind::ProgrammableTransaction(pt) => {
                Self::parse_programmable_transaction(sender, status, pt)?
            }
            _ => vec![Operation::generic_op(status, sender, tx)],
        })
    }

    fn parse_programmable_transaction(
        sender: HaneulAddress,
        status: Option<OperationStatus>,
        pt: HaneulProgrammableTransaction,
    ) -> Result<Vec<Operation>, Error> {
        enum KnownValue {
            GasCoin(u64),
        }
        macro_rules! bcs_json_to_value {
            ($value:expr) => {
                $value.to_json_value().as_array().and_then(|v| {
                    // value is a byte array
                    let bytes = v
                        .iter()
                        .flat_map(|v| v.as_u64().map(|n| n as u8))
                        .collect::<Vec<_>>();
                    bcs::from_bytes(&bytes).ok()
                })
            };
        }
        fn resolve_result(
            known_results: &[Vec<KnownValue>],
            i: u16,
            j: u16,
        ) -> Option<&KnownValue> {
            known_results
                .get(i as usize)
                .and_then(|inner| inner.get(j as usize))
        }
        fn split_coin(
            inputs: &[HaneulJsonValue],
            _known_results: &[Vec<KnownValue>],
            _coin: HaneulArgument,
            amount: HaneulArgument,
        ) -> Option<Vec<KnownValue>> {
            let amount: u64 = match amount {
                HaneulArgument::Input(i) => bcs_json_to_value!(&inputs[i as usize])?,
                HaneulArgument::GasCoin | HaneulArgument::Result(_) | HaneulArgument::NestedResult(_, _) => {
                    return None
                }
            };
            Some(vec![KnownValue::GasCoin(amount)])
        }
        fn transfer_object(
            aggregated_recipients: &mut HashMap<HaneulAddress, u64>,
            inputs: &[HaneulJsonValue],
            known_results: &[Vec<KnownValue>],
            objs: &[HaneulArgument],
            recipient: HaneulArgument,
        ) -> Option<Vec<KnownValue>> {
            let addr = match recipient {
                HaneulArgument::Input(i) => inputs[i as usize].to_haneul_address().ok()?,
                HaneulArgument::GasCoin | HaneulArgument::Result(_) | HaneulArgument::NestedResult(_, _) => {
                    return None
                }
            };
            for obj in objs {
                let value = match *obj {
                    HaneulArgument::Result(i) => {
                        let KnownValue::GasCoin(value) = resolve_result(known_results, i, 0)?;
                        value
                    }
                    HaneulArgument::NestedResult(i, j) => {
                        let KnownValue::GasCoin(value) = resolve_result(known_results, i, j)?;
                        value
                    }
                    HaneulArgument::GasCoin | HaneulArgument::Input(_) => return None,
                };
                let aggregate = aggregated_recipients.entry(addr).or_default();
                *aggregate += value;
            }
            Some(vec![])
        }
        fn delegation_call(
            inputs: &[HaneulJsonValue],
            _known_results: &[Vec<KnownValue>],
            call: &HaneulProgrammableMoveCall,
        ) -> Result<Option<(Option<u64>, HaneulAddress)>, Error> {
            let HaneulProgrammableMoveCall { arguments, .. } = call;
            let (amount, validator) = match &arguments[..] {
                [_, _, amount, validator] => {
                    let amount = match amount {
                        HaneulArgument::Input(i) => match bcs_json_to_value!(&inputs[*i as usize]){
                            Some(amount) => amount,
                            None => return Ok(None),
                        },
                        HaneulArgument::GasCoin |
                        HaneulArgument::Result(_) |
                        HaneulArgument::NestedResult(_, _) => return Ok(None),
                    };
                    let validator = match validator {
                        HaneulArgument::Input(i) => inputs[*i as usize].to_haneul_address(),
                        HaneulArgument::GasCoin |
                        HaneulArgument::Result(_) |
                        HaneulArgument::NestedResult(_, _) => return Ok(None),
                    };
                    (amount, validator)
                },
                _ => return Err(Error::InternalError(anyhow!("Error encountered when extracting arguments from move call, expecting 4 elements, got {}", arguments.len()))),
            };
            let validator = validator.map_err(|_| {
                Error::InternalError(anyhow!("Error parsing Validator address from call arg."))
            })?;
            Ok(Some((amount, validator)))
        }

        let HaneulProgrammableTransaction { inputs, commands } = &pt;
        let mut known_results: Vec<Vec<KnownValue>> = vec![];
        let mut aggregated_recipients: HashMap<HaneulAddress, u64> = HashMap::new();
        let mut needs_generic = false;
        let mut operations = vec![];
        for command in commands {
            let result = match command {
                HaneulCommand::SplitCoin(coin, amount) => {
                    split_coin(inputs, &known_results, *coin, *amount)
                }
                HaneulCommand::TransferObjects(objs, addr) => transfer_object(
                    &mut aggregated_recipients,
                    inputs,
                    &known_results,
                    objs,
                    *addr,
                ),
                HaneulCommand::MoveCall(m) if Self::is_delegation_call(m) => {
                    delegation_call(inputs, &known_results, m)?.map(|(amount, validator)| {
                        let amount = amount.map(|amount| Amount::new(-(amount as i128)));
                        operations.push(Operation {
                            operation_identifier: Default::default(),
                            type_: OperationType::Delegation,
                            status,
                            account: Some(sender.into()),
                            amount,
                            coin_change: None,
                            metadata: Some(OperationMetadata::Delegation { validator }),
                        });
                        vec![]
                    })
                }
                _ => None,
            };
            if let Some(result) = result {
                known_results.push(result)
            } else {
                needs_generic = true;
                known_results.push(vec![])
            }
        }

        let total_paid: u64 = aggregated_recipients.values().copied().sum();
        operations.extend(
            aggregated_recipients
                .into_iter()
                .map(|(recipient, amount)| Operation::pay_haneul(status, recipient, amount.into())),
        );
        operations.push(Operation::pay_haneul(status, sender, -(total_paid as i128)));
        if needs_generic {
            operations.push(Operation::generic_op(
                status,
                sender,
                HaneulTransactionKind::ProgrammableTransaction(pt),
            ))
        }
        Ok(operations)
    }

    fn is_delegation_call(tx: &HaneulProgrammableMoveCall) -> bool {
        tx.package == HANEUL_FRAMEWORK_OBJECT_ID
            && tx.module == HANEUL_SYSTEM_MODULE_NAME.as_str()
            && (tx.function == ADD_DELEGATION_LOCKED_COIN_FUN_NAME.as_str()
                || tx.function == ADD_DELEGATION_MUL_COIN_FUN_NAME.as_str())
    }

    fn get_balance_operation_from_events(
        events: &[HaneulEvent],
        status: Option<OperationStatus>,
        balances: HashMap<HaneulAddress, i128>,
    ) -> impl Iterator<Item = Operation> {
        let (balances, gas) = events
            .iter()
            .flat_map(Self::get_balance_change_from_event)
            .fold(
                (balances, HashMap::<HaneulAddress, i128>::new()),
                |(mut balances, mut gas), (type_, address, amount)| {
                    if type_ == BalanceChangeType::Gas {
                        *gas.entry(address).or_default() += amount;
                    } else {
                        *balances.entry(address).or_default() += amount;
                    }
                    (balances, gas)
                },
            );

        let balance_change = balances
            .into_iter()
            .filter(|(_, amount)| *amount != 0)
            .map(move |(addr, amount)| Operation::balance_change(status, addr, amount));
        let gas = gas
            .into_iter()
            .map(|(addr, amount)| Operation::gas(addr, amount));

        balance_change.chain(gas)
    }

    fn get_balance_change_from_event(
        event: &HaneulEvent,
    ) -> Option<(BalanceChangeType, HaneulAddress, i128)> {
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
                return Some((*change_type, *owner, *amount));
            }
        }
        None
    }
}

impl TryFrom<HaneulTransactionData> for Operations {
    type Error = Error;
    fn try_from(data: HaneulTransactionData) -> Result<Self, Self::Error> {
        let sender = *data.sender();
        data.transactions()
            .iter()
            .map(|tx| Self::from_transaction(tx.clone(), sender, None))
            .collect()
    }
}

impl TryFrom<HaneulTransactionResponse> for Operations {
    type Error = Error;
    fn try_from(response: HaneulTransactionResponse) -> Result<Self, Self::Error> {
        let status = Some(
            response
                .effects
                .ok_or_else(|| {
                    Error::InternalError(anyhow!("Response effects should not be empty"))
                })?
                .into_status()
                .into(),
        );
        let ops: Operations = response
            .transaction
            .ok_or_else(|| {
                Error::InternalError(anyhow!("Response transaction should not be empty"))
            })?
            .data
            .try_into()?;
        let ops = ops.set_status(status).into_iter();

        // We will need to subtract the operation amounts from the actual balance
        // change amount extracted from event to prevent double counting.
        let accounted_balances = ops
            .as_ref()
            .iter()
            .filter_map(|op| match (&op.account, &op.amount, &op.status) {
                (Some(acc), Some(amount), Some(OperationStatus::Success)) => {
                    Some((acc.address, -amount.value))
                }
                _ => None,
            })
            .fold(HashMap::new(), |mut balances, (addr, amount)| {
                *balances.entry(addr).or_default() += amount;
                balances
            });

        // Extract coin change operations from events
        let coin_change_operations = Self::get_balance_operation_from_events(
            &response
                .events
                .ok_or_else(|| {
                    Error::InternalError(anyhow!("Response events should not be empty"))
                })?
                .data,
            status,
            accounted_balances,
        );
        Ok(ops.into_iter().chain(coin_change_operations).collect())
    }
}

impl TryFrom<TransactionData> for Operations {
    type Error = Error;
    fn try_from(data: TransactionData) -> Result<Self, Self::Error> {
        HaneulTransactionData::try_from(data)?.try_into()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Operation {
    operation_identifier: OperationIdentifier,
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
    pub metadata: Option<OperationMetadata>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum OperationMetadata {
    GenericTransaction(HaneulTransactionKind),
    Delegation { validator: HaneulAddress },
}

impl Operation {
    fn generic_op(
        status: Option<OperationStatus>,
        sender: HaneulAddress,
        tx: HaneulTransactionKind,
    ) -> Self {
        Operation {
            operation_identifier: Default::default(),
            type_: (&tx).into(),
            status,
            account: Some(sender.into()),
            amount: None,
            coin_change: None,
            metadata: Some(OperationMetadata::GenericTransaction(tx)),
        }
    }

    pub fn genesis(index: u64, sender: HaneulAddress, coin: GasCoin) -> Self {
        Operation {
            operation_identifier: index.into(),
            type_: OperationType::Genesis,
            status: Some(OperationStatus::Success),
            account: Some(sender.into()),
            amount: Some(Amount::new(coin.value().into())),
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: CoinID {
                        id: *coin.id(),
                        version: SequenceNumber::new(),
                    },
                },
                coin_action: CoinAction::CoinCreated,
            }),
            metadata: None,
        }
    }

    fn pay_haneul(status: Option<OperationStatus>, address: HaneulAddress, amount: i128) -> Self {
        Operation {
            operation_identifier: Default::default(),
            type_: OperationType::PayHaneul,
            status,
            account: Some(address.into()),
            amount: Some(Amount::new(amount)),
            coin_change: None,
            metadata: None,
        }
    }

    fn balance_change(status: Option<OperationStatus>, addr: HaneulAddress, amount: i128) -> Self {
        Self {
            operation_identifier: Default::default(),
            type_: OperationType::HaneulBalanceChange,
            status,
            account: Some(addr.into()),
            amount: Some(Amount::new(amount)),
            coin_change: None,
            metadata: None,
        }
    }
    fn gas(addr: HaneulAddress, amount: i128) -> Self {
        Self {
            operation_identifier: Default::default(),
            type_: OperationType::Gas,
            status: Some(OperationStatus::Success),
            account: Some(addr.into()),
            amount: Some(Amount::new(amount)),
            coin_change: None,
            metadata: None,
        }
    }
}
