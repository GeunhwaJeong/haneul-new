// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use std::sync::Arc;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use crate::config::{Config, PersistedConfig, HaneulClientConfig, HaneulEnv};
use anyhow::{anyhow, ensure};
use bip32::DerivationPath;
use clap::*;
use colored::Colorize;
use fastcrypto::{
    encoding::{Base64, Encoding},
    traits::ToFromBytes,
};
use move_core_types::language_storage::TypeTag;
use move_package::BuildConfig as MoveBuildConfig;
use prettytable::Table;
use prettytable::{row, table};
use serde::Serialize;
use serde_json::{json, Value};
use haneul_framework::build_move_package;
use haneul_move::build::resolve_lock_file_path;
use haneul_source_validation::{BytecodeSourceVerifier, SourceMode};
use haneul_types::error::HaneulError;

use haneul_framework_build::compiled_package::{
    build_from_resolution_graph, ensure_published_dependencies, BuildConfig,
};
use haneul_json::HaneulJsonValue;
use haneul_json_rpc_types::{
    DynamicFieldPage, HaneulObjectData, HaneulObjectInfo, HaneulObjectResponse, HaneulRawData,
    HaneulTransactionEffectsAPI, HaneulTransactionResponse,
};
use haneul_json_rpc_types::{HaneulExecutionStatus, HaneulObjectDataOptions};
use haneul_keys::keystore::AccountKeystore;
use haneul_sdk::HaneulClient;
use haneul_types::crypto::SignatureScheme;
use haneul_types::dynamic_field::DynamicFieldType;
use haneul_types::intent::Intent;
use haneul_types::signature::GenericSignature;
use haneul_types::{
    base_types::{ObjectID, ObjectRef, HaneulAddress},
    gas_coin::GasCoin,
    messages::{Transaction, VerifiedTransaction},
    object::Owner,
    parse_haneul_type_tag, HANEUL_FRAMEWORK_ADDRESS,
};
use tokio::sync::RwLock;
use tracing::{info, warn};

