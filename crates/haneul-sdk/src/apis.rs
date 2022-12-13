// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::error::{RpcError, HaneulRpcResult};
use crate::{RpcClient, TransactionExecutionResult, WAIT_FOR_TX_TIMEOUT_SEC};
use futures::stream;
use futures_core::Stream;
use jsonrpsee::core::client::Subscription;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use haneul_json_rpc_types::{
    Balance, Coin, CoinPage, EventPage, GetObjectDataResponse, GetPastObjectDataResponse,
    GetRawObjectDataResponse, HaneulCoinMetadata, HaneulEventEnvelope, HaneulEventFilter,
    HaneulExecuteTransactionResponse, HaneulMoveNormalizedModule, HaneulObjectInfo, HaneulTransactionResponse,
    TransactionsPage,
};
use haneul_types::balance::Supply;
use haneul_types::base_types::{ObjectID, SequenceNumber, HaneulAddress, TransactionDigest};
use haneul_types::batch::TxSequenceNumber;
use haneul_types::committee::EpochId;
use haneul_types::error::TRANSACTION_NOT_FOUND_MSG_PREFIX;
use haneul_types::event::EventID;
use haneul_types::messages::{
    CommitteeInfoResponse, ExecuteTransactionRequestType, VerifiedTransaction,
};
use haneul_types::query::{EventQuery, TransactionQuery};
use haneul_types::haneul_system_state::HaneulSystemState;

use futures::StreamExt;
use haneul_json_rpc::api::{
    CoinReadApiClient, EventReadApiClient, EventStreamingApiClient, RpcBcsApiClient,
    RpcFullNodeReadApiClient, RpcReadApiClient, TransactionExecutionApiClient,
};
#[derive(Debug)]
pub struct ReadApi {
    api: Arc<RpcClient>,
}

impl ReadApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    pub async fn get_objects_owned_by_address(
        &self,
        address: HaneulAddress,
    ) -> HaneulRpcResult<Vec<HaneulObjectInfo>> {
        Ok(self.api.http.get_objects_owned_by_address(address).await?)
    }

    pub async fn get_objects_owned_by_object(
        &self,
        object_id: ObjectID,
    ) -> HaneulRpcResult<Vec<HaneulObjectInfo>> {
        Ok(self.api.http.get_objects_owned_by_object(object_id).await?)
    }

    pub async fn get_parsed_object(
        &self,
        object_id: ObjectID,
    ) -> HaneulRpcResult<GetObjectDataResponse> {
        Ok(self.api.http.get_object(object_id).await?)
    }

    pub async fn try_get_parsed_past_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
    ) -> HaneulRpcResult<GetPastObjectDataResponse> {
        Ok(self
            .api
            .http
            .try_get_past_object(object_id, version)
            .await?)
    }

    pub async fn get_object(&self, object_id: ObjectID) -> HaneulRpcResult<GetRawObjectDataResponse> {
        Ok(self.api.http.get_raw_object(object_id).await?)
    }

    pub async fn get_total_transaction_number(&self) -> HaneulRpcResult<u64> {
        Ok(self.api.http.get_total_transaction_number().await?)
    }

    pub async fn get_transactions_in_range(
        &self,
        start: TxSequenceNumber,
        end: TxSequenceNumber,
    ) -> HaneulRpcResult<Vec<TransactionDigest>> {
        Ok(self.api.http.get_transactions_in_range(start, end).await?)
    }

    pub async fn get_transaction(
        &self,
        digest: TransactionDigest,
    ) -> HaneulRpcResult<HaneulTransactionResponse> {
        Ok(self.api.http.get_transaction(digest).await?)
    }

    pub async fn get_committee_info(
        &self,
        epoch: Option<EpochId>,
    ) -> HaneulRpcResult<CommitteeInfoResponse> {
        Ok(self.api.http.get_committee_info(epoch).await?)
    }

    pub async fn get_transactions(
        &self,
        query: TransactionQuery,
        cursor: Option<TransactionDigest>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> HaneulRpcResult<TransactionsPage> {
        Ok(self
            .api
            .http
            .get_transactions(query, cursor, limit, Some(descending_order))
            .await?)
    }

    pub fn get_transactions_stream(
        &self,
        query: TransactionQuery,
        cursor: Option<TransactionDigest>,
        descending_order: bool,
    ) -> impl Stream<Item = TransactionDigest> + '_ {
        stream::unfold(
            (vec![], cursor, true, query),
            move |(mut data, cursor, first, query)| async move {
                if let Some(item) = data.pop() {
                    Some((item, (data, cursor, false, query)))
                } else if (cursor.is_none() && first) || cursor.is_some() {
                    let page = self
                        .get_transactions(query.clone(), cursor, Some(100), descending_order)
                        .await
                        .ok()?;
                    let mut data = page.data;
                    data.reverse();
                    data.pop()
                        .map(|item| (item, (data, page.next_cursor, false, query)))
                } else {
                    None
                }
            },
        )
    }

    pub async fn get_normalized_move_modules_by_package(
        &self,
        package: ObjectID,
    ) -> HaneulRpcResult<BTreeMap<String, HaneulMoveNormalizedModule>> {
        Ok(self
            .api
            .http
            .get_normalized_move_modules_by_package(package)
            .await?)
    }

    pub async fn get_haneul_system_state(&self) -> HaneulRpcResult<HaneulSystemState> {
        Ok(self.api.http.get_haneul_system_state().await?)
    }
}

