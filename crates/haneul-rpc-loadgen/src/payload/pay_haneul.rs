// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::payload::rpc_command_processor::{sign_and_execute, DEFAULT_GAS_BUDGET};
use crate::payload::{PayHaneul, ProcessPayload, RpcCommandProcessor, SignerInfo};
use async_trait::async_trait;
use futures::future::join_all;
use haneul_json_rpc_types::HaneulTransactionResponse;
use haneul_sdk::HaneulClient;
use haneul_types::base_types::{ObjectID, HaneulAddress};
use haneul_types::crypto::{EncodeDecodeBase64, HaneulKeyPair};
use tracing::debug;

#[async_trait]
impl<'a> ProcessPayload<'a, &'a PayHaneul> for RpcCommandProcessor {
    async fn process(
        &'a self,
        _op: &'a PayHaneul,
        signer_info: &Option<SignerInfo>,
    ) -> anyhow::Result<()> {
        let clients = self.get_clients().await?;
        let SignerInfo {
            encoded_keypair,
            gas_budget,
            gas_payment,
        } = signer_info.clone().unwrap();
        let recipient = HaneulAddress::random_for_testing_only();
        let amount = 1;
        let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
        let gas_payments = gas_payment.unwrap();

        let keypair =
            HaneulKeyPair::decode_base64(&encoded_keypair).expect("Decoding keypair should not fail");

        debug!(
            "Transfer Haneul {} time to {recipient} with {amount} GEUNHWA with {gas_payments:?}",
            gas_payments.len()
        );
        for client in clients.iter() {
            join_all(gas_payments.iter().map(|gas| async {
                transfer_haneul(client, &keypair, *gas, gas_budget, recipient, amount).await;
            }))
            .await;
        }

        Ok(())
    }
}

async fn transfer_haneul(
    client: &HaneulClient,
    keypair: &HaneulKeyPair,
    gas_payment: ObjectID,
    gas_budget: u64,
    recipient: HaneulAddress,
    amount: u64,
) -> HaneulTransactionResponse {
    let sender = HaneulAddress::from(&keypair.public());
    let tx = client
        .transaction_builder()
        .transfer_haneul(sender, gas_payment, gas_budget, recipient, Some(amount))
        .await
        .expect("Failed to construct transfer coin transaction");
    sign_and_execute(client, keypair, tx).await
}
