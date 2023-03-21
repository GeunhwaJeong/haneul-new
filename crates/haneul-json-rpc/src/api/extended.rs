// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee_proc_macros::rpc;

use haneul_json_rpc_types::{EpochInfo, EpochPage};
use haneul_open_rpc_macros::open_rpc;
use haneul_types::base_types::EpochId;

#[open_rpc(namespace = "haneulx", tag = "Extended API")]
#[rpc(server, client, namespace = "haneulx")]
pub trait ExtendedApi {
    /// Return a list of epoch info
    #[method(name = "getEpochs")]
    async fn get_epoch(
        &self,
        /// optional paging cursor
        cursor: Option<EpochId>,
        /// maximum number of items per page
        limit: Option<usize>,
    ) -> RpcResult<EpochPage>;

    /// Return current epoch info
    #[method(name = "getCurrentEpoch")]
    async fn get_current_epoch(&self) -> RpcResult<EpochInfo>;
}
