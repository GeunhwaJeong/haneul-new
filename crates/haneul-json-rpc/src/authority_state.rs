// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use arc_swap::Guard;
use async_trait::async_trait;
use move_core_types::language_storage::TypeTag;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use haneul_core::accumulators::balances::{get_all_balances_for_owner, get_balance};
use haneul_core::authority::AuthorityState;
use haneul_core::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use haneul_core::execution_cache::ObjectCacheRead;
use haneul_core::jsonrpc_index::TotalBalance;
use haneul_core::subscription_handler::SubscriptionHandler;
use haneul_json_rpc_types::{
    Coin as HaneulCoin, DevInspectResults, DryRunTransactionBlockResponse, EventFilter, HaneulEvent,
    HaneulObjectDataFilter, TransactionFilter,
};
use haneul_storage::key_value_store::{
    KVStoreTransactionData, TransactionKeyValueStore, TransactionKeyValueStoreTrait,
};
use haneul_types::base_types::{
    MoveObjectType, ObjectID, ObjectInfo, ObjectRef, SequenceNumber, HaneulAddress,
};
use haneul_types::bridge::Bridge;
use haneul_types::committee::{Committee, EpochId};
use haneul_types::digests::{ChainIdentifier, TransactionDigest};
use haneul_types::dynamic_field::DynamicFieldInfo;
use haneul_types::effects::TransactionEffects;
use haneul_types::error::{HaneulError, HaneulErrorKind, HaneulResult, UserInputError};
use haneul_types::event::EventID;
use haneul_types::governance::StakedHaneul;
use haneul_types::messages_checkpoint::{
    CheckpointContents, CheckpointContentsDigest, CheckpointDigest, CheckpointSequenceNumber,
    VerifiedCheckpoint,
};
use haneul_types::object::{Object, ObjectRead, PastObjectRead};
use haneul_types::storage::{BackingPackageStore, ObjectStore, WriteKind};
use haneul_types::haneul_serde::BigInt;
use haneul_types::haneul_system_state::HaneulSystemState;
use haneul_types::transaction::{Transaction, TransactionData, TransactionKind};
use thiserror::Error;
use tokio::task::JoinError;

use crate::ObjectProvider;
#[cfg(test)]
use mockall::automock;
use typed_store_error::TypedStoreError;

pub type StateReadResult<T = ()> = Result<T, StateReadError>;

