// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::anyhow;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use haneul_rpc::client::Client;
use haneul_rpc::proto::haneul::rpc::v2::{
    BatchGetObjectsRequest, GetObjectRequest, Object, get_object_result,
};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::TypeTag;
use prost_types::FieldMask;
use rand::Rng;
use serde::{Deserialize, Serialize};

use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::proto::haneul::rpc::v2::{
    GasPayment, ObjectReference, ProgrammableTransaction as ProtoProgrammableTransaction,
    SimulateTransactionRequest, Transaction, TransactionKind,
    simulate_transaction_request::TransactionChecks, transaction_kind,
};
use haneul_types::HANEUL_FRAMEWORK_PACKAGE_ID;
use haneul_types::base_types::{HaneulAddress, ObjectID, ObjectRef, SequenceNumber};
use haneul_types::digests::{ChainIdentifier, CheckpointDigest};
use haneul_types::transaction::{
    Argument, CallArg, Command, FundsWithdrawalArg, ProgrammableTransaction, TransactionData,
};

use crate::errors::Error;
use crate::types::ConstructionMetadata;
pub use consolidate_to_fungible::ConsolidateAllStakedHaneulToFungible;
pub(crate) use consolidate_to_fungible::consolidate_to_fungible_pt;
pub(crate) use consolidate_to_fungible::get_validator_pool_id;
pub use merge_and_redeem::MergeAndRedeemFungibleStakedHaneul;
pub(crate) use merge_and_redeem::merge_and_redeem_fss_pt;
pub use pay_coin::PayCoin;
pub(crate) use pay_coin::pay_coin_pt;
pub use pay_haneul::PayHaneul;
use pay_haneul::{pay_haneul_pt_ab_gas, pay_haneul_pt_coin_gas};
pub use stake::Stake;
use stake::{stake_pt_ab_gas, stake_pt_coin_gas};
pub use withdraw_stake::WithdrawStake;
use withdraw_stake::withdraw_stake_pt;

mod consolidate_to_fungible;
mod merge_and_redeem;
mod pay_coin;
mod pay_haneul;
mod stake;
mod withdraw_stake;

pub const MAX_GAS_COINS: usize = 255;
const MAX_COMMAND_ARGS: usize = 511;

pub struct TransactionObjectData {
    pub gas_coins: Vec<ObjectRef>,
    /// For PayHaneul/Stake: extra gas coins to merge into gas
    /// For PayCoin: payment coins of the specified type
    /// For WithdrawStake: stake objects to withdraw
    pub objects: Vec<ObjectRef>,
    /// Party-owned (ConsensusAddress) version of objects
    pub party_objects: Vec<(ObjectID, SequenceNumber)>,
    /// Refers to the sum of the `Coin<HANEUL>` balance of the coins participating in the transaction;
    /// either as gas or as objects.
    pub total_haneul_balance: i128,
    pub budget: u64,
    /// Amount to withdraw from address balance for payment
    pub address_balance_withdrawal: u64,
    /// Number of FungibleStakedHaneul objects in the `objects` array (the rest are StakedHaneul).
    /// Used by ConsolidateAllStakedHaneulToFungible to split objects for PTB construction.
    pub fss_object_count: Option<u64>,
    /// Pool tokens to redeem. None = redeem all.
    /// Used by MergeAndRedeemFungibleStakedHaneul.
    pub redeem_token_amount: Option<u64>,
}

#[async_trait]
#[enum_dispatch]
pub trait TryConstructTransaction {
    async fn try_fetch_needed_objects(
        self,
        client: &mut Client,
        gas_price: Option<u64>,
        budget: Option<u64>,
    ) -> Result<TransactionObjectData, Error>;
}

#[enum_dispatch(TryConstructTransaction)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalOperation {
    PayHaneul(PayHaneul),
    PayCoin(PayCoin),
    Stake(Stake),
    WithdrawStake(WithdrawStake),
    ConsolidateAllStakedHaneulToFungible(ConsolidateAllStakedHaneulToFungible),
    MergeAndRedeemFungibleStakedHaneul(MergeAndRedeemFungibleStakedHaneul),
}

impl InternalOperation {
    pub fn sender(&self) -> HaneulAddress {
        match self {
            InternalOperation::PayHaneul(PayHaneul { sender, .. })
            | InternalOperation::PayCoin(PayCoin { sender, .. })
            | InternalOperation::Stake(Stake { sender, .. })
            | InternalOperation::WithdrawStake(WithdrawStake { sender, .. })
            | InternalOperation::ConsolidateAllStakedHaneulToFungible(
                ConsolidateAllStakedHaneulToFungible { sender, .. },
            )
            | InternalOperation::MergeAndRedeemFungibleStakedHaneul(
                MergeAndRedeemFungibleStakedHaneul { sender, .. },
            ) => *sender,
        }
    }

