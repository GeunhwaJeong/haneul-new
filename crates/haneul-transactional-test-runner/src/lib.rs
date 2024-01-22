// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains the transactional test runner instantiation for the Haneul adapter

pub mod args;
pub mod programmable_transaction_test_parser;
mod simulator_persisted_store;
pub mod test_adapter;

pub use move_transactional_test_runner::framework::run_test_impl;
use rand::rngs::StdRng;
use simulacrum::Simulacrum;
use simulacrum::SimulatorStore;
use simulator_persisted_store::PersistedStore;
use std::path::Path;
use std::sync::Arc;
use haneul_core::authority::authority_test_utils::send_and_confirm_transaction_with_execution_error;
use haneul_core::authority::AuthorityState;
use haneul_json_rpc::authority_state::StateRead;
use haneul_json_rpc_types::DevInspectResults;
use haneul_json_rpc_types::EventFilter;
use haneul_storage::key_value_store::TransactionKeyValueStore;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::HaneulAddress;
use haneul_types::base_types::VersionNumber;
use haneul_types::digests::TransactionDigest;
use haneul_types::digests::TransactionEventsDigest;
use haneul_types::effects::TransactionEffects;
use haneul_types::effects::TransactionEvents;
use haneul_types::error::ExecutionError;
use haneul_types::error::HaneulError;
use haneul_types::error::HaneulResult;
use haneul_types::event::Event;
use haneul_types::messages_checkpoint::CheckpointContentsDigest;
use haneul_types::messages_checkpoint::VerifiedCheckpoint;
use haneul_types::object::Object;
use haneul_types::storage::ObjectStore;
use haneul_types::storage::ReadStore;
use haneul_types::haneul_system_state::epoch_start_haneul_system_state::EpochStartSystemStateTrait;
use haneul_types::haneul_system_state::HaneulSystemStateTrait;
use haneul_types::transaction::Transaction;
use haneul_types::transaction::TransactionDataAPI;
use haneul_types::transaction::TransactionKind;
use test_adapter::{HaneulTestAdapter, PRE_COMPILED};

#[cfg_attr(not(msim), tokio::main)]
#[cfg_attr(msim, msim::main)]
pub async fn run_test(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    telemetry_subscribers::init_for_testing();
    run_test_impl::<HaneulTestAdapter>(path, Some(&*PRE_COMPILED)).await?;
    Ok(())
}

pub struct ValidatorWithFullnode {
    pub validator: Arc<AuthorityState>,
    pub fullnode: Arc<AuthorityState>,
    pub kv_store: Arc<TransactionKeyValueStore>,
}

#[allow(unused_variables)]
/// TODO: better name?
#[async_trait::async_trait]
pub trait TransactionalAdapter: Send + Sync + ReadStore {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)>;

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint>;

    async fn advance_clock(
        &mut self,
        duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects>;

    async fn advance_epoch(&mut self, create_random_state: bool) -> anyhow::Result<()>;

    async fn request_gas(
        &mut self,
        address: HaneulAddress,
        amount: u64,
    ) -> anyhow::Result<TransactionEffects>;

    async fn dev_inspect_transaction_block(
        &self,
        sender: HaneulAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
    ) -> HaneulResult<DevInspectResults>;

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        limit: usize,
    ) -> HaneulResult<Vec<Event>>;

    async fn get_active_validator_addresses(&self) -> HaneulResult<Vec<HaneulAddress>>;
}

#[async_trait::async_trait]
impl TransactionalAdapter for ValidatorWithFullnode {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        let with_shared = transaction
            .data()
            .intent_message()
            .value
            .contains_shared_object();
        let (_, effects, execution_error) = send_and_confirm_transaction_with_execution_error(
            &self.validator,
            Some(&self.fullnode),
            transaction,
            with_shared,
        )
        .await?;
        Ok((effects.into_data(), execution_error))
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender: HaneulAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
    ) -> HaneulResult<DevInspectResults> {
        self.fullnode
            .dev_inspect_transaction_block(
                sender,
                transaction_kind,
                gas_price,
                None,
                None,
                None,
                None,
                None,
            )
            .await
    }

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        limit: usize,
    ) -> HaneulResult<Vec<Event>> {
        Ok(self
            .validator
            .query_events(
                &self.kv_store,
                EventFilter::Transaction(*tx_digest),
                None,
                limit,
                false,
            )
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|haneul_event| haneul_event.into())
            .collect())
    }

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint> {
        unimplemented!("create_checkpoint not supported")
    }

    async fn advance_clock(
        &mut self,
        _duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects> {
        unimplemented!("advance_clock not supported")
    }

    async fn advance_epoch(&mut self, _create_random_state: bool) -> anyhow::Result<()> {
        unimplemented!("advance_epoch not supported")
    }

    async fn request_gas(
        &mut self,
        _address: HaneulAddress,
        _amount: u64,
    ) -> anyhow::Result<TransactionEffects> {
        unimplemented!("request_gas not supported")
    }

    async fn get_active_validator_addresses(&self) -> HaneulResult<Vec<HaneulAddress>> {
        Ok(self
            .fullnode
            .get_system_state()
            .map_err(|e| {
                HaneulError::HaneulSystemStateReadError(format!(
                    "Failed to get system state from fullnode: {}",
                    e
                ))
            })?
            .into_haneul_system_state_summary()
            .active_validators
            .iter()
            .map(|x| x.haneul_address)
            .collect::<Vec<_>>())
    }
}

impl ReadStore for ValidatorWithFullnode {
    fn get_committee(
        &self,
        _epoch: haneul_types::committee::EpochId,
    ) -> haneul_types::storage::error::Result<Option<Arc<haneul_types::committee::Committee>>> {
        todo!()
    }

