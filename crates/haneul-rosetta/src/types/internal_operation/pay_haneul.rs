// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use haneul_sdk::HaneulClient;
use haneul_types::base_types::{ObjectRef, HaneulAddress};
use haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use haneul_types::transaction::{Argument, Command, ObjectArg, ProgrammableTransaction};

use crate::errors::Error;

use super::{
    collect_coins_until_budget_met, TransactionObjectData, TryConstructTransaction,
    MAX_COMMAND_ARGS, MAX_GAS_COINS,
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
        client: &HaneulClient,
        gas_price: Option<u64>,
        budget: Option<u64>,
    ) -> Result<TransactionObjectData, Error> {
        let Self {
            sender,
            recipients,
            amounts,
        } = self;

        let total_amount = amounts.iter().sum::<u64>();
        if let Some(budget) = budget {
            // We have a constant budget, so no need to dry-run
            let all_coins = client
                .coin_read_api()
                .select_coins(sender, None, (total_amount + budget) as u128, vec![])
                .await?;

            let total_haneul_balance = all_coins.iter().map(|c| c.balance).sum::<u64>() as i128;

            let mut iter = all_coins.into_iter().map(|c| c.object_ref());
            let gas_coins: Vec<_> = iter.by_ref().take(MAX_GAS_COINS).collect();
            let extra_gas_coins: Vec<_> = iter.collect();

            return Ok(TransactionObjectData {
                gas_coins,
                extra_gas_coins,
                objects: vec![],
                total_haneul_balance,
                budget,
            });
        };

        let total_amount = amounts.iter().sum::<u64>();
        let pay_haneul_pt = |extra_gas_coins: &[ObjectRef]| {
            pay_haneul_pt(recipients.clone(), amounts.clone(), extra_gas_coins)
        };
        collect_coins_until_budget_met(client, sender, pay_haneul_pt, total_amount, gas_price).await
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
) -> anyhow::Result<ProgrammableTransaction> {
    let mut builder = ProgrammableTransactionBuilder::new();
    if !coins_to_merge.is_empty() {
        // We need to merge the rest of the coins.
        // Each merge has a limit of 511 arguments.
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
    builder.pay_haneul(recipients, amounts)?;
    Ok(builder.finish())
}
