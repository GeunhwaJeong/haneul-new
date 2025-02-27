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

/// Current timestamp of the chain - represented as number of milliseconds from the Unix epoch
pub const X_HANEUL_TIMESTAMP_MS: &str = "x-haneul-timestamp-ms";

/// Current timestamp of the chain - encoded in the [RFC 3339] format.
///
/// [RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
pub const X_HANEUL_TIMESTAMP: &str = "x-haneul-timestamp";

/// Response type for the transaction simulation endpoint
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TransactionSimulationResponse {
    pub effects: haneul_sdk_types::TransactionEffects,
    pub events: Option<haneul_sdk_types::TransactionEvents>,
    pub balance_changes: Option<Vec<haneul_sdk_types::BalanceChange>>,
    pub input_objects: Option<Vec<haneul_sdk_types::Object>>,
    pub output_objects: Option<Vec<haneul_sdk_types::Object>>,
}

/// Query parameters for the simulate transaction endpoint
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SimulateTransactionQueryParameters {
    /// Request `BalanceChanges` be included in the Response.
    #[serde(default)]
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub balance_changes: bool,
    /// Request input `Object`s be included in the Response.
    #[serde(default)]
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub input_objects: bool,
    /// Request output `Object`s be included in the Response.
    #[serde(default)]
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub output_objects: bool,
}

/// Response type for the execute transaction endpoint
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResolveTransactionResponse {
    pub transaction: haneul_sdk_types::Transaction,
    pub simulation: Option<TransactionSimulationResponse>,
}

/// Query parameters for the resolve transaction endpoint
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ResolveTransactionQueryParameters {
    /// Request that the fully resolved transaction be simulated and have its results sent back in
    /// the response.
    #[serde(default)]
    pub simulate: bool,
    #[serde(flatten)]
    pub simulate_transaction_parameters: SimulateTransactionQueryParameters,
}