/// Trait for AuthorityState methods commonly used by at least two api.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait StateRead: Send + Sync {
    async fn multi_get(
        &self,
        transactions: &[TransactionDigest],
        effects: &[TransactionDigest],
    ) -> StateReadResult<KVStoreTransactionData>;

    fn get_object_read(&self, object_id: &ObjectID) -> StateReadResult<ObjectRead>;

    fn get_past_object_read(
        &self,
        object_id: &ObjectID,
        version: SequenceNumber,
    ) -> StateReadResult<PastObjectRead>;

    async fn get_object(&self, object_id: &ObjectID) -> StateReadResult<Option<Object>>;

    fn load_epoch_store_one_call_per_task(&self) -> Guard<Arc<AuthorityPerEpochStore>>;

    fn get_dynamic_fields(
        &self,
        owner: ObjectID,
        cursor: Option<ObjectID>,
        limit: usize,
    ) -> StateReadResult<Vec<(ObjectID, DynamicFieldInfo)>>;

    fn get_cache_reader(&self) -> &Arc<dyn ObjectCacheRead>;

    fn get_object_store(&self) -> &Arc<dyn ObjectStore + Send + Sync>;

    fn get_backing_package_store(&self) -> &Arc<dyn BackingPackageStore + Send + Sync>;

    fn get_owner_objects(
        &self,
        owner: HaneulAddress,
        cursor: Option<ObjectID>,
        filter: Option<HaneulObjectDataFilter>,
    ) -> StateReadResult<Vec<ObjectInfo>>;

    async fn query_events(
        &self,
        kv_store: &Arc<TransactionKeyValueStore>,
        query: EventFilter,
        // If `Some`, the query will start from the next item after the specified cursor
        cursor: Option<EventID>,
        limit: usize,
        descending: bool,
    ) -> StateReadResult<Vec<HaneulEvent>>;

    // transaction_execution_api
    #[allow(clippy::type_complexity)]
    async fn dry_exec_transaction(
        &self,
        transaction: TransactionData,
        transaction_digest: TransactionDigest,
    ) -> StateReadResult<(
        DryRunTransactionBlockResponse,
        BTreeMap<ObjectID, (ObjectRef, Object, WriteKind)>,
        TransactionEffects,
        Option<ObjectID>,
    )>;

    async fn dev_inspect_transaction_block(
        &self,
        sender: HaneulAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
        gas_budget: Option<u64>,
        gas_sponsor: Option<HaneulAddress>,
        gas_objects: Option<Vec<ObjectRef>>,
        show_raw_txn_data_and_effects: Option<bool>,
        skip_checks: Option<bool>,
    ) -> StateReadResult<DevInspectResults>;

    // indexer_api
    fn get_subscription_handler(&self) -> Arc<SubscriptionHandler>;

    fn get_owner_objects_with_limit(
        &self,
        owner: HaneulAddress,
        cursor: Option<ObjectID>,
        limit: usize,
        filter: Option<HaneulObjectDataFilter>,
    ) -> StateReadResult<Vec<ObjectInfo>>;

    async fn get_transactions(
        &self,
        kv_store: &Arc<TransactionKeyValueStore>,
        filter: Option<TransactionFilter>,
        cursor: Option<TransactionDigest>,
        limit: Option<usize>,
        reverse: bool,
    ) -> StateReadResult<Vec<TransactionDigest>>;

    fn get_dynamic_field_object_id(
        &self,
        owner: ObjectID,
        name_type: TypeTag,
        name_bcs_bytes: &[u8],
    ) -> StateReadResult<Option<ObjectID>>;

    // governance_api
    async fn get_staked_haneul(&self, owner: HaneulAddress) -> StateReadResult<Vec<StakedHaneul>>;
    fn get_system_state(&self) -> StateReadResult<HaneulSystemState>;
    fn get_or_latest_committee(&self, epoch: Option<BigInt<u64>>) -> StateReadResult<Committee>;

    // bridge_api
    fn get_bridge(&self) -> StateReadResult<Bridge>;

    // coin_api
    fn find_publish_txn_digest(&self, package_id: ObjectID) -> StateReadResult<TransactionDigest>;
    fn get_owned_coins(
        &self,
        owner: HaneulAddress,
        cursor: (String, u64, ObjectID),
        limit: usize,
        one_coin_type_only: bool,
    ) -> StateReadResult<Vec<HaneulCoin>>;
    async fn get_executed_transaction_and_effects(
        &self,
        digest: TransactionDigest,
        kv_store: Arc<TransactionKeyValueStore>,
    ) -> StateReadResult<(Transaction, TransactionEffects)>;
    async fn get_balance(
        &self,
        owner: HaneulAddress,
        coin_type: TypeTag,
    ) -> StateReadResult<TotalBalance>;
    async fn get_all_balance(
        &self,
        owner: HaneulAddress,
    ) -> StateReadResult<Arc<HashMap<TypeTag, TotalBalance>>>;

    // read_api
    fn get_verified_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> StateReadResult<VerifiedCheckpoint>;

    fn get_checkpoint_contents(
        &self,
        digest: CheckpointContentsDigest,
    ) -> StateReadResult<CheckpointContents>;

    fn get_verified_checkpoint_summary_by_digest(
        &self,
        digest: CheckpointDigest,
    ) -> StateReadResult<VerifiedCheckpoint>;

    fn deprecated_multi_get_transaction_checkpoint(
        &self,
        digests: &[TransactionDigest],
    ) -> StateReadResult<Vec<Option<(EpochId, CheckpointSequenceNumber)>>>;

    fn deprecated_get_transaction_checkpoint(
        &self,
        digest: &TransactionDigest,
    ) -> StateReadResult<Option<(EpochId, CheckpointSequenceNumber)>>;

    fn multi_get_checkpoint_by_sequence_number(
        &self,
        sequence_numbers: &[CheckpointSequenceNumber],
    ) -> StateReadResult<Vec<Option<VerifiedCheckpoint>>>;

    fn get_total_transaction_blocks(&self) -> StateReadResult<u64>;

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> StateReadResult<Option<VerifiedCheckpoint>>;

    fn get_latest_checkpoint_sequence_number(&self) -> StateReadResult<CheckpointSequenceNumber>;

    fn get_chain_identifier(&self) -> StateReadResult<ChainIdentifier>;
}

