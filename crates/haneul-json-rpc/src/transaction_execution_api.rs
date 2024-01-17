// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use fastcrypto::traits::ToFromBytes;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use haneullabs_metrics::spawn_monitored_task;
use shared_crypto::intent::{AppId, Intent, IntentMessage, IntentScope, IntentVersion};
use haneul_core::authority::AuthorityState;
use haneul_core::authority_client::NetworkAuthorityClient;
use haneul_core::transaction_orchestrator::TransactiondOrchestrator;
use haneul_json_rpc_api::{JsonRpcMetrics, WriteApiOpenRpc, WriteApiServer};
use haneul_json_rpc_types::{
    DevInspectArgs, DevInspectResults, DryRunTransactionBlockResponse, HaneulTransactionBlock,
    HaneulTransactionBlockEvents, HaneulTransactionBlockResponse, HaneulTransactionBlockResponseOptions,
};
use haneul_open_rpc::Module;
use haneul_types::base_types::HaneulAddress;
use haneul_types::crypto::default_hash;
use haneul_types::digests::TransactionDigest;
use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::quorum_driver_types::{
    ExecuteTransactionRequest, ExecuteTransactionRequestType, ExecuteTransactionResponse,
};
use haneul_types::signature::GenericSignature;
use haneul_types::haneul_serde::BigInt;
use haneul_types::transaction::{
    InputObjectKind, Transaction, TransactionData, TransactionDataAPI, TransactionKind,
};
use tracing::instrument;

use crate::authority_state::StateRead;
use crate::error::{Error, HaneulRpcInputError};
use crate::{
    get_balance_changes_from_effect, get_object_changes, with_tracing, ObjectProviderCache,
    HaneulRpcModule,
};

pub struct TransactionExecutionApi {
    state: Arc<dyn StateRead>,
    transaction_orchestrator: Arc<TransactiondOrchestrator<NetworkAuthorityClient>>,
    metrics: Arc<JsonRpcMetrics>,
}

impl TransactionExecutionApi {
    pub fn new(
        state: Arc<AuthorityState>,
        transaction_orchestrator: Arc<TransactiondOrchestrator<NetworkAuthorityClient>>,
        metrics: Arc<JsonRpcMetrics>,
    ) -> Self {
        Self {
            state,
            transaction_orchestrator,
            metrics,
        }
    }

    pub fn convert_bytes<T: serde::de::DeserializeOwned>(
        &self,
        tx_bytes: Base64,
    ) -> Result<T, HaneulRpcInputError> {
        let data: T = bcs::from_bytes(&tx_bytes.to_vec()?)?;
        Ok(data)
    }

    #[allow(clippy::type_complexity)]
    fn prepare_execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        opts: Option<HaneulTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> Result<
        (
            HaneulTransactionBlockResponseOptions,
            ExecuteTransactionRequestType,
            HaneulAddress,
            Vec<InputObjectKind>,
            Transaction,
            Option<HaneulTransactionBlock>,
            Vec<u8>,
        ),
        HaneulRpcInputError,
    > {
        let opts = opts.unwrap_or_default();
        let request_type = match (request_type, opts.require_local_execution()) {
            (Some(ExecuteTransactionRequestType::WaitForEffectsCert), true) => {
                Err(HaneulRpcInputError::InvalidExecuteTransactionRequestType)?
            }
            (t, _) => t.unwrap_or_else(|| opts.default_execution_request_type()),
        };
        let tx_data: TransactionData = self.convert_bytes(tx_bytes)?;
        let sender = tx_data.sender();
        let input_objs = tx_data.input_objects().unwrap_or_default();

        let mut sigs = Vec::new();
        for sig in signatures {
            sigs.push(GenericSignature::from_bytes(&sig.to_vec()?)?);
        }
        let txn = Transaction::from_generic_sig_data(tx_data, sigs);
        let raw_transaction = if opts.show_raw_input {
            bcs::to_bytes(txn.data())?
        } else {
            vec![]
        };
        let transaction = if opts.show_input {
            let epoch_store = self.state.load_epoch_store_one_call_per_task();
            Some(HaneulTransactionBlock::try_from(
                txn.data().clone(),
                epoch_store.module_cache(),
            )?)
        } else {
            None
        };
        Ok((
            opts,
            request_type,
            sender,
            input_objs,
            txn,
            transaction,
            raw_transaction,
        ))
    }

    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        opts: Option<HaneulTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> Result<HaneulTransactionBlockResponse, Error> {
        let (opts, request_type, sender, input_objs, txn, transaction, raw_transaction) =
            self.prepare_execute_transaction_block(tx_bytes, signatures, opts, request_type)?;
        let digest = *txn.digest();

        let transaction_orchestrator = self.transaction_orchestrator.clone();
        let orch_timer = self.metrics.orchestrator_latency_ms.start_timer();
        let response = spawn_monitored_task!(transaction_orchestrator.execute_transaction_block(
            ExecuteTransactionRequest {
                transaction: txn,
                request_type,
            }
        ))
        .await?
        .map_err(Error::from)?;
        drop(orch_timer);

        let _post_orch_timer = self.metrics.post_orchestrator_latency_ms.start_timer();
        let ExecuteTransactionResponse::EffectsCert(cert) = response;
        let (effects, transaction_events, is_executed_locally) = *cert;
        let mut events: Option<HaneulTransactionBlockEvents> = None;
        if opts.show_events {
            let epoch_store = self.state.load_epoch_store_one_call_per_task();
            let mut layout_resolver = epoch_store
                .executor()
                .type_layout_resolver(Box::new(self.state.get_db()));
            events = Some(HaneulTransactionBlockEvents::try_from(
                transaction_events,
                digest,
                None,
                layout_resolver.as_mut(),
            )?);
        }

        let object_cache = ObjectProviderCache::new(self.state.clone());
        let balance_changes = if opts.show_balance_changes && is_executed_locally {
            Some(
                get_balance_changes_from_effect(&object_cache, &effects.effects, input_objs, None)
                    .await?,
            )
        } else {
            None
        };
        let object_changes = if opts.show_object_changes && is_executed_locally {
            Some(
                get_object_changes(
                    &object_cache,
                    sender,
                    effects.effects.modified_at_versions(),
                    effects.effects.all_changed_objects(),
                    effects.effects.all_removed_objects(),
                )
                .await?,
            )
        } else {
            None
        };

        Ok(HaneulTransactionBlockResponse {
            digest,
            transaction,
            raw_transaction,
            effects: opts.show_effects.then_some(effects.effects.try_into()?),
            events,
            object_changes,
            balance_changes,
            timestamp_ms: None,
            confirmed_local_execution: Some(is_executed_locally),
            checkpoint: None,
            errors: vec![],
        })
    }

