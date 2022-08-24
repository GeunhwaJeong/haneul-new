// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::anyhow;
use clap::*;
use colored::Colorize;
use move_core_types::{language_storage::TypeTag, parser::parse_type_tag};
use move_package::BuildConfig;
use serde::Serialize;
use serde_json::json;
use haneul_framework::build_move_package_to_bytes;
use haneul_json::HaneulJsonValue;
use haneul_json_rpc_types::HaneulData;
use haneul_json_rpc_types::{
    GetObjectDataResponse, HaneulExecuteTransactionResponse, HaneulObjectInfo, HaneulParsedObject,
    HaneulTransactionResponse,
};
use haneul_json_rpc_types::{HaneulCertifiedTransaction, HaneulExecutionStatus, HaneulTransactionEffects};
use haneul_sdk::crypto::HaneulKeystore;
use haneul_sdk::{ClientType, HaneulClient};
use haneul_types::haneul_serde::{Base64, Encoding};
use haneul_types::{
    base_types::{ObjectID, HaneulAddress},
    gas_coin::GasCoin,
    messages::ExecuteTransactionRequestType,
    messages::Transaction,
    object::Owner,
    HANEUL_FRAMEWORK_ADDRESS,
};
use tracing::info;

use crate::config::{Config, PersistedConfig, HaneulClientConfig};

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
        /// The gateway URL (e.g., local rpc server, devnet rpc server, etc) to be
        /// used for subsequent commands.
        #[clap(long, value_hint = ValueHint::Url)]
        gateway: Option<String>,
        /// The fullnode URL
        #[clap(long, value_hint = ValueHint::Url)]
        fullnode: Option<String>,
    },

    /// Default address used for commands when none specified
    #[clap(name = "active-address")]
    ActiveAddress,

    /// Get object info
    #[clap(name = "object")]
    Object {
        /// Object ID of the object to fetch
        #[clap(long)]
        id: ObjectID,
    },

    /// Publish Move modules
    #[clap(name = "publish")]
    Publish {
        /// Path to directory containing a Move package
        #[clap(
            long = "path",
            short = 'p',
            global = true,
            parse(from_os_str),
            default_value = "."
        )]
        package_path: PathBuf,

        /// Package build options
        #[clap(flatten)]
        build_config: BuildConfig,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for running module initializers
        #[clap(long)]
        gas_budget: u64,
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
        parse(try_from_str = parse_type_tag),
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
    /// Synchronize client state with authorities.
    #[clap(name = "sync")]
    SyncClientState {
        #[clap(long)]
        address: Option<HaneulAddress>,
    },

    /// Obtain the Addresses managed by the client.
    #[clap(name = "addresses")]
    Addresses,

    /// Generate new address and keypair, with optional keypair scheme {ed25519 | secp256k1}, default to ed25519.
    #[clap(name = "new-address")]
    NewAddress { key_scheme: Option<String> },

    /// Obtain all objects owned by the address.
    #[clap(name = "objects")]
    Objects {
        /// Address owning the objects
        #[clap(long)]
        address: Option<HaneulAddress>,
    },

    /// Obtain all gas objects owned by the address.
    #[clap(name = "gas")]
    Gas {
        /// Address owning the objects
        #[clap(long)]
        address: Option<HaneulAddress>,
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
        count: u64,
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
            } => {
                let sender = context.try_get_object_owner(&gas).await?;
                let sender = sender.unwrap_or(context.active_address()?);

                let compiled_modules = build_move_package_to_bytes(&package_path, build_config)?;
                let data = context
                    .gateway
                    .transaction_builder()
                    .publish(sender, compiled_modules, gas, gas_budget)
                    .await?;
                let signature = context.keystore.sign(&sender, &data.to_bytes())?;
                let response = context
                    .execute_transaction(Transaction::new(data, signature))
                    .await?;

                HaneulClientCommandResult::Publish(response)
            }

            HaneulClientCommands::Object { id } => {
                // Fetch the object ref
                let object_read = context.gateway.read_api().get_parsed_object(id).await?;
                HaneulClientCommandResult::Object(object_read)
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
                let (cert, effects) = call_move(
                    package, &module, &function, type_args, gas, gas_budget, args, context,
                )
                .await?;
                HaneulClientCommandResult::Call(cert, effects)
            }

            HaneulClientCommands::Transfer {
                to,
                object_id,
                gas,
                gas_budget,
            } => {
                let from = context.get_object_owner(&object_id).await?;
                let time_start = Instant::now();

                let data = context
                    .gateway
                    .transaction_builder()
                    .transfer_object(from, object_id, gas, gas_budget, to)
                    .await?;
                let signature = context.keystore.sign(&from, &data.to_bytes())?;
                let response = context
                    .execute_transaction(Transaction::new(data, signature))
                    .await?;
                let cert = response.certificate;
                let effects = response.effects;

                let time_total = time_start.elapsed().as_micros();
                if matches!(effects.status, HaneulExecutionStatus::Failure { .. }) {
                    return Err(anyhow!("Error transferring object: {:#?}", effects.status));
                }
                HaneulClientCommandResult::Transfer(time_total, cert, effects)
            }

            HaneulClientCommands::TransferHaneul {
                to,
                haneul_coin_object_id: object_id,
                gas_budget,
                amount,
            } => {
                let from = context.get_object_owner(&object_id).await?;

                let data = context
                    .gateway
                    .transaction_builder()
                    .transfer_haneul(from, object_id, gas_budget, to, amount)
                    .await?;
                let signature = context.keystore.sign(&from, &data.to_bytes())?;
                let response = context
                    .execute_transaction(Transaction::new(data, signature))
                    .await?;
                let cert = response.certificate;
                let effects = response.effects;

                if matches!(effects.status, HaneulExecutionStatus::Failure { .. }) {
                    return Err(anyhow!("Error transferring HANEUL: {:#?}", effects.status));
                }
                HaneulClientCommandResult::TransferHaneul(cert, effects)
            }

            HaneulClientCommands::Addresses => {
                HaneulClientCommandResult::Addresses(context.keystore.addresses())
            }

            HaneulClientCommands::Objects { address } => {
                let address = address.unwrap_or(context.active_address()?);
                let mut address_object = context
                    .gateway
                    .read_api()
                    .get_objects_owned_by_address(address)
                    .await?;
                let object_objects = context
                    .gateway
                    .read_api()
                    .get_objects_owned_by_object(address.into())
                    .await?;
                address_object.extend(object_objects);

                HaneulClientCommandResult::Objects(address_object)
            }

            HaneulClientCommands::SyncClientState { address } => {
                let address = address.unwrap_or(context.active_address()?);
                context
                    .gateway
                    .wallet_sync_api()
                    .sync_account_state(address)
                    .await?;
                HaneulClientCommandResult::SyncClientState
            }
            HaneulClientCommands::NewAddress { key_scheme } => {
                let (address, phrase, flag) = context.keystore.generate_new_key(key_scheme)?;
                HaneulClientCommandResult::NewAddress((address, phrase, flag))
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
                let data = if let Some(amounts) = amounts {
                    context
                        .gateway
                        .transaction_builder()
                        .split_coin(signer, coin_id, amounts, gas, gas_budget)
                        .await?
                } else {
                    if count == 0 {
                        return Err(anyhow!("Coin split count must be greater than 0"));
                    }
                    context
                        .gateway
                        .transaction_builder()
                        .split_coin_equal(signer, coin_id, count, gas, gas_budget)
                        .await?
                };
                let signature = context.keystore.sign(&signer, &data.to_bytes())?;
                let response = context
                    .execute_transaction(Transaction::new(data, signature))
                    .await?;
                HaneulClientCommandResult::SplitCoin(response)
            }
            HaneulClientCommands::MergeCoin {
                primary_coin,
                coin_to_merge,
                gas,
                gas_budget,
            } => {
                let signer = context.get_object_owner(&primary_coin).await?;
                let data = context
                    .gateway
                    .transaction_builder()
                    .merge_coins(signer, primary_coin, coin_to_merge, gas, gas_budget)
                    .await?;
                let signature = context.keystore.sign(&signer, &data.to_bytes())?;
                let response = context
                    .execute_transaction(Transaction::new(data, signature))
                    .await?;

                HaneulClientCommandResult::MergeCoin(response)
            }
            HaneulClientCommands::Switch {
                address,
                gateway,
                fullnode,
            } => {
                if let Some(addr) = address {
                    if !context.keystore.addresses().contains(&addr) {
                        return Err(anyhow!("Address {} not managed by wallet", addr));
                    }
                    context.config.active_address = Some(addr);
                    context.config.save()?;
                }

                if let Some(gateway) = &gateway {
                    // TODO: handle embedded gateway
                    context.config.gateway = ClientType::RPC(gateway.clone(), None);
                    context.config.save()?;
                }

                if let Some(fullnode) = &fullnode {
                    context.config.fullnode = Some(ClientType::RPC(fullnode.clone(), None));
                    context.config.save()?;
                }

                if Option::is_none(&address)
                    && Option::is_none(&gateway)
                    && Option::is_none(&fullnode)
                {
                    return Err(anyhow!(
                        "No address or gateway specified. Please Specify one."
                    ));
                }

                HaneulClientCommandResult::Switch(SwitchResponse {
                    address,
                    gateway,
                    fullnode,
                })
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
                let (_, effects) = call_move(
                    ObjectID::from(HANEUL_FRAMEWORK_ADDRESS),
                    "devnet_nft",
                    "mint",
                    vec![],
                    gas,
                    gas_budget.unwrap_or(3000),
                    args,
                    context,
                )
                .await?;
                let nft_id = effects
                    .created
                    .first()
                    .ok_or_else(|| anyhow!("Failed to create NFT"))?
                    .reference
                    .object_id;
                let object_read = context.gateway.read_api().get_parsed_object(nft_id).await?;
                HaneulClientCommandResult::CreateExampleNFT(object_read)
            }
        });
        ret
    }
}

