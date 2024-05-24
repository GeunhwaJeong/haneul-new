// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Chain ID of the current chain
pub const X_HANEUL_CHAIN_ID: &str = "x-haneul-chain-id";

/// Chain name of the current chain
pub const X_HANEUL_CHAIN: &str = "x-haneul-chain";

/// Current checkpoint height
pub const X_HANEUL_CHECKPOINT_HEIGHT: &str = "x-haneul-checkpoint-height";

/// Oldest non-pruned checkpoint height
pub const X_HANEUL_OLDEST_CHECKPOINT_HEIGHT: &str = "x-haneul-oldest-checkpoint-height";

/// Current epoch of the chain
pub const X_HANEUL_EPOCH: &str = "x-haneul-epoch";

/// Cursor to be used for endpoints that support cursor-based pagination. Pass this to the start field of the endpoint on the next call to get the next page of results.
pub const X_HANEUL_CURSOR: &str = "x-haneul-cursor";

/// Current timestamp of the chain - represented as number of milliseconds from the Unix epoch
pub const X_HANEUL_TIMESTAMP_MS: &str = "x-haneul-timestamp-ms";