#[derive(Debug, Clone)]
pub struct CoinReadApi {
    api: Arc<RpcClient>,
}

impl CoinReadApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    pub async fn get_coins(
        &self,
        owner: HaneulAddress,
        coin_type: Option<String>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> HaneulRpcResult<CoinPage> {
        Ok(self
            .api
            .http
            .get_coins(owner, coin_type, cursor, limit)
            .await?)
    }

    pub fn get_coins_stream(
        &self,
        owner: HaneulAddress,
        coin_type: Option<String>,
    ) -> impl Stream<Item = Coin> + '_ {
        stream::unfold(
            (vec![], None, true, coin_type),
            move |(mut data, cursor, first, coin_type)| async move {
                if let Some(item) = data.pop() {
                    Some((item, (data, cursor, false, coin_type)))
                } else if (cursor.is_none() && first) || cursor.is_some() {
                    let page = self
                        .get_coins(owner, coin_type.clone(), cursor, Some(100))
                        .await
                        .ok()?;
                    let mut data = page.data;
                    data.reverse();
                    data.pop()
                        .map(|item| (item, (data, page.next_cursor, false, coin_type)))
                } else {
                    None
                }
            },
        )
    }

    pub async fn get_balances(
        &self,
        owner: HaneulAddress,
        coin_type: Option<String>,
    ) -> HaneulRpcResult<Vec<Balance>> {
        Ok(self.api.http.get_balances(owner, coin_type).await?)
    }

    pub async fn get_coin_metadata(&self, coin_type: String) -> HaneulRpcResult<HaneulCoinMetadata> {
        Ok(self.api.http.get_coin_metadata(coin_type).await?)
    }

    pub async fn get_total_supply(&self, coin_type: String) -> HaneulRpcResult<Supply> {
        Ok(self.api.http.get_total_supply(coin_type).await?)
    }
}

#[derive(Clone)]
pub struct EventApi {
    api: Arc<RpcClient>,
}

impl EventApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    pub async fn subscribe_event(
        &self,
        filter: HaneulEventFilter,
    ) -> HaneulRpcResult<impl Stream<Item = HaneulRpcResult<HaneulEventEnvelope>>> {
        match &self.api.ws {
            Some(c) => {
                let subscription: Subscription<HaneulEventEnvelope> =
                    c.subscribe_event(filter).await?;
                Ok(subscription.map(|item| Ok(item?)))
            }
            _ => Err(RpcError::Subscription(
                "Subscription only supported by WebSocket client.".to_string(),
            )),
        }
    }

    pub async fn get_events(
        &self,
        query: EventQuery,
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> HaneulRpcResult<EventPage> {
        Ok(self
            .api
            .http
            .get_events(query, cursor, limit, Some(descending_order))
            .await?)
    }

    pub fn get_events_stream(
        &self,
        query: EventQuery,
        cursor: Option<EventID>,
        descending_order: bool,
    ) -> impl Stream<Item = HaneulEventEnvelope> + '_ {
        stream::unfold(
            (vec![], cursor, true, query),
            move |(mut data, cursor, first, query)| async move {
                if let Some(item) = data.pop() {
                    Some((item, (data, cursor, false, query)))
                } else if (cursor.is_none() && first) || cursor.is_some() {
                    let page = self
                        .get_events(query.clone(), cursor, Some(100), descending_order)
                        .await
                        .ok()?;
                    let mut data = page.data;
                    data.reverse();
                    data.pop()
                        .map(|item| (item, (data, page.next_cursor, false, query)))
                } else {
                    None
                }
            },
        )
    }
}