    /// Combine with ConstructionMetadata to form the TransactionData
    pub fn try_into_data(self, metadata: ConstructionMetadata) -> Result<TransactionData, Error> {
        let use_addr_balance_gas = metadata.gas_coins.is_empty();
        let withdrawal = metadata.address_balance_withdrawal;
        let pt = match self {
            Self::PayHaneul(PayHaneul {
                sender,
                recipients,
                amounts,
            }) => {
                let coins = if !metadata.objects.is_empty() {
                    &metadata.objects
                } else {
                    &metadata.extra_gas_coins
                };
                if use_addr_balance_gas {
                    pay_haneul_pt_ab_gas(
                        sender,
                        recipients,
                        amounts,
                        coins,
                        &metadata.party_objects,
                        withdrawal,
                    )?
                } else {
                    pay_haneul_pt_coin_gas(
                        recipients,
                        amounts,
                        coins,
                        &metadata.party_objects,
                        withdrawal,
                    )?
                }
            }
            Self::PayCoin(PayCoin {
                sender,
                recipients,
                amounts,
                ..
            }) => {
                let currency = &metadata
                    .currency
                    .ok_or(anyhow!("metadata.coin_type is needed to PayCoin"))?;
                pay_coin_pt(
                    sender,
                    recipients,
                    amounts,
                    &metadata.objects,
                    &metadata.party_objects,
                    withdrawal,
                    currency,
                )?
            }
            InternalOperation::Stake(Stake {
                sender,
                validator,
                amount,
            }) => {
                let (stake_all, amount) = match amount {
                    Some(amount) => (false, amount),
                    None => {
                        if (metadata.total_coin_value - metadata.budget as i128) < 0 {
                            return Err(anyhow!(
                                "ConstructionMetadata malformed. total_coin_value - budget < 0"
                            )
                            .into());
                        }
                        (true, metadata.total_coin_value as u64 - metadata.budget)
                    }
                };
                let coins = if !metadata.objects.is_empty() {
                    &metadata.objects
                } else {
                    &metadata.extra_gas_coins
                };
                if use_addr_balance_gas {
                    stake_pt_ab_gas(
                        sender,
                        validator,
                        amount,
                        stake_all,
                        coins,
                        &metadata.party_objects,
                        withdrawal,
                    )?
                } else {
                    stake_pt_coin_gas(
                        validator,
                        amount,
                        stake_all,
                        coins,
                        &metadata.party_objects,
                        withdrawal,
                    )?
                }
            }
            InternalOperation::WithdrawStake(WithdrawStake { stake_ids, .. }) => {
                let withdraw_all = stake_ids.is_empty();
                withdraw_stake_pt(metadata.objects, withdraw_all)?
            }
            InternalOperation::ConsolidateAllStakedHaneulToFungible(
                ConsolidateAllStakedHaneulToFungible { sender, .. },
            ) => {
                // objects[0..fss_count] are FungibleStakedHaneul, objects[fss_count..] are StakedHaneul
                let fss_count = metadata.fss_object_count.unwrap_or(0) as usize;
                let (fss_refs, staked_haneul_refs) = metadata
                    .objects
                    .split_at(fss_count.min(metadata.objects.len()));
                consolidate_to_fungible_pt(sender, fss_refs.to_vec(), staked_haneul_refs.to_vec())?
            }
            InternalOperation::MergeAndRedeemFungibleStakedHaneul(
                MergeAndRedeemFungibleStakedHaneul { sender, .. },
            ) => merge_and_redeem_fss_pt(sender, metadata.objects, metadata.redeem_token_amount)?,
        };

        if metadata.gas_coins.is_empty() {
            let chain_id_str = metadata
                .chain_id
                .ok_or(anyhow!("chain_id required for address-balance gas"))?;
            let digest = CheckpointDigest::from_str(&chain_id_str)
                .map_err(|e| anyhow!("invalid chain_id: {e}"))?;
            let chain_id = ChainIdentifier::from(digest);
            let epoch = metadata
                .epoch
                .ok_or(anyhow!("epoch required for address-balance gas"))?;
            let nonce = rand::thread_rng().r#gen::<u32>();

            Ok(TransactionData::new_programmable_with_address_balance_gas(
                metadata.sender,
                pt,
                metadata.budget,
                metadata.gas_price,
                chain_id,
                epoch,
                nonce,
            ))
        } else {
            Ok(TransactionData::new_programmable(
                metadata.sender,
                metadata.gas_coins,
                pt,
                metadata.budget,
                metadata.gas_price,
            ))
        }
    }
}

