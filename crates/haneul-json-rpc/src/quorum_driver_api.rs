// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::api::QuorumDriverApiServer;
use crate::HaneulRpcModule;
use anyhow::anyhow;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee_core::server::rpc_module::RpcModule;
use move_bytecode_utils::module_cache::SyncModuleCache;
use signature::Signature;
use std::sync::Arc;
use haneul_core::authority::{AuthorityStore, ResolverWrapper};
use haneul_core::authority_client::NetworkAuthorityClient;
use haneul_json_rpc_types::HaneulExecuteTransactionResponse;
use haneul_open_rpc::Module;
use haneul_quorum_driver::QuorumDriver;
use haneul_types::crypto::SignatureScheme;
use haneul_types::messages::{ExecuteTransactionRequest, ExecuteTransactionRequestType};
use haneul_types::haneul_serde::Base64;
use haneul_types::{
    crypto,
    crypto::SignableBytes,
    messages::{Transaction, TransactionData},
};

pub struct FullNodeQuorumDriverApi {
    pub quorum_driver: Arc<QuorumDriver<NetworkAuthorityClient>>,
    pub module_cache: Arc<SyncModuleCache<ResolverWrapper<AuthorityStore>>>,
}

impl FullNodeQuorumDriverApi {
    pub fn new(
        quorum_driver: Arc<QuorumDriver<NetworkAuthorityClient>>,
        module_cache: Arc<SyncModuleCache<ResolverWrapper<AuthorityStore>>>,
    ) -> Self {
        Self {
            quorum_driver,
            module_cache,
        }
    }
}

#[async_trait]
impl QuorumDriverApiServer for FullNodeQuorumDriverApi {
    async fn execute_transaction(
        &self,
        tx_bytes: Base64,
        sig_scheme: SignatureScheme,
        signature: Base64,
        pub_key: Base64,
        request_type: ExecuteTransactionRequestType,
    ) -> RpcResult<HaneulExecuteTransactionResponse> {
        let data = TransactionData::from_signable_bytes(&tx_bytes.to_vec()?)?;
        let flag = vec![sig_scheme.flag()];
        let signature = crypto::Signature::from_bytes(
            &[&*flag, &*signature.to_vec()?, &pub_key.to_vec()?].concat(),
        )
        .map_err(|e| anyhow!(e))?;
        let txn = Transaction::new(data, signature);
        let txn_digest = *txn.digest();
        let response = self
            .quorum_driver
            .execute_transaction(ExecuteTransactionRequest {
                transaction: txn,
                request_type,
            })
            .await
            .map_err(|e| anyhow!(e))?;
        HaneulExecuteTransactionResponse::from_execute_transaction_response(
            response,
            txn_digest,
            self.module_cache.as_ref(),
        )
        .map_err(jsonrpsee_core::Error::from)
    }
}

impl HaneulRpcModule for FullNodeQuorumDriverApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::QuorumDriverApiOpenRpc::module_doc()
    }
}