#[async_trait]
impl StateRead for AuthorityState {
    async fn multi_get(
        &self,
        transactions: &[TransactionDigest],
        effects: &[TransactionDigest],
    ) -> StateReadResult<KVStoreTransactionData> {
        Ok(
            <AuthorityState as TransactionKeyValueStoreTrait>::multi_get(
                self,
                transactions,
                effects,
            )
            .await?,
        )
    }

    fn get_object_read(&self, object_id: &ObjectID) -> StateReadResult<ObjectRead> {
        Ok(self.get_object_read(object_id)?)
    }

    async fn get_object(&self, object_id: &ObjectID) -> StateReadResult<Option<Object>> {
        Ok(self.get_object(object_id).await)
    }

    fn get_past_object_read(
        &self,
        object_id: &ObjectID,
        version: SequenceNumber,
    ) -> StateReadResult<PastObjectRead> {
        Ok(self.get_past_object_read(object_id, version)?)
    }

    fn load_epoch_store_one_call_per_task(&self) -> Guard<Arc<AuthorityPerEpochStore>> {
        self.load_epoch_store_one_call_per_task()
    }

    fn get_dynamic_fields(
        &self,
        owner: ObjectID,
        cursor: Option<ObjectID>,
        limit: usize,
    ) -> StateReadResult<Vec<(ObjectID, DynamicFieldInfo)>> {
        Ok(self.get_dynamic_fields(owner, cursor, limit)?)
    }

    fn get_cache_reader(&self) -> &Arc<dyn ObjectCacheRead> {
        self.get_object_cache_reader()
    }

    fn get_object_store(&self) -> &Arc<dyn ObjectStore + Send + Sync> {
        self.get_object_store()
    }

    fn get_backing_package_store(&self) -> &Arc<dyn BackingPackageStore + Send + Sync> {
        self.get_backing_package_store()
    }

    fn get_owner_objects(
        &self,
        owner: HaneulAddress,
        cursor: Option<ObjectID>,
        filter: Option<HaneulObjectDataFilter>,
    ) -> StateReadResult<Vec<ObjectInfo>> {
        Ok(self
            .get_owner_objects_iterator(owner, cursor, filter)?
            .collect())
    }

    async fn query_events(
        &self,
        kv_store: &Arc<TransactionKeyValueStore>,
        query: EventFilter,
        // If `Some`, the query will start from the next item after the specified cursor
        cursor: Option<EventID>,
        limit: usize,
        descending: bool,
    ) -> StateReadResult<Vec<HaneulEvent>> {
        Ok(self
            .query_events(kv_store, query, cursor, limit, descending)
            .await?)
    }

    #[allow(clippy::type_complexity)]
    async fn dry_exec_transaction(
        &self,
        transaction: TransactionData,
        transaction_digest: TransactionDigest,
    ) -> StateReadResult<(
        DryRunTransactionBlockResponse,
        BTreeMap<ObjectID, (ObjectRef, Object, WriteKind)>,
        TransactionEffects,
        Option<ObjectID>,
    )> {
        Ok(self
            .dry_exec_transaction(transaction, transaction_digest)
            .await?)
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender: HaneulAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
        gas_budget: Option<u64>,
        gas_sponsor: Option<HaneulAddress>,
        gas_objects: Option<Vec<ObjectRef>>,
        show_raw_txn_data_and_effects: Option<bool>,
        skip_checks: Option<bool>,
    ) -> StateReadResult<DevInspectResults> {
        Ok(self
            .dev_inspect_transaction_block(
                sender,
                transaction_kind,
                gas_price,
                gas_budget,
                gas_sponsor,
                gas_objects,
                show_raw_txn_data_and_effects,
                skip_checks,
            )
            .await?)
    }

    fn get_subscription_handler(&self) -> Arc<SubscriptionHandler> {
        self.subscription_handler.clone()
    }

