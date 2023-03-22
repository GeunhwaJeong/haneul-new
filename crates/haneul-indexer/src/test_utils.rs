// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use prometheus::Registry;
use haneul_json_rpc_types::HaneulTransactionResponse;
use tokio::task::JoinHandle;

use crate::errors::IndexerError;
use crate::store::PgIndexerStore;
use crate::utils::reset_database;
use crate::{new_pg_connection_pool, Indexer, IndexerConfig};

/// Spawns an indexer thread with provided Postgres DB url
pub async fn start_test_indexer(
    config: IndexerConfig,
) -> Result<(PgIndexerStore, JoinHandle<Result<(), IndexerError>>), anyhow::Error> {
    let pg_connection_pool = new_pg_connection_pool(&config.base_connection_url())
        .await
        .map_err(|e| anyhow!("unable to connect to Postgres, is it running? {e}"))?;
    if config.reset_db {
        reset_database(
            &mut pg_connection_pool
                .get()
                .map_err(|e| anyhow!("Fail to get pg_connection_pool {e}"))?,
            true,
        )?;
    }
    let store = PgIndexerStore::new(pg_connection_pool);

    let registry = Registry::default();
    let store_clone = store.clone();
    let handle = tokio::spawn(async move { Indexer::start(&config, &registry, store_clone).await });
    Ok((store, handle))
}

#[derive(Clone)]
pub struct HaneulTransactionResponseBuilder<'a> {
    response: HaneulTransactionResponse,
    full_response: &'a HaneulTransactionResponse,
    required_fields: &'a HaneulTransactionResponse,
}

impl<'a> HaneulTransactionResponseBuilder<'a> {
    pub fn new(
        full_response: &'a HaneulTransactionResponse,
        required_fields: &'a HaneulTransactionResponse,
    ) -> Self {
        Self {
            response: HaneulTransactionResponse::default(),
            full_response,
            required_fields,
        }
    }

    pub fn with_transaction(mut self) -> Self {
        self.response = HaneulTransactionResponse {
            transaction: self.full_response.transaction.clone(),
            ..self.response
        };
        self
    }

    pub fn with_raw_transaction(mut self) -> Self {
        self.response = HaneulTransactionResponse {
            raw_transaction: self.full_response.raw_transaction.clone(),
            ..self.response
        };
        self
    }

    pub fn with_effects(mut self) -> Self {
        self.response = HaneulTransactionResponse {
            effects: self.full_response.effects.clone(),
            ..self.response
        };
        self
    }

    pub fn with_events(mut self) -> Self {
        self.response = HaneulTransactionResponse {
            events: self.full_response.events.clone(),
            ..self.response
        };
        self
    }

    pub fn with_balance_changes(mut self) -> Self {
        self.response = HaneulTransactionResponse {
            balance_changes: self.full_response.balance_changes.clone(),
            ..self.response
        };
        self
    }

    pub fn with_object_changes(mut self) -> Self {
        self.response = HaneulTransactionResponse {
            object_changes: self.full_response.object_changes.clone(),
            ..self.response
        };
        self
    }

    pub fn with_input_and_changes(mut self) -> Self {
        self.response = HaneulTransactionResponse {
            transaction: self.full_response.transaction.clone(),
            balance_changes: self.full_response.balance_changes.clone(),
            object_changes: self.full_response.object_changes.clone(),
            ..self.response
        };
        self
    }

    pub fn with_all(mut self) -> Self {
        self.response = HaneulTransactionResponse {
            transaction: self.full_response.transaction.clone(),
            raw_transaction: self.full_response.raw_transaction.clone(),
            effects: self.full_response.effects.clone(),
            events: self.full_response.events.clone(),
            balance_changes: self.full_response.balance_changes.clone(),
            object_changes: self.full_response.object_changes.clone(),
            ..self.response
        };
        self
    }

    pub fn build(self) -> HaneulTransactionResponse {
        HaneulTransactionResponse {
            transaction: self.response.transaction,
            raw_transaction: self.response.raw_transaction,
            effects: self.response.effects,
            events: self.response.events,
            balance_changes: self.response.balance_changes,
            object_changes: self.response.object_changes,
            ..self.required_fields.clone()
        }
    }
}
