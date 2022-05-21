// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! IndexStore supports creation of various ancillary indexes of state in HaneulDataStore.
//! The main user of this data is the explorer.

use rocksdb::Options;
use serde::{de::DeserializeOwned, Serialize};

use std::path::Path;
use haneul_types::base_types::{ObjectID, HaneulAddress, TransactionDigest};
use haneul_types::batch::TxSequenceNumber;

use haneul_types::error::HaneulResult;

use haneul_types::object::Object;

use typed_store::rocks::DBMap;
use typed_store::{reopen, traits::Map};

pub struct IndexStore {
    /// Index from haneul address to transactions initiated by that address.
    transactions_from_addr: DBMap<(HaneulAddress, TxSequenceNumber), TransactionDigest>,

    /// Index from haneul address to transactions that were sent to that address.
    transactions_to_addr: DBMap<(HaneulAddress, TxSequenceNumber), TransactionDigest>,

    /// Index from object id to transactions that used that object id as input.
    transactions_by_input_object_id: DBMap<(ObjectID, TxSequenceNumber), TransactionDigest>,

    /// Index from object id to transactions that modified/created that object id.
    transactions_by_mutated_object_id: DBMap<(ObjectID, TxSequenceNumber), TransactionDigest>,
}

impl IndexStore {
    pub fn open<P: AsRef<Path>>(path: P, db_options: Option<Options>) -> Self {
        let mut options = db_options.unwrap_or_default();

        // The table cache is locked for updates and this determines the number
        // of shareds, ie 2^10. Increase in case of lock contentions.
        let row_cache = rocksdb::Cache::new_lru_cache(1_000_000).expect("Cache is ok");
        options.set_row_cache(&row_cache);
        options.set_table_cache_num_shard_bits(10);
        options.set_compression_type(rocksdb::DBCompressionType::None);

        let db = {
            let path = &path;
            let db_options = Some(options.clone());
            let opt_cfs: &[(&str, &rocksdb::Options)] = &[
                ("transactions_from_addr", &options),
                ("transactions_to_addr", &options),
                ("transactions_by_input_object_id", &options),
                ("transactions_by_mutated_object_id", &options),
            ];
            typed_store::rocks::open_cf_opts(path, db_options, opt_cfs)
        }
        .expect("Cannot open DB.");

        let (
            transactions_from_addr,
            transactions_to_addr,
            transactions_by_input_object_id,
            transactions_by_mutated_object_id,
        ) = reopen!(
            &db,
            "transactions_from_addr"; <(HaneulAddress, TxSequenceNumber), TransactionDigest>,
            "transactions_to_addr"; <(HaneulAddress, TxSequenceNumber), TransactionDigest>,
            "transactions_by_input_object_id"; <(ObjectID, TxSequenceNumber), TransactionDigest>,
            "transactions_by_mutated_object_id"; <(ObjectID, TxSequenceNumber), TransactionDigest>
        );

        Self {
            transactions_from_addr,
            transactions_to_addr,
            transactions_by_input_object_id,
            transactions_by_mutated_object_id,
        }
    }

    pub fn index_tx(
        &self,
        sender: HaneulAddress,
        active_inputs: &[&Object],
        mutated_objects: &[&Object],
        sequence: TxSequenceNumber,
        digest: &TransactionDigest,
    ) -> HaneulResult {
        let batch = self.transactions_from_addr.batch();

        let batch = batch.insert_batch(
            &self.transactions_from_addr,
            std::iter::once(((sender, sequence), *digest)),
        )?;

        let batch = batch.insert_batch(
            &self.transactions_by_input_object_id,
            active_inputs
                .iter()
                .map(|object| ((object.id(), sequence), *digest)),
        )?;

        let batch = batch.insert_batch(
            &self.transactions_by_mutated_object_id,
            mutated_objects
                .iter()
                .map(|object| ((object.id(), sequence), *digest)),
        )?;

        let batch = batch.insert_batch(
            &self.transactions_to_addr,
            mutated_objects.iter().filter_map(|object| {
                object
                    .get_single_owner()
                    .map(|addr| ((addr, sequence), digest))
            }),
        )?;

        batch.write()?;

        Ok(())
    }

    fn get_transactions_by_object<
        KeyT: Clone + Serialize + DeserializeOwned + std::cmp::PartialEq,
    >(
        index: &DBMap<(KeyT, TxSequenceNumber), TransactionDigest>,
        object_id: KeyT,
    ) -> HaneulResult<Vec<(TxSequenceNumber, TransactionDigest)>> {
        Ok(index
            .iter()
            .skip_to(&(object_id.clone(), TxSequenceNumber::MIN))?
            .take_while(|((id, _), _)| *id == object_id)
            .map(|((_, seq), digest)| (seq, digest))
            .collect())
    }

    pub fn get_transactions_by_input_object(
        &self,
        input_object: ObjectID,
    ) -> HaneulResult<Vec<(TxSequenceNumber, TransactionDigest)>> {
        Self::get_transactions_by_object(&self.transactions_by_input_object_id, input_object)
    }

    pub fn get_transactions_by_mutated_object(
        &self,
        mutated_object: ObjectID,
    ) -> HaneulResult<Vec<(TxSequenceNumber, TransactionDigest)>> {
        Self::get_transactions_by_object(&self.transactions_by_mutated_object_id, mutated_object)
    }

    pub fn get_transactions_from_addr(
        &self,
        addr: HaneulAddress,
    ) -> HaneulResult<Vec<(TxSequenceNumber, TransactionDigest)>> {
        Self::get_transactions_by_object(&self.transactions_from_addr, addr)
    }

    pub fn get_transactions_to_addr(
        &self,
        addr: HaneulAddress,
    ) -> HaneulResult<Vec<(TxSequenceNumber, TransactionDigest)>> {
        Self::get_transactions_by_object(&self.transactions_to_addr, addr)
    }
}
