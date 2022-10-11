// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    helper::{ObjectChecker, TransferObjectEventChecker},
    TestCaseImpl, TestContext,
};
use async_trait::async_trait;
use jsonrpsee::rpc_params;
use haneul_json_rpc_types::{HaneulCertifiedTransaction, HaneulTransactionEffects};
use haneul_types::{
    base_types::{ObjectID, HaneulAddress},
    crypto::{get_key_pair, AccountKeyPair},
    event::TransferType,
    object::Owner,
    HANEUL_FRAMEWORK_OBJECT_ID,
};
use tracing::info;
pub struct NativeTransferTest;

#[async_trait]
impl TestCaseImpl for NativeTransferTest {
    fn name(&self) -> &'static str {
        "NativeTransfer"
    }

    fn description(&self) -> &'static str {
        "Test tranferring HANEUL coins natively"
    }

    async fn run(&self, ctx: &mut TestContext) -> Result<(), anyhow::Error> {
        info!("Testing gas coin transfer");
        let mut haneul_objs = ctx.get_haneul_from_faucet(Some(4)).await;
        let gas_obj = haneul_objs.swap_remove(0);
        let signer = ctx.get_wallet_address();
        let (recipient_addr, _): (_, AccountKeyPair) = get_key_pair();

        // Test transfer object
        let obj_to_transfer = *haneul_objs.swap_remove(0).id();
        let params = rpc_params![
            signer,
            obj_to_transfer,
            Some(*gas_obj.id()),
            5000,
            recipient_addr
        ];
        let data = ctx
            .build_transaction_remotely("haneul_transferObject", params)
            .await?;
        let (tx_cert, effects) = ctx.sign_and_execute(data, "coin transfer").await;

        Self::examine_response(
            ctx,
            tx_cert,
            effects,
            signer,
            recipient_addr,
            obj_to_transfer,
        )
        .await;

        // Test transfer haneul
        let obj_to_transfer = *haneul_objs.swap_remove(0).id();
        let params = rpc_params![signer, obj_to_transfer, 5000, recipient_addr, None::<u64>];
        let data = ctx
            .build_transaction_remotely("haneul_transferHaneul", params)
            .await?;
        let (tx_cert, effects) = ctx.sign_and_execute(data, "coin transfer").await;

        Self::examine_response(
            ctx,
            tx_cert,
            effects,
            signer,
            recipient_addr,
            obj_to_transfer,
        )
        .await;
        Ok(())
    }
}

impl NativeTransferTest {
    async fn examine_response(
        ctx: &TestContext,
        tx_cert: HaneulCertifiedTransaction,
        mut effects: HaneulTransactionEffects,
        signer: HaneulAddress,
        recipient: HaneulAddress,
        obj_to_transfer_id: ObjectID,
    ) {
        let events = &mut effects.events;
        assert_eq!(
            events.len(),
            1,
            "Expect one event emitted, but got {}",
            events.len()
        );
        let event = events.remove(0);

        TransferObjectEventChecker::new()
            .package_id(HANEUL_FRAMEWORK_OBJECT_ID)
            .transaction_module("native".into())
            .sender(signer)
            .recipient(Owner::AddressOwner(recipient))
            .object_id(obj_to_transfer_id)
            .type_(TransferType::Coin)
            .check(&event);

        // Verify fullnode observes the txn
        ctx.let_fullnode_sync(vec![tx_cert.transaction_digest], 5)
            .await;

        let _ = ObjectChecker::new(obj_to_transfer_id)
            .owner(Owner::AddressOwner(recipient))
            .check(ctx.get_fullnode_client())
            .await;
    }
}
