// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::types::object::Object;
use crate::types::protocol_config::ProtocolConfigs;
use async_graphql::*;
use async_trait::async_trait;
use haneul_json_rpc_types::HaneulObjectDataOptions;
use haneul_sdk::types::base_types::ObjectID;
use haneul_sdk::types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;

#[async_trait]
pub(crate) trait DataProvider: Send + Sync {
    async fn get_object_with_options(
        &self,
        object_id: ObjectID,
        options: HaneulObjectDataOptions,
    ) -> Result<Option<Object>>;

    async fn multi_get_object_with_options(
        &self,
        object_ids: Vec<ObjectID>,
        options: HaneulObjectDataOptions,
    ) -> Result<Vec<Object>>;

    async fn fetch_protocol_config(&self, version: Option<u64>) -> Result<ProtocolConfigs>;

    async fn get_latest_haneul_system_state(&self) -> Result<HaneulSystemStateSummary>;
}