pub const EXAMPLE_NFT_NAME: &str = "Example NFT";
pub const EXAMPLE_NFT_DESCRIPTION: &str = "An NFT created by the Haneul Command Line Tool";
pub const EXAMPLE_NFT_URL: &str =
    "ipfs://bafkreibngqhl3gaa7daob4i2vccziay2jjlp435cf66vhono7nrvww53ty";

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum HaneulClientCommands {
    /// Switch active address and network(e.g., devnet, local rpc server)
    #[clap(name = "switch")]
    Switch {
        /// An Haneul address to be used as the active address for subsequent
        /// commands.
        #[clap(long)]
        address: Option<HaneulAddress>,
        /// The RPC server URL (e.g., local rpc server, devnet rpc server, etc) to be
        /// used for subsequent commands.
        #[clap(long)]
        env: Option<String>,
    },
    /// Add new Haneul environment.
    #[clap(name = "new-env")]
    NewEnv {
        #[clap(long)]
        alias: String,
        #[clap(long, value_hint = ValueHint::Url)]
        rpc: String,
        #[clap(long, value_hint = ValueHint::Url)]
        ws: Option<String>,
    },
    /// List all Haneul environments
    Envs,

    /// Default address used for commands when none specified
    #[clap(name = "active-address")]
    ActiveAddress,

    /// Default environment used for commands when none specified
    #[clap(name = "active-env")]
    ActiveEnv,

    /// Get object info
    #[clap(name = "object")]
    Object {
        /// Object ID of the object to fetch
        #[clap(name = "object_id")]
        id: ObjectID,

        /// Return the bcs serialized version of the object
        #[clap(long)]
        bcs: bool,
    },

    /// Publish Move modules
    #[clap(name = "publish")]
    Publish {
        /// Path to directory containing a Move package
        #[clap(
            name = "package_path",
            global = true,
            parse(from_os_str),
            default_value = "."
        )]
        package_path: PathBuf,

        /// Package build options
        #[clap(flatten)]
        build_config: MoveBuildConfig,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for running module initializers
        #[clap(long)]
        gas_budget: u64,

        /// Publish the package without checking whether compiling dependencies from source results
        /// in bytecode matching the dependencies found on-chain.
        #[clap(long)]
        skip_dependency_verification: bool,

        /// Also publish transitive dependencies that have not already been published.
        #[clap(long)]
        with_unpublished_dependencies: bool,
    },

    /// Verify local Move packages against on-chain packages, and optionally their dependencies.
    #[clap(name = "verify-source")]
    VerifySource {
        /// Path to directory containing a Move package
        #[clap(
            name = "package_path",
            global = true,
            parse(from_os_str),
            default_value = "."
        )]
        package_path: PathBuf,

        /// Package build options
        #[clap(flatten)]
        build_config: MoveBuildConfig,

        /// Verify on-chain dependencies.
        #[clap(long)]
        verify_deps: bool,

        /// Don't verify source (only valid if --verify-deps is enabled).
        #[clap(long)]
        skip_source: bool,

        /// If specified, override the addresses for the package's own modules with this address.
        /// Only works for unpublished modules (whose addresses are currently 0x0).
        #[clap(long)]
        address_override: Option<ObjectID>,
    },

    /// Call Move function
    #[clap(name = "call")]
    Call {
        /// Object ID of the package, which contains the module
        #[clap(long)]
        package: ObjectID,
        /// The name of the module in the package
        #[clap(long)]
        module: String,
        /// Function name in module
        #[clap(long)]
        function: String,
        /// Function name in module
        #[clap(
        long,
        parse(try_from_str = parse_haneul_type_tag),
        multiple_occurrences = false,
        multiple_values = true
        )]
        type_args: Vec<TypeTag>,
        /// Simplified ordered args like in the function syntax
        /// ObjectIDs, Addresses must be hex strings
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        args: Vec<HaneulJsonValue>,
        /// ID of the gas object for gas payment, in 20 bytes Hex string
        #[clap(long)]
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
    },

    /// Transfer object
    #[clap(name = "transfer")]
    Transfer {
        /// Recipient address
        #[clap(long)]
        to: HaneulAddress,

        /// Object to transfer, in 20 bytes Hex string
        #[clap(long)]
        object_id: ObjectID,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for this transfer
        #[clap(long)]
        gas_budget: u64,
    },
    /// Transfer HANEUL, and pay gas with the same HANEUL coin object.
    /// If amount is specified, only the amount is transferred; otherwise the entire object
    /// is transferred.
    #[clap(name = "transfer-haneul")]
    TransferHaneul {
        /// Recipient address
        #[clap(long)]
        to: HaneulAddress,

        /// Haneul coin object to transfer, ID in 20 bytes Hex string. This is also the gas object.
        #[clap(long)]
        haneul_coin_object_id: ObjectID,

        /// Gas budget for this transfer
        #[clap(long)]
        gas_budget: u64,

        /// The amount to transfer, if not specified, the entire coin object will be transferred.
        #[clap(long)]
        amount: Option<u64>,
    },
    /// Pay coins to recipients following specified amounts, with input coins.
    /// Length of recipients must be the same as that of amounts.
    #[clap(name = "pay")]
    Pay {
        /// The input coins to be used for pay recipients, following the specified amounts.
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        input_coins: Vec<ObjectID>,

        /// The recipient addresses, must be of same length as amounts
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        recipients: Vec<HaneulAddress>,

        /// The amounts to be paid, following the order of recipients.
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        amounts: Vec<u64>,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for this transaction
        #[clap(long)]
        gas_budget: u64,
    },

    /// Pay HANEUL coins to recipients following following specified amounts, with input coins.
    /// Length of recipients must be the same as that of amounts.
    /// The input coins also include the coin for gas payment, so no extra gas coin is required.
    #[clap(name = "pay_haneul")]
    PayHaneul {
        /// The input coins to be used for pay recipients, including the gas coin.
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        input_coins: Vec<ObjectID>,

        /// The recipient addresses, must be of same length as amounts.
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        recipients: Vec<HaneulAddress>,

        /// The amounts to be paid, following the order of recipients.
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        amounts: Vec<u64>,

        /// Gas budget for this transaction
        #[clap(long)]
        gas_budget: u64,
    },

    /// Pay all residual HANEUL coins to the recipient with input coins, after deducting the gas cost.
    /// The input coins also include the coin for gas payment, so no extra gas coin is required.
    #[clap(name = "pay_all_haneul")]
    PayAllHaneul {
        /// The input coins to be used for pay recipients, including the gas coin.
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        input_coins: Vec<ObjectID>,

        /// The recipient address.
        #[clap(long, multiple_occurrences = false)]
        recipient: HaneulAddress,

        /// Gas budget for this transaction
        #[clap(long)]
        gas_budget: u64,
    },

    /// Obtain the Addresses managed by the client.
    #[clap(name = "addresses")]
    Addresses,

    /// Generate new address and keypair with keypair scheme flag {ed25519 | secp256k1 | secp256r1}
    /// with optional derivation path, default to m/44'/8282'/0'/0'/0' for ed25519 or m/54'/8282'/0'/0/0 for secp256k1 or m/74'/8282'/0'/0/0 for secp256r1.
    #[clap(name = "new-address")]
    NewAddress {
        key_scheme: SignatureScheme,
        derivation_path: Option<DerivationPath>,
    },

    /// Obtain all objects owned by the address
    #[clap(name = "objects")]
    Objects {
        /// Address owning the objects
        /// Shows all objects owned by `haneul client active-address` if no argument is passed
        #[clap(name = "owner_address")]
        address: Option<HaneulAddress>,
    },

    /// Obtain all gas objects owned by the address.
    #[clap(name = "gas")]
    Gas {
        /// Address owning the objects
        #[clap(name = "owner_address")]
        address: Option<HaneulAddress>,
    },

    /// Query a dynamic field by its address.
    #[clap(name = "dynamic-field")]
    DynamicFieldQuery {
        ///The ID of the parent object
        #[clap(name = "object_id")]
        id: ObjectID,
        /// Optional paging cursor
        #[clap(long)]
        cursor: Option<ObjectID>,
        /// Maximum item returned per page
        #[clap(long, default_value = "50")]
        limit: usize,
    },

    /// Split a coin object into multiple coins.
    #[clap(group(ArgGroup::new("split").required(true).args(&["amounts", "count"])))]
    SplitCoin {
        /// Coin to Split, in 20 bytes Hex string
        #[clap(long)]
        coin_id: ObjectID,
        /// Specific amounts to split out from the coin
        #[clap(long, multiple_occurrences = false, multiple_values = true)]
        amounts: Option<Vec<u64>>,
        /// Count of equal-size coins to split into
        #[clap(long)]
        count: Option<u64>,
        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
    },

    /// Merge two coin objects into one coin
    MergeCoin {
        /// Coin to merge into, in 20 bytes Hex string
        #[clap(long)]
        primary_coin: ObjectID,
        /// Coin to be merged, in 20 bytes Hex string
        #[clap(long)]
        coin_to_merge: ObjectID,
        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
    },

    /// Create an example NFT
    #[clap(name = "create-example-nft")]
    CreateExampleNFT {
        /// Name of the NFT
        #[clap(long)]
        name: Option<String>,

        /// Description of the NFT
        #[clap(long)]
        description: Option<String>,

        /// Display url(e.g., an image url) of the NFT
        #[clap(long)]
        url: Option<String>,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for this transfer
        #[clap(long)]
        gas_budget: Option<u64>,
    },

    /// Serialize a transfer that can be signed. This is useful when user prefers to take the data to sign elsewhere.
    #[clap(name = "serialize-transfer-haneul")]
    SerializeTransferHaneul {
        /// Recipient address
        #[clap(long)]
        to: HaneulAddress,

        /// Haneul coin object to transfer, ID in 20 bytes Hex string. This is also the gas object.
        #[clap(long)]
        haneul_coin_object_id: ObjectID,

        /// Gas budget for this transfer
        #[clap(long)]
        gas_budget: u64,

        /// The amount to transfer, if not specified, the entire coin object will be transferred.
        #[clap(long)]
        amount: Option<u64>,
    },

    /// Execute a Signed Transaction. This is useful when the user prefers to sign elsewhere and use this command to execute.
    ExecuteSignedTx {
        /// BCS serialized transaction data bytes without its type tag, as base-64 encoded string.
        #[clap(long)]
        tx_bytes: String,

        /// A list of Base64 encoded signatures `flag || signature || pubkey`.
        #[clap(long)]
        signatures: Vec<String>,
    },
}

