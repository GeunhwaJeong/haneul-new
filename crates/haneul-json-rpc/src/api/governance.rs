// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee_proc_macros::rpc;

use haneul_json_rpc_types::HaneulCommittee;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::base_types::HaneulAddress;

use haneul_types::committee::EpochId;
use haneul_types::governance::DelegatedStake;

use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;

#[open_rpc(namespace = "haneul", tag = "Governance Read API")]
#[rpc(server, client, namespace = "haneul")]
pub trait GovernanceReadApi {
    /// Return all [DelegatedStake].
    #[method(name = "getDelegatedStakes")]
    async fn get_delegated_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(
        &self,
        /// The epoch of interest. If None, default to the latest epoch
        epoch: Option<EpochId>,
    ) -> RpcResult<HaneulCommittee>;

    /// Return the latest HANEUL system state object on-chain.
    #[method(name = "getLatestHaneulSystemState")]
    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<u64>;
}
