// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::Error as JsonRpseeError;
use move_binary_format::CompiledModule;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, StructTag};
use std::fmt::Debug;
use haneul_json_rpc_types::HaneulEvent;
use haneul_json_rpc_types::HaneulTransactionBlockEffects;
use haneul_protocol_config::ProtocolConfig;
use haneul_sdk::error::Error as HaneulRpcError;
use haneul_types::base_types::{ObjectID, ObjectRef, SequenceNumber, HaneulAddress, VersionNumber};
use haneul_types::digests::{ObjectDigest, TransactionDigest};
use haneul_types::error::{HaneulError, HaneulObjectResponseError, HaneulResult, UserInputError};
use haneul_types::object::Object;
use haneul_types::transaction::{InputObjectKind, SenderSignedData, TransactionKind};
use thiserror::Error;
use tokio::time::Duration;
use tracing::error;

use crate::config::ReplayableNetworkConfigSet;

// TODO: make these configurable
pub(crate) const RPC_TIMEOUT_ERR_SLEEP_RETRY_PERIOD: Duration = Duration::from_millis(10_000);
pub(crate) const RPC_TIMEOUT_ERR_NUM_RETRIES: u32 = 2;
pub(crate) const MAX_CONCURRENT_REQUESTS: usize = 1_000;

// Struct tag used in system epoch change events
pub(crate) const EPOCH_CHANGE_STRUCT_TAG: &str =
    "0x3::haneul_system_state_inner::SystemEpochInfoEvent";

pub(crate) const ONE_DAY_MS: u64 = 24 * 60 * 60 * 1000;

#[derive(Clone, Debug)]
pub struct OnChainTransactionInfo {
    pub tx_digest: TransactionDigest,
    pub sender_signed_data: SenderSignedData,
    pub sender: HaneulAddress,
    pub input_objects: Vec<InputObjectKind>,
    pub kind: TransactionKind,
    pub modified_at_versions: Vec<(ObjectID, SequenceNumber)>,
    pub shared_object_refs: Vec<ObjectRef>,
    pub gas: Vec<(ObjectID, SequenceNumber, ObjectDigest)>,
    pub gas_budget: u64,
    pub gas_price: u64,
    pub executed_epoch: u64,
    pub dependencies: Vec<TransactionDigest>,
    pub effects: HaneulTransactionBlockEffects,
    pub protocol_config: ProtocolConfig,
    pub epoch_start_timestamp: u64,
    pub reference_gas_price: u64,
}

#[derive(Clone, Debug, Default)]
pub struct DiagInfo {
    pub loaded_child_objects: Vec<(ObjectID, VersionNumber)>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error, Clone)]
pub enum ReplayEngineError {
    #[error("HaneulError: {:#?}", err)]
    HaneulError { err: HaneulError },

    #[error("HaneulRpcError: {:#?}", err)]
    HaneulRpcError { err: String },

    #[error("HaneulObjectResponseError: {:#?}", err)]
    HaneulObjectResponseError { err: HaneulObjectResponseError },

    #[error("UserInputError: {:#?}", err)]
    UserInputError { err: UserInputError },

    #[error("GeneralError: {:#?}", err)]
    GeneralError { err: String },

    #[error("HaneulRpcRequestTimeout")]
    HaneulRpcRequestTimeout,

    #[error("ObjectNotExist: {:#?}", id)]
    ObjectNotExist { id: ObjectID },

    #[error("ObjectVersionNotFound: {:#?} version {}", id, version)]
    ObjectVersionNotFound {
        id: ObjectID,
        version: SequenceNumber,
    },

