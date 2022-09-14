// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{fmt::Write, fs::read_dir, path::PathBuf, str, time::Duration};

use anyhow::anyhow;
use move_package::BuildConfig;
use serde_json::json;

use haneul::client_commands::SwitchResponse;
use haneul::{
    client_commands::{HaneulClientCommandResult, HaneulClientCommands, WalletContext},
    config::HaneulClientConfig,
    haneul_commands::HaneulCommand,
};
use haneul_config::gateway::GatewayConfig;
use haneul_config::genesis_config::{AccountConfig, GenesisConfig, ObjectConfig};
use haneul_config::{
    Config, NetworkConfig, PersistedConfig, ValidatorInfo, HANEUL_CLIENT_CONFIG, HANEUL_FULLNODE_CONFIG,
    HANEUL_GATEWAY_CONFIG, HANEUL_GENESIS_FILENAME, HANEUL_KEYSTORE_FILENAME, HANEUL_NETWORK_CONFIG,
};
use haneul_json::HaneulJsonValue;
use haneul_json_rpc_types::{GetObjectDataResponse, HaneulData, HaneulParsedObject, HaneulTransactionEffects};
use haneul_sdk::crypto::KeystoreType;
use haneul_sdk::ClientType;
use haneul_types::crypto::{
    AccountKeyPair, AuthorityKeyPair, Ed25519HaneulSignature, KeypairTraits, Secp256k1HaneulSignature,
    SignatureScheme, HaneulKeyPair, HaneulSignatureInner,
};
use haneul_types::{base_types::ObjectID, crypto::get_key_pair, gas_coin::GasCoin};
use haneul_types::{haneul_framework_address_concat_string, HANEUL_FRAMEWORK_ADDRESS};
use test_utils::network::{setup_network_and_wallet, start_test_network};

const TEST_DATA_DIR: &str = "src/unit_tests/data/";

#[tokio::test]
async fn test_genesis() -> Result<(), anyhow::Error> {
    let temp_dir = tempfile::tempdir()?;
    let working_dir = temp_dir.path();
    let config = working_dir.join(HANEUL_NETWORK_CONFIG);

    // Start network without authorities
    let start = HaneulCommand::Start {
        config: Some(config),
    }
    .execute()
    .await;
    assert!(matches!(start, Err(..)));
    // Genesis
    HaneulCommand::Genesis {
        working_dir: Some(working_dir.to_path_buf()),
        write_config: None,
        force: false,
        from_config: None,
    }
    .execute()
    .await?;

    // Get all the new file names
    let files = read_dir(working_dir)?
        .flat_map(|r| r.map(|file| file.file_name().to_str().unwrap().to_owned()))
        .collect::<Vec<_>>();

    assert_eq!(10, files.len());
    assert!(files.contains(&HANEUL_CLIENT_CONFIG.to_string()));
    assert!(files.contains(&HANEUL_GATEWAY_CONFIG.to_string()));
    assert!(files.contains(&HANEUL_NETWORK_CONFIG.to_string()));
    assert!(files.contains(&HANEUL_FULLNODE_CONFIG.to_string()));
    assert!(files.contains(&HANEUL_GENESIS_FILENAME.to_string()));

    assert!(files.contains(&HANEUL_KEYSTORE_FILENAME.to_string()));

    // Check network config
    let network_conf =
        PersistedConfig::<NetworkConfig>::read(&working_dir.join(HANEUL_NETWORK_CONFIG))?;
    assert_eq!(4, network_conf.validator_configs().len());

    // Check wallet config
    let wallet_conf =
        PersistedConfig::<HaneulClientConfig>::read(&working_dir.join(HANEUL_CLIENT_CONFIG))?;

    if let ClientType::Embedded(config) = &wallet_conf.client_type {
        assert_eq!(4, config.validator_set.len());
        assert_eq!(working_dir.join("client_db"), config.db_folder_path);
    } else {
        panic!()
    }

    assert_eq!(5, wallet_conf.keystore.init().unwrap().addresses().len());

    // Genesis 2nd time should fail
    let result = HaneulCommand::Genesis {
        working_dir: Some(working_dir.to_path_buf()),
        write_config: None,
        force: false,
        from_config: None,
    }
    .execute()
    .await;
    assert!(matches!(result, Err(..)));

    temp_dir.close()?;
    Ok(())
}