impl HaneulClientCommands {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<HaneulClientCommandResult, anyhow::Error> {
        let ret = Ok(match self {
            HaneulClientCommands::Publish {
                package_path,
                gas,
                build_config,
                gas_budget,
                skip_dependency_verification,
                with_unpublished_dependencies,
            } => {
                let sender = context.try_get_object_owner(&gas).await?;
                let sender = sender.unwrap_or(context.active_address()?);

                let build_config =
                    resolve_lock_file_path(build_config, Some(package_path.clone()))?;

                let resolution_graph = build_config
                    .resolution_graph_for_package(&package_path, &mut std::io::stderr())
                    .map_err(|err| HaneulError::ModuleBuildFailure {
                        error: format!("{:?}", err),
                    })?;

                if !with_unpublished_dependencies {
                    ensure_published_dependencies(&resolution_graph)?;
                };

                let compiled_package = build_from_resolution_graph(
                    package_path,
                    resolution_graph,
                    /* run_bytecode_verifier */ true,
                    /* print_diags_to_stderr */ true,
                )?;

                if !compiled_package.is_framework() {
                    if let Some(already_published) = compiled_package.published_root_module() {
                        return Err(HaneulError::ModulePublishFailure {
                            error: format!(
                                "Modules must all have 0x0 as their addresses. \
                                 Violated by module {:?}",
                                already_published.self_id(),
                            ),
                        }
                        .into());
                    }
                }

                let client = context.get_client().await?;
                let compiled_modules =
                    compiled_package.get_package_bytes(with_unpublished_dependencies);

                if !skip_dependency_verification {
                    BytecodeSourceVerifier::new(client.read_api(), false)
                        .verify_package_deps(&compiled_package.package)
                        .await?;
                    eprintln!(
                        "{}",
                        "Successfully verified dependencies on-chain against source."
                            .bold()
                            .green(),
                    );
                } else {
                    eprintln!("{}", "Skipping dependency verification".bold().yellow());
                }

                let data = client
                    .transaction_builder()
                    .publish(sender, compiled_modules, gas, gas_budget)
                    .await?;
                let signature =
                    context
                        .config
                        .keystore
                        .sign_secure(&sender, &data, Intent::default())?;
                let response = context
                    .execute_transaction(
                        Transaction::from_data(data, Intent::default(), vec![signature])
                            .verify()?,
                    )
                    .await?;

                HaneulClientCommandResult::Publish(response)
            }

            HaneulClientCommands::Object { id, bcs } => {
                // Fetch the object ref
                let client = context.get_client().await?;
                if !bcs {
                    let object_read = client
                        .read_api()
                        .get_object_with_options(id, HaneulObjectDataOptions::full_content())
                        .await?;
                    HaneulClientCommandResult::Object(object_read)
                } else {
                    let raw_object_read = client
                        .read_api()
                        .get_object_with_options(id, HaneulObjectDataOptions::bcs_lossless())
                        .await?;
                    HaneulClientCommandResult::RawObject(raw_object_read)
                }
            }

            HaneulClientCommands::DynamicFieldQuery { id, cursor, limit } => {
                let client = context.get_client().await?;
                let df_read = client
                    .read_api()
                    .get_dynamic_fields(id, cursor, Some(limit))
                    .await?;
                HaneulClientCommandResult::DynamicFieldQuery(df_read)
            }

            HaneulClientCommands::Call {
                package,
                module,
                function,
                type_args,
                gas,
                gas_budget,
                args,
            } => {
                let response = call_move(
                    package, &module, &function, type_args, gas, gas_budget, args, context,
                )
                .await?;
                HaneulClientCommandResult::Call(response)
            }

            HaneulClientCommands::Transfer {
                to,
                object_id,
                gas,
                gas_budget,
            } => {
                let from = context.get_object_owner(&object_id).await?;
                let time_start = Instant::now();

                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .transfer_object(from, object_id, gas, gas_budget, to)
                    .await?;
                let signature =
                    context
                        .config
                        .keystore
                        .sign_secure(&from, &data, Intent::default())?;
                let response = context
                    .execute_transaction(
                        Transaction::from_data(data, Intent::default(), vec![signature])
                            .verify()?,
                    )
                    .await?;
                let effects = response.effects.as_ref().ok_or_else(|| {
                    anyhow!("Effects from HaneulTransactionResult should not be empty")
                })?;
                let time_total = time_start.elapsed().as_micros();
                if matches!(effects.status(), HaneulExecutionStatus::Failure { .. }) {
                    return Err(anyhow!(
                        "Error transferring object: {:#?}",
                        effects.status()
                    ));
                }
                HaneulClientCommandResult::Transfer(time_total, response)
            }

            HaneulClientCommands::TransferHaneul {
                to,
                haneul_coin_object_id: object_id,
                gas_budget,
                amount,
            } => {
                let from = context.get_object_owner(&object_id).await?;

                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .transfer_haneul(from, object_id, gas_budget, to, amount)
                    .await?;
                let signature =
                    context
                        .config
                        .keystore
                        .sign_secure(&from, &data, Intent::default())?;
                let response = context
                    .execute_transaction(
                        Transaction::from_data(data, Intent::default(), vec![signature])
                            .verify()?,
                    )
                    .await?;
                let effects = response.effects.as_ref().ok_or_else(|| {
                    anyhow!("Effects from HaneulTransactionResult should not be empty")
                })?;
                if matches!(effects.status(), HaneulExecutionStatus::Failure { .. }) {
                    return Err(anyhow!("Error transferring HANEUL: {:#?}", effects.status()));
                }
                HaneulClientCommandResult::TransferHaneul(response)
            }

            HaneulClientCommands::Pay {
                input_coins,
                recipients,
                amounts,
                gas,
                gas_budget,
            } => {
                ensure!(
                    !input_coins.is_empty(),
                    "Pay transaction requires a non-empty list of input coins"
                );
                ensure!(
                    !recipients.is_empty(),
                    "Pay transaction requires a non-empty list of recipient addresses"
                );
                ensure!(
                    recipients.len() == amounts.len(),
                    format!(
                        "Found {:?} recipient addresses, but {:?} recipient amounts",
                        recipients.len(),
                        amounts.len()
                    ),
                );
                let from = context.get_object_owner(&input_coins[0]).await?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .pay(from, input_coins, recipients, amounts, gas, gas_budget)
                    .await?;
                let signature =
                    context
                        .config
                        .keystore
                        .sign_secure(&from, &data, Intent::default())?;
                let response = context
                    .execute_transaction(
                        Transaction::from_data(data, Intent::default(), vec![signature])
                            .verify()?,
                    )
                    .await?;
                let effects = response.effects.as_ref().ok_or_else(|| {
                    anyhow!("Effects from HaneulTransactionResult should not be empty")
                })?;
                if matches!(effects.status(), HaneulExecutionStatus::Failure { .. }) {
                    return Err(anyhow!(
                        "Error executing Pay transaction: {:#?}",
                        effects.status()
                    ));
                }
                HaneulClientCommandResult::Pay(response)
            }

            HaneulClientCommands::PayHaneul {
                input_coins,
                recipients,
                amounts,
                gas_budget,
            } => {
                ensure!(
                    !input_coins.is_empty(),
                    "PayHaneul transaction requires a non-empty list of input coins"
                );
                ensure!(
                    !recipients.is_empty(),
                    "PayHaneul transaction requires a non-empty list of recipient addresses"
                );
                ensure!(
                    recipients.len() == amounts.len(),
                    format!(
                        "Found {:?} recipient addresses, but {:?} recipient amounts",
                        recipients.len(),
                        amounts.len()
                    ),
                );
                let signer = context.get_object_owner(&input_coins[0]).await?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .pay_haneul(signer, input_coins, recipients, amounts, gas_budget)
                    .await?;
                let signature =
                    context
                        .config
                        .keystore
                        .sign_secure(&signer, &data, Intent::default())?;
                let response = context
                    .execute_transaction(
                        Transaction::from_data(data, Intent::default(), vec![signature])
                            .verify()?,
                    )
                    .await?;
                let effects = response.effects.as_ref().ok_or_else(|| {
                    anyhow!("Effects from HaneulTransactionResult should not be empty")
                })?;
                if matches!(effects.status(), HaneulExecutionStatus::Failure { .. }) {
                    return Err(anyhow!(
                        "Error executing PayHaneul transaction: {:#?}",
                        effects.status()
                    ));
                }
                HaneulClientCommandResult::PayHaneul(response)
            }

            HaneulClientCommands::PayAllHaneul {
                input_coins,
                recipient,
                gas_budget,
            } => {
                ensure!(
                    !input_coins.is_empty(),
                    "PayAllHaneul transaction requires a non-empty list of input coins"
                );
                let signer = context.get_object_owner(&input_coins[0]).await?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .pay_all_haneul(signer, input_coins, recipient, gas_budget)
                    .await?;

                let signature =
                    context
                        .config
                        .keystore
                        .sign_secure(&signer, &data, Intent::default())?;
                let response = context
                    .execute_transaction(
                        Transaction::from_data(data, Intent::default(), vec![signature])
                            .verify()?,
                    )
                    .await?;
                let effects = response.effects.as_ref().ok_or_else(|| {
                    anyhow!("Effects from HaneulTransactionResult should not be empty")
                })?;
                if matches!(effects.status(), HaneulExecutionStatus::Failure { .. }) {
                    return Err(anyhow!(
                        "Error executing PayAllHaneul transaction: {:#?}",
                        effects.status()
                    ));
                }
                HaneulClientCommandResult::PayAllHaneul(response)
            }

            HaneulClientCommands::Addresses => HaneulClientCommandResult::Addresses(
                context.config.keystore.addresses(),
                context.active_address().ok(),
            ),

            HaneulClientCommands::Objects { address } => {
                let address = address.unwrap_or(context.active_address()?);
                let client = context.get_client().await?;
                let address_object = client
                    .read_api()
                    .get_objects_owned_by_address(address)
                    .await?;
                HaneulClientCommandResult::Objects(address_object)
            }

            HaneulClientCommands::NewAddress {
                key_scheme,
                derivation_path,
            } => {
                let (address, phrase, scheme) = context
                    .config
                    .keystore
                    .generate_and_add_new_key(key_scheme, derivation_path)?;
                HaneulClientCommandResult::NewAddress((address, phrase, scheme))
            }
            HaneulClientCommands::Gas { address } => {
                let address = address.unwrap_or(context.active_address()?);
                let coins = context
                    .gas_objects(address)
                    .await?
                    .iter()
                    // Ok to unwrap() since `get_gas_objects` guarantees gas
                    .map(|(_val, object, _object_ref)| GasCoin::try_from(object).unwrap())
                    .collect();
                HaneulClientCommandResult::Gas(coins)
            }
            HaneulClientCommands::SplitCoin {
                coin_id,
                amounts,
                count,
                gas,
                gas_budget,
            } => {
                let signer = context.get_object_owner(&coin_id).await?;
                let client = context.get_client().await?;
                let data = match (amounts, count) {
                    (Some(amounts), None) => {
                        client
                            .transaction_builder()
                            .split_coin(signer, coin_id, amounts, gas, gas_budget)
                            .await?
                    }
                    (None, Some(count)) => {
                        if count == 0 {
                            return Err(anyhow!("Coin split count must be greater than 0"));
                        }
                        client
                            .transaction_builder()
                            .split_coin_equal(signer, coin_id, count, gas, gas_budget)
                            .await?
                    }
                    _ => {
                        return Err(anyhow!("Exactly one of `count` and `amounts` must be present for split-coin command."));
                    }
                };
                let signature =
                    context
                        .config
                        .keystore
                        .sign_secure(&signer, &data, Intent::default())?;
                let response = context
                    .execute_transaction(
                        Transaction::from_data(data, Intent::default(), vec![signature])
                            .verify()?,
                    )
                    .await?;
                HaneulClientCommandResult::SplitCoin(response)
            }
            HaneulClientCommands::MergeCoin {
                primary_coin,
                coin_to_merge,
                gas,
                gas_budget,
            } => {
                let client = context.get_client().await?;
                let signer = context.get_object_owner(&primary_coin).await?;
                let data = client
                    .transaction_builder()
                    .merge_coins(signer, primary_coin, coin_to_merge, gas, gas_budget)
                    .await?;
                let signature =
                    context
                        .config
                        .keystore
                        .sign_secure(&signer, &data, Intent::default())?;
                let response = context
                    .execute_transaction(
                        Transaction::from_data(data, Intent::default(), vec![signature])
                            .verify()?,
                    )
                    .await?;

                HaneulClientCommandResult::MergeCoin(response)
            }
            HaneulClientCommands::Switch { address, env } => {
                match (address, &env) {
                    (None, Some(env)) => {
                        Self::switch_env(&mut context.config, env)?;
                    }
                    (Some(addr), None) => {
                        if !context.config.keystore.addresses().contains(&addr) {
                            return Err(anyhow!("Address {} not managed by wallet", addr));
                        }
                        context.config.active_address = Some(addr);
                    }
                    _ => return Err(anyhow!("No address or env specified. Please Specify one.")),
                }
                context.config.save()?;
                HaneulClientCommandResult::Switch(SwitchResponse { address, env })
            }
            HaneulClientCommands::ActiveAddress => {
                HaneulClientCommandResult::ActiveAddress(context.active_address().ok())
            }
            HaneulClientCommands::CreateExampleNFT {
                name,
                description,
                url,
                gas,
                gas_budget,
            } => {
                let args_json = json!([
                    unwrap_or(&name, EXAMPLE_NFT_NAME),
                    unwrap_or(&description, EXAMPLE_NFT_DESCRIPTION),
                    unwrap_or(&url, EXAMPLE_NFT_URL)
                ]);
                let mut args = vec![];
                for a in args_json.as_array().unwrap() {
                    args.push(HaneulJsonValue::new(a.clone()).unwrap());
                }
                let response = call_move(
                    ObjectID::from(HANEUL_FRAMEWORK_ADDRESS),
                    "devnet_nft",
                    "mint",
                    vec![],
                    gas,
                    gas_budget.unwrap_or(100_000),
                    args,
                    context,
                )
                .await?;
                let nft_id = response
                    .effects
                    .ok_or_else(|| anyhow!("Failed to fetch transaction effects"))?
                    .created()
                    .first()
                    .ok_or_else(|| anyhow!("Failed to create NFT"))?
                    .reference
                    .object_id;
                let client = context.get_client().await?;
                let object_read = client
                    .read_api()
                    .get_object_with_options(nft_id, HaneulObjectDataOptions::full_content())
                    .await?;
                HaneulClientCommandResult::CreateExampleNFT(object_read)
            }

            HaneulClientCommands::SerializeTransferHaneul {
                to,
                haneul_coin_object_id: object_id,
                gas_budget,
                amount,
            } => {
                let from = context.get_object_owner(&object_id).await?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .transfer_haneul(from, object_id, gas_budget, to, amount)
                    .await?;
                HaneulClientCommandResult::SerializeTransferHaneul(Base64::encode(
                    bcs::to_bytes(&data).unwrap(),
                ))
            }

            HaneulClientCommands::ExecuteSignedTx {
                tx_bytes,
                signatures,
            } => {
                let data = bcs::from_bytes(
                    &Base64::try_from(tx_bytes)
                        .map_err(|e| anyhow!(e))?
                        .to_vec()
                        .map_err(|e| anyhow!(e))?,
                )?;

                let mut sigs = Vec::new();
                for sig in signatures {
                    sigs.push(
                        GenericSignature::from_bytes(
                            &Base64::try_from(sig)
                                .map_err(|e| anyhow!(e))?
                                .to_vec()
                                .map_err(|e| anyhow!(e))?,
                        )
                        .map_err(|e| anyhow!(e))?,
                    );
                }
                let verified =
                    Transaction::from_generic_sig_data(data, Intent::default(), sigs).verify()?;

                let response = context.execute_transaction(verified).await?;
                HaneulClientCommandResult::ExecuteSignedTx(response)
            }
            HaneulClientCommands::NewEnv { alias, rpc, ws } => {
                if context.config.envs.iter().any(|env| env.alias == alias) {
                    return Err(anyhow!(
                        "Environment config with name [{alias}] already exists."
                    ));
                }
                let env = HaneulEnv { alias, rpc, ws };

                // Check urls are valid and server is reachable
                env.create_rpc_client(None).await?;
                context.config.envs.push(env.clone());
                context.config.save()?;
                HaneulClientCommandResult::NewEnv(env)
            }
            HaneulClientCommands::ActiveEnv => {
                HaneulClientCommandResult::ActiveEnv(context.config.active_env.clone())
            }
            HaneulClientCommands::Envs => HaneulClientCommandResult::Envs(
                context.config.envs.clone(),
                context.config.active_env.clone(),
            ),
            HaneulClientCommands::VerifySource {
                package_path,
                build_config,
                verify_deps,
                skip_source,
                address_override,
            } => {
                if skip_source && !verify_deps {
                    return Err(anyhow!(
                        "Source skipped and not verifying deps: Nothing to verify."
                    ));
                }

                let build_config =
                    resolve_lock_file_path(build_config, Some(package_path.clone()))?;

                let compiled_package = build_move_package(
                    &package_path,
                    BuildConfig {
                        config: build_config,
                        run_bytecode_verifier: true,
                        print_diags_to_stderr: true,
                    },
                )?;

                let client = context.get_client().await?;

                BytecodeSourceVerifier::new(client.read_api(), false)
                    .verify_package(
                        &compiled_package.package,
                        verify_deps,
                        match (skip_source, address_override) {
                            (true, _) => SourceMode::Skip,
                            (false, None) => SourceMode::Verify,
                            (false, Some(addr)) => SourceMode::VerifyAt(addr.into()),
                        },
                    )
                    .await?;

                HaneulClientCommandResult::VerifySource
            }
        });
        ret
    }

