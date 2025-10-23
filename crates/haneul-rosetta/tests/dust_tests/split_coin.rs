// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Result, anyhow};

use shared_crypto::intent::Intent;
use haneul_json_rpc_types::{
    Coin, ObjectChange, HaneulExecutionStatus, HaneulMoveValue, HaneulObjectDataOptions, HaneulObjectRef,
    HaneulParsedData, HaneulTransactionBlockEffectsAPI, HaneulTransactionBlockResponse,
    HaneulTransactionBlockResponseOptions,
};
use haneul_keys::keystore::{AccountKeystore, Keystore};
use haneul_sdk::{HANEUL_COIN_TYPE, HaneulClient};
use haneul_types::base_types::{ObjectID, ObjectRef, HaneulAddress};
use haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use haneul_types::quorum_driver_types::ExecuteTransactionRequestType;
use haneul_types::transaction::{
    Argument, Command, ObjectArg, Transaction, TransactionData, TransactionDataAPI,
};
use test_cluster::TestClusterBuilder;

pub const DEFAULT_GAS_BUDGET: u64 = 900_000_000;
const DEFAULT_INIT_COIN_BALANCE: u64 = 30_000_000_000_000_000;
const MAX_NEW_COINS: usize = 511; // maximum arguments in a programmable transaction command is 511

pub async fn split_coins(
    client: &HaneulClient,
    keystore: &Keystore,
    sender: HaneulAddress,
    coin: ObjectRef,
    amounts: &[u64],
    gas: Option<ObjectRef>,
    reference_gas_price: Option<u64>,
    budget: Option<u64>,
) -> Result<HaneulTransactionBlockResponse> {
    if amounts.len() > MAX_NEW_COINS {
        return Err(anyhow!("Max new coins: {}", MAX_NEW_COINS));
    }
    let budget = budget.unwrap_or(DEFAULT_GAS_BUDGET);
    let reference_gas_price = match reference_gas_price {
        Some(price) => price,
        None => client.read_api().get_reference_gas_price().await?,
    };
    let mut ptb = ProgrammableTransactionBuilder::new();

    let amounts_len = amounts.len();
    let amounts = amounts
        .iter()
        .map(|amount| ptb.pure(amount))
        .collect::<Result<Vec<_>>>()?;
    let (split_coin, gas) = match gas {
        Some(gas) => (ptb.obj(ObjectArg::ImmOrOwnedObject(coin))?, gas),
        None => (Argument::GasCoin, coin),
    };
    ptb.command(Command::SplitCoins(split_coin, amounts));
    let sender_arg = ptb.pure(sender)?;
    let results = (0..amounts_len)
        .map(|i| Argument::NestedResult(0, i as u16))
        .collect::<Vec<_>>();
    ptb.command(Command::TransferObjects(results, sender_arg));
    let builder = ptb.finish();

    // Sign transaction
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas], builder, budget, reference_gas_price);
    let sig = keystore
        .sign_secure(&tx_data.sender(), &tx_data, Intent::haneul_transaction())
        .await?;

    let res = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![sig]),
            HaneulTransactionBlockResponseOptions::new()
                .with_effects()
                .with_object_changes(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(res)
}