    fn get_latest_checkpoint(&self) -> haneul_types::storage::error::Result<VerifiedCheckpoint> {
        let sequence_number = self
            .validator
            .get_latest_checkpoint_sequence_number()
            .unwrap();
        self.get_checkpoint_by_sequence_number(sequence_number)
            .map(|c| c.unwrap())
    }

    fn get_highest_verified_checkpoint(
        &self,
    ) -> haneul_types::storage::error::Result<VerifiedCheckpoint> {
        todo!()
    }

    fn get_highest_synced_checkpoint(
        &self,
    ) -> haneul_types::storage::error::Result<VerifiedCheckpoint> {
        todo!()
    }

    fn get_lowest_available_checkpoint(
        &self,
    ) -> haneul_types::storage::error::Result<haneul_types::messages_checkpoint::CheckpointSequenceNumber>
    {
        todo!()
    }

    fn get_checkpoint_by_digest(
        &self,
        _digest: &haneul_types::messages_checkpoint::CheckpointDigest,
    ) -> haneul_types::storage::error::Result<Option<VerifiedCheckpoint>> {
        todo!()
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: haneul_types::messages_checkpoint::CheckpointSequenceNumber,
    ) -> haneul_types::storage::error::Result<Option<VerifiedCheckpoint>> {
        self.validator
            .get_checkpoint_store()
            .get_checkpoint_by_sequence_number(sequence_number)
            .map_err(haneul_types::storage::error::Error::custom)
    }

    fn get_checkpoint_contents_by_digest(
        &self,
        digest: &CheckpointContentsDigest,
    ) -> haneul_types::storage::error::Result<Option<haneul_types::messages_checkpoint::CheckpointContents>>
    {
        self.validator
            .get_checkpoint_store()
            .get_checkpoint_contents(digest)
            .map_err(haneul_types::storage::error::Error::custom)
    }

    fn get_checkpoint_contents_by_sequence_number(
        &self,
        _sequence_number: haneul_types::messages_checkpoint::CheckpointSequenceNumber,
    ) -> haneul_types::storage::error::Result<Option<haneul_types::messages_checkpoint::CheckpointContents>>
    {
        todo!()
    }

    fn get_transaction(
        &self,
        tx_digest: &TransactionDigest,
    ) -> haneul_types::storage::error::Result<Option<haneul_types::transaction::VerifiedTransaction>>
    {
        self.validator
            .database
            .get_transaction_block(tx_digest)
            .map_err(haneul_types::storage::error::Error::custom)
    }

    fn get_transaction_effects(
        &self,
        tx_digest: &TransactionDigest,
    ) -> haneul_types::storage::error::Result<Option<TransactionEffects>> {
        self.validator
            .database
            .get_executed_effects(tx_digest)
            .map_err(haneul_types::storage::error::Error::custom)
    }

    fn get_events(
        &self,
        event_digest: &TransactionEventsDigest,
    ) -> haneul_types::storage::error::Result<Option<TransactionEvents>> {
        self.validator
            .database
            .get_events(event_digest)
            .map_err(haneul_types::storage::error::Error::custom)
    }

    fn get_full_checkpoint_contents_by_sequence_number(
        &self,
        _sequence_number: haneul_types::messages_checkpoint::CheckpointSequenceNumber,
    ) -> haneul_types::storage::error::Result<
        Option<haneul_types::messages_checkpoint::FullCheckpointContents>,
    > {
        todo!()
    }

    fn get_full_checkpoint_contents(
        &self,
        _digest: &CheckpointContentsDigest,
    ) -> haneul_types::storage::error::Result<
        Option<haneul_types::messages_checkpoint::FullCheckpointContents>,
    > {
        todo!()
    }
}

impl ObjectStore for ValidatorWithFullnode {
    fn get_object(
        &self,
        object_id: &ObjectID,
    ) -> Result<Option<Object>, haneul_types::storage::error::Error> {
        self.validator.database.get_object(object_id)
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> Result<Option<Object>, haneul_types::storage::error::Error> {
        self.validator
            .database
            .get_object_by_key(object_id, version)
    }
}

#[async_trait::async_trait]
impl TransactionalAdapter for Simulacrum<StdRng, PersistedStore> {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        Ok(self.execute_transaction(transaction)?)
    }

    async fn dev_inspect_transaction_block(
        &self,
        _sender: HaneulAddress,
        _transaction_kind: TransactionKind,
        _gas_price: Option<u64>,
    ) -> HaneulResult<DevInspectResults> {
        unimplemented!("dev_inspect_transaction_block not supported in simulator mode")
    }

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        _limit: usize,
    ) -> HaneulResult<Vec<Event>> {
        Ok(self
            .store()
            .get_transaction_events_by_tx_digest(tx_digest)
            .map(|x| x.data)
            .unwrap_or_default())
    }

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint> {
        Ok(self.create_checkpoint())
    }

    async fn advance_clock(
        &mut self,
        duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects> {
        Ok(self.advance_clock(duration))
    }

    async fn advance_epoch(&mut self, create_random_state: bool) -> anyhow::Result<()> {
        self.advance_epoch(create_random_state);
        Ok(())
    }

    async fn request_gas(
        &mut self,
        address: HaneulAddress,
        amount: u64,
    ) -> anyhow::Result<TransactionEffects> {
        self.request_gas(address, amount)
    }

    async fn get_active_validator_addresses(&self) -> HaneulResult<Vec<HaneulAddress>> {
        // TODO: this is a hack to get the validator addresses. Currently using start state
        //       but we should have a better way to get this information after reconfig
        Ok(self.epoch_start_state().get_validator_addresses())
    }
}
