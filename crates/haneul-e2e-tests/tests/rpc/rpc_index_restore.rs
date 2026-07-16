// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Rebuild behavior of the legacy `rpc-index` index backend (`enable_indexing`).
//!
//! A node that ran without indexing and is restarted with it on rebuilds the
//! `rpc-index` from the perpetual store: `RpcIndexStore::init` bulk-loads the
//! live object set (the `balance` index) and backfills the historical
//! checkpoint indexes (the `transaction_bitmap`), then the checkpoint executor
//! resumes forward indexing from `highest_executed`.
//!
//! The bulk restore reads the live object set, which the perpetual store writes
//! atomically with the `highest_committed` watermark. The checkpoint store's
//! `highest_executed` watermark is bumped in a *separate* write afterward, so an
//! unclean stop can leave the live set reflecting a checkpoint that
//! `highest_executed` does not yet count. If the restore stamps its watermark at
//! the lagging `highest_executed`, the executor re-applies the checkpoints in
//! `(highest_executed, highest_committed]` that the live set already captured,
//! double-counting additive indexes -- a recipient's coin balance shows up at
//! twice its value. This test exercises that path: it asserts the recipient's
//! balance is reported exactly once after the rebuild.
//!
//! The unit under test is a fullnode spawned into the swarm separately from the
//! cluster's primary fullnode. The cluster's wallet executes transactions
//! against the primary (which stays up the whole time); the dedicated node
//! follows by state sync and is restarted with a mutated `NodeConfig.rpc`
//! between runs. Its `db_path` is stable across restarts, so each restart sees
//! the previous run's perpetual store and exercises the real rebuild path.
//!
//! Restart correctness note: the swarm node holds only a `Weak` reference to the
//! running `HaneulNode`, so a stop releases the node's RocksDB locks only if no
//! strong handle outlives it. These helpers therefore never retain a
//! `HaneulNodeHandle` across a restart -- reads fetch a transient handle and drop
//! it immediately.

use std::collections::HashSet;
use std::time::Duration;

use haneul_config::RpcConfig;
use haneul_macros::sim_test;
use haneul_node::HaneulNode;
use haneul_rpc::Client;
use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::proto::haneul::rpc::v2::GetBalanceRequest;
use haneul_rpc::proto::haneul::rpc::v2alpha::ListTransactionsRequest;
use haneul_rpc::proto::haneul::rpc::v2alpha::QueryOptions;
use haneul_rpc::proto::haneul::rpc::v2alpha::SenderFilter;
use haneul_rpc::proto::haneul::rpc::v2alpha::TransactionFilter;
use haneul_rpc::proto::haneul::rpc::v2alpha::TransactionLiteral;
use haneul_rpc::proto::haneul::rpc::v2alpha::TransactionPredicate;
use haneul_rpc::proto::haneul::rpc::v2alpha::TransactionTerm;
use haneul_rpc::proto::haneul::rpc::v2alpha::ledger_service_client::LedgerServiceClient;
use haneul_rpc::proto::haneul::rpc::v2alpha::list_transactions_response;
use haneul_rpc::proto::haneul::rpc::v2alpha::transaction_literal;
use haneul_rpc::proto::haneul::rpc::v2alpha::transaction_predicate;
use haneul_test_transaction_builder::make_transfer_haneul_transaction;
use haneul_types::base_types::AuthorityName;
use haneul_types::base_types::HaneulAddress;
use haneul_types::base_types::TransactionDigest;
use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::transaction::TransactionDataAPI;
use prost_types::FieldMask;
use rand::rngs::OsRng;
use test_cluster::TestCluster;
use test_cluster::TestClusterBuilder;

const HANEUL_COIN_TYPE: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000002::haneul::HANEUL";

/// How long to wait for the dedicated fullnode to sync and index a target
/// checkpoint. A rebuild reloads the live object set and backfills the history
/// indexes from genesis, so this is generous.
const WAIT_TIMEOUT: Duration = Duration::from_secs(60);

/// An rpc config that builds the legacy `rpc-index` with the ledger-history
/// (bitmap) indexes enabled.
fn legacy_indexing_config() -> RpcConfig {
    RpcConfig {
        enable_indexing: Some(true),
        ledger_history_indexing: Some(true),
        ..Default::default()
    }
}

/// An rpc config that builds no index at all.
fn no_indexing_config() -> RpcConfig {
    RpcConfig {
        enable_indexing: Some(false),
        ..Default::default()
    }
}

/// Spawn a dedicated fullnode into the swarm with `rpc`, returning its (stable)
/// name and rpc url. The handle `spawn_new_node` returns is dropped immediately
/// so the node keeps no external strong reference and a later
/// [`restart_fullnode`] can release its DB locks on stop.
async fn spawn_fullnode(cluster: &mut TestCluster, rpc: RpcConfig) -> (AuthorityName, String) {
    let config = cluster
        .fullnode_config_builder()
        .with_rpc_config(rpc)
        .build(&mut OsRng, cluster.swarm.config());
    let name = config.protocol_public_key();
    let rpc_url = format!("http://{}", config.json_rpc_address);
    cluster.swarm.spawn_new_node(config).await;
    (name, rpc_url)
}