    fn get_owner_objects_with_limit(
        &self,
        owner: HaneulAddress,
        cursor: Option<ObjectID>,
        limit: usize,
        filter: Option<HaneulObjectDataFilter>,
    ) -> StateReadResult<Vec<ObjectInfo>> {
        Ok(self.get_owner_objects(owner, cursor, limit, filter)?)
    }

    async fn get_transactions(
        &self,
        kv_store: &Arc<TransactionKeyValueStore>,
        filter: Option<TransactionFilter>,
        cursor: Option<TransactionDigest>,
        limit: Option<usize>,
        reverse: bool,
    ) -> StateReadResult<Vec<TransactionDigest>> {
        Ok(self
            .get_transactions(kv_store, filter, cursor, limit, reverse)
            .await?)
    }

    fn get_dynamic_field_object_id(
        // indexer
        &self,
        owner: ObjectID,
        name_type: TypeTag,
        name_bcs_bytes: &[u8],
    ) -> StateReadResult<Option<ObjectID>> {
        Ok(self.get_dynamic_field_object_id(owner, name_type, name_bcs_bytes)?)
    }

    async fn get_staked_haneul(&self, owner: HaneulAddress) -> StateReadResult<Vec<StakedHaneul>> {
        Ok(self
            .get_move_objects(owner, MoveObjectType::staked_haneul())
            .await?)
    }
    fn get_system_state(&self) -> StateReadResult<HaneulSystemState> {
        Ok(self
            .get_object_cache_reader()
            .get_haneul_system_state_object_unsafe()?)
    }
    fn get_or_latest_committee(&self, epoch: Option<BigInt<u64>>) -> StateReadResult<Committee> {
        Ok(self
            .committee_store()
            .get_or_latest_committee(epoch.map(|e| *e))?)
    }

    fn get_bridge(&self) -> StateReadResult<Bridge> {
        self.get_cache_reader()
            .get_bridge_object_unsafe()
            .map_err(|err| err.into())
    }

    fn find_publish_txn_digest(&self, package_id: ObjectID) -> StateReadResult<TransactionDigest> {
        Ok(self.find_publish_txn_digest(package_id)?)
    }
    fn get_owned_coins(
        &self,
        owner: HaneulAddress,
        cursor: (String, u64, ObjectID),
        limit: usize,
        one_coin_type_only: bool,
    ) -> StateReadResult<Vec<HaneulCoin>> {
        Ok(self
            .get_owned_coins_iterator_with_cursor(owner, cursor, limit, one_coin_type_only)?
            .map(|(key, coin)| HaneulCoin {
                coin_type: key.coin_type,
                coin_object_id: key.object_id,
                version: coin.version,
                digest: coin.digest,
                balance: coin.balance,
                previous_transaction: coin.previous_transaction,
            })
            .collect())
    }

    async fn get_executed_transaction_and_effects(
        &self,
        digest: TransactionDigest,
        kv_store: Arc<TransactionKeyValueStore>,
    ) -> StateReadResult<(Transaction, TransactionEffects)> {
        Ok(self
            .get_executed_transaction_and_effects(digest, kv_store)
            .await?)
    }

    async fn get_balance(
        &self,
        owner: HaneulAddress,
        coin_type: TypeTag,
    ) -> StateReadResult<TotalBalance> {
        let indexes = self.indexes.clone();
        let child_object_resolver = self.get_child_object_resolver().clone();
        Ok(
            tokio::task::spawn_blocking(move || -> HaneulResult<TotalBalance> {
                let address_balance =
                    get_balance(owner, child_object_resolver.as_ref(), coin_type.clone())?;
                let coin_balance = indexes
                    .as_ref()
                    .ok_or(HaneulErrorKind::IndexStoreNotAvailable)?
                    .get_coin_object_balance(owner, coin_type)?;
                let mut total_balance = coin_balance;
                if address_balance > 0 {
                    total_balance.balance += address_balance as i128;
                    total_balance.num_coins += 1;
                }
                total_balance.address_balance = address_balance;
                Ok(total_balance)
            })
            .await
            .map_err(|e: JoinError| {
                HaneulError(Box::new(HaneulErrorKind::ExecutionError(e.to_string())))
            })??,
        )
    }