    pub fn switch_env(config: &mut HaneulClientConfig, env: &str) -> Result<(), anyhow::Error> {
        let env = Some(env.into());
        ensure!(config.get_env(&env).is_some(), "Environment config not found for [{env:?}], add new environment config using the `haneul client new-env` command.");
        config.active_env = env;
        Ok(())
    }
}

pub struct WalletContext {
    pub config: PersistedConfig<HaneulClientConfig>,
    request_timeout: Option<std::time::Duration>,
    client: Arc<RwLock<Option<HaneulClient>>>,
}

impl WalletContext {
    pub async fn new(
        config_path: &Path,
        request_timeout: Option<std::time::Duration>,
    ) -> Result<Self, anyhow::Error> {
        let config: HaneulClientConfig = PersistedConfig::read(config_path).map_err(|err| {
            err.context(format!(
                "Cannot open wallet config file at {:?}",
                config_path
            ))
        })?;

        let config = config.persisted(config_path);
        let context = Self {
            config,
            request_timeout,
            client: Default::default(),
        };
        Ok(context)
    }

    pub async fn get_client(&self) -> Result<HaneulClient, anyhow::Error> {
        let read = self.client.read().await;

        Ok(if let Some(client) = read.as_ref() {
            client.clone()
        } else {
            drop(read);
            let client = self
                .config
                .get_active_env()?
                .create_rpc_client(self.request_timeout)
                .await?;

            if let Err(e) = client.check_api_version() {
                warn!("{e}");
                println!("{}", format!("[warn] {e}").yellow().bold());
            }
            self.client.write().await.insert(client).clone()
        })
    }

