// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::api::TransactionExecutionApiServer;
use crate::HaneulRpcModule;
use anyhow::anyhow;
use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use fastcrypto::traits::ToFromBytes;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use move_bytecode_utils::module_cache::SyncModuleCache;
use haneullabs_metrics::spawn_monitored_task;
use std::sync::Arc;
use haneul_core::authority::{AuthorityStore, ResolverWrapper};
use haneul_core::authority_client::NetworkAuthorityClient;
use haneul_core::transaction_orchestrator::TransactiondOrchestrator;
use haneul_json_rpc_types::{HaneulTransactionEffects, HaneulTransactionResponse};
use haneul_open_rpc::Module;
use haneul_types::intent::Intent;
use haneul_types::messages::{ExecuteTransactionRequest, ExecuteTransactionRequestType};
use haneul_types::messages::{ExecuteTransactionResponse, Transaction};
use haneul_types::signature::GenericSignature;
pub struct FullNodeTransactionExecutionApi {
    pub transaction_orchestrator: Arc<TransactiondOrchestrator<NetworkAuthorityClient>>,
    pub module_cache: Arc<SyncModuleCache<ResolverWrapper<AuthorityStore>>>,
}

impl FullNodeTransactionExecutionApi {
    pub fn new(
        transaction_orchestrator: Arc<TransactiondOrchestrator<NetworkAuthorityClient>>,
        module_cache: Arc<SyncModuleCache<ResolverWrapper<AuthorityStore>>>,
    ) -> Self {
        Self {
            transaction_orchestrator,
            module_cache,
        }
    }
}

#[async_trait]
impl TransactionExecutionApiServer for FullNodeTransactionExecutionApi {
    async fn execute_transaction(
        &self,
        tx_bytes: Base64,
        signature: Base64,
        request_type: ExecuteTransactionRequestType,
    ) -> RpcResult<HaneulTransactionResponse> {
        self.submit_transaction(tx_bytes, vec![signature], request_type)
            .await
    }

    // TODO: remove this or execute_transaction
    async fn execute_transaction_serialized_sig(
        &self,
        tx_bytes: Base64,
        signature: Base64,
        request_type: ExecuteTransactionRequestType,
    ) -> RpcResult<HaneulTransactionResponse> {
        self.execute_transaction(tx_bytes, signature, request_type)
            .await
    }

    async fn submit_transaction(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        request_type: ExecuteTransactionRequestType,
    ) -> RpcResult<HaneulTransactionResponse> {
        let tx_data =
            bcs::from_bytes(&tx_bytes.to_vec().map_err(|e| anyhow!(e))?).map_err(|e| anyhow!(e))?;

        let mut sigs = Vec::new();
        for sig in signatures {
            sigs.push(
                GenericSignature::from_bytes(&sig.to_vec().map_err(|e| anyhow!(e))?)
                    .map_err(|e| anyhow!(e))?,
            );
        }

        let txn = Transaction::from_generic_sig_data(tx_data, Intent::default(), sigs);
        let tx = txn.data().clone().try_into()?;

        let transaction_orchestrator = self.transaction_orchestrator.clone();
        let response = spawn_monitored_task!(transaction_orchestrator.execute_transaction(
            ExecuteTransactionRequest {
                transaction: txn,
                request_type,
            }
        ))
        .await
        .map_err(|e| anyhow!(e))? // for JoinError
        .map_err(|e| anyhow!(e))?; // For Haneul transaction execution error (HaneulResult<ExecuteTransactionResponse>)

        match response {
            ExecuteTransactionResponse::EffectsCert(cert) => {
                let (_, effects, is_executed_locally) = *cert;
                Ok(HaneulTransactionResponse {
                    transaction: tx,
                    effects: HaneulTransactionEffects::try_from(
                        effects.effects,
                        self.module_cache.as_ref(),
                    )?,
                    timestamp_ms: None,
                    confirmed_local_execution: Some(is_executed_locally),
                    checkpoint: None,
                })
            }
        }
    }
}

impl HaneulRpcModule for FullNodeTransactionExecutionApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::TransactionExecutionApiOpenRpc::module_doc()
    }
}