pub struct WalletContext {
    pub config: PersistedConfig<HaneulClientConfig>,
    pub keystore: HaneulKeystore,
    pub gateway: HaneulClient,
    pub fullnode: Option<HaneulClient>,
}

impl WalletContext {
    pub async fn new(config_path: &Path) -> Result<Self, anyhow::Error> {
        let config: HaneulClientConfig = PersistedConfig::read(config_path).map_err(|err| {
            err.context(format!(
                "Cannot open wallet config file at {:?}",
                config_path
            ))
        })?;
        let config = config.persisted(config_path);
        let keystore = config.keystore.init()?;
        let client = config.gateway.init().await?;
        let fullnode_client = match &config.fullnode {
            Some(client) => Some(client.init().await?),
            None => None,
        };
        let context = Self {
            config,
            keystore,
            gateway: client,
            fullnode: fullnode_client,
        };
        Ok(context)
    }
    pub fn active_address(&mut self) -> Result<HaneulAddress, anyhow::Error> {
        if self.keystore.addresses().is_empty() {
            return Err(anyhow!(
                "No managed addresses. Create new address with `new-address` command."
            ));
        }

        // Ok to unwrap because we checked that config addresses not empty
        // Set it if not exists
        self.config.active_address = Some(
            self.config
                .active_address
                .unwrap_or(*self.keystore.addresses().get(0).unwrap()),
        );

        Ok(self.config.active_address.unwrap())
    }