    pub fn active_address(&mut self) -> Result<HaneulAddress, anyhow::Error> {
        if self.config.keystore.addresses().is_empty() {
            return Err(anyhow!(
                "No managed addresses. Create new address with `new-address` command."
            ));
        }

        // Ok to unwrap because we checked that config addresses not empty
        // Set it if not exists
        self.config.active_address = Some(
            self.config
                .active_address
                .unwrap_or(*self.config.keystore.addresses().get(0).unwrap()),
        );

        Ok(self.config.active_address.unwrap())
    }

    /// Get the latest object reference given a object id
    pub async fn get_object_ref(&self, object_id: ObjectID) -> Result<ObjectRef, anyhow::Error> {
        let client = self.get_client().await?;
        Ok(client
            .read_api()
            .get_object_with_options(object_id, HaneulObjectDataOptions::new())
            .await?
            .into_object()?
            .object_ref())
    }

    /// Get all the gas objects (and conveniently, gas amounts) for the address
    pub async fn gas_objects(
        &self,
        address: HaneulAddress,
    ) -> Result<Vec<(u64, HaneulObjectData, HaneulObjectInfo)>, anyhow::Error> {
        let client = self.get_client().await?;
        let object_refs = client
            .read_api()
            .get_objects_owned_by_address(address)
            .await?;

        // TODO: We should ideally fetch the objects from local cache
        // TODO: replace with multi-get
        let mut values_objects = Vec::new();
        for oref in object_refs {
            let response = client
                .read_api()
                .get_object_with_options(oref.object_id, HaneulObjectDataOptions::full_content())
                .await?;
            match response {
                HaneulObjectResponse::Exists(o) => {
                    if matches!( &o.type_, Some(type_)  if type_.is_gas_coin()) {
                        // Okay to unwrap() since we already checked type
                        let gas_coin = GasCoin::try_from(&o)?;
                        values_objects.push((gas_coin.value(), o, oref));
                    }
                }
                _ => continue,
            }
        }

        Ok(values_objects)
    }

