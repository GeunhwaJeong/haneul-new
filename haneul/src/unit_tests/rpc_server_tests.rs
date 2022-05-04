// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};

use jsonrpsee::{
    http_client::{HttpClient, HttpClientBuilder},
    http_server::{HttpServerBuilder, HttpServerHandle},
};
use move_core_types::identifier::Identifier;

use haneul::{
    config::{PersistedConfig, WalletConfig, HANEUL_GATEWAY_CONFIG, HANEUL_WALLET_CONFIG},
    keystore::{Keystore, HaneulKeystore},
    rpc_gateway::{
        responses::ObjectResponse, RpcGatewayClient, RpcGatewayImpl, RpcGatewayServer,
        SignedTransaction, TransactionBytes,
    },
    haneul_commands::HaneulNetwork,
};
use haneul_core::gateway_state::gateway_responses::{TransactionEffectsResponse, TransactionResponse};
use haneul_core::gateway_state::GatewayTxSeqNumber;
use haneul_core::haneul_json::HaneulJsonValue;
use haneul_framework::build_move_package_to_bytes;
use haneul_types::{
    base_types::{ObjectID, HaneulAddress, TransactionDigest},
    json_schema::Base64,
    object::ObjectRead,
    HANEUL_FRAMEWORK_ADDRESS,
};

use crate::rpc_server_tests::haneul_network::start_test_network;

mod haneul_network;

#[tokio::test]
async fn test_get_objects() -> Result<(), anyhow::Error> {
    let test_network = setup_test_network().await?;
    let http_client = test_network.http_client;
    let address = test_network.accounts.first().unwrap();

    http_client.sync_account_state(*address).await?;
    let result: ObjectResponse = http_client.get_owned_objects(*address).await?;
    let result = result
        .objects
        .into_iter()
        .map(|o| o.to_object_ref())
        .collect::<Result<Vec<_>, _>>()?;

    assert_eq!(5, result.len());
    Ok(())
}

#[tokio::test]
async fn test_transfer_coin() -> Result<(), anyhow::Error> {
    let test_network = setup_test_network().await?;
    let http_client = test_network.http_client;
    let address = test_network.accounts.first().unwrap();
    http_client.sync_account_state(*address).await?;
    let result: ObjectResponse = http_client.get_owned_objects(*address).await?;
    let objects = result
        .objects
        .into_iter()
        .map(|o| o.to_object_ref())
        .collect::<Result<Vec<_>, _>>()?;

    let tx_data: TransactionBytes = http_client
        .transfer_coin(
            *address,
            objects.first().unwrap().0,
            objects.last().unwrap().0,
            1000,
            *address,
        )
        .await?;

    let keystore = HaneulKeystore::load_or_create(&test_network.working_dir.join("wallet.key"))?;
    let signature = keystore.sign(address, &tx_data.tx_bytes)?;

    let tx_response: TransactionResponse = http_client
        .execute_transaction(SignedTransaction::new(tx_data.tx_bytes, signature))
        .await?;

    let (_cert, effect) = tx_response.to_effect_response()?;
    assert_eq!(2, effect.mutated.len());

    Ok(())
}

#[tokio::test]
async fn test_publish() -> Result<(), anyhow::Error> {
    let test_network = setup_test_network().await?;
    let http_client = test_network.http_client;
    let address = test_network.accounts.first().unwrap();
    http_client.sync_account_state(*address).await?;
    let result: ObjectResponse = http_client.get_owned_objects(*address).await?;
    let objects = result
        .objects
        .into_iter()
        .map(|o| o.to_object_ref())
        .collect::<Result<Vec<_>, _>>()?;

    let gas = objects.first().unwrap();

    let compiled_modules = build_move_package_to_bytes(
        Path::new("../haneul_programmability/examples/fungible_tokens"),
        false,
    )?
    .into_iter()
    .map(Base64)
    .collect::<Vec<_>>();

    let tx_data: TransactionBytes = http_client
        .publish(*address, compiled_modules, gas.0, 10000)
        .await?;

    let keystore = HaneulKeystore::load_or_create(&test_network.working_dir.join("wallet.key"))?;
    let signature = keystore.sign(address, &tx_data.tx_bytes)?;

    let tx_response: TransactionResponse = http_client
        .execute_transaction(SignedTransaction::new(tx_data.tx_bytes, signature))
        .await?;

    let response = tx_response.to_publish_response()?;
    assert_eq!(2, response.created_objects.len());
    Ok(())
}

#[tokio::test]
async fn test_move_call() -> Result<(), anyhow::Error> {
    let test_network = setup_test_network().await?;
    let http_client = test_network.http_client;
    let address = test_network.accounts.first().unwrap();
    http_client.sync_account_state(*address).await?;
    let result: ObjectResponse = http_client.get_owned_objects(*address).await?;
    let objects = result
        .objects
        .into_iter()
        .map(|o| o.to_object_ref())
        .collect::<Result<Vec<_>, _>>()?;

    let gas = objects.first().unwrap();

    let package_id = ObjectID::new(HANEUL_FRAMEWORK_ADDRESS.into_bytes());
    let module = Identifier::new("ObjectBasics")?;
    let function = Identifier::new("create")?;

    let json_args = vec![
        HaneulJsonValue::from_str("10000")?,
        HaneulJsonValue::from_str(&format!("\"0x{}\"", address))?,
    ];

    let tx_data: TransactionBytes = http_client
        .move_call(
            *address,
            package_id,
            module,
            function,
            vec![],
            json_args,
            gas.0,
            1000,
        )
        .await?;

    let keystore = HaneulKeystore::load_or_create(&test_network.working_dir.join("wallet.key"))?;
    let signature = keystore.sign(address, &tx_data.tx_bytes)?;

    let tx_response: TransactionResponse = http_client
        .execute_transaction(SignedTransaction::new(tx_data.tx_bytes, signature))
        .await?;

    let (_cert, effect) = tx_response.to_effect_response()?;
    assert_eq!(1, effect.created.len());
    Ok(())
}