#[tokio::test]
async fn test_addresses_command() -> Result<(), anyhow::Error> {
    let temp_dir = tempfile::tempdir().unwrap();
    let working_dir = temp_dir.path();
    let keypair: AuthorityKeyPair = get_key_pair().1;
    let worker_keypair: AuthorityKeyPair = get_key_pair().1;
    let network_keypair: AuthorityKeyPair = get_key_pair().1;
    let account_keypair: HaneulKeyPair = get_key_pair::<AccountKeyPair>().1.into();

    let wallet_config = HaneulClientConfig {
        keystore: KeystoreType::File(working_dir.join(HANEUL_KEYSTORE_FILENAME)),
        client_type: ClientType::Embedded(GatewayConfig {
            db_folder_path: working_dir.join("client_db"),
            validator_set: vec![ValidatorInfo {
                name: "0".into(),
                protocol_key: keypair.public().into(),
                worker_key: worker_keypair.public().into(),
                account_key: account_keypair.public(),
                network_key: network_keypair.public().into(),
                stake: 1,
                delegation: 1,
                gas_price: 1,
                network_address: haneul_config::utils::new_network_address(),
                narwhal_primary_address: haneul_config::utils::new_network_address(),
                narwhal_worker_address: haneul_config::utils::new_network_address(),
                narwhal_consensus_address: haneul_config::utils::new_network_address(),
            }],
            ..Default::default()
        }),
        active_address: None,
    };
    let wallet_conf_path = working_dir.join(HANEUL_CLIENT_CONFIG);
    let wallet_config = wallet_config.persisted(&wallet_conf_path);
    wallet_config.save().unwrap();
    let mut context = WalletContext::new(&wallet_conf_path).await.unwrap();

    // Add 3 accounts
    for _ in 0..3 {
        context
            .keystore
            .add_key(HaneulKeyPair::Ed25519HaneulKeyPair(get_key_pair().1))?;
    }

    // Print all addresses
    HaneulClientCommands::Addresses
        .execute(&mut context)
        .await
        .unwrap()
        .print(true);

    Ok(())
}

