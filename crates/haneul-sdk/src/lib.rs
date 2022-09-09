// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use anyhow::anyhow;
use futures::StreamExt;
use futures_core::Stream;
use jsonrpsee::core::client::{ClientT, Subscription};
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use rpc_types::{GetPastObjectDataResponse, HaneulExecuteTransactionResponse};
pub use haneul_config::gateway;
use haneul_config::gateway::GatewayConfig;
use haneul_core::gateway_state::{GatewayClient, GatewayState};
pub use haneul_json as json;
use haneul_json_rpc::api::EventStreamingApiClient;
use haneul_json_rpc::api::QuorumDriverApiClient;
use haneul_json_rpc::api::RpcBcsApiClient;
use haneul_json_rpc::api::RpcFullNodeReadApiClient;
use haneul_json_rpc::api::RpcGatewayApiClient;
use haneul_json_rpc::api::RpcReadApiClient;
use haneul_json_rpc::api::WalletSyncApiClient;
pub use haneul_json_rpc_types as rpc_types;
use haneul_json_rpc_types::{
    GatewayTxSeqNumber, GetObjectDataResponse, GetRawObjectDataResponse, HaneulEventEnvelope,
    HaneulEventFilter, HaneulObjectInfo, HaneulTransactionResponse,
};
pub use haneul_types as types;
use haneul_types::base_types::{ObjectID, HaneulAddress, TransactionDigest};
use haneul_types::messages::Transaction;
use types::base_types::SequenceNumber;
use types::messages::ExecuteTransactionRequestType;

use crate::transaction_builder::TransactionBuilder;

// re-export essential haneul crates
pub mod crypto;
mod transaction_builder;

pub struct HaneulClient {
    api: Arc<HaneulClientApi>,
    transaction_builder: TransactionBuilder,
    read_api: Arc<ReadApi>,
    full_node_api: FullNodeApi,
    event_api: EventApi,
    quorum_driver: QuorumDriver,
    wallet_sync_api: WalletSyncApi,
}

#[allow(clippy::large_enum_variant)]
enum HaneulClientApi {
    Rpc(RpcClient),
    Embedded(GatewayClient),
}

struct RpcClient {
    http: HttpClient,
    ws: Option<WsClient>,
    info: ServerInfo,
}

struct ServerInfo {
    rpc_methods: Vec<String>,
    subscriptions: Vec<String>,
    version: String,
}

impl RpcClient {
    pub async fn new(http: &str, ws: Option<&str>) -> Result<Self, anyhow::Error> {
        let http = HttpClientBuilder::default().build(http)?;
        let ws = if let Some(url) = ws {
            Some(WsClientBuilder::default().build(url).await?)
        } else {
            None
        };
        let info = Self::get_server_info(&http, &ws).await?;
        Ok(Self { http, ws, info })
    }

    async fn get_server_info(
        http: &HttpClient,
        ws: &Option<WsClient>,
    ) -> Result<ServerInfo, anyhow::Error> {
        let rpc_spec: Value = http
            .request("rpc.discover", None)
            .await
            .map_err(|e| anyhow!("Fail to connect to the RPC server: {e}"))?;
        let version = rpc_spec
            .pointer("/info/version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Fail parsing server version from rpc.discover endpoint."))?;
        let rpc_methods = Self::parse_methods(&rpc_spec)?;

        let subscriptions = if let Some(ws) = ws {
            let rpc_spec: Value = ws
                .request("rpc.discover", None)
                .await
                .map_err(|e| anyhow!("Fail to connect to the Websocket server: {e}"))?;
            Self::parse_methods(&rpc_spec)?
        } else {
            Vec::new()
        };
        Ok(ServerInfo {
            rpc_methods,
            subscriptions,
            version: version.to_string(),
        })
    }

    fn parse_methods(server_spec: &Value) -> Result<Vec<String>, anyhow::Error> {
        let methods = server_spec
            .pointer("/methods")
            .and_then(|methods| methods.as_array())
            .ok_or_else(|| {
                anyhow!("Fail parsing server information from rpc.discover endpoint.")
            })?;

        Ok(methods
            .iter()
            .flat_map(|method| method["name"].as_str())
            .map(|s| s.into())
            .collect())
    }

    fn is_gateway(&self) -> bool {
        self.info
            .rpc_methods
            .contains(&"haneul_syncAccountState".to_string())
    }
}