#[derive(Clone)]
pub struct QuorumDriver {
    api: Arc<RpcClient>,
}

impl QuorumDriver {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    /// Execute a transaction with a FullNode client. `request_type`
    /// defaults to `ExecuteTransactionRequestType::WaitForLocalExecution`.
    /// When `ExecuteTransactionRequestType::WaitForLocalExecution` is used,
    /// but returned `confirmed_local_execution` is false, the client polls
    /// the fullnode untils the fullnode recognizes this transaction, or
    /// until times out (see WAIT_FOR_TX_TIMEOUT_SEC). If it times out, an
    /// error is returned from this call.
    pub async fn execute_transaction(
        &self,
        tx: VerifiedTransaction,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> HaneulRpcResult<TransactionExecutionResult> {
        let (tx_bytes, flag, signature, pub_key) = tx.to_network_data_for_execution();
        let request_type =
            request_type.unwrap_or(ExecuteTransactionRequestType::WaitForLocalExecution);
        let resp = TransactionExecutionApiClient::execute_transaction(
            &self.api.http,
            tx_bytes,
            flag,
            signature,
            pub_key,
            request_type.clone(),
        )
        .await?;

        Ok(match (request_type, resp) {
            (
                ExecuteTransactionRequestType::ImmediateReturn,
                HaneulExecuteTransactionResponse::ImmediateReturn { tx_digest },
            ) => TransactionExecutionResult {
                tx_digest,
                tx_cert: None,
                effects: None,
                confirmed_local_execution: false,
                timestamp_ms: None,
                parsed_data: None,
            },
            (
                ExecuteTransactionRequestType::WaitForTxCert,
                HaneulExecuteTransactionResponse::TxCert { certificate },
            ) => TransactionExecutionResult {
                tx_digest: certificate.transaction_digest,
                tx_cert: Some(certificate),
                effects: None,
                confirmed_local_execution: false,
                timestamp_ms: None,
                parsed_data: None,
            },
            (
                ExecuteTransactionRequestType::WaitForEffectsCert,
                HaneulExecuteTransactionResponse::EffectsCert {
                    certificate,
                    effects,
                    confirmed_local_execution,
                },
            ) => TransactionExecutionResult {
                tx_digest: certificate.transaction_digest,
                tx_cert: Some(certificate),
                effects: Some(effects.effects),
                confirmed_local_execution,
                timestamp_ms: None,
                parsed_data: None,
            },
            (
                ExecuteTransactionRequestType::WaitForLocalExecution,
                HaneulExecuteTransactionResponse::EffectsCert {
                    certificate,
                    effects,
                    confirmed_local_execution,
                },
            ) => {
                if !confirmed_local_execution {
                    Self::wait_until_fullnode_sees_tx(&self.api, certificate.transaction_digest)
                        .await?;
                }
                TransactionExecutionResult {
                    tx_digest: certificate.transaction_digest,
                    tx_cert: Some(certificate),
                    effects: Some(effects.effects),
                    confirmed_local_execution,
                    timestamp_ms: None,
                    parsed_data: None,
                }
            }
            (other_request_type, other_resp) => {
                return Err(RpcError::InvalidTransactionResponse(
                    other_resp,
                    other_request_type,
                ))
            }
        })
    }

    async fn wait_until_fullnode_sees_tx(
        c: &RpcClient,
        tx_digest: TransactionDigest,
    ) -> HaneulRpcResult<()> {
        let start = Instant::now();
        loop {
            let resp = RpcReadApiClient::get_transaction(&c.http, tx_digest).await;
            if let Err(err) = resp {
                if err.to_string().contains(TRANSACTION_NOT_FOUND_MSG_PREFIX) {
                    tokio::time::sleep(Duration::from_millis(300)).await;
                } else {
                    // immediately return on other types of errors
                    return Err(RpcError::TransactionConfirmationError(tx_digest, err));
                }
            } else {
                return Ok(());
            }
            if start.elapsed().as_secs() >= WAIT_FOR_TX_TIMEOUT_SEC {
                return Err(RpcError::FailToConfirmTransactionStatus(
                    tx_digest,
                    WAIT_FOR_TX_TIMEOUT_SEC,
                ));
            }
        }
    }
}