    #[error(
        "ObjectVersionTooHigh: {:#?}, requested version {}, latest version found {}",
        id,
        asked_version,
        latest_version
    )]
    ObjectVersionTooHigh {
        id: ObjectID,
        asked_version: SequenceNumber,
        latest_version: SequenceNumber,
    },

    #[error(
        "ObjectDeleted: {:#?} at version {:#?} digest {:#?}",
        id,
        version,
        digest
    )]
    ObjectDeleted {
        id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
    },

    #[error(
        "EffectsForked: Effects for digest {} forked with diff {}",
        digest,
        diff
    )]
    EffectsForked {
        digest: TransactionDigest,
        diff: String,
        on_chain: Box<HaneulTransactionBlockEffects>,
        local: Box<HaneulTransactionBlockEffects>,
    },

    #[error("Genesis replay not supported digest {:#?}", digest)]
    GenesisReplayNotSupported { digest: TransactionDigest },

    #[error(
        "Fatal! No framework versions for protocol version {protocol_version}. Make sure version tables are populated"
    )]
    FrameworkObjectVersionTableNotPopulated { protocol_version: u64 },

    #[error("Protocol version not found for epoch {epoch}")]
    ProtocolVersionNotFound { epoch: u64 },

    #[error("Error querying system events for epoch {epoch}")]
    ErrorQueryingSystemEvents { epoch: u64 },

    #[error("Invalid epoch change transaction in events for epoch {epoch}")]
    InvalidEpochChangeTx { epoch: u64 },

    #[error("Unexpected event format {:#?}", event)]
    UnexpectedEventFormat { event: HaneulEvent },

    #[error("Unable to find event for epoch {epoch}")]
    EventNotFound { epoch: u64 },

    #[error("Unable to find checkpoints for epoch {epoch}")]
    UnableToDetermineCheckpoint { epoch: u64 },

    #[error("Unable to query system events; {}", rpc_err)]
    UnableToQuerySystemEvents { rpc_err: String },

    #[error("Internal error or cache corrupted! Object {id}{} should be in cache.", version.map(|q| format!(" version {:#?}", q)).unwrap_or_default() )]
    InternalCacheInvariantViolation {
        id: ObjectID,
        version: Option<SequenceNumber>,
    },

    #[error("Error getting dynamic fields loaded objects: {}", rpc_err)]
    UnableToGetDynamicFieldLoadedObjects { rpc_err: String },

    #[error("Unsupported epoch in replay engine: {epoch}")]
    EpochNotSupported { epoch: u64 },

    #[error("Unable to open yaml cfg file at {}: {}", path, err)]
    UnableToOpenYamlFile { path: String, err: String },

    #[error("Unable to write yaml file at {}: {}", path, err)]
    UnableToWriteYamlFile { path: String, err: String },

    #[error("Unable to convert string {} to URL {}", url, err)]
    InvalidUrl { url: String, err: String },

    #[error(
        "Unable to execute transaction with existing network configs {:#?}",
        cfgs
    )]
    UnableToExecuteWithNetworkConfigs { cfgs: ReplayableNetworkConfigSet },
}

impl From<HaneulObjectResponseError> for ReplayEngineError {
    fn from(err: HaneulObjectResponseError) -> Self {
        match err {
            HaneulObjectResponseError::NotExists { object_id } => {
                ReplayEngineError::ObjectNotExist { id: object_id }
            }
            HaneulObjectResponseError::Deleted {
                object_id,
                digest,
                version,
            } => ReplayEngineError::ObjectDeleted {
                id: object_id,
                version,
                digest,
            },
            _ => ReplayEngineError::HaneulObjectResponseError { err },
        }
    }
}

impl From<ReplayEngineError> for HaneulError {
    fn from(err: ReplayEngineError) -> Self {
        HaneulError::Unknown(format!("{:#?}", err))
    }
}

impl From<HaneulError> for ReplayEngineError {
    fn from(err: HaneulError) -> Self {
        ReplayEngineError::HaneulError { err }
    }
}
impl From<HaneulRpcError> for ReplayEngineError {
    fn from(err: HaneulRpcError) -> Self {
        match err {
            HaneulRpcError::RpcError(JsonRpseeError::RequestTimeout) => {
                ReplayEngineError::HaneulRpcRequestTimeout
            }
            _ => ReplayEngineError::HaneulRpcError {
                err: format!("{:?}", err),
            },
        }
    }
}

impl From<UserInputError> for ReplayEngineError {
    fn from(err: UserInputError) -> Self {
        ReplayEngineError::UserInputError { err }
    }
}

impl From<anyhow::Error> for ReplayEngineError {
    fn from(err: anyhow::Error) -> Self {
        ReplayEngineError::GeneralError {
            err: format!("{:#?}", err),
        }
    }
}

/// TODO: Limited set but will add more
#[derive(Debug)]
pub enum ExecutionStoreEvent {
    BackingPackageGetPackageObject {
        package_id: ObjectID,
        result: HaneulResult<Option<Object>>,
    },
    ChildObjectResolverStoreReadChildObject {
        parent: ObjectID,
        child: ObjectID,
        result: HaneulResult<Option<Object>>,
    },
    ParentSyncStoreGetLatestParentEntryRef {
        object_id: ObjectID,
        result: HaneulResult<Option<ObjectRef>>,
    },
    ResourceResolverGetResource {
        address: AccountAddress,
        typ: StructTag,
        result: HaneulResult<Option<Vec<u8>>>,
    },
    ModuleResolverGetModule {
        module_id: ModuleId,
        result: HaneulResult<Option<Vec<u8>>>,
    },
    ObjectStoreGetObject {
        object_id: ObjectID,
        result: HaneulResult<Option<Object>>,
    },
    ObjectStoreGetObjectByKey {
        object_id: ObjectID,
        version: VersionNumber,
        result: HaneulResult<Option<Object>>,
    },
    GetModuleGetModuleByModuleId {
        id: ModuleId,
        result: HaneulResult<Option<CompiledModule>>,
    },
    ReceiveObject {
        owner: ObjectID,
        receive: ObjectID,
        receive_at_version: SequenceNumber,
        result: HaneulResult<Option<Object>>,
    },
}