impl HaneulClient {
    pub async fn new_rpc_client(
        http_url: &str,
        ws_url: Option<&str>,
    ) -> Result<HaneulClient, anyhow::Error> {
        let rpc = RpcClient::new(http_url, ws_url).await?;
        Ok(HaneulClient::new(HaneulClientApi::Rpc(rpc)))
    }

    pub fn new_embedded_client(config: &GatewayConfig) -> Result<HaneulClient, anyhow::Error> {
        let state = GatewayState::create_client(config, None)?;
        Ok(HaneulClient::new(HaneulClientApi::Embedded(state)))
    }

    fn new(api: HaneulClientApi) -> Self {
        let api = Arc::new(api);
        let read_api = Arc::new(ReadApi { api: api.clone() });
        let quorum_driver = QuorumDriver { api: api.clone() };

        let full_node_api = FullNodeApi(api.clone());
        let event_api = EventApi(api.clone());
        let transaction_builder = TransactionBuilder(read_api.clone());
        let wallet_sync_api = WalletSyncApi(api.clone());

        HaneulClient {
            api,
            transaction_builder,
            read_api,
            full_node_api,
            event_api,
            quorum_driver,
            wallet_sync_api,
        }
    }

    pub fn is_gateway(&self) -> bool {
        match &*self.api {
            HaneulClientApi::Rpc(c) => c.is_gateway(),
            HaneulClientApi::Embedded(_) => true,
        }
    }

    pub fn available_rpc_methods(&self) -> Vec<String> {
        match &*self.api {
            HaneulClientApi::Rpc(c) => c.info.rpc_methods.clone(),
            HaneulClientApi::Embedded(_) => vec![],
        }
    }

    pub fn available_subscriptions(&self) -> Vec<String> {
        match &*self.api {
            HaneulClientApi::Rpc(c) => c.info.subscriptions.clone(),
            HaneulClientApi::Embedded(_) => vec![],
        }
    }

    pub fn api_version(&self) -> String {
        match &*self.api {
            HaneulClientApi::Rpc(c) => c.info.version.clone(),
            HaneulClientApi::Embedded(_) => env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}

pub struct ReadApi {
    api: Arc<HaneulClientApi>,
}

impl ReadApi {
    pub async fn get_objects_owned_by_address(
        &self,
        address: HaneulAddress,
    ) -> anyhow::Result<Vec<HaneulObjectInfo>> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.get_objects_owned_by_address(address).await?,
            HaneulClientApi::Embedded(c) => c.get_objects_owned_by_address(address).await?,
        })
    }

    pub async fn get_objects_owned_by_object(
        &self,
        object_id: ObjectID,
    ) -> anyhow::Result<Vec<HaneulObjectInfo>> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.get_objects_owned_by_object(object_id).await?,
            HaneulClientApi::Embedded(c) => c.get_objects_owned_by_object(object_id).await?,
        })
    }

    pub async fn get_parsed_object(
        &self,
        object_id: ObjectID,
    ) -> anyhow::Result<GetObjectDataResponse> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.get_object(object_id).await?,
            HaneulClientApi::Embedded(c) => c.get_object(object_id).await?,
        })
    }

    pub async fn try_get_parsed_past_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
    ) -> anyhow::Result<GetPastObjectDataResponse> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.try_get_past_object(object_id, version).await?,
            // Gateway does not support get past object
            HaneulClientApi::Embedded(_) => {
                unimplemented!("Gateway/embedded client does not support get past object")
            }
        })
    }

    pub async fn get_object(
        &self,
        object_id: ObjectID,
    ) -> anyhow::Result<GetRawObjectDataResponse> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.get_raw_object(object_id).await?,
            HaneulClientApi::Embedded(c) => c.get_raw_object(object_id).await?,
        })
    }

    pub async fn get_total_transaction_number(&self) -> anyhow::Result<u64> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.get_total_transaction_number().await?,
            HaneulClientApi::Embedded(c) => c.get_total_transaction_number()?,
        })
    }

    pub async fn get_transactions_in_range(
        &self,
        start: GatewayTxSeqNumber,
        end: GatewayTxSeqNumber,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.get_transactions_in_range(start, end).await?,
            HaneulClientApi::Embedded(c) => c.get_transactions_in_range(start, end)?,
        })
    }

    pub async fn get_recent_transactions(
        &self,
        count: u64,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.get_recent_transactions(count).await?,
            HaneulClientApi::Embedded(c) => c.get_recent_transactions(count)?,
        })
    }

    pub async fn get_transaction(
        &self,
        digest: TransactionDigest,
    ) -> anyhow::Result<HaneulTransactionResponse> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => c.http.get_transaction(digest).await?,
            HaneulClientApi::Embedded(c) => c.get_transaction(digest).await?,
        })
    }
}

