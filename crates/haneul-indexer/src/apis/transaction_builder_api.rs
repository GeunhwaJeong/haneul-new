// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::governance_api::GovernanceReadApi;
use crate::indexer_reader::IndexerReader;
use async_trait::async_trait;
use move_core_types::language_storage::StructTag;
use haneul_json_rpc::transaction_builder_api::TransactionBuilderApi as HaneulTransactionBuilderApi;
use haneul_json_rpc_types::{HaneulObjectDataFilter, HaneulObjectDataOptions, HaneulObjectResponse};
use haneul_transaction_builder::DataReader;
use haneul_types::base_types::{ObjectID, ObjectInfo, HaneulAddress};
use haneul_types::object::Object;

pub(crate) struct TransactionBuilderApi {
    inner: IndexerReader,
}

impl TransactionBuilderApi {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(inner: IndexerReader) -> HaneulTransactionBuilderApi {
        HaneulTransactionBuilderApi::new_with_data_reader(std::sync::Arc::new(Self { inner }))
    }
}

#[async_trait]
impl DataReader for TransactionBuilderApi {
    async fn get_owned_objects(
        &self,
        address: HaneulAddress,
        object_type: StructTag,
    ) -> Result<Vec<ObjectInfo>, anyhow::Error> {
        let stored_objects = self
            .inner
            .get_owned_objects(
                address,
                Some(HaneulObjectDataFilter::StructType(object_type)),
                None,
                50, // Limit the number of objects returned to 50
            )
            .await?;

        stored_objects
            .into_iter()
            .map(|object| {
                let object = Object::try_from(object)?;
                let object_ref = object.compute_object_reference();
                let info = ObjectInfo::new(&object_ref, &object);
                Ok(info)
            })
            .collect::<Result<Vec<_>, _>>()
    }

    async fn get_object_with_options(
        &self,
        object_id: ObjectID,
        options: HaneulObjectDataOptions,
    ) -> Result<HaneulObjectResponse, anyhow::Error> {
        let result = self.inner.get_object_read(object_id).await?;
        Ok((result, options).try_into()?)
    }

    async fn get_reference_gas_price(&self) -> Result<u64, anyhow::Error> {
        let epoch_info = GovernanceReadApi::new(self.inner.clone())
            .get_epoch_info(None)
            .await?;
        Ok(epoch_info
            .reference_gas_price
            .ok_or_else(|| anyhow::anyhow!("missing latest reference_gas_price"))?)
    }
}
