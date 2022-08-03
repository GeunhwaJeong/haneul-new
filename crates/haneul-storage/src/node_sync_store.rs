// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_types::{
    base_types::TransactionDigest,
    error::HaneulResult,
    messages::{CertifiedTransaction, SignedTransactionEffects},
};

use typed_store::rocks::DBMap;
use typed_store::traits::DBMapTableUtil;
use typed_store::traits::Map;
use typed_store_macros::DBMapUtils;

/// NodeSyncStore store is used by nodes to store downloaded objects (certs, etc) that have
/// not yet been applied to the node's HaneulDataStore.
#[derive(DBMapUtils)]
pub struct NodeSyncStore {
    /// Certificates/Effects that have been fetched from remote validators, but not sequenced.
    certs_and_fx: DBMap<TransactionDigest, (CertifiedTransaction, SignedTransactionEffects)>,
}

impl NodeSyncStore {
    pub fn has_cert_and_effects(&self, tx: &TransactionDigest) -> HaneulResult<bool> {
        Ok(self.certs_and_fx.contains_key(tx)?)
    }

    pub fn store_cert_and_effects(
        &self,
        tx: &TransactionDigest,
        val: &(CertifiedTransaction, SignedTransactionEffects),
    ) -> HaneulResult {
        Ok(self.certs_and_fx.insert(tx, val)?)
    }

    pub fn get_cert_and_effects(
        &self,
        tx: &TransactionDigest,
    ) -> HaneulResult<Option<(CertifiedTransaction, SignedTransactionEffects)>> {
        Ok(self.certs_and_fx.get(tx)?)
    }

    pub fn delete_cert_and_effects(&self, tx: &TransactionDigest) -> HaneulResult {
        Ok(self.certs_and_fx.remove(tx)?)
    }
}