    pub fn prepare_dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> Result<(TransactionData, TransactionDigest, Vec<InputObjectKind>), HaneulRpcInputError> {
        let tx_data: TransactionData = self.convert_bytes(tx_bytes)?;
        let input_objs = tx_data.input_objects()?;
        let intent_msg = IntentMessage::new(
            Intent {
                version: IntentVersion::V0,
                scope: IntentScope::TransactionData,
                app_id: AppId::Haneul,
            },
            tx_data,
        );
        let txn_digest = TransactionDigest::new(default_hash(&intent_msg.value));
        Ok((intent_msg.value, txn_digest, input_objs))
    }

    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> Result<DryRunTransactionBlockResponse, Error> {
        let (txn_data, txn_digest, input_objs) =
            self.prepare_dry_run_transaction_block(tx_bytes)?;
        let sender = txn_data.sender();
        let (resp, written_objects, transaction_effects, mock_gas) = self
            .state
            .dry_exec_transaction(txn_data.clone(), txn_digest)
            .await?;
        let object_cache = ObjectProviderCache::new_with_cache(self.state.clone(), written_objects);
        let balance_changes = get_balance_changes_from_effect(
            &object_cache,
            &transaction_effects,
            input_objs,
            mock_gas,
        )
        .await?;
        let object_changes = get_object_changes(
            &object_cache,
            sender,
            transaction_effects.modified_at_versions(),
            transaction_effects.all_changed_objects(),
            transaction_effects.all_removed_objects(),
        )
        .await?;

        Ok(DryRunTransactionBlockResponse {
            effects: resp.effects,
            events: resp.events,
            object_changes,
            balance_changes,
            input: resp.input,
        })
    }
}

#[async_trait]
impl WriteApiServer for TransactionExecutionApi {
    #[instrument(skip(self))]
    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        opts: Option<HaneulTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<HaneulTransactionBlockResponse> {
        with_tracing!(Duration::from_secs(10), async move {
            self.execute_transaction_block(tx_bytes, signatures, opts, request_type)
                .await
        })
    }

    #[instrument(skip(self))]
    async fn dev_inspect_transaction_block(
        &self,
        sender_address: HaneulAddress,
        tx_bytes: Base64,
        gas_price: Option<BigInt<u64>>,
        epoch: Option<BigInt<u64>>,
        additional_args: Option<DevInspectArgs>,
    ) -> RpcResult<DevInspectResults> {
        with_tracing!(async move {
            let DevInspectArgs {
                gas_sponsor,
                gas_budget,
                gas_objects,
                skip_checks,
            } = additional_args.unwrap_or_default();
            let tx_kind: TransactionKind = self.convert_bytes(tx_bytes)?;
            self.state
                .dev_inspect_transaction_block(
                    sender_address,
                    tx_kind,
                    gas_price.map(|i| *i),
                    gas_budget.map(|i| *i),
                    gas_sponsor,
                    gas_objects,
                    epoch.map(|i| *i),
                    skip_checks,
                )
                .await
                .map_err(Error::from)
        })
    }

    #[instrument(skip(self))]
    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        with_tracing!(async move { self.dry_run_transaction_block(tx_bytes).await })
    }
}

impl HaneulRpcModule for TransactionExecutionApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        WriteApiOpenRpc::module_doc()
    }
}
