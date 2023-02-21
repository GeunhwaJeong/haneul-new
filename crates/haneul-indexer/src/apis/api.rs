// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee_proc_macros::rpc;
use haneul_open_rpc_macros::open_rpc;

#[open_rpc(namespace = "haneul", tag = "Checkpoint API")]
#[rpc(server, client, namespace = "haneul")]
pub trait CheckpointApi {
    /// Returns the latest checkpoint sequence number,
    /// which starts at 0 and increments by 1 each time.
    #[method(name = "getLatestCheckpointSequenceNumber")]
    async fn get_latest_checkpoint_sequence_number(&self) -> RpcResult<i64>;
}