    pub async fn get_object_owner(&self, id: &ObjectID) -> Result<HaneulAddress, anyhow::Error> {
        let client = self.get_client().await?;
        let object = client
            .read_api()
            .get_object_with_options(*id, HaneulObjectDataOptions::new().with_owner())
            .await?
            .into_object()?;
        Ok(object
            .owner
            .ok_or_else(|| anyhow!("Owner field is None"))?
            .get_owner_address()?)
    }

    pub async fn try_get_object_owner(
        &self,
        id: &Option<ObjectID>,
    ) -> Result<Option<HaneulAddress>, anyhow::Error> {
        if let Some(id) = id {
            Ok(Some(self.get_object_owner(id).await?))
        } else {
            Ok(None)
        }
    }

    /// Find a gas object which fits the budget
    pub async fn gas_for_owner_budget(
        &self,
        address: HaneulAddress,
        budget: u64,
        forbidden_gas_objects: BTreeSet<ObjectID>,
    ) -> Result<(u64, HaneulObjectData), anyhow::Error> {
        for o in self.gas_objects(address).await.unwrap() {
            if o.0 >= budget && !forbidden_gas_objects.contains(&o.1.object_id) {
                return Ok((o.0, o.1));
            }
        }
        Err(anyhow!(
            "No non-argument gas objects found with value >= budget {budget}"
        ))
    }

