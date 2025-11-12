// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use haneul_rpc::client::Client;
use haneul_rpc::proto::haneul::rpc::v2::{Object, owner::OwnerKind};
use haneul_sdk_types::{Address, StructTag};
use haneul_types::base_types::{ObjectID, ObjectRef, SequenceNumber, HaneulAddress};
use haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use haneul_types::rpc_proto_conversions::ObjectReferenceExt;
use haneul_types::transaction::{Argument, Command, ObjectArg, ProgrammableTransaction};

use crate::errors::Error;

use super::{
    MAX_COMMAND_ARGS, MAX_GAS_COINS, TransactionObjectData, TryConstructTransaction,
    simulate_transaction,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PayHaneul {
    pub sender: HaneulAddress,
    pub recipients: Vec<HaneulAddress>,
    pub amounts: Vec<u64>,
}

#[async_trait]
impl TryConstructTransaction for PayHaneul {
    async fn try_fetch_needed_objects(
        self,
        client: &mut Client,
        gas_price: Option<u64>,
        budget: Option<u64>,
    ) -> Result<TransactionObjectData, Error> {
        let Self {
            sender,
            recipients,
            amounts,
        } = self;

        // PayHaneul needs enough HANEUL to cover both payment amount and gas. We select up to 1500
        // coins (we observed ~1650 coins in a single transaction hits transaction size limits)
        // and merge them all together, then split off the payment amount.
        //
        // This handles cases where the user has sufficient total balance but no single coin or
        // simple combination covers payment + gas without merging/splitting. For example, with
        // [40, 35, 25] HANEUL coins and needing to pay 50 + gas, no discrete set works - we must
        // merge first to create a coin large enough to split appropriately.
        //
        // This approach is optimal because storage refunds from merging dust outweigh smashing
        // costs by an order of magnitude, and ensures we can handle fragmented balances.
        // Use the full Coin<HANEUL> struct tag
        let all_coins = client
            .select_up_to_n_largest_coins(
                &Address::from(sender),
                &StructTag::haneul().into(),
                1500,
                &[],
            )
            .await?;

        let total_haneul_balance = all_coins.iter().map(|c| c.balance()).sum::<u64>() as i128;

        // Separate party objects (ConsensusAddressOwner) from regular objects.
        // Party objects cannot be used as gas but can be merged into the gas coin.
        let (party_objects, non_party_objects): (Vec<_>, Vec<_>) = all_coins
            .iter()
            .partition(|obj| obj.owner().kind() == OwnerKind::ConsensusAddress);

        let mut iter = non_party_objects
            .iter()
            .map(|obj: &&Object| obj.object_reference().try_to_object_ref());
        let gas_coins = iter
            .by_ref()
            .take(MAX_GAS_COINS)
            .collect::<Result<Vec<_>, _>>()?;

        let extra_coins = iter.collect::<Result<Vec<_>, _>>()?;

        let extra_party_coins: Vec<(ObjectID, SequenceNumber)> = party_objects
            .iter()
            .map(|obj: &&Object| -> Result<_, Error> {
                let id = ObjectID::from_str(obj.object_id())
                    .map_err(|e| Error::DataError(format!("Invalid party object ID: {}", e)))?;
                let start_version = SequenceNumber::from_u64(obj.owner().version());
                Ok((id, start_version))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Simulate to get budget if necessary and validate we can cover payment + gas amount.
        let pt = pay_haneul_pt(recipients, amounts, &extra_coins, &extra_party_coins)?;
        let (budget, gas_coin_objs) =
            simulate_transaction(client, pt, sender, gas_coins, gas_price, budget).await?;

        let gas_coins = gas_coin_objs
            .iter()
            .map(|obj| obj.object_reference().try_to_object_ref())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TransactionObjectData {
            gas_coins,
            objects: extra_coins,
            party_objects: extra_party_coins,
            total_haneul_balance,
            budget,
        })
    }
}

/// Creates the `ProgrammableTransaction` for a pay-haneul operation.
/// In case pay-haneul needs more than 255 gas-coins to be smashed, it tries to merge the surplus
/// coins into the gas coin as regular transaction inputs - not gas-payment.
/// This approach has the limit at around 1650 coins in total which triggers transaction-size
/// limit (see also test_limit_many_small_coins test).
pub fn pay_haneul_pt(
    recipients: Vec<HaneulAddress>,
    amounts: Vec<u64>,
    coins_to_merge: &[ObjectRef],
    party_coins: &[(ObjectID, SequenceNumber)],
) -> anyhow::Result<ProgrammableTransaction> {
    let mut builder = ProgrammableTransactionBuilder::new();
    if !coins_to_merge.is_empty() {
        coins_to_merge
            .chunks(MAX_COMMAND_ARGS)
            .try_for_each(|chunk| -> anyhow::Result<()> {
                let to_merge = chunk
                    .iter()
                    .map(|&o| builder.obj(ObjectArg::ImmOrOwnedObject(o)))
                    .collect::<Result<Vec<Argument>, anyhow::Error>>()?;
                builder.command(Command::MergeCoins(Argument::GasCoin, to_merge));
                Ok(())
            })?;
    };

    if !party_coins.is_empty() {
        party_coins
            .chunks(MAX_COMMAND_ARGS)
            .try_for_each(|chunk| -> anyhow::Result<()> {
                let to_merge = chunk
                    .iter()
                    .map(|&(id, initial_shared_version)| {
                        builder.obj(ObjectArg::SharedObject {
                            id,
                            initial_shared_version,
                            mutability: haneul_types::transaction::SharedObjectMutability::Mutable,
                        })
                    })
                    .collect::<Result<Vec<Argument>, anyhow::Error>>()?;
                builder.command(Command::MergeCoins(Argument::GasCoin, to_merge));
                Ok(())
            })?;
    };

    builder.pay_haneul(recipients, amounts)?;
    Ok(builder.finish())
}
