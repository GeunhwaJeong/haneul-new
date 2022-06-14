// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use haneul_core::authority::AuthorityState;
use haneul_core::gateway_state::GatewayClient;
use haneul_open_rpc::Module;
use haneul_types::base_types::ObjectID;

use crate::HaneulRpcModule;
use haneul_json_rpc_api::rpc_types::GetRawObjectDataResponse;
use haneul_json_rpc_api::RpcBcsApiServer;

pub struct BcsApiImpl {
    client: ClientStateAdaptor,
}

impl BcsApiImpl {
    pub fn new_with_gateway(client: GatewayClient) -> Self {
        Self {
            client: ClientStateAdaptor::Gateway(client),
        }
    }

    pub fn new(client: Arc<AuthorityState>) -> Self {
        Self {
            client: ClientStateAdaptor::FullNode(client),
        }
    }
}

enum ClientStateAdaptor {
    Gateway(GatewayClient),
    FullNode(Arc<AuthorityState>),
}

impl ClientStateAdaptor {
    async fn get_raw_object(
        &self,
        object_id: ObjectID,
    ) -> Result<GetRawObjectDataResponse, anyhow::Error> {
        match self {
            ClientStateAdaptor::Gateway(client) => client.get_raw_object(object_id).await,
            ClientStateAdaptor::FullNode(client) => client
                .get_object_read(&object_id)
                .await
                .map_err(|e| anyhow!("{e}"))?
                .try_into(),
        }
    }
}

#[async_trait]
impl RpcBcsApiServer for BcsApiImpl {
    async fn get_raw_object(&self, object_id: ObjectID) -> RpcResult<GetRawObjectDataResponse> {
        Ok(self.client.get_raw_object(object_id).await?)
    }
}

impl HaneulRpcModule for BcsApiImpl {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc_api::RpcBcsApiOpenRpc::module_doc()
    }
}
