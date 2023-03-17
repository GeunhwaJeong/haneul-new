// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::error::{Error, HaneulRpcResult};
use crate::{RpcClient, WAIT_FOR_TX_TIMEOUT_SEC};
use fastcrypto::encoding::Base64;
use futures::stream;
use futures_core::Stream;
use jsonrpsee::core::client::Subscription;
use std::collections::BTreeMap;
use std::future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use haneul_json_rpc::api::GovernanceReadApiClient;
use haneul_json_rpc_types::{
    Balance, Checkpoint, CheckpointId, Coin, CoinPage, DelegatedStake, DryRunTransactionResponse,
    DynamicFieldPage, EventFilter, EventPage, ObjectsPage, HaneulCoinMetadata, HaneulCommittee, HaneulEvent,
    HaneulGetPastObjectRequest, HaneulMoveNormalizedModule, HaneulObjectDataOptions, HaneulObjectResponse,
    HaneulObjectResponseQuery, HaneulPastObjectResponse, HaneulTransactionEffectsAPI,
    HaneulTransactionResponse, HaneulTransactionResponseOptions, HaneulTransactionResponseQuery,
    TransactionsPage,
};
use haneul_types::balance::Supply;
use haneul_types::base_types::{
    ObjectID, SequenceNumber, HaneulAddress, TransactionDigest, TxSequenceNumber,
};
use haneul_types::committee::EpochId;
use haneul_types::error::TRANSACTION_NOT_FOUND_MSG_PREFIX;
use haneul_types::event::EventID;
use haneul_types::messages::{ExecuteTransactionRequestType, TransactionData, VerifiedTransaction};
use haneul_types::messages_checkpoint::CheckpointSequenceNumber;

use futures::StreamExt;
use haneul_json_rpc::api::{CoinReadApiClient, EventReadApiClient, ReadApiClient, WriteApiClient};
use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;

#[derive(Debug)]
pub struct ReadApi {
    api: Arc<RpcClient>,
}