pub struct FullNodeApi(Arc<HaneulClientApi>);

impl FullNodeApi {
    pub async fn get_transactions_by_input_object(
        &self,
        object: ObjectID,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &*self.0 {
            HaneulClientApi::Rpc(c) => c.http.get_transactions_by_input_object(object).await?,
            HaneulClientApi::Embedded(_) => {
                return Err(anyhow!("Method not supported by embedded gateway client."))
            }
        })
    }

    pub async fn get_transactions_by_mutated_object(
        &self,
        object: ObjectID,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &*self.0 {
            HaneulClientApi::Rpc(c) => c.http.get_transactions_by_mutated_object(object),
            HaneulClientApi::Embedded(_) => {
                return Err(anyhow!("Method not supported by embedded gateway client."))
            }
        }
        .await?)
    }

    pub async fn get_transactions_by_move_function(
        &self,
        package: ObjectID,
        module: Option<String>,
        function: Option<String>,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &*self.0 {
            HaneulClientApi::Rpc(c) => c
                .http
                .get_transactions_by_move_function(package, module, function),
            HaneulClientApi::Embedded(_) => {
                return Err(anyhow!("Method not supported by embedded gateway client."))
            }
        }
        .await?)
    }

    pub async fn get_transactions_from_addr(
        &self,
        addr: HaneulAddress,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &*self.0 {
            HaneulClientApi::Rpc(c) => c.http.get_transactions_from_addr(addr),
            HaneulClientApi::Embedded(_) => {
                return Err(anyhow!("Method not supported by embedded gateway client."))
            }
        }
        .await?)
    }

    pub async fn get_transactions_to_addr(
        &self,
        addr: HaneulAddress,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &*self.0 {
            HaneulClientApi::Rpc(c) => c.http.get_transactions_to_addr(addr),
            HaneulClientApi::Embedded(_) => {
                return Err(anyhow!("Method not supported by embedded gateway client."))
            }
        }
        .await?)
    }
}
pub struct EventApi(Arc<HaneulClientApi>);

impl EventApi {
    pub async fn subscribe_event(
        &self,
        filter: HaneulEventFilter,
    ) -> anyhow::Result<impl Stream<Item = Result<HaneulEventEnvelope, anyhow::Error>>> {
        match &*self.0 {
            HaneulClientApi::Rpc(RpcClient { ws: Some(c), .. }) => {
                let subscription: Subscription<HaneulEventEnvelope> =
                    c.subscribe_event(filter).await?;
                Ok(subscription.map(|item| Ok(item?)))
            }
            _ => Err(anyhow!("Subscription only supported by WebSocket client.")),
        }
    }
}
pub struct QuorumDriver {
    api: Arc<HaneulClientApi>,
}

