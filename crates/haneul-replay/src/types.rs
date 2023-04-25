// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::Error as JsonRpseeError;
use move_binary_format::CompiledModule;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, StructTag};
use std::fmt::Debug;
use haneul_json_rpc_types::{HaneulEvent, HaneulTransactionBlockEffectsV1};
use haneul_protocol_config::ProtocolConfig;
use haneul_sdk::error::Error as HaneulRpcError;
use haneul_types::base_types::{ObjectID, ObjectRef, SequenceNumber, HaneulAddress, VersionNumber};
use haneul_types::digests::{ObjectDigest, TransactionDigest};
use haneul_types::error::{HaneulError, HaneulObjectResponseError, HaneulResult, UserInputError};
use haneul_types::messages::{InputObjectKind, TransactionKind};
use haneul_types::object::Object;
use thiserror::Error;
use tokio::time::Duration;

// These are very testnet specific
pub(crate) const GENESIX_TX_DIGEST: &str = "Cgww1sn7XViCPSdDcAPmVcARueWuexJ8af8zD842Ff43";
pub(crate) const SAFE_MODE_TX_1_DIGEST: &str = "AGBCaUGj4iGpGYyQvto9Bke1EwouY8LGMoTzzuPMx4nd";

// TODO: make these configurable
pub(crate) const RPC_TIMEOUT_ERR_SLEEP_RETRY_PERIOD: Duration = Duration::from_millis(10_000);
pub(crate) const RPC_TIMEOUT_ERR_NUM_RETRIES: u32 = 3;
pub(crate) const MAX_CONCURRENT_REQUESTS: usize = 1_000;

// Struct tag used in system epoch change events
pub(crate) const EPOCH_CHANGE_STRUCT_TAG: &str =
    "0x3::haneul_system_state_inner::SystemEpochInfoEvent";

#[derive(Clone, Debug)]
pub(crate) struct TxInfo {
    pub tx_digest: TransactionDigest,
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
    pub effects: HaneulTransactionBlockEffectsV1,
    pub protocol_config: ProtocolConfig,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error, Clone)]
pub enum LocalExecError {
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
        on_chain: Box<HaneulTransactionBlockEffectsV1>,
        local: Box<HaneulTransactionBlockEffectsV1>,
    },

    #[error("Genesis replay not supported digest {:#?}", digest)]
    GenesisReplayNotSupported { digest: TransactionDigest },

    #[error(
        "Fatal! No framework versions for epoch {epoch}. Make sure version tables are populated"
    )]
    FrameworkObjectVersionTableNotPopulated { epoch: u64 },

    #[error("Protocol version not found for epoch {epoch}")]
    ProtocolVersionNotFound { epoch: u64 },

    #[error("Protocol version not found for epoch {epoch}")]
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
}

impl From<HaneulObjectResponseError> for LocalExecError {
    fn from(err: HaneulObjectResponseError) -> Self {
        match err {
            HaneulObjectResponseError::NotExists { object_id } => {
                LocalExecError::ObjectNotExist { id: object_id }
            }
            HaneulObjectResponseError::Deleted {
                object_id,
                digest,
                version,
            } => LocalExecError::ObjectDeleted {
                id: object_id,
                version,
                digest,
            },
            _ => LocalExecError::HaneulObjectResponseError { err },
        }
    }
}

impl From<LocalExecError> for HaneulError {
    fn from(err: LocalExecError) -> Self {
        HaneulError::Unknown(format!("{:#?}", err))
    }
}

impl From<HaneulError> for LocalExecError {
    fn from(err: HaneulError) -> Self {
        LocalExecError::HaneulError { err }
    }
}
impl From<HaneulRpcError> for LocalExecError {
    fn from(err: HaneulRpcError) -> Self {
        match err {
            HaneulRpcError::RpcError(JsonRpseeError::RequestTimeout) => {
                LocalExecError::HaneulRpcRequestTimeout
            }
            _ => LocalExecError::HaneulRpcError {
                err: format!("{:?}", err),
            },
        }
    }
}

impl From<UserInputError> for LocalExecError {
    fn from(err: UserInputError) -> Self {
        LocalExecError::UserInputError { err }
    }
}

impl From<anyhow::Error> for LocalExecError {
    fn from(err: anyhow::Error) -> Self {
        LocalExecError::GeneralError {
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
        result: Result<Option<Vec<u8>>, LocalExecError>,
    },
    ModuleResolverGetModule {
        module_id: ModuleId,
        result: Result<Option<Vec<u8>>, LocalExecError>,
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
        result: Result<Option<CompiledModule>, LocalExecError>,
    },
}
