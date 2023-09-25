// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// TODO remove after the functions are implemented
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::errors::IndexerError;
use crate::indexer_reader::IndexerReader;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use haneul_json_rpc::api::GovernanceReadApiServer;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{DelegatedStake, ValidatorApys};
use haneul_json_rpc_types::{EpochInfo, HaneulCommittee};
use haneul_open_rpc::Module;
use haneul_types::base_types::{ObjectID, HaneulAddress};
use haneul_types::committee::EpochId;
use haneul_types::haneul_serde::BigInt;
use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;

#[derive(Clone)]
pub(crate) struct GovernanceReadApiV2 {
    inner: IndexerReader,
}

impl GovernanceReadApiV2 {
    pub fn new(inner: IndexerReader) -> Self {
        Self { inner }
    }

    async fn get_epoch_info(&self, epoch: Option<EpochId>) -> Result<EpochInfo, IndexerError> {
        match self
            .inner
            .spawn_blocking(move |this| this.get_epoch_info(epoch))
            .await
        {
            Ok(Some(epoch_info)) => Ok(epoch_info),
            Ok(None) => Err(IndexerError::InvalidArgumentError(format!(
                "Missing epoch {epoch:?}"
            ))),
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl GovernanceReadApiServer for GovernanceReadApiV2 {
    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        // Need Dynamic field queries
        unimplemented!()
    }

    async fn get_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>> {
        // Need Dynamic field queries
        unimplemented!()
    }

    async fn get_committee_info(&self, epoch: Option<BigInt<u64>>) -> RpcResult<HaneulCommittee> {
        let epoch = self.get_epoch_info(epoch.as_deref().copied()).await?;
        Ok(epoch.committee().map_err(IndexerError::from)?.into())
    }

    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary> {
        self.inner
            .spawn_blocking(|this| this.get_latest_haneul_system_state())
            .await
            .map_err(Into::into)
    }

    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        let epoch = self.get_epoch_info(None).await?;
        Ok(BigInt::from(epoch.reference_gas_price.ok_or_else(
            || {
                IndexerError::PersistentStorageDataCorruptionError(
                    "missing latest reference gas price".to_owned(),
                )
            },
        )?))
    }

    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        // Need Dynamic field queries
        unimplemented!()
    }
}

impl HaneulRpcModule for GovernanceReadApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::GovernanceReadApiOpenRpc::module_doc()
    }
}