#[tokio::test]
async fn test_objects_command() -> Result<(), anyhow::Error> {
    let (_network, mut context, address) = setup_network_and_wallet().await?;

    // Print objects owned by `address`
    HaneulClientCommands::Objects {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);

    let _object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_create_example_nft_command() {
    let (_network, mut context, address) = setup_network_and_wallet().await.unwrap();

    let result = HaneulClientCommands::CreateExampleNFT {
        name: None,
        description: None,
        url: None,
        gas: None,
        gas_budget: None,
    }
    .execute(&mut context)
    .await
    .unwrap();

    match result {
        HaneulClientCommandResult::CreateExampleNFT(GetObjectDataResponse::Exists(obj)) => {
            assert_eq!(obj.owner, address);
            assert_eq!(
                obj.data.type_().unwrap(),
                haneul_framework_address_concat_string("::devnet_nft::DevNetNFT")
            );
            Ok(obj)
        }
        _ => Err(anyhow!(
            "WalletCommands::CreateExampleNFT returns wrong type"
        )),
    }
    .unwrap();
}

#[tokio::test]
async fn test_custom_genesis() -> Result<(), anyhow::Error> {
    // Create and save genesis config file
    // Create 4 authorities, 1 account with 1 gas object with custom id

    let mut config = GenesisConfig::for_local_testing();
    config.accounts.clear();
    let object_id = ObjectID::random();
    config.accounts.push(AccountConfig {
        address: None,
        gas_objects: vec![ObjectConfig {
            object_id,
            gas_value: 500,
        }],
        gas_object_ranges: None,
    });

    let network = start_test_network(Some(config)).await?;

    // Wallet config
    let mut context = WalletContext::new(&network.dir().join(HANEUL_CLIENT_CONFIG)).await?;
    assert_eq!(1, context.keystore.addresses().len());
    let address = context.keystore.addresses().first().cloned().unwrap();

    // Sync client to retrieve objects from the network.
    HaneulClientCommands::SyncClientState {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);

    // Print objects owned by `address`
    HaneulClientCommands::Objects {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);

    Ok(())
}

#[tokio::test]
async fn test_object_info_get_command() -> Result<(), anyhow::Error> {
    let (_network, mut context, address) = setup_network_and_wallet().await?;

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    // Check log output contains all object ids.
    let object_id = object_refs.first().unwrap().object_id;

    HaneulClientCommands::Object { id: object_id }
        .execute(&mut context)
        .await?
        .print(true);

    Ok(())
}

#[tokio::test]
async fn test_gas_command() -> Result<(), anyhow::Error> {
    let (_network, mut context, address) = setup_network_and_wallet().await?;
    let recipient = context.keystore.addresses().get(1).cloned().unwrap();

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    let object_id = object_refs.first().unwrap().object_id;
    let object_to_send = object_refs.get(1).unwrap().object_id;

    HaneulClientCommands::Gas {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send an object
    HaneulClientCommands::Transfer {
        to: recipient,
        object_id: object_to_send,
        gas: Some(object_id),
        gas_budget: 50000,
    }
    .execute(&mut context)
    .await?;

    // Fetch gas again
    HaneulClientCommands::Gas {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);

    Ok(())
}

#[allow(clippy::assertions_on_constants)]
#[tokio::test]
async fn test_move_call_args_linter_command() -> Result<(), anyhow::Error> {
    let (_network, mut context, address1) = setup_network_and_wallet().await?;
    let address2 = context.keystore.addresses().get(1).cloned().unwrap();

    // publish the object basics package
    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address1)
        .await?;
    let gas_obj_id = object_refs.first().unwrap().object_id;
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("move_call_args_linter");
    let build_config = BuildConfig::default();
    let resp = HaneulClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: 1000,
    }
    .execute(&mut context)
    .await?;
    let package = if let HaneulClientCommandResult::Publish(response) = resp {
        let publish_resp = response.parsed_data.unwrap().to_publish_response().unwrap();
        publish_resp.package.object_id
    } else {
        unreachable!("Invalid response");
    };

    // Sync client to retrieve objects from the network.
    HaneulClientCommands::SyncClientState {
        address: Some(address2),
    }
    .execute(&mut context)
    .await?
    .print(true);

    // Print objects owned by `address1`
    HaneulClientCommands::Objects {
        address: Some(address1),
    }
    .execute(&mut context)
    .await?
    .print(true);
    tokio::time::sleep(Duration::from_millis(2000)).await;

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address1)
        .await?;

    // Create an object for address1 using Move call

    // Certain prep work
    // Get a gas object
    let gas = object_refs.first().unwrap().object_id;
    let obj = object_refs.get(1).unwrap().object_id;

    // Create the args
    let args = vec![
        HaneulJsonValue::new(json!(123u8))?,
        HaneulJsonValue::new(json!(address1))?,
    ];

    // Test case with no gas specified
    let resp = HaneulClientCommands::Call {
        package,
        module: "object_basics".to_string(),
        function: "create".to_string(),
        type_args: vec![],
        args,
        gas: None,
        gas_budget: 1000,
    }
    .execute(&mut context)
    .await?;
    resp.print(true);

    // Get the created object
    let created_obj: ObjectID = if let HaneulClientCommandResult::Call(
        _,
        HaneulTransactionEffects {
            created: new_objs, ..
        },
    ) = resp
    {
        new_objs.first().unwrap().reference.object_id
    } else {
        // User assert since panic causes test issues
        assert!(false);
        // Use this to satisfy type checker
        ObjectID::random()
    };

    // Try a bad argument: decimal
    let args_json = json!([0.3f32, address1]);
    assert!(HaneulJsonValue::new(args_json.as_array().unwrap().get(0).unwrap().clone()).is_err());

    // Try a bad argument: too few args
    let args_json = json!([300usize]);
    let mut args = vec![];
    for a in args_json.as_array().unwrap() {
        args.push(HaneulJsonValue::new(a.clone()).unwrap());
    }

    let resp = HaneulClientCommands::Call {
        package,
        module: "object_basics".to_string(),
        function: "create".to_string(),
        type_args: vec![],
        args: args.to_vec(),
        gas: Some(gas),
        gas_budget: 1000,
    }
    .execute(&mut context)
    .await;

    assert!(resp.is_err());

    let err_string = format!("{} ", resp.err().unwrap());
    assert!(err_string.contains("Expected 2 args, found 1"));

    // Try a transfer
    // This should fail due to mismatch of object being sent
    let args = vec![
        HaneulJsonValue::new(json!(obj))?,
        HaneulJsonValue::new(json!(address2))?,
    ];

    let resp = HaneulClientCommands::Call {
        package,
        module: "object_basics".to_string(),
        function: "transfer".to_string(),
        type_args: vec![],
        args: args.to_vec(),
        gas: Some(gas),
        gas_budget: 1000,
    }
    .execute(&mut context)
    .await;

    assert!(resp.is_err());

    let err_string = format!("{} ", resp.err().unwrap());
    let framework_addr = HANEUL_FRAMEWORK_ADDRESS.to_hex_literal();
    let package_addr = package.to_hex_literal();
    assert!(err_string.contains(&format!("Expected argument of type {package_addr}::object_basics::Object, but found type {framework_addr}::coin::Coin<{framework_addr}::haneul::HANEUL>")));

    // Try a proper transfer
    let args = vec![
        HaneulJsonValue::new(json!(created_obj))?,
        HaneulJsonValue::new(json!(address2))?,
    ];

    HaneulClientCommands::Call {
        package,
        module: "object_basics".to_string(),
        function: "transfer".to_string(),
        type_args: vec![],
        args: args.to_vec(),
        gas: Some(gas),
        gas_budget: 1000,
    }
    .execute(&mut context)
    .await?;

    Ok(())
}