impl ReadApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    pub async fn get_owned_objects(
        &self,
        address: HaneulAddress,
        query: Option<HaneulObjectResponseQuery>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
        checkpoint: Option<CheckpointId>,
    ) -> HaneulRpcResult<ObjectsPage> {
        Ok(self
            .api
            .http
            .get_owned_objects(address, query, cursor, limit, checkpoint)
            .await?)
    }

    pub async fn get_dynamic_fields(
        &self,
        object_id: ObjectID,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> HaneulRpcResult<DynamicFieldPage> {
        Ok(self
            .api
            .http
            .get_dynamic_fields(object_id, cursor, limit)
            .await?)
    }

    pub async fn try_get_parsed_past_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
        options: HaneulObjectDataOptions,
    ) -> HaneulRpcResult<HaneulPastObjectResponse> {
        Ok(self
            .api
            .http
            .try_get_past_object(object_id, version, Some(options))
            .await?)
    }

    pub async fn try_multi_get_parsed_past_object(
        &self,
        past_objects: Vec<HaneulGetPastObjectRequest>,
        options: HaneulObjectDataOptions,
    ) -> HaneulRpcResult<Vec<HaneulPastObjectResponse>> {
        Ok(self
            .api
            .http
            .try_multi_get_past_objects(past_objects, Some(options))
            .await?)
    }

    pub async fn get_object_with_options(
        &self,
        object_id: ObjectID,
        options: HaneulObjectDataOptions,
    ) -> HaneulRpcResult<HaneulObjectResponse> {
        Ok(self
            .api
            .http
            .get_object_with_options(object_id, Some(options))
            .await?)
    }

    pub async fn multi_get_object_with_options(
        &self,
        object_ids: Vec<ObjectID>,
        options: HaneulObjectDataOptions,
    ) -> HaneulRpcResult<Vec<HaneulObjectResponse>> {
        Ok(self
            .api
            .http
            .multi_get_object_with_options(object_ids, Some(options))
            .await?)
    }

    pub async fn get_total_transaction_number(&self) -> HaneulRpcResult<u64> {
        Ok(self.api.http.get_total_transaction_number().await?)
    }

    pub async fn get_transactions_in_range_deprecated(
        &self,
        start: TxSequenceNumber,
        end: TxSequenceNumber,
    ) -> HaneulRpcResult<Vec<TransactionDigest>> {
        Ok(self
            .api
            .http
            .get_transactions_in_range_deprecated(start, end)
            .await?)
    }

    pub async fn get_transaction_with_options(
        &self,
        digest: TransactionDigest,
        options: HaneulTransactionResponseOptions,
    ) -> HaneulRpcResult<HaneulTransactionResponse> {
        Ok(self
            .api
            .http
            .get_transaction_with_options(digest, Some(options))
            .await?)
    }

    pub async fn multi_get_transactions_with_options(
        &self,
        digests: Vec<TransactionDigest>,
        options: HaneulTransactionResponseOptions,
    ) -> HaneulRpcResult<Vec<HaneulTransactionResponse>> {
        Ok(self
            .api
            .http
            .multi_get_transactions_with_options(digests, Some(options))
            .await?)
    }

    pub async fn get_committee_info(&self, epoch: Option<EpochId>) -> HaneulRpcResult<HaneulCommittee> {
        Ok(self.api.http.get_committee_info(epoch).await?)
    }

    pub async fn query_transactions(
        &self,
        query: HaneulTransactionResponseQuery,
        cursor: Option<TransactionDigest>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> HaneulRpcResult<TransactionsPage> {
        Ok(self
            .api
            .http
            .query_transactions(query, cursor, limit, Some(descending_order))
            .await?)
    }

    /// Return a checkpoint
    pub async fn get_checkpoint(&self, id: CheckpointId) -> HaneulRpcResult<Checkpoint> {
        Ok(self.api.http.get_checkpoint(id).await?)
    }

    /// Return the sequence number of the latest checkpoint that has been executed
    pub async fn get_latest_checkpoint_sequence_number(
        &self,
    ) -> HaneulRpcResult<CheckpointSequenceNumber> {
        Ok(self
            .api
            .http
            .get_latest_checkpoint_sequence_number()
            .await?)
    }

    pub fn get_transactions_stream(
        &self,
        query: HaneulTransactionResponseQuery,
        cursor: Option<TransactionDigest>,
        descending_order: bool,
    ) -> impl Stream<Item = HaneulTransactionResponse> + '_ {
        stream::unfold(
            (vec![], cursor, true, query),
            move |(mut data, cursor, first, query)| async move {
                if let Some(item) = data.pop() {
                    Some((item, (data, cursor, false, query)))
                } else if (cursor.is_none() && first) || cursor.is_some() {
                    let page = self
                        .query_transactions(query.clone(), cursor, Some(100), descending_order)
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

    // TODO(devx): we can probably cache this given an epoch
    pub async fn get_reference_gas_price(&self) -> HaneulRpcResult<u64> {
        Ok(self.api.http.get_reference_gas_price().await?)
    }

    pub async fn dry_run_transaction(
        &self,
        tx: TransactionData,
    ) -> HaneulRpcResult<DryRunTransactionResponse> {
        Ok(self
            .api
            .http
            .dry_run_transaction(Base64::from_bytes(&bcs::to_bytes(&tx)?))
            .await?)
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

    pub async fn get_all_coins(
        &self,
        owner: HaneulAddress,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> HaneulRpcResult<CoinPage> {
        Ok(self.api.http.get_all_coins(owner, cursor, limit).await?)
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

    pub async fn select_coins(
        &self,
        address: HaneulAddress,
        coin_type: Option<String>,
        amount: u128,
        locked_until_epoch: Option<EpochId>,
        exclude: Vec<ObjectID>,
    ) -> HaneulRpcResult<Vec<Coin>> {
        let mut total = 0u128;
        let coins = self
            .get_coins_stream(address, coin_type)
            .filter(|coin: &Coin| {
                future::ready(
                    locked_until_epoch == coin.locked_until_epoch
                        && !exclude.contains(&coin.coin_object_id),
                )
            })
            .take_while(|coin: &Coin| {
                let ready = future::ready(total < amount);
                total += coin.balance as u128;
                ready
            })
            .collect::<Vec<_>>()
            .await;

        if total < amount {
            return Err(Error::InsufficientFund { address, amount });
        }
        Ok(coins)
    }

    pub async fn get_balance(
        &self,
        owner: HaneulAddress,
        coin_type: Option<String>,
    ) -> HaneulRpcResult<Balance> {
        Ok(self.api.http.get_balance(owner, coin_type).await?)
    }

    pub async fn get_all_balances(&self, owner: HaneulAddress) -> HaneulRpcResult<Vec<Balance>> {
        Ok(self.api.http.get_all_balances(owner).await?)
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
        filter: EventFilter,
    ) -> HaneulRpcResult<impl Stream<Item = HaneulRpcResult<HaneulEvent>>> {
        match &self.api.ws {
            Some(c) => {
                let subscription: Subscription<HaneulEvent> = c.subscribe_event(filter).await?;
                Ok(subscription.map(|item| Ok(item?)))
            }
            _ => Err(Error::Subscription(
                "Subscription only supported by WebSocket client.".to_string(),
            )),
        }
    }

    pub async fn get_events(&self, digest: TransactionDigest) -> HaneulRpcResult<Vec<HaneulEvent>> {
        Ok(self.api.http.get_events(digest).await?)
    }

    pub async fn query_events(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> HaneulRpcResult<EventPage> {
        Ok(self
            .api
            .http
            .query_events(query, cursor, limit, Some(descending_order))
            .await?)
    }

    pub fn get_events_stream(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
        descending_order: bool,
    ) -> impl Stream<Item = HaneulEvent> + '_ {
        stream::unfold(
            (vec![], cursor, true, query),
            move |(mut data, cursor, first, query)| async move {
                if let Some(item) = data.pop() {
                    Some((item, (data, cursor, false, query)))
                } else if (cursor.is_none() && first) || cursor.is_some() {
                    let page = self
                        .query_events(query.clone(), cursor, Some(100), descending_order)
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
    /// the fullnode until the fullnode recognizes this transaction, or
    /// until times out (see WAIT_FOR_TX_TIMEOUT_SEC). If it times out, an
    /// error is returned from this call.
    pub async fn execute_transaction(
        &self,
        tx: VerifiedTransaction,
        options: HaneulTransactionResponseOptions,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> HaneulRpcResult<HaneulTransactionResponse> {
        let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
        let request_type = request_type.unwrap_or_else(|| options.default_execution_request_type());
        let mut response: HaneulTransactionResponse = self
            .api
            .http
            .execute_transaction(
                tx_bytes,
                signatures,
                Some(options),
                Some(request_type.clone()),
            )
            .await?;

        Ok(match request_type {
            ExecuteTransactionRequestType::WaitForEffectsCert => response,
            ExecuteTransactionRequestType::WaitForLocalExecution => {
                if let Some(confirmed_local_execution) = response.confirmed_local_execution {
                    if !confirmed_local_execution {
                        Self::wait_until_fullnode_sees_tx(
                            &self.api,
                            *response
                                .effects
                                .as_ref()
                                .map(|e| e.transaction_digest())
                                .ok_or_else(|| {
                                    Error::DataError("Expect effects to be non-empty".to_string())
                                })?,
                        )
                        .await?;
                    }
                }
                response.confirmed_local_execution = Some(true);
                response
            }
        })
    }

    async fn wait_until_fullnode_sees_tx(
        c: &RpcClient,
        tx_digest: TransactionDigest,
    ) -> HaneulRpcResult<()> {
        let start = Instant::now();
        loop {
            let resp = ReadApiClient::get_transaction_with_options(
                &c.http,
                tx_digest,
                Some(HaneulTransactionResponseOptions::new()),
            )
            .await;
            if let Err(err) = resp {
                if err.to_string().contains(TRANSACTION_NOT_FOUND_MSG_PREFIX) {
                    tokio::time::sleep(Duration::from_millis(300)).await;
                } else {
                    // immediately return on other types of errors
                    return Err(Error::TransactionConfirmationError(tx_digest, err));
                }
            } else {
                return Ok(());
            }
            if start.elapsed().as_secs() >= WAIT_FOR_TX_TIMEOUT_SEC {
                return Err(Error::FailToConfirmTransactionStatus(
                    tx_digest,
                    WAIT_FOR_TX_TIMEOUT_SEC,
                ));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GovernanceApi {
    api: Arc<RpcClient>,
}

impl GovernanceApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    /// Return all [DelegatedStake].
    pub async fn get_stakes(&self, owner: HaneulAddress) -> HaneulRpcResult<Vec<DelegatedStake>> {
        Ok(self.api.http.get_stakes(owner).await?)
    }

    /// Return the committee information for the asked `epoch`.
    /// `epoch`: The epoch of interest. If None, default to the latest epoch
    pub async fn get_committee_info(&self, epoch: Option<EpochId>) -> HaneulRpcResult<HaneulCommittee> {
        Ok(self.api.http.get_committee_info(epoch).await?)
    }

    /// Return the latest HANEUL system state object on-chain.
    pub async fn get_latest_haneul_system_state(&self) -> HaneulRpcResult<HaneulSystemStateSummary> {
        Ok(self.api.http.get_latest_haneul_system_state().await?)
    }

    /// Return the reference gas price for the network
    pub async fn get_reference_gas_price(&self) -> HaneulRpcResult<u64> {
        Ok(self.api.http.get_reference_gas_price().await?)
    }
}
