// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee_proc_macros::rpc;

use haneul_open_rpc_macros::open_rpc;
use haneul_types::base_types::HaneulAddress;

use haneul_types::committee::EpochId;
use haneul_types::governance::DelegatedStake;
use haneul_types::messages::CommitteeInfoResponse;

use haneul_types::haneul_system_state::{HaneulSystemState, ValidatorMetadata};

#[open_rpc(namespace = "haneul", tag = "Governance Read API")]
#[rpc(server, client, namespace = "haneul")]
pub trait GovernanceReadApi {
    /// Return all [DelegatedStake].
    #[method(name = "getDelegatedStakes")]
    async fn get_delegated_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all validators available for stake delegation.
    #[method(name = "getValidators")]
    async fn get_validators(&self) -> RpcResult<Vec<ValidatorMetadata>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(
        &self,
        /// The epoch of interest. If None, default to the latest epoch
        epoch: Option<EpochId>,
    ) -> RpcResult<CommitteeInfoResponse>;

    /// Return [HaneulSystemState]
    #[method(name = "getCurrentEpochStaticInfo")]
    async fn get_current_epoch_static_info(&self) -> RpcResult<HaneulSystemState>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<u64>;
}