    /// Get all the gas objects (and conveniently, gas amounts) for the address
    pub async fn gas_objects(
        &self,
        address: HaneulAddress,
    ) -> Result<Vec<(u64, HaneulParsedObject, HaneulObjectInfo)>, anyhow::Error> {
        let object_refs = self
            .gateway
            .read_api()
            .get_objects_owned_by_address(address)
            .await?;

        // TODO: We should ideally fetch the objects from local cache
        let mut values_objects = Vec::new();
        for oref in object_refs {
            let response = self
                .gateway
                .read_api()
                .get_parsed_object(oref.object_id)
                .await?;
            match response {
                GetObjectDataResponse::Exists(o) => {
                    if matches!( o.data.type_(), Some(v)  if *v == GasCoin::type_().to_string()) {
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
        let object = self
            .gateway
            .read_api()
            .get_object(*id)
            .await?
            .into_object()?;
        Ok(object.owner.get_owner_address()?)
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
    ) -> Result<(u64, HaneulParsedObject), anyhow::Error> {
        for o in self.gas_objects(address).await.unwrap() {
            if o.0 >= budget && !forbidden_gas_objects.contains(&o.1.id()) {
                return Ok((o.0, o.1));
            }
        }
        Err(anyhow!(
            "No non-argument gas objects found with value >= budget {budget}"
        ))
    }

    /// A backward-compatible migration of transaction execution from gateway to fullnode
    async fn execute_transaction(&self, tx: Transaction) -> anyhow::Result<HaneulTransactionResponse> {
        let tx_digest = *tx.digest();
        match &self.fullnode {
            None => self.gateway.quorum_driver().execute_transaction(tx).await,
            Some(client) => {
                let result = client
                    .quorum_driver()
                    .execute_transaction_by_fullnode(
                        tx,
                        ExecuteTransactionRequestType::WaitForEffectsCert,
                    )
                    .await;
                match result {
                    Ok(HaneulExecuteTransactionResponse::EffectsCert {
                        certificate,
                        effects,
                    }) => Ok(HaneulTransactionResponse {
                        certificate,
                        effects: effects.effects,
                        timestamp_ms: None,
                        parsed_data: None,
                    }),
                    Err(err) => Err(anyhow!(
                        "Failed to execute transaction {tx_digest:?} with error {err:?}"
                    )),
                    other => Err(anyhow!(
                        "Expect HaneulExecuteTransactionResponse::EffectsCert but got {other:?}"
                    )),
                }
            }
        }
    }
}

impl Display for HaneulClientCommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            HaneulClientCommandResult::Publish(response) => {
                write!(
                    writer,
                    "{}",
                    write_cert_and_effects(&response.certificate, &response.effects)?
                )?;
                if let Some(parsed_resp) = &response.parsed_data {
                    writeln!(writer, "{}", parsed_resp)?;
                }
            }
            HaneulClientCommandResult::Object(object_read) => {
                let object = unwrap_err_to_string(|| Ok(object_read.object()?));
                writeln!(writer, "{}", object)?;
            }
            HaneulClientCommandResult::Call(cert, effects) => {
                write!(writer, "{}", write_cert_and_effects(cert, effects)?)?;
            }
            HaneulClientCommandResult::Transfer(time_elapsed, cert, effects) => {
                writeln!(writer, "Transfer confirmed after {} us", time_elapsed)?;
                write!(writer, "{}", write_cert_and_effects(cert, effects)?)?;
            }
            HaneulClientCommandResult::TransferHaneul(cert, effects) => {
                write!(writer, "{}", write_cert_and_effects(cert, effects)?)?;
            }
            HaneulClientCommandResult::Addresses(addresses) => {
                writeln!(writer, "Showing {} results.", addresses.len())?;
                for address in addresses {
                    writeln!(writer, "{}", address)?;
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
                        Owner::Shared => "Shared",
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
            HaneulClientCommandResult::SyncClientState => {
                writeln!(writer, "Client state sync complete.")?;
            }
            HaneulClientCommandResult::NewAddress((address, recovery_phrase, flag)) => {
                writeln!(
                    writer,
                    "Created new keypair for address with flag {flag}: [{address}]"
                )?;
                writeln!(writer, "Secret Recovery Phrase : [{recovery_phrase}]")?;
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
                write!(
                    writer,
                    "{}",
                    write_cert_and_effects(&response.certificate, &response.effects)?
                )?;
                if let Some(parsed_resp) = &response.parsed_data {
                    writeln!(writer, "{}", parsed_resp)?;
                }
            }
            HaneulClientCommandResult::MergeCoin(response) => {
                write!(
                    writer,
                    "{}",
                    write_cert_and_effects(&response.certificate, &response.effects)?
                )?;
                if let Some(parsed_resp) = &response.parsed_data {
                    writeln!(writer, "{}", parsed_resp)?;
                }
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
) -> Result<(HaneulCertifiedTransaction, HaneulTransactionEffects), anyhow::Error> {
    let gas_owner = context.try_get_object_owner(&gas).await?;
    let sender = gas_owner.unwrap_or(context.active_address()?);

    let data = context
        .gateway
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
    let signature = context.keystore.sign(&sender, &data.to_bytes())?;
    let transaction = Transaction::new(data, signature);

    let response = context.execute_transaction(transaction).await?;
    let cert = response.certificate;
    let effects = response.effects;

    if matches!(effects.status, HaneulExecutionStatus::Failure { .. }) {
        return Err(anyhow!("Error calling module: {:#?}", effects.status));
    }
    Ok((cert, effects))
}

fn unwrap_or<'a>(val: &'a Option<String>, default: &'a str) -> &'a str {
    match val {
        Some(v) => v,
        None => default,
    }
}

fn write_cert_and_effects(
    cert: &HaneulCertifiedTransaction,
    effects: &HaneulTransactionEffects,
) -> Result<String, fmt::Error> {
    let mut writer = String::new();
    writeln!(writer, "{}", "----- Certificate ----".bold())?;
    write!(writer, "{}", cert)?;
    writeln!(writer, "{}", "----- Transaction Effects ----".bold())?;
    write!(writer, "{}", effects)?;
    Ok(writer)
}

impl Debug for HaneulClientCommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = unwrap_err_to_string(|| match self {
            HaneulClientCommandResult::Object(object_read) => {
                let object = object_read.object()?;
                Ok(serde_json::to_string_pretty(&object)?)
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
    Object(GetObjectDataResponse),
    Call(HaneulCertifiedTransaction, HaneulTransactionEffects),
    Transfer(
        // Skipping serialisation for elapsed time.
        #[serde(skip)] u128,
        HaneulCertifiedTransaction,
        HaneulTransactionEffects,
    ),
    TransferHaneul(HaneulCertifiedTransaction, HaneulTransactionEffects),
    Addresses(Vec<HaneulAddress>),
    Objects(Vec<HaneulObjectInfo>),
    SyncClientState,
    NewAddress((HaneulAddress, String, u8)),
    Gas(Vec<GasCoin>),
    SplitCoin(HaneulTransactionResponse),
    MergeCoin(HaneulTransactionResponse),
    Switch(SwitchResponse),
    ActiveAddress(Option<HaneulAddress>),
    CreateExampleNFT(GetObjectDataResponse),
}

#[derive(Serialize, Clone, Debug)]
pub struct SwitchResponse {
    /// Active address
    pub address: Option<HaneulAddress>,
    pub gateway: Option<String>,
    pub fullnode: Option<String>,
}

impl Display for SwitchResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        if let Some(addr) = self.address {
            writeln!(writer, "Active address switched to {}", addr)?;
        }
        if let Some(gateway) = &self.gateway {
            writeln!(writer, "Active gateway switched to {}", gateway)?;
        }
        if let Some(fullnode) = &self.fullnode {
            writeln!(writer, "Active fullnode switched to {}", fullnode)?;
        }
        write!(f, "{}", writer)
    }
}