    async fn get_all_balance(
        &self,
        owner: HaneulAddress,
    ) -> StateReadResult<Arc<HashMap<TypeTag, TotalBalance>>> {
        let indexes = self.indexes.clone();
        let child_object_resolver = self.get_child_object_resolver().clone();
        Ok(tokio::task::spawn_blocking(
            move || -> HaneulResult<Arc<HashMap<TypeTag, TotalBalance>>> {
                let indexes = indexes
                    .as_ref()
                    .ok_or(HaneulErrorKind::IndexStoreNotAvailable)?;
                let address_balances =
                    get_all_balances_for_owner(owner, child_object_resolver.as_ref(), indexes)?;
                let coin_balances = (*indexes.get_all_coin_object_balances(owner)?).clone();
                let mut all_balances = coin_balances;
                for (coin_type, balance) in address_balances {
                    let existing_balance = all_balances.entry(coin_type).or_insert(TotalBalance {
                        balance: 0,
                        num_coins: 0,
                        address_balance: 0,
                    });
                    existing_balance.balance += balance as i128;
                    existing_balance.num_coins += 1;
                    existing_balance.address_balance = balance;
                }
                Ok(Arc::new(all_balances))
            },
        )
        .await
        .map_err(|e: JoinError| {
            HaneulError(Box::new(HaneulErrorKind::ExecutionError(e.to_string())))
        })??)
    }

    fn get_verified_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> StateReadResult<VerifiedCheckpoint> {
        Ok(self.get_verified_checkpoint_by_sequence_number(sequence_number)?)
    }

    fn get_checkpoint_contents(
        &self,
        digest: CheckpointContentsDigest,
    ) -> StateReadResult<CheckpointContents> {
        Ok(self.get_checkpoint_contents(digest)?)
    }

    fn get_verified_checkpoint_summary_by_digest(
        &self,
        digest: CheckpointDigest,
    ) -> StateReadResult<VerifiedCheckpoint> {
        Ok(self.get_verified_checkpoint_summary_by_digest(digest)?)
    }

    fn deprecated_multi_get_transaction_checkpoint(
        &self,
        digests: &[TransactionDigest],
    ) -> StateReadResult<Vec<Option<(EpochId, CheckpointSequenceNumber)>>> {
        Ok(self
            .get_checkpoint_cache()
            .deprecated_multi_get_transaction_checkpoint(digests))
    }

    fn deprecated_get_transaction_checkpoint(
        &self,
        digest: &TransactionDigest,
    ) -> StateReadResult<Option<(EpochId, CheckpointSequenceNumber)>> {
        Ok(self
            .get_checkpoint_cache()
            .deprecated_get_transaction_checkpoint(digest))
    }

    fn multi_get_checkpoint_by_sequence_number(
        &self,
        sequence_numbers: &[CheckpointSequenceNumber],
    ) -> StateReadResult<Vec<Option<VerifiedCheckpoint>>> {
        Ok(self.multi_get_checkpoint_by_sequence_number(sequence_numbers)?)
    }

    fn get_total_transaction_blocks(&self) -> StateReadResult<u64> {
        Ok(self.get_total_transaction_blocks()?)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> StateReadResult<Option<VerifiedCheckpoint>> {
        Ok(self.get_checkpoint_by_sequence_number(sequence_number)?)
    }

    fn get_latest_checkpoint_sequence_number(&self) -> StateReadResult<CheckpointSequenceNumber> {
        Ok(self.get_latest_checkpoint_sequence_number()?)
    }

    fn get_chain_identifier(&self) -> StateReadResult<ChainIdentifier> {
        Ok(self.get_chain_identifier())
    }
}

/// This implementation allows `S` to be a dynamically sized type (DST) that implements ObjectProvider
/// Valid as `S` is referenced only, and memory management is handled by `Arc`
#[async_trait]
impl<S: ?Sized + StateRead> ObjectProvider for Arc<S> {
    type Error = StateReadError;

    async fn get_object(
        &self,
        id: &ObjectID,
        version: &SequenceNumber,
    ) -> Result<Object, Self::Error> {
        Ok(self.get_past_object_read(id, *version)?.into_object()?)
    }

