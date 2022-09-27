// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{helper::ObjectChecker, TestCaseImpl, TestContext};
use async_trait::async_trait;
use haneul::client_commands::WalletContext;
use haneul_json_rpc_types::{HaneulExecutionStatus, HaneulTransactionResponse};
use haneul_types::base_types::SequenceNumber;
use haneul_types::object::Owner;
use test_utils::transaction::{increment_counter, publish_basics_package_and_make_counter};
use tracing::info;

pub struct SharedCounterTest;

#[async_trait]
impl TestCaseImpl for SharedCounterTest {
    fn name(&self) -> &'static str {
        "SharedCounter"
    }

    fn description(&self) -> &'static str {
        "Test publishing basics packages and incrementing Counter (shared object)"
    }

    async fn run(&self, ctx: &mut TestContext) -> Result<(), anyhow::Error> {
        info!("Testing shared object transactions.");

        let haneul_objs = ctx.get_haneul_from_faucet(Some(1)).await;
        assert!(!haneul_objs.is_empty());

        let wallet_context: &WalletContext = ctx.get_wallet();
        let address = ctx.get_wallet_address();
        let (package_ref, counter_id) =
            publish_basics_package_and_make_counter(wallet_context, address).await;

        let response: HaneulTransactionResponse =
            increment_counter(wallet_context, address, None, package_ref, counter_id).await;
        let effects = response.effects;
        assert_eq!(
            effects.status,
            HaneulExecutionStatus::Success,
            "Increment counter txn failed: {:?}",
            effects.status
        );
        effects
            .shared_objects
            .iter()
            .find(|o| o.object_id == counter_id)
            .expect("Expect obj {counter_id} in shared_objects");

        // Verify fullnode observes the txn
        ctx.let_fullnode_sync(vec![response.certificate.transaction_digest], 5)
            .await;

        let counter_object = ObjectChecker::new(counter_id)
            .owner(Owner::Shared)
            .check_into_object(ctx.get_fullnode())
            .await;

        assert_eq!(
            counter_object.reference.version,
            SequenceNumber::from_u64(2),
            "Expect sequence number to be 2"
        );

        Ok(())
    }
}