pub async fn make_change(
    client: &HaneulClient,
    keystore: &Keystore,
    sender: HaneulAddress,
    coin: Coin,
    gas: Option<ObjectRef>,
    amount_per_change: u64,
) -> Result<Vec<HaneulTransactionBlockResponse>> {
    let remainder = coin.balance % amount_per_change;
    let n_new_coins = (coin.balance / amount_per_change) as usize - (remainder == 0) as usize;
    let vecs_remainder = n_new_coins % MAX_NEW_COINS;
    let n_vecs = n_new_coins / MAX_NEW_COINS;
    assert!(n_new_coins as u64 * amount_per_change < coin.balance);

    let mut amounts_vec = vec![vec![amount_per_change; MAX_NEW_COINS]; n_vecs];
    amounts_vec.push(vec![amount_per_change; vecs_remainder]);

    let mut responses = Vec::with_capacity(amounts_vec.len());
    let mut coin_ref = coin.object_ref();
    let mut gas_ref = gas;
    let ref_gas_price = client.read_api().get_reference_gas_price().await?;
    let mut progress = 0;
    let len = amounts_vec.len();
    for amounts in amounts_vec.into_iter() {
        let resp = split_coins(
            client,
            keystore,
            sender,
            coin_ref,
            &amounts,
            gas_ref,
            Some(ref_gas_price),
            None,
        )
        .await?;
        progress += 1;
        if progress % 4 == 0 {
            println!(
                "Splitting progress: {}%",
                progress as f32 * 100. / len as f32
            );
        }
        if !resp.status_ok().ok_or(anyhow!("Expected effects"))? {
            println!("resp: {resp:#?}");
            return Err(anyhow!("split_coins errored"));
        }
        coin_ref = resp
            .object_changes
            .as_ref()
            .ok_or(anyhow!("Expected object_changes"))?
            .iter()
            .find(|&chng| chng.object_id() == coin.coin_object_id)
            .ok_or(anyhow!("Expected object_changes to contain coin_object_id"))?
            .object_ref();
        gas_ref = match gas_ref {
            Some(_) => {
                let HaneulObjectRef {
                    object_id,
                    version,
                    digest,
                } = resp
                    .effects
                    .as_ref()
                    .ok_or(anyhow!("Expected balance_changes"))?
                    .gas_object()
                    .reference;
                Some((object_id, version, digest))
            }
            None => None,
        };

        // Make sure the tx has executed locally
        client
            .read_api()
            .get_transaction_with_options(
                resp.digest,
                HaneulTransactionBlockResponseOptions::default(),
            )
            .await?;
        responses.push(resp);
    }
    Ok(responses)
}

#[tokio::test]
async fn test_make_change_exact_div() -> Result<()> {
    const HANEUL_100: u64 = 100_000_000_000;
    const HANEUL_10: u64 = 10_000_000_000;

    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(36000000)
        .build()
        .await;
    let sender = test_cluster.get_address_0();
    let client = test_cluster.wallet.get_client().await.unwrap();
    let keystore = &test_cluster.wallet.config.keystore;

    let coin = client
        .coin_read_api()
        .get_all_coins(sender, None, Some(1))
        .await?
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("Expected 1 coin"))?;

    assert!(
        coin.balance == DEFAULT_INIT_COIN_BALANCE,
        "Coin did not match INIT_COIN_BALANCE"
    );

    let split_resp = split_coins(
        &client,
        keystore,
        sender,
        coin.object_ref(),
        &[HANEUL_100],
        None,
        None,
        None,
    )
    .await?;

    let tx_digest = split_resp.digest;
    if split_resp
        .effects
        .as_ref()
        .ok_or(anyhow!("Expected effects"))?
        .status()
        .clone()
        != HaneulExecutionStatus::Success
    {
        return Err(anyhow!("Transaction failed!: {:#?}", split_resp));
    }

    // let digest = (split_resp.digest).clone();
    let (initial, splitted) = match split_resp
        .object_changes
        .ok_or(anyhow!("Expected object_changes"))?
        .as_slice()
    {
        [coin0, coin1] if coin0.object_id() == coin.coin_object_id => {
            (coin0.object_ref(), coin1.object_ref())
        }
        [coin0, coin1] => (coin1.object_ref(), coin0.object_ref()),
        obj_chngs => {
            println!("object_changes: {obj_chngs:#?}");
            return Err(anyhow!("Expected two items in object changes"));
        }
    };

    let splitted = Coin {
        coin_type: HANEUL_COIN_TYPE.to_string(),
        coin_object_id: splitted.0,
        version: splitted.1,
        digest: splitted.2,
        balance: HANEUL_100,
        previous_transaction: tx_digest,
    };

    let txs = make_change(&client, keystore, sender, splitted, Some(initial), HANEUL_10).await?;

    assert!(txs.len() == 1, "Should only have 1 tx");
    let new_coins = txs
        .into_iter()
        .next()
        .unwrap()
        .object_changes
        .ok_or(anyhow!("Expected object_changes"))?
        .into_iter()
        .filter(|chng| {
            if let ObjectChange::Created { .. } = chng {
                return true;
            }
            false
        })
        .collect::<Vec<_>>();
    assert!(
        new_coins.len() == (HANEUL_100 / HANEUL_10) as usize - 1,
        "Expected {} new coins. New coins: {new_coins:#?}",
        HANEUL_100 / HANEUL_10 - 1
    );

    Ok(())
}