#[allow(clippy::assertions_on_constants)]
#[tokio::test]
async fn test_package_publish_command() -> Result<(), anyhow::Error> {
    let (_network, mut context, address) = setup_network_and_wallet().await?;

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    // Check log output contains all object ids.
    let gas_obj_id = object_refs.first().unwrap().object_id;

    // Provide path to well formed package sources
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("dummy_modules_publish");
    let build_config = BuildConfig::default();
    let resp = HaneulClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: 1000,
    }
    .execute(&mut context)
    .await?;

    // Print it out to CLI/logs
    resp.print(true);

    let (package, created_obj) = if let HaneulClientCommandResult::Publish(response) = resp {
        let publish_resp = response.parsed_data.unwrap().to_publish_response().unwrap();
        (
            publish_resp.package,
            publish_resp.created_objects[0].reference.clone(),
        )
    } else {
        unreachable!("Invalid response");
    };

    // Check the objects
    let resp = HaneulClientCommands::Object {
        id: package.object_id,
    }
    .execute(&mut context)
    .await?;
    assert!(matches!(
        resp,
        HaneulClientCommandResult::Object(GetObjectDataResponse::Exists(..))
    ));

    let resp = HaneulClientCommands::Object {
        id: created_obj.object_id,
    }
    .execute(&mut context)
    .await?;
    assert!(matches!(
        resp,
        HaneulClientCommandResult::Object(GetObjectDataResponse::Exists(..))
    ));

    Ok(())
}

