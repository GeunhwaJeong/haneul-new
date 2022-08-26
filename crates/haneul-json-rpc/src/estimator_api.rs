// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::api::EstimatorApiServer;
use crate::HaneulRpcModule;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use haneul_cost::estimator::estimate_computational_costs_for_transaction;
use haneul_json_rpc_types::HaneulGasCostSummary;
use haneul_open_rpc::Module;
use haneul_types::haneul_serde::Base64;
use haneul_types::{crypto::SignableBytes, messages::TransactionData};

pub struct EstimatorApi {}

#[async_trait]
impl EstimatorApiServer for EstimatorApi {
    async fn estimate_transaction_computation_cost(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<HaneulGasCostSummary> {
        let data = TransactionData::from_signable_bytes(&tx_bytes.to_vec()?)?;
        let est = estimate_computational_costs_for_transaction(data.kind)?;
        Ok(HaneulGasCostSummary::from(est))
    }
}

impl HaneulRpcModule for EstimatorApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::EstimatorApiOpenRpc::module_doc()
    }
}