    async fn find_object_lt_or_eq_version(
        &self,
        id: &ObjectID,
        version: &SequenceNumber,
    ) -> Result<Option<Object>, Self::Error> {
        Ok(self
            .get_cache_reader()
            .find_object_lt_or_eq_version(*id, *version))
    }
}

#[async_trait]
impl<S: ?Sized + StateRead> ObjectProvider for (Arc<S>, Arc<TransactionKeyValueStore>) {
    type Error = StateReadError;

    async fn get_object(
        &self,
        id: &ObjectID,
        version: &SequenceNumber,
    ) -> Result<Object, Self::Error> {
        let object_read = self.0.get_past_object_read(id, *version)?;
        match object_read {
            PastObjectRead::ObjectNotExists(_) | PastObjectRead::VersionNotFound(..) => {
                match self.1.get_object(*id, *version).await? {
                    Some(object) => Ok(object),
                    None => Ok(PastObjectRead::VersionNotFound(*id, *version).into_object()?),
                }
            }
            _ => Ok(object_read.into_object()?),
        }
    }

    async fn find_object_lt_or_eq_version(
        &self,
        id: &ObjectID,
        version: &SequenceNumber,
    ) -> Result<Option<Object>, Self::Error> {
        Ok(self
            .0
            .get_cache_reader()
            .find_object_lt_or_eq_version(*id, *version))
    }
}

#[derive(Debug, Error)]
pub enum StateReadInternalError {
    #[error(transparent)]
    HaneulError(#[from] HaneulError),
    #[error(transparent)]
    JoinError(#[from] JoinError),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl From<HaneulErrorKind> for StateReadInternalError {
    fn from(e: HaneulErrorKind) -> Self {
        StateReadInternalError::HaneulError(HaneulError::from(e))
    }
}

#[derive(Debug, Error)]
pub enum StateReadClientError {
    #[error(transparent)]
    HaneulError(#[from] HaneulError),
    #[error(transparent)]
    UserInputError(#[from] UserInputError),
}

impl From<HaneulErrorKind> for StateReadClientError {
    fn from(e: HaneulErrorKind) -> Self {
        StateReadClientError::HaneulError(HaneulError::from(e))
    }
}

/// `StateReadError` is the error type for callers to work with.
/// It captures all possible errors that can occur while reading state, classifying them into two categories.
/// Unless `StateReadError` is the final error state before returning to caller, the app may still want error context.
/// This context is preserved in `Internal` and `Client` variants.
#[derive(Debug, Error)]
pub enum StateReadError {
    // haneul_json_rpc::Error will do the final conversion to generic error message
    #[error(transparent)]
    Internal(#[from] StateReadInternalError),

    // Client errors
    #[error(transparent)]
    Client(#[from] StateReadClientError),
}

impl From<HaneulErrorKind> for StateReadError {
    fn from(e: HaneulErrorKind) -> Self {
        match e {
            HaneulErrorKind::IndexStoreNotAvailable
            | HaneulErrorKind::TransactionNotFound { .. }
            | HaneulErrorKind::UnsupportedFeatureError { .. }
            | HaneulErrorKind::UserInputError { .. }
            | HaneulErrorKind::WrongMessageVersion { .. } => StateReadError::Client(e.into()),
            _ => StateReadError::Internal(e.into()),
        }
    }
}

impl From<HaneulError> for StateReadError {
    fn from(e: HaneulError) -> Self {
        e.into_inner().into()
    }
}

impl From<UserInputError> for StateReadError {
    fn from(e: UserInputError) -> Self {
        StateReadError::Client(e.into())
    }
}

impl From<JoinError> for StateReadError {
    fn from(e: JoinError) -> Self {
        StateReadError::Internal(e.into())
    }
}

impl From<anyhow::Error> for StateReadError {
    fn from(e: anyhow::Error) -> Self {
        StateReadError::Internal(e.into())
    }
}

impl From<TypedStoreError> for StateReadError {
    fn from(e: TypedStoreError) -> Self {
        let error: HaneulError = e.into();
        StateReadError::Internal(error.into())
    }
}
