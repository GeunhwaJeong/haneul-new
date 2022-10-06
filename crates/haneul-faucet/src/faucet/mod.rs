// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::FaucetError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use haneul_types::base_types::{ObjectID, HaneulAddress, TransactionDigest};
use uuid::Uuid;

mod simple_faucet;
pub use self::simple_faucet::SimpleFaucet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FaucetReceipt {
    pub sent: Vec<CoinInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinInfo {
    pub amount: u64,
    pub id: ObjectID,
    pub transfer_tx_digest: TransactionDigest,
}

#[async_trait]
pub trait Faucet {
    /// Send `Coin<HANEUL>` of the specified amount to the recipient
    async fn send(
        &self,
        id: Uuid,
        recipient: HaneulAddress,
        amounts: &[u64],
    ) -> Result<FaucetReceipt, FaucetError>;
}
