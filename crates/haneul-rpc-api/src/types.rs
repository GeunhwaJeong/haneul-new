// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Chain ID of the current chain
pub const X_HANEUL_CHAIN_ID: &str = "x-haneul-chain-id";

/// Chain name of the current chain
pub const X_HANEUL_CHAIN: &str = "x-haneul-chain";

/// Current checkpoint height
pub const X_HANEUL_CHECKPOINT_HEIGHT: &str = "x-haneul-checkpoint-height";

/// Lowest available checkpoint for which transaction and checkpoint data can be requested.
///
/// Specifically this is the lowest checkpoint for which the following data can be requested:
///  - checkpoints
///  - transactions
///  - effects
///  - events
pub const X_HANEUL_LOWEST_AVAILABLE_CHECKPOINT: &str = "x-haneul-lowest-available-checkpoint";

/// Lowest available checkpoint for which object data can be requested.
///
/// Specifically this is the lowest checkpoint for which input/output object data will be
/// available.
pub const X_HANEUL_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS: &str =
    "x-haneul-lowest-available-checkpoint-objects";

/// Current epoch of the chain
pub const X_HANEUL_EPOCH: &str = "x-haneul-epoch";

/// Cursor to be used for endpoints that support cursor-based pagination. Pass this to the start field of the endpoint on the next call to get the next page of results.
pub const X_HANEUL_CURSOR: &str = "x-haneul-cursor";

/// Current timestamp of the chain - represented as number of milliseconds from the Unix epoch
pub const X_HANEUL_TIMESTAMP_MS: &str = "x-haneul-timestamp-ms";

/// Basic information about the state of a Node
#[serde_with::serde_as]
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct NodeInfo {
    /// The chain identifier of the chain that this Node is on
    pub chain_id: haneul_sdk_types::types::CheckpointDigest,

    /// Human readable name of the chain that this Node is on
    pub chain: std::borrow::Cow<'static, str>,

    /// Current epoch of the Node based on its highest executed checkpoint
    #[serde_as(as = "haneul_types::haneul_serde::BigInt<u64>")]
    #[schemars(with = "crate::rest::_schemars::U64")]
    pub epoch: u64,

    /// Checkpoint height of the most recently executed checkpoint
    #[serde_as(as = "haneul_types::haneul_serde::BigInt<u64>")]
    #[schemars(with = "crate::rest::_schemars::U64")]
    pub checkpoint_height: u64,

    /// Unix timestamp of the most recently executed checkpoint
    #[serde_as(as = "haneul_types::haneul_serde::BigInt<u64>")]
    #[schemars(with = "crate::rest::_schemars::U64")]
    pub timestamp_ms: u64,

    /// The lowest checkpoint for which checkpoints and transaction data is available
    #[serde_as(as = "Option<haneul_types::haneul_serde::BigInt<u64>>")]
    #[schemars(with = "Option<crate::rest::_schemars::U64>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_available_checkpoint: Option<u64>,

    /// The lowest checkpoint for which object data is available
    #[serde_as(as = "Option<haneul_types::haneul_serde::BigInt<u64>>")]
    #[schemars(with = "Option<crate::rest::_schemars::U64>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_available_checkpoint_objects: Option<u64>,
    pub software_version: std::borrow::Cow<'static, str>,
    //TODO include current protocol version
}