    pub async fn execute_transaction(
        &self,
        tx: VerifiedTransaction,
    ) -> anyhow::Result<HaneulTransactionResponse> {
        let client = self.get_client().await?;
        Ok(client
            .quorum_driver()
            .execute_transaction(
                tx,
                Some(haneul_types::messages::ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await?)
    }
}

impl Display for HaneulClientCommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            HaneulClientCommandResult::Publish(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::Object(object_read) => {
                let object = unwrap_err_to_string(|| Ok(object_read.object()?));
                writeln!(writer, "{}", object)?;
            }
            HaneulClientCommandResult::RawObject(raw_object_read) => {
                let raw_object = match raw_object_read.object() {
                    Ok(v) => match &v.bcs {
                        Some(HaneulRawData::MoveObject(o)) => {
                            format!("{:?}\nNumber of bytes: {}", o.bcs_bytes, o.bcs_bytes.len())
                        }
                        Some(HaneulRawData::Package(p)) => {
                            let mut temp = String::new();
                            let mut bcs_bytes = 0usize;
                            for m in &p.module_map {
                                temp.push_str(&format!("{:?}\n", m));
                                bcs_bytes += m.1.len()
                            }
                            format!("{}Number of bytes: {}", temp, bcs_bytes)
                        }
                        None => "Bcs field is None".to_string().red().to_string(),
                    },
                    Err(err) => format!("{err}").red().to_string(),
                };
                writeln!(writer, "{}", raw_object)?;
            }
            HaneulClientCommandResult::Call(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::Transfer(time_elapsed, response) => {
                writeln!(writer, "Transfer confirmed after {} us", time_elapsed)?;
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::TransferHaneul(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::Pay(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::PayHaneul(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::PayAllHaneul(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::Addresses(addresses, active_address) => {
                writeln!(writer, "Showing {} results.", addresses.len())?;
                for address in addresses {
                    if *active_address == Some(*address) {
                        writeln!(writer, "{} <=", address)?;
                    } else {
                        writeln!(writer, "{}", address)?;
                    }
                }
            }
            HaneulClientCommandResult::Objects(object_refs) => {
                writeln!(
                    writer,
                    " {0: ^42} | {1: ^10} | {2: ^44} | {3: ^15} | {4: ^40}",
                    "Object ID", "Version", "Digest", "Owner Type", "Object Type"
                )?;
                writeln!(writer, "{}", ["-"; 165].join(""))?;
                for oref in object_refs {
                    let owner_type = match oref.owner {
                        Owner::AddressOwner(_) => "AddressOwner",
                        Owner::ObjectOwner(_) => "object_owner",
                        Owner::Shared { .. } => "Shared",
                        Owner::Immutable => "Immutable",
                    };
                    writeln!(
                        writer,
                        " {0: ^42} | {1: ^10} | {2: ^44} | {3: ^15} | {4: ^40}",
                        oref.object_id,
                        oref.version.value(),
                        Base64::encode(oref.digest),
                        owner_type,
                        oref.type_
                    )?
                }
                writeln!(writer, "Showing {} results.", object_refs.len())?;
            }
            HaneulClientCommandResult::DynamicFieldQuery(df_refs) => {
                let mut table: Table = table!([
                    "Name",
                    "Type",
                    "Object Type",
                    "Object Id",
                    "Version",
                    "Digest"
                ]);
                for df_ref in df_refs.data.iter() {
                    let df_type = match df_ref.type_ {
                        DynamicFieldType::DynamicField => "DynamicField",
                        DynamicFieldType::DynamicObject => "DynamicObject",
                    };
                    table.add_row(row![
                        df_ref.name,
                        df_type,
                        df_ref.object_type,
                        df_ref.object_id,
                        df_ref.version.value(),
                        Base64::encode(df_ref.digest)
                    ]);
                }
                write!(writer, "{table}")?;
                writeln!(writer, "Showing {} results.", df_refs.data.len())?;
                if let Some(cursor) = df_refs.next_cursor {
                    writeln!(writer, "Next cursor: {cursor}")?;
                }
            }
            HaneulClientCommandResult::SyncClientState => {
                writeln!(writer, "Client state sync complete.")?;
            }
            // Do not use writer for new address output, which may get sent to logs.
            #[allow(clippy::print_in_format_impl)]
            HaneulClientCommandResult::NewAddress((address, recovery_phrase, scheme)) => {
                println!(
                    "Created new keypair for address with scheme {:?}: [{address}]",
                    scheme
                );
                println!("Secret Recovery Phrase : [{recovery_phrase}]");
            }
            HaneulClientCommandResult::Gas(gases) => {
                // TODO: generalize formatting of CLI
                writeln!(writer, " {0: ^42} | {1: ^11}", "Object ID", "Gas Value")?;
                writeln!(
                    writer,
                    "----------------------------------------------------------------------"
                )?;
                for gas in gases {
                    writeln!(writer, " {0: ^42} | {1: ^11}", gas.id(), gas.value())?;
                }
            }
            HaneulClientCommandResult::SplitCoin(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::MergeCoin(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::Switch(response) => {
                write!(writer, "{}", response)?;
            }
            HaneulClientCommandResult::ActiveAddress(response) => {
                match response {
                    Some(r) => write!(writer, "{}", r)?,
                    None => write!(writer, "None")?,
                };
            }
            HaneulClientCommandResult::CreateExampleNFT(object_read) => {
                // TODO: display the content of the object
                let object = unwrap_err_to_string(|| Ok(object_read.object()?));
                writeln!(writer, "{}\n", "Successfully created an ExampleNFT:".bold())?;
                writeln!(writer, "{}", object)?;
            }
            HaneulClientCommandResult::ExecuteSignedTx(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            HaneulClientCommandResult::SerializeTransferHaneul(data) => {
                writeln!(writer, "Raw tx_bytes to execute: {}", data)?;
            }
            HaneulClientCommandResult::ActiveEnv(env) => {
                write!(writer, "{}", env.as_deref().unwrap_or("None"))?;
            }
            HaneulClientCommandResult::NewEnv(env) => {
                writeln!(writer, "Added new Haneul env [{}] to config.", env.alias)?;
            }
            HaneulClientCommandResult::Envs(envs, active) => {
                for env in envs {
                    write!(writer, "{} => {}", env.alias, env.rpc)?;
                    if Some(env.alias.as_str()) == active.as_deref() {
                        write!(writer, " (active)")?;
                    }
                    writeln!(writer)?;
                }
            }
            HaneulClientCommandResult::VerifySource => {
                writeln!(writer, "Source verification succeeded!")?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

pub async fn call_move(
    package: ObjectID,
    module: &str,
    function: &str,
    type_args: Vec<TypeTag>,
    gas: Option<ObjectID>,
    gas_budget: u64,
    args: Vec<HaneulJsonValue>,
    context: &mut WalletContext,
) -> Result<HaneulTransactionResponse, anyhow::Error> {
    // Convert all numeric input to String, this will allow number input from the CLI without failing HaneulJSON's checks.
    let args = args
        .into_iter()
        .map(|value| HaneulJsonValue::new(convert_number_to_string(value.to_json_value())))
        .collect::<Result<_, _>>()?;

    let gas_owner = context.try_get_object_owner(&gas).await?;
    let sender = gas_owner.unwrap_or(context.active_address()?);

    let client = context.get_client().await?;
    let data = client
        .transaction_builder()
        .move_call(
            sender,
            package,
            module,
            function,
            type_args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<Vec<_>, _>>()?,
            args,
            gas,
            gas_budget,
        )
        .await?;
    let signature = context
        .config
        .keystore
        .sign_secure(&sender, &data, Intent::default())?;
    let transaction = Transaction::from_data(data, Intent::default(), vec![signature]).verify()?;

    let response = context.execute_transaction(transaction).await?;
    let effects = response
        .effects
        .as_ref()
        .ok_or_else(|| anyhow!("Effects from HaneulTransactionResult should not be empty"))?;
    if matches!(effects.status(), HaneulExecutionStatus::Failure { .. }) {
        return Err(anyhow!("Error calling module: {:#?}", effects.status()));
    }
    Ok(response)
}

fn convert_number_to_string(value: Value) -> Value {
    match value {
        Value::Number(n) => Value::String(n.to_string()),
        Value::Array(a) => Value::Array(a.into_iter().map(convert_number_to_string).collect()),
        Value::Object(o) => Value::Object(
            o.into_iter()
                .map(|(k, v)| (k, convert_number_to_string(v)))
                .collect(),
        ),
        _ => value,
    }
}

fn unwrap_or<'a>(val: &'a Option<String>, default: &'a str) -> &'a str {
    match val {
        Some(v) => v,
        None => default,
    }
}

fn write_transaction_response(response: &HaneulTransactionResponse) -> Result<String, fmt::Error> {
    let mut writer = String::new();
    writeln!(writer, "{}", "----- Transaction Data ----".bold())?;
    if let Some(t) = &response.transaction {
        write!(writer, "{}", t)?;
    }

    writeln!(writer, "{}", "----- Transaction Effects ----".bold())?;
    if let Some(e) = &response.effects {
        write!(writer, "{}", e)?;
    }
    Ok(writer)
}

impl Debug for HaneulClientCommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = unwrap_err_to_string(|| match self {
            HaneulClientCommandResult::Object(object_read) => {
                let object = object_read.object()?;
                Ok(serde_json::to_string_pretty(&object)?)
            }
            HaneulClientCommandResult::RawObject(raw_object_read) => {
                let raw_object = raw_object_read.object()?;
                Ok(serde_json::to_string_pretty(&raw_object)?)
            }
            _ => Ok(serde_json::to_string_pretty(self)?),
        });
        write!(f, "{}", s)
    }
}

fn unwrap_err_to_string<T: Display, F: FnOnce() -> Result<T, anyhow::Error>>(func: F) -> String {
    match func() {
        Ok(s) => format!("{s}"),
        Err(err) => format!("{err}").red().to_string(),
    }
}

impl HaneulClientCommandResult {
    pub fn print(&self, pretty: bool) {
        let line = if pretty {
            format!("{self}")
        } else {
            format!("{:?}", self)
        };
        // Log line by line
        for line in line.lines() {
            // Logs write to a file on the side.  Print to stdout and also log to file, for tests to pass.
            println!("{line}");
            info!("{line}")
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum HaneulClientCommandResult {
    Publish(HaneulTransactionResponse),
    VerifySource,
    Object(HaneulObjectResponse),
    RawObject(HaneulObjectResponse),
    Call(HaneulTransactionResponse),
    Transfer(
        // Skipping serialisation for elapsed time.
        #[serde(skip)] u128,
        HaneulTransactionResponse,
    ),
    TransferHaneul(HaneulTransactionResponse),
    Pay(HaneulTransactionResponse),
    PayHaneul(HaneulTransactionResponse),
    PayAllHaneul(HaneulTransactionResponse),
    Addresses(Vec<HaneulAddress>, Option<HaneulAddress>),
    Objects(Vec<HaneulObjectInfo>),
    DynamicFieldQuery(DynamicFieldPage),
    SyncClientState,
    NewAddress((HaneulAddress, String, SignatureScheme)),
    Gas(Vec<GasCoin>),
    SplitCoin(HaneulTransactionResponse),
    MergeCoin(HaneulTransactionResponse),
    Switch(SwitchResponse),
    ActiveAddress(Option<HaneulAddress>),
    ActiveEnv(Option<String>),
    Envs(Vec<HaneulEnv>, Option<String>),
    CreateExampleNFT(HaneulObjectResponse),
    SerializeTransferHaneul(String),
    ExecuteSignedTx(HaneulTransactionResponse),
    NewEnv(HaneulEnv),
}

#[derive(Serialize, Clone, Debug)]
pub struct SwitchResponse {
    /// Active address
    pub address: Option<HaneulAddress>,
    pub env: Option<String>,
}

impl Display for SwitchResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        if let Some(addr) = self.address {
            writeln!(writer, "Active address switched to {addr}")?;
        }
        if let Some(env) = &self.env {
            writeln!(writer, "Active environment switched to [{env}]")?;
        }
        write!(f, "{}", writer)
    }
}
