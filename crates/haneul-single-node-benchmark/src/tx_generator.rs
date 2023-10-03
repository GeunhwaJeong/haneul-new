// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

pub use move_tx_generator::MoveTxGenerator;
pub use non_move_tx_generator::NonMoveTxGenerator;
pub use root_object_create_tx_generator::RootObjectCreateTxGenerator;
use std::sync::Arc;
use haneul_types::base_types::{ObjectRef, HaneulAddress};
use haneul_types::crypto::AccountKeyPair;
use haneul_types::transaction::Transaction;

mod move_tx_generator;
mod non_move_tx_generator;
mod root_object_create_tx_generator;

pub(crate) trait TxGenerator: Send + Sync {
    /// Given a sender address, a keypair for that address, and a list of gas objects owned
    /// by this address, generate a single transaction.
    fn generate_tx(
        &self,
        sender: HaneulAddress,
        keypair: Arc<AccountKeyPair>,
        gas_objects: Arc<Vec<ObjectRef>>,
    ) -> Transaction;

    fn name(&self) -> &'static str;
}