#[tokio::test]
async fn test_make_change_remainder_div() -> Result<()> {
    const HANEUL_100: u64 = 100_000_000_000;
    const HANEUL_12: u64 = 12_000_000_000;

    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(36000000)
        .build()
        .await;
    let sender = test_cluster.get_address_0();
    let client = test_cluster.wallet.get_client().await.unwrap();
    let keystore = &test_cluster.wallet.config.keystore;

    let coin = client
        .coin_read_api()
        .get_all_coins(sender, None, Some(1))
        .await?
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("Expected 1 coin"))?;

    assert!(
        coin.balance == DEFAULT_INIT_COIN_BALANCE,
        "Coin did not match INIT_COIN_BALANCE"
    );

    let split_resp = split_coins(
        &client,
        keystore,
        sender,
        coin.object_ref(),
        &[HANEUL_100],
        None,
        None,
        None,
    )
    .await?;

    let tx_digest = split_resp.digest;
    if split_resp
        .effects
        .as_ref()
        .ok_or(anyhow!("Expected effects"))?
        .status()
        .clone()
        != HaneulExecutionStatus::Success
    {
        return Err(anyhow!("Transaction failed!: {:#?}", split_resp));
    }

    let (initial, splitted) = match split_resp
        .object_changes
        .ok_or(anyhow!("Expected object_changes"))?
        .as_slice()
    {
        [coin0, coin1] if coin0.object_id() == coin.coin_object_id => {
            (coin0.object_ref(), coin1.object_ref())
        }
        [coin0, coin1] => (coin1.object_ref(), coin0.object_ref()),
        obj_chngs => {
            println!("object_changes: {obj_chngs:#?}");
            return Err(anyhow!("Expected two items in object changes"));
        }
    };

    let splitted = Coin {
        coin_type: HANEUL_COIN_TYPE.to_string(),
        coin_object_id: splitted.0,
        version: splitted.1,
        digest: splitted.2,
        balance: HANEUL_100,
        previous_transaction: tx_digest,
    };

    let splitted_id = splitted.coin_object_id;
    let txs = make_change(&client, keystore, sender, splitted, Some(initial), HANEUL_12).await?;

    assert!(txs.len() == 1, "Should only have 1 tx");
    let new_coins = txs
        .into_iter()
        .next()
        .unwrap()
        .object_changes
        .ok_or(anyhow!("Expected object_changes"))?
        .into_iter()
        .filter(|chng| {
            if let ObjectChange::Created { .. } = chng {
                return true;
            }
            false
        })
        .collect::<Vec<_>>();
    assert!(
        new_coins.len() == (HANEUL_100 / HANEUL_12) as usize,
        "Expected {} new coins. New coins: {new_coins:#?}",
        HANEUL_100 / HANEUL_12
    );

    let mut all_coins: Vec<ObjectID> = new_coins.into_iter().map(|c| c.object_id()).collect();
    all_coins.push(splitted_id);
    let coins_with_data = client
        .read_api()
        .multi_get_object_with_options(all_coins, HaneulObjectDataOptions::full_content())
        .await?;

    let (mut twelve_count, mut four_count) = (0, 0);
    for coin in coins_with_data {
        let HaneulParsedData::MoveObject(object) = coin
            .data
            .ok_or(anyhow!("No data in coin"))?
            .content
            .ok_or(anyhow!("No coin.data.content"))?
        else {
            return Err(anyhow!("Coin should be a MoveObject"));
        };
        let HaneulMoveValue::String(balance) = object
            .fields
            .field_value("balance")
            .ok_or(anyhow!("No field coin.balance"))?
        else {
            return Err(anyhow!("Expected coin.balance to be a string"));
        };
        let b = balance.parse::<u64>()?;
        match b {
            HANEUL_12 => {
                twelve_count += 1;
            }
            4_000_000_000 => {
                four_count += 1;
            }
            b => {
                return Err(anyhow!(
                    "Did not expect anything else other than 12 or 4 HANEUL. Found {b}"
                ));
            }
        }
    }

    assert!(twelve_count == 8, "Expected 8 coins with 12 HANEUL");
    assert!(four_count == 1, "Expected 1 coin with 4 HANEUL");

    Ok(())
}