#[tokio::test]
async fn test_get_object_info() -> Result<(), anyhow::Error> {
    let test_network = setup_test_network().await?;
    let http_client = test_network.http_client;
    let address = test_network.accounts.first().unwrap();
    http_client.sync_account_state(*address).await?;
    let result: ObjectResponse = http_client.get_owned_objects(*address).await?;
    let result = result
        .objects
        .into_iter()
        .map(|o| o.to_object_ref())
        .collect::<Result<Vec<_>, _>>()?;

    for (id, _, _) in result {
        let result: ObjectRead = http_client.get_object_info(id).await?;
        assert!(
            matches!(result, ObjectRead::Exists((obj_id,_,_), object, _) if id == obj_id && &object.owner.get_owner_address()? == address)
        );
    }
    Ok(())
}

#[tokio::test]
async fn test_get_transaction() -> Result<(), anyhow::Error> {
    let test_network = setup_test_network().await?;
    let http_client = test_network.http_client;
    let address = test_network.accounts.first().unwrap();

    http_client.sync_account_state(*address).await?;

    let result: ObjectResponse = http_client.get_owned_objects(*address).await?;
    let objects = result
        .objects
        .into_iter()
        .map(|o| o.to_object_ref())
        .collect::<Result<Vec<_>, _>>()?;

    let (gas_id, _, _) = objects.last().unwrap();

    // Make some transactions
    let mut tx_responses = Vec::new();
    for (id, _, _) in &objects[..objects.len() - 1] {
        let tx_data: TransactionBytes = http_client
            .transfer_coin(*address, *id, *gas_id, 1000, *address)
            .await?;

        let keystore = HaneulKeystore::load_or_create(&test_network.working_dir.join("wallet.key"))?;
        let signature = keystore.sign(address, &tx_data.tx_bytes)?;

        let response: TransactionResponse = http_client
            .execute_transaction(SignedTransaction::new(tx_data.tx_bytes, signature))
            .await?;

        if let TransactionResponse::EffectResponse(effects) = response {
            tx_responses.push(effects);
        }
    }
    // test get_transactions_in_range
    let tx: Vec<(GatewayTxSeqNumber, TransactionDigest)> =
        http_client.get_transactions_in_range(0, 10).await?;
    assert_eq!(4, tx.len());

    // test get_transactions_in_range with smaller range
    let tx: Vec<(GatewayTxSeqNumber, TransactionDigest)> =
        http_client.get_transactions_in_range(1, 3).await?;
    assert_eq!(2, tx.len());

    // test get_recent_transactions with smaller range
    let tx: Vec<(GatewayTxSeqNumber, TransactionDigest)> =
        http_client.get_recent_transactions(3).await?;
    assert_eq!(3, tx.len());

    // test get_recent_transactions
    let tx: Vec<(GatewayTxSeqNumber, TransactionDigest)> =
        http_client.get_recent_transactions(10).await?;
    assert_eq!(4, tx.len());

    // test get_transaction
    for (_, tx_digest) in tx {
        let response: TransactionEffectsResponse = http_client.get_transaction(tx_digest).await?;
        assert!(tx_responses.iter().any(
            |effects| effects.effects.transaction_digest == response.effects.transaction_digest
        ))
    }

    Ok(())
}

async fn setup_test_network() -> Result<TestNetwork, anyhow::Error> {
    let working_dir = tempfile::tempdir()?.path().to_path_buf();
    let _network = start_test_network(&working_dir, None, None).await?;
    let (server_addr, rpc_server_handle) =
        start_rpc_gateway(&working_dir.join(HANEUL_GATEWAY_CONFIG)).await?;
    let wallet_conf: WalletConfig = PersistedConfig::read(&working_dir.join(HANEUL_WALLET_CONFIG))?;
    let http_client = HttpClientBuilder::default().build(format!("http://{}", server_addr))?;
    Ok(TestNetwork {
        _network,
        _rpc_server: rpc_server_handle,
        accounts: wallet_conf.accounts,
        http_client,
        working_dir,
    })
}

struct TestNetwork {
    _network: HaneulNetwork,
    _rpc_server: HttpServerHandle,
    accounts: Vec<HaneulAddress>,
    http_client: HttpClient,
    working_dir: PathBuf,
}

async fn start_rpc_gateway(
    config_path: &Path,
) -> Result<(SocketAddr, HttpServerHandle), anyhow::Error> {
    let server = HttpServerBuilder::default().build("127.0.0.1:0").await?;
    let addr = server.local_addr()?;
    let handle = server.start(RpcGatewayImpl::new(config_path)?.into_rpc())?;
    Ok((addr, handle))
}