/// Withdraw from address balance as a Coin<T>.
/// FundsWithdrawal → coin::redeem_funds → Coin<T>
pub(crate) fn withdraw_coin_from_address_balance(
    builder: &mut haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder,
    amount: u64,
    type_tag: TypeTag,
) -> anyhow::Result<Argument> {
    let withdrawal_arg = builder.input(CallArg::FundsWithdrawal(
        FundsWithdrawalArg::balance_from_sender(amount, type_tag.clone()),
    ))?;

    let coin = builder.command(Command::move_call(
        HANEUL_FRAMEWORK_PACKAGE_ID,
        Identifier::new("coin")?,
        Identifier::new("redeem_funds")?,
        vec![type_tag],
        vec![withdrawal_arg],
    ));

    Ok(coin)
}

/// RPC auto-selects gas coins if empty, uses reference gas price if None, and estimates budget if None.
/// Returns the resolved budget and gas coins used by the transaction.
async fn simulate_transaction(
    client: &mut Client,
    pt: ProgrammableTransaction,
    sender: HaneulAddress,
    gas_coins: Vec<ObjectRef>,
    gas_price: Option<u64>,
    budget: Option<u64>,
) -> Result<(u64, Vec<Object>), Error> {
    let ptb_proto: ProtoProgrammableTransaction = pt.into();
    let mut transaction = Transaction::default()
        .with_kind(
            TransactionKind::default()
                .with_programmable_transaction(ptb_proto)
                .with_kind(transaction_kind::Kind::ProgrammableTransaction),
        )
        .with_sender(sender.to_string());

    let mut gas_payment = GasPayment::default();
    gas_payment.objects = gas_coins
        .into_iter()
        .map(|gas_ref| {
            let mut obj_ref = ObjectReference::default();
            obj_ref.object_id = Some(gas_ref.0.to_string());
            obj_ref.version = Some(gas_ref.1.value());
            obj_ref.digest = Some(gas_ref.2.to_string());
            obj_ref
        })
        .collect();
    gas_payment.budget = budget;
    gas_payment.price = gas_price;
    gas_payment.owner = Some(sender.to_string());
    transaction.gas_payment = Some(gas_payment);

    let request = SimulateTransactionRequest::default()
        .with_transaction(transaction)
        .with_read_mask(FieldMask::from_paths([
            "transaction.effects.status",
            "transaction.transaction.gas_payment",
        ]))
        .with_checks(TransactionChecks::Enabled)
        .with_do_gas_selection(true);

    let response = client
        .execution_client()
        .simulate_transaction(request)
        .await?
        .into_inner();

    let executed_tx = response.transaction();
    let effects = executed_tx.effects();
    if !effects.status().success() {
        return Err(Error::TransactionDryRunError(Box::new(
            effects.status().error().clone(),
        )));
    }

    let resolved_tx = executed_tx.transaction();
    let gas_payment = resolved_tx.gas_payment();

    // When gas_payment has no objects, the transaction uses address-balance gas.
    // Skip the batch fetch and return empty gas coins to signal this.
    let gas_objects = gas_payment.objects();
    if gas_objects.is_empty() {
        return Ok((gas_payment.budget(), vec![]));
    }

    let mut batch_request =
        BatchGetObjectsRequest::default().with_read_mask(FieldMask::from_paths([
            "object_id",
            "version",
            "digest",
            "balance",
        ]));

    for obj_ref in gas_objects {
        let get_request = GetObjectRequest::default()
            .with_object_id(obj_ref.object_id().to_string())
            .with_version(obj_ref.version());
        batch_request.requests.push(get_request);
    }

    let batch_response = client
        .ledger_client()
        .batch_get_objects(batch_request)
        .await?
        .into_inner();

    let mut gas_coins = Vec::new();
    for result in batch_response.objects {
        match result.result {
            Some(get_object_result::Result::Object(obj)) => {
                gas_coins.push(obj);
            }
            Some(get_object_result::Result::Error(err)) => {
                return Err(Error::DataError(format!(
                    "Failed to fetch gas coin object: {:?}",
                    err
                )));
            }
            None => {
                return Err(Error::DataError(
                    "Failed to fetch gas coin object: no result returned".to_string(),
                ));
            }
            Some(_) => {
                return Err(Error::DataError(
                    "Failed to fetch gas coin object: unexpected result type".to_string(),
                ));
            }
        }
    }

    Ok((gas_payment.budget(), gas_coins))
}