impl QuorumDriver {
    pub async fn execute_transaction(
        &self,
        tx: Transaction,
    ) -> anyhow::Result<HaneulTransactionResponse> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => {
                let (tx_bytes, flag, signature, pub_key) = tx.to_network_data_for_execution();
                RpcGatewayApiClient::execute_transaction(
                    &c.http, tx_bytes, flag, signature, pub_key,
                )
                .await?
            }
            HaneulClientApi::Embedded(c) => c.execute_transaction(tx).await?,
        })
    }

    pub async fn execute_transaction_by_fullnode(
        &self,
        tx: Transaction,
        request_type: ExecuteTransactionRequestType,
    ) -> anyhow::Result<HaneulExecuteTransactionResponse> {
        Ok(match &*self.api {
            HaneulClientApi::Rpc(c) => {
                let (tx_bytes, flag, signature, pub_key) = tx.to_network_data_for_execution();
                QuorumDriverApiClient::execute_transaction(
                    &c.http,
                    tx_bytes,
                    flag,
                    signature,
                    pub_key,
                    request_type,
                )
                .await?
            }
            // TODO do we want to support an embedded quorum driver?
            HaneulClientApi::Embedded(_c) => unimplemented!(),
        })
    }
}

pub struct WalletSyncApi(Arc<HaneulClientApi>);

impl WalletSyncApi {
    pub async fn sync_account_state(&self, address: HaneulAddress) -> anyhow::Result<()> {
        match &*self.0 {
            HaneulClientApi::Rpc(c) => {
                if c.is_gateway() {
                    c.http.sync_account_state(address).await?
                }
            }
            HaneulClientApi::Embedded(c) => c.sync_account_state(address).await?,
        }
        Ok(())
    }
}

impl HaneulClient {
    pub fn transaction_builder(&self) -> &TransactionBuilder {
        &self.transaction_builder
    }
    pub fn read_api(&self) -> &ReadApi {
        &self.read_api
    }
    pub fn full_node_api(&self) -> &FullNodeApi {
        &self.full_node_api
    }
    pub fn event_api(&self) -> &EventApi {
        &self.event_api
    }
    pub fn quorum_driver(&self) -> &QuorumDriver {
        &self.quorum_driver
    }
    pub fn wallet_sync_api(&self) -> &WalletSyncApi {
        &self.wallet_sync_api
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    Embedded(GatewayConfig),
    RPC(
        String,
        #[serde(default, skip_serializing_if = "Option::is_none")] Option<String>,
    ),
}

impl Display for ClientType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();

        match self {
            ClientType::Embedded(config) => {
                writeln!(writer, "Client Type : Embedded Gateway")?;
                writeln!(
                    writer,
                    "Gateway state DB folder path : {:?}",
                    config.db_folder_path
                )?;
                let authorities = config
                    .validator_set
                    .iter()
                    .map(|info| info.network_address());
                write!(
                    writer,
                    "Authorities : {:?}",
                    authorities.collect::<Vec<_>>()
                )?;
            }
            ClientType::RPC(url, ws_url) => {
                writeln!(writer, "Client Type : JSON-RPC")?;
                writeln!(writer, "HTTP RPC URL : {}", url)?;
                write!(
                    writer,
                    "WS RPC URL : {}",
                    ws_url.clone().unwrap_or_else(|| "None".to_string())
                )?;
            }
        }
        write!(f, "{}", writer)
    }
}

impl ClientType {
    pub async fn init(&self) -> Result<HaneulClient, anyhow::Error> {
        Ok(match self {
            ClientType::Embedded(config) => HaneulClient::new_embedded_client(config)?,
            ClientType::RPC(url, ws_url) => {
                HaneulClient::new_rpc_client(url, ws_url.as_deref()).await?
            }
        })
    }
}