#[allow(clippy::assertions_on_constants)]
#[tokio::test]
async fn test_native_transfer() -> Result<(), anyhow::Error> {
    let (_network, mut context, address) = setup_network_and_wallet().await?;
    let recipient = context.keystore.addresses().get(1).cloned().unwrap();

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    // Check log output contains all object ids.
    let gas_obj_id = object_refs.first().unwrap().object_id;
    let obj_id = object_refs.get(1).unwrap().object_id;

    let resp = HaneulClientCommands::Transfer {
        gas: Some(gas_obj_id),
        to: recipient,
        object_id: obj_id,
        gas_budget: 50000,
    }
    .execute(&mut context)
    .await?;

    // Print it out to CLI/logs
    resp.print(true);

    // Get the mutated objects
    let (mut_obj1, mut_obj2) =
        if let HaneulClientCommandResult::Transfer(_, _, HaneulTransactionEffects { mutated, .. }) = resp
        {
            (
                mutated.get(0).unwrap().reference.object_id,
                mutated.get(1).unwrap().reference.object_id,
            )
        } else {
            assert!(false);
            panic!()
        };

    // Sync both to fetch objects
    HaneulClientCommands::SyncClientState {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);
    HaneulClientCommands::SyncClientState {
        address: Some(recipient),
    }
    .execute(&mut context)
    .await?
    .print(true);

    // Check the objects
    let resp = HaneulClientCommands::Object { id: mut_obj1 }
        .execute(&mut context)
        .await?;
    let mut_obj1 =
        if let HaneulClientCommandResult::Object(GetObjectDataResponse::Exists(object)) = resp {
            object
        } else {
            // Fail this way because Panic! causes test issues
            assert!(false);
            panic!()
        };

    let resp = HaneulClientCommands::Object { id: mut_obj2 }
        .execute(&mut context)
        .await?;
    let mut_obj2 =
        if let HaneulClientCommandResult::Object(GetObjectDataResponse::Exists(object)) = resp {
            object
        } else {
            // Fail this way because Panic! causes test issues
            assert!(false);
            panic!()
        };

    let (gas, obj) = if mut_obj1.owner.get_owner_address().unwrap() == address {
        (mut_obj1, mut_obj2)
    } else {
        (mut_obj2, mut_obj1)
    };

    assert_eq!(gas.owner.get_owner_address().unwrap(), address);
    assert_eq!(obj.owner.get_owner_address().unwrap(), recipient);

    // Sync client to retrieve objects from the network.
    HaneulClientCommands::SyncClientState {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    // Check log output contains all object ids.
    let obj_id = object_refs.get(1).unwrap().object_id;

    let resp = HaneulClientCommands::Transfer {
        gas: None,
        to: recipient,
        object_id: obj_id,
        gas_budget: 50000,
    }
    .execute(&mut context)
    .await?;

    // Print it out to CLI/logs
    resp.print(true);

    // Get the mutated objects
    let (_mut_obj1, _mut_obj2) =
        if let HaneulClientCommandResult::Transfer(_, _, HaneulTransactionEffects { mutated, .. }) = resp
        {
            (
                mutated.get(0).unwrap().reference.object_id,
                mutated.get(1).unwrap().reference.object_id,
            )
        } else {
            assert!(false);
            panic!()
        };

    Ok(())
}

#[test]
// Test for issue https://github.com/GeunhwaJeong/haneul/issues/1078
fn test_bug_1078() {
    let read = HaneulClientCommandResult::Object(GetObjectDataResponse::NotExists(ObjectID::random()));
    let mut writer = String::new();
    // fmt ObjectRead should not fail.
    write!(writer, "{}", read).unwrap();
    write!(writer, "{:?}", read).unwrap();
}

#[allow(clippy::assertions_on_constants)]
#[tokio::test]
async fn test_switch_command() -> Result<(), anyhow::Error> {
    let network = start_test_network(None).await?;

    // Create Wallet context.
    let wallet_conf = network.dir().join(HANEUL_CLIENT_CONFIG);

    let mut context = WalletContext::new(&wallet_conf).await?;

    // Get the active address
    let addr1 = context.active_address()?;

    // Sync client to retrieve objects from the network.
    HaneulClientCommands::SyncClientState {
        address: Some(addr1),
    }
    .execute(&mut context)
    .await?;

    // Run a command with address omitted
    let os = HaneulClientCommands::Objects { address: None }
        .execute(&mut context)
        .await?;

    let mut cmd_objs = if let HaneulClientCommandResult::Objects(v) = os {
        v
    } else {
        panic!("Command failed")
    };

    // Check that we indeed fetched for addr1
    let mut actual_objs = context
        .client
        .read_api()
        .get_objects_owned_by_address(addr1)
        .await
        .unwrap();
    cmd_objs.sort();
    actual_objs.sort();
    assert_eq!(cmd_objs, actual_objs);

    // Switch the address
    let addr2 = context.keystore.addresses().get(1).cloned().unwrap();
    let resp = HaneulClientCommands::Switch {
        address: Some(addr2),
        rpc: None,
        ws: None,
    }
    .execute(&mut context)
    .await?;
    assert_eq!(addr2, context.active_address()?);
    assert_ne!(addr1, context.active_address()?);
    assert_eq!(
        format!("{resp}"),
        format!(
            "{}",
            HaneulClientCommandResult::Switch(SwitchResponse {
                address: Some(addr2),
                rpc: None,
                ws: None
            })
        )
    );

    // Wipe all the address info
    context.config.active_address = None;

    // Create a new address
    let os = HaneulClientCommands::NewAddress {
        key_scheme: SignatureScheme::ED25519,
    }
    .execute(&mut context)
    .await?;
    let new_addr = if let HaneulClientCommandResult::NewAddress((a, _, _)) = os {
        a
    } else {
        panic!("Command failed")
    };

    // Check that we can switch to this address
    // Switch the address
    let resp = HaneulClientCommands::Switch {
        address: Some(new_addr),
        rpc: None,
        ws: None,
    }
    .execute(&mut context)
    .await?;
    assert_eq!(new_addr, context.active_address()?);
    assert_eq!(
        format!("{resp}"),
        format!(
            "{}",
            HaneulClientCommandResult::Switch(SwitchResponse {
                address: Some(new_addr),
                rpc: None,
                ws: None
            })
        )
    );
    Ok(())
}

#[tokio::test]
async fn test_new_address_command_by_flag() -> Result<(), anyhow::Error> {
    // Create Wallet context.
    let network = start_test_network(None).await?;
    let wallet_conf = network.dir().join(HANEUL_CLIENT_CONFIG);
    let mut context = WalletContext::new(&wallet_conf).await?;

    // keypairs loaded from config are Ed25519
    assert_eq!(
        context
            .keystore
            .keys()
            .iter()
            .filter(|k| k.flag() == Ed25519HaneulSignature::SCHEME.flag())
            .count(),
        5
    );

    HaneulClientCommands::NewAddress {
        key_scheme: SignatureScheme::Secp256k1,
    }
    .execute(&mut context)
    .await?;

    // new keypair generated is Secp256k1
    assert_eq!(
        context
            .keystore
            .keys()
            .iter()
            .filter(|k| k.flag() == Secp256k1HaneulSignature::SCHEME.flag())
            .count(),
        1
    );

    Ok(())
}

#[allow(clippy::assertions_on_constants)]
#[tokio::test]
async fn test_active_address_command() -> Result<(), anyhow::Error> {
    let network = start_test_network(None).await?;

    // Create Wallet context.
    let wallet_conf = network.dir().join(HANEUL_CLIENT_CONFIG);

    let mut context = WalletContext::new(&wallet_conf).await?;

    // Get the active address
    let addr1 = context.active_address()?;

    // Sync client to retrieve objects from the network.
    HaneulClientCommands::SyncClientState {
        address: Some(addr1),
    }
    .execute(&mut context)
    .await?;

    // Run a command with address omitted
    let os = HaneulClientCommands::ActiveAddress {}
        .execute(&mut context)
        .await?;

    let a = if let HaneulClientCommandResult::ActiveAddress(Some(v)) = os {
        v
    } else {
        panic!("Command failed")
    };
    assert_eq!(a, addr1);

    let addr2 = context.keystore.addresses().get(1).cloned().unwrap();
    let resp = HaneulClientCommands::Switch {
        address: Some(addr2),
        rpc: None,
        ws: None,
    }
    .execute(&mut context)
    .await?;
    assert_eq!(
        format!("{resp}"),
        format!(
            "{}",
            HaneulClientCommandResult::Switch(SwitchResponse {
                address: Some(addr2),
                rpc: None,
                ws: None
            })
        )
    );
    Ok(())
}

fn get_gas_value(o: &HaneulParsedObject) -> u64 {
    GasCoin::try_from(o).unwrap().value()
}

async fn get_object(id: ObjectID, context: &mut WalletContext) -> Option<HaneulParsedObject> {
    let response = context
        .client
        .read_api()
        .get_parsed_object(id)
        .await
        .unwrap();
    if let GetObjectDataResponse::Exists(o) = response {
        Some(o)
    } else {
        None
    }
}

#[allow(clippy::assertions_on_constants)]
#[tokio::test]
async fn test_merge_coin() -> Result<(), anyhow::Error> {
    let (_network, mut context, address) = setup_network_and_wallet().await?;

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    // Check log output contains all object ids.
    let gas = object_refs.first().unwrap().object_id;
    let primary_coin = object_refs.get(1).unwrap().object_id;
    let coin_to_merge = object_refs.get(2).unwrap().object_id;

    let total_value = get_gas_value(&get_object(primary_coin, &mut context).await.unwrap())
        + get_gas_value(&get_object(coin_to_merge, &mut context).await.unwrap());

    // Test with gas specified
    let resp = HaneulClientCommands::MergeCoin {
        primary_coin,
        coin_to_merge,
        gas: Some(gas),
        gas_budget: 1000,
    }
    .execute(&mut context)
    .await?;

    let g = if let HaneulClientCommandResult::MergeCoin(r) = resp {
        r.parsed_data.unwrap().to_merge_coin_response().unwrap()
    } else {
        panic!("Command failed")
    };

    // Check total value is expected
    assert_eq!(get_gas_value(&g.updated_coin), total_value);

    // Check that old coin is deleted
    assert_eq!(get_object(coin_to_merge, &mut context).await, None);

    // Sync client to retrieve objects from the network.
    HaneulClientCommands::SyncClientState {
        address: Some(address),
    }
    .execute(&mut context)
    .await?;
    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    let primary_coin = object_refs.get(1).unwrap().object_id;
    let coin_to_merge = object_refs.get(2).unwrap().object_id;

    let total_value = get_gas_value(&get_object(primary_coin, &mut context).await.unwrap())
        + get_gas_value(&get_object(coin_to_merge, &mut context).await.unwrap());

    // Test with no gas specified
    let resp = HaneulClientCommands::MergeCoin {
        primary_coin,
        coin_to_merge,
        gas: None,
        gas_budget: 1000,
    }
    .execute(&mut context)
    .await?;

    let g = if let HaneulClientCommandResult::MergeCoin(r) = resp {
        r.parsed_data.unwrap().to_merge_coin_response().unwrap()
    } else {
        panic!("Command failed")
    };

    // Check total value is expected
    assert_eq!(get_gas_value(&g.updated_coin), total_value);

    // Check that old coin is deleted
    assert_eq!(get_object(coin_to_merge, &mut context).await, None);

    Ok(())
}

#[allow(clippy::assertions_on_constants)]
#[tokio::test]
async fn test_split_coin() -> Result<(), anyhow::Error> {
    let (_network, mut context, address) = setup_network_and_wallet().await?;
    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    // Check log output contains all object ids.
    let gas = object_refs.first().unwrap().object_id;
    let mut coin = object_refs.get(1).unwrap().object_id;

    let orig_value = get_gas_value(&get_object(coin, &mut context).await.unwrap());

    // Test with gas specified
    let resp = HaneulClientCommands::SplitCoin {
        gas: Some(gas),
        gas_budget: 1000,
        coin_id: coin,
        amounts: Some(vec![1000, 10]),
        count: 0,
    }
    .execute(&mut context)
    .await?;

    let g = if let HaneulClientCommandResult::SplitCoin(r) = resp {
        r.parsed_data.unwrap().to_split_coin_response().unwrap()
    } else {
        panic!("Command failed")
    };

    // Check values expected
    assert_eq!(get_gas_value(&g.updated_coin) + 1000 + 10, orig_value);
    assert!((get_gas_value(&g.new_coins[0]) == 1000) || (get_gas_value(&g.new_coins[0]) == 10));
    assert!((get_gas_value(&g.new_coins[1]) == 1000) || (get_gas_value(&g.new_coins[1]) == 10));

    HaneulClientCommands::SyncClientState {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    // Get another coin
    for c in object_refs {
        if get_gas_value(&get_object(c.object_id, &mut context).await.unwrap()) > 2000 {
            coin = c.object_id;
        }
    }
    let orig_value = get_gas_value(&get_object(coin, &mut context).await.unwrap());

    // Test split coin into equal parts
    let resp = HaneulClientCommands::SplitCoin {
        gas: None,
        gas_budget: 1000,
        coin_id: coin,
        amounts: None,
        count: 3,
    }
    .execute(&mut context)
    .await?;

    let g = if let HaneulClientCommandResult::SplitCoin(r) = resp {
        r.parsed_data.unwrap().to_split_coin_response().unwrap()
    } else {
        panic!("Command failed")
    };

    // Check values expected
    assert_eq!(
        get_gas_value(&g.updated_coin),
        orig_value / 3 + orig_value % 3
    );
    assert_eq!(get_gas_value(&g.new_coins[0]), orig_value / 3);
    assert_eq!(get_gas_value(&g.new_coins[1]), orig_value / 3);

    HaneulClientCommands::SyncClientState {
        address: Some(address),
    }
    .execute(&mut context)
    .await?
    .print(true);

    let object_refs = context
        .client
        .read_api()
        .get_objects_owned_by_address(address)
        .await?;

    // Get another coin
    for c in object_refs {
        if get_gas_value(&get_object(c.object_id, &mut context).await.unwrap()) > 2000 {
            coin = c.object_id;
        }
    }
    let orig_value = get_gas_value(&get_object(coin, &mut context).await.unwrap());

    // Test with no gas specified
    let resp = HaneulClientCommands::SplitCoin {
        gas: None,
        gas_budget: 1000,
        coin_id: coin,
        amounts: Some(vec![1000, 10]),
        count: 0,
    }
    .execute(&mut context)
    .await?;

    let g = if let HaneulClientCommandResult::SplitCoin(r) = resp {
        r.parsed_data.unwrap().to_split_coin_response().unwrap()
    } else {
        panic!("Command failed")
    };

    // Check values expected
    assert_eq!(get_gas_value(&g.updated_coin) + 1000 + 10, orig_value);
    assert!((get_gas_value(&g.new_coins[0]) == 1000) || (get_gas_value(&g.new_coins[0]) == 10));
    assert!((get_gas_value(&g.new_coins[1]) == 1000) || (get_gas_value(&g.new_coins[1]) == 10));
    Ok(())
}

#[tokio::test]
async fn test_signature_flag() -> Result<(), anyhow::Error> {
    let res = SignatureScheme::from_flag("0");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().flag(), SignatureScheme::ED25519.flag());

    let res = SignatureScheme::from_flag("1");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().flag(), SignatureScheme::Secp256k1.flag());

    let res = SignatureScheme::from_flag("2");
    assert!(res.is_err());

    let res = SignatureScheme::from_flag("something");
    assert!(res.is_err());
    Ok(())
}