/// Stop the fullnode `name`, swap in `rpc` (the `db_path` is unchanged), and
/// restart it. The stop releases the previous run's RocksDB locks because no
/// strong `HaneulNodeHandle` is held across the call.
async fn restart_fullnode(cluster: &TestCluster, name: &AuthorityName, rpc: RpcConfig) {
    let node = cluster.swarm.node(name).unwrap();
    node.stop();
    node.config().rpc = Some(rpc);
    // Under simulation, stopping a node only schedules its teardown via
    // `delete_node`; the old node's RocksDB handles (and on-disk file locks)
    // are released asynchronously. Restarting immediately races the previous
    // instance for the same `db_path` and panics opening it. Give the simulator
    // time to finish the teardown. A real runtime joins the node's thread on
    // stop, so the handles are already released there.
    if cfg!(msim) {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    node.start().await.unwrap();
}

/// Run `f` against the dedicated fullnode through a transient handle that is
/// dropped before returning (see the module note on restart safety).
fn with_node<T>(
    cluster: &TestCluster,
    name: &AuthorityName,
    f: impl FnOnce(&HaneulNode) -> T,
) -> T {
    let handle = cluster.swarm.node(name).unwrap().get_node_handle().unwrap();
    handle.with(f)
}

fn has_rpc_index(cluster: &TestCluster, name: &AuthorityName) -> bool {
    with_node(cluster, name, |node| node.state().rpc_index.is_some())
}

/// The highest checkpoint the dedicated node's `rpc-index` has indexed, or
/// `None` if the node has no index.
fn highest_indexed(cluster: &TestCluster, name: &AuthorityName) -> Option<u64> {
    with_node(cluster, name, |node| {
        node.state().rpc_index.as_ref().and_then(|index| {
            index
                .get_highest_indexed_checkpoint_seq_number()
                .ok()
                .flatten()
        })
    })
}

/// Block until the dedicated fullnode's `rpc-index` has indexed through
/// `target`. Panics on timeout with the last-observed watermark.
async fn wait_for_indexed(cluster: &TestCluster, name: &AuthorityName, target: u64) {
    let deadline = tokio::time::Instant::now() + WAIT_TIMEOUT;
    loop {
        let indexed = highest_indexed(cluster, name);
        if indexed.is_some_and(|c| c >= target) {
            return;
        }
        if tokio::time::Instant::now() >= deadline {
            panic!(
                "timed out waiting for rpc-index to index checkpoint {target} \
                 (indexed={indexed:?})"
            );
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// A transfer executed against the cluster, with the facts a test asserts on
/// afterward.
struct Transfer {
    /// The transaction's sender (whichever account funded the gas).
    sender: HaneulAddress,
    /// A fresh address with no prior coins, so its post-transfer HANEUL balance is
    /// exactly `amount`.
    receiver: HaneulAddress,
    amount: u64,
    digest: TransactionDigest,
}

/// Transfer `amount` GEUNHWA of HANEUL to a fresh address through the cluster's
/// primary fullnode and wait for the transaction to land in an executed
/// checkpoint.
async fn transfer_to_fresh_address(cluster: &TestCluster, amount: u64) -> Transfer {
    let receiver = HaneulAddress::random_for_testing_only();
    let txn = make_transfer_haneul_transaction(&cluster.wallet, Some(receiver), Some(amount)).await;
    let executed = cluster.execute_transaction(txn).await;
    let transfer = Transfer {
        sender: executed.transaction.sender(),
        receiver,
        amount,
        digest: *executed.effects.transaction_digest(),
    };
    cluster.wait_for_tx_settlement(&[transfer.digest]).await;
    transfer
}

/// Push the chain a few checkpoints beyond the current tip and return the new
/// tip. Used after a rebuild so the dedicated node's forward indexing has to
/// process (and, on buggy code, re-apply) the checkpoints the restore already
/// covered before the test reads the index. Each transfer settles in its own
/// checkpoint, so the returned tip is strictly above the rebuild's restore
/// watermark.
async fn advance_chain_past_rebuild(cluster: &TestCluster) -> u64 {
    for _ in 0..3 {
        transfer_to_fresh_address(cluster, 1_000_000).await;
    }
    chain_tip(cluster)
}

/// The chain tip as seen by the cluster's primary fullnode -- an upper bound on
/// the checkpoints the dedicated node must sync and index.
fn chain_tip(cluster: &TestCluster) -> u64 {
    cluster.fullnode_handle.haneul_node.with(|node| {
        node.state()
            .get_checkpoint_store()
            .get_highest_executed_checkpoint_seq_number()
            .unwrap()
            .unwrap_or(0)
    })
}

/// The total HANEUL balance the `rpc-index` reports for `owner`.
async fn haneul_balance(rpc_url: &str, owner: HaneulAddress) -> u64 {
    let mut client = Client::new(rpc_url.to_owned()).unwrap();
    let mut request = GetBalanceRequest::default();
    request.owner = Some(owner.to_string());
    request.coin_type = Some(HANEUL_COIN_TYPE.to_string());
    client
        .state_client()
        .get_balance(request)
        .await
        .unwrap()
        .into_inner()
        .balance
        .unwrap()
        .balance
        .unwrap()
}

/// A `ListTransactions` filter matching a single sender.
fn sender_filter(sender: HaneulAddress) -> TransactionFilter {
    let mut sender_filter = SenderFilter::default();
    sender_filter.address = Some(sender.to_string());
    let mut predicate = TransactionPredicate::default();
    predicate.predicate = Some(transaction_predicate::Predicate::Sender(sender_filter));
    let mut literal = TransactionLiteral::default();
    literal.polarity = Some(transaction_literal::Polarity::Include(predicate));
    let mut term = TransactionTerm::default();
    term.literals = vec![literal];
    let mut filter = TransactionFilter::default();
    filter.terms = vec![term];
    filter
}

/// The set of transaction digests the ledger-history `ListTransactions` API
/// returns for `sender`, scanning the whole indexed range.
async fn list_transaction_digests_by_sender(
    rpc_url: &str,
    sender: HaneulAddress,
) -> HashSet<String> {
    let mut client = LedgerServiceClient::connect(rpc_url.to_owned())
        .await
        .unwrap();
    let mut options = QueryOptions::default();
    options.limit = Some(500);
    let mut request = ListTransactionsRequest::default();
    request.read_mask = Some(FieldMask::from_paths(["digest"]));
    request.filter = Some(sender_filter(sender));
    request.options = Some(options);
    let mut stream = client
        .list_transactions(request)
        .await
        .unwrap()
        .into_inner();
    let mut digests = HashSet::new();
    while let Some(response) = stream.message().await.unwrap() {
        if let Some(list_transactions_response::Response::Item(item)) = response.response
            && let Some(digest) = item.transaction.and_then(|tx| tx.digest)
        {
            digests.insert(digest);
        }
    }
    digests
}

/// Assert both index surfaces reflect `transfer`: the `balance` index reports
/// the recipient's exact balance (the double-count regression shows up here as
/// twice `amount`), and the `transaction_bitmap` index returns the transfer
/// under a sender filter.
async fn assert_transfer_indexed(rpc_url: &str, transfer: &Transfer) {
    assert_eq!(
        haneul_balance(rpc_url, transfer.receiver).await,
        transfer.amount,
        "GetBalance should report the recipient's exact HANEUL balance",
    );
    let digests = list_transaction_digests_by_sender(rpc_url, transfer.sender).await;
    assert!(
        digests.contains(&transfer.digest.to_string()),
        "ListTransactions(sender={}) should include {}",
        transfer.sender,
        transfer.digest,
    );
}

/// Enabling the legacy `rpc-index` on a node that ran unindexed rebuilds the
/// indexes from the perpetual store and reports the pre-enable transfer's
/// balance exactly once -- it must not double-count the live object set against
/// the executor's forward indexing of the checkpoints the restore already
/// captured.
#[sim_test]
async fn enabling_legacy_index_rebuilds_without_double_counting() {
    let mut cluster = TestClusterBuilder::new()
        .with_num_validators(1)
        .disable_fullnode_pruning()
        .build()
        .await;
    let (name, rpc_url) = spawn_fullnode(&mut cluster, no_indexing_config()).await;
    assert!(
        !has_rpc_index(&cluster, &name),
        "indexing is off, so the node should build no rpc-index",
    );

    // Run a transfer while the node is unindexed.
    let transfer = transfer_to_fresh_address(&cluster, 7_000_000).await;

    // Turn on indexing and restart. With no prior rpc-index database the store
    // rebuilds from the perpetual store (bulk-loading the live object set and
    // backfilling the history indexes).
    restart_fullnode(&cluster, &name, legacy_indexing_config()).await;
    assert!(
        has_rpc_index(&cluster, &name),
        "indexing is on, so the node should build the rpc-index",
    );

    // Advance the chain past the rebuild point and wait for the node's forward
    // indexing to catch up beyond it. The double-count happens when the executor
    // re-applies a checkpoint that was committed (in the live object set the
    // restore loaded) but not yet executed when the node stopped; the executor
    // resumes from `highest_executed` and re-indexes it. We must read *after*
    // that re-application, not while the index still sits at the freshly stamped
    // restore watermark -- otherwise the bug is masked by reading too early.
    let target = advance_chain_past_rebuild(&cluster).await;
    wait_for_indexed(&cluster, &name, target).await;

    // The pre-enable transfer is visible through both index surfaces, and the
    // balance is reported exactly once (not double-counted).
    assert_transfer_indexed(&rpc_url, &transfer).await;
}
