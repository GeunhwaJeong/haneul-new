// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// TODO remove when integrated
#![allow(unused)]

use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use haneul_sdk::{HaneulClient as HaneulClientInner, HaneulClientBuilder};
use haneul_types::base_types::HaneulAddress;

use crate::error::BridgeResult;

pub(crate) struct HaneulClient {
    inner: HaneulClientInner,
}

impl HaneulClient {
    pub async fn new(rpc_url: &str) -> anyhow::Result<Self> {
        let inner = HaneulClientBuilder::default().build(rpc_url).await?;
        let self_ = Self { inner };
        self_.describe().await?;
        Ok(self_)
    }

    // TODO assert chain identifier
    async fn describe(&self) -> anyhow::Result<()> {
        let chain_id = self.inner.read_api().get_chain_identifier().await?;
        let block_number = self
            .inner
            .read_api()
            .get_latest_checkpoint_sequence_number()
            .await?;
        tracing::info!(
            "HaneulClient is connected to chain {chain_id}, current block number: {block_number}"
        );
        Ok(())
    }

    pub async fn get_bridge_events_maybe(
        &self,
        tx_digest: &str,
    ) -> BridgeResult<Vec<HaneulBridgeEvent>> {
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaneulToEthBridgeEvent {
    pub source_address: HaneulAddress,
    pub destination_address: Address,
    pub coin_name: String,
    pub amount: U256,
}

pub enum HaneulBridgeEvent {
    HaneulToEthBridge(HaneulToEthBridgeEvent),
}
