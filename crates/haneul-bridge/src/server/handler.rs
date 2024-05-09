// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::type_complexity)]

use crate::crypto::{BridgeAuthorityKeyPair, BridgeAuthoritySignInfo};
use crate::error::{BridgeError, BridgeResult};
use crate::eth_client::EthClient;
use crate::haneul_client::{HaneulClient, HaneulClientInner};
use crate::types::{BridgeAction, SignedBridgeAction};
use async_trait::async_trait;
use axum::Json;
use ethers::providers::JsonRpcClient;
use ethers::types::TxHash;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::str::FromStr;
use std::sync::Arc;
use haneul_types::digests::TransactionDigest;
use tap::TapFallible;
use tokio::sync::{oneshot, Mutex};
use tracing::info;
use tracing::instrument;

use super::governance_verifier::GovernanceVerifier;

#[async_trait]
pub trait BridgeRequestHandlerTrait {
    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Ethereum to Haneul. The inputs are a transaction hash on Ethereum
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_eth_tx_hash(
        &self,
        tx_hash_hex: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;
    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Haneul to Ethereum. The inputs are a transaction digest on Haneul
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_haneul_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;

    /// Handles a request to sign a governance action.
    async fn handle_governance_action(
        &self,
        action: BridgeAction,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;
}

#[async_trait::async_trait]
pub trait ActionVerifier<K>: Send + Sync {
    async fn verify(&self, key: K) -> BridgeResult<BridgeAction>;
}

struct HaneulActionVerifier<C> {
    haneul_client: Arc<HaneulClient<C>>,
}

struct EthActionVerifier<P> {
    eth_client: Arc<EthClient<P>>,
}

#[async_trait::async_trait]
impl<C> ActionVerifier<(TransactionDigest, u16)> for HaneulActionVerifier<C>
where
    C: HaneulClientInner + Send + Sync + 'static,
{
    async fn verify(&self, key: (TransactionDigest, u16)) -> BridgeResult<BridgeAction> {
        let (tx_digest, event_idx) = key;
        self.haneul_client
            .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, event_idx)
            .await
            .tap_ok(|action| info!("Haneul action found: {:?}", action))
    }
}

#[async_trait::async_trait]
impl<C> ActionVerifier<(TxHash, u16)> for EthActionVerifier<C>
where
    C: JsonRpcClient + Send + Sync + 'static,
{
    async fn verify(&self, key: (TxHash, u16)) -> BridgeResult<BridgeAction> {
        let (tx_hash, event_idx) = key;
        self.eth_client
            .get_finalized_bridge_action_maybe(tx_hash, event_idx)
            .await
            .tap_ok(|action| info!("Eth action found: {:?}", action))
    }
}

struct SignerWithCache<K> {
    signer: Arc<BridgeAuthorityKeyPair>,
    verifier: Arc<dyn ActionVerifier<K>>,
    mutex: Arc<Mutex<()>>,
    cache: LruCache<K, Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>>>,
}

impl<K> SignerWithCache<K>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
{
    fn new(
        signer: Arc<BridgeAuthorityKeyPair>,
        verifier: impl ActionVerifier<K> + 'static,
    ) -> Self {
        Self {
            signer,
            verifier: Arc::new(verifier),
            mutex: Arc::new(Mutex::new(())),
            cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
        }
    }

    fn spawn(
        mut self,
        mut rx: haneullabs_metrics::metered_channel::Receiver<(
            K,
            oneshot::Sender<BridgeResult<SignedBridgeAction>>,
        )>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let (key, tx) = rx
                    .recv()
                    .await
                    .unwrap_or_else(|| panic!("Server signer's channel is closed"));
                let result = self.sign(key).await;
                // The receiver may be dropped before the sender (client connection was dropped for example),
                // we ignore the error in that case.
                let _ = tx.send(result);
            }
        })
    }

    async fn get_cache_entry(
        &mut self,
        key: K,
    ) -> Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>> {
        // This mutex exists to make sure everyone gets the same entry, namely no double insert
        let _ = self.mutex.lock().await;
        self.cache
            .get_or_insert(key, || Arc::new(Mutex::new(None)))
            .clone()
    }

    async fn sign(&mut self, key: K) -> BridgeResult<SignedBridgeAction> {
        let signer = self.signer.clone();
        let verifier = self.verifier.clone();
        let entry = self.get_cache_entry(key.clone()).await;
        let mut guard = entry.lock().await;
        if let Some(result) = &*guard {
            return result.clone();
        }
        match verifier.verify(key.clone()).await {
            Ok(bridge_action) => {
                let sig = BridgeAuthoritySignInfo::new(&bridge_action, &signer);
                let result = SignedBridgeAction::new_from_data_and_sig(bridge_action, sig);
                // Cache result if Ok
                *guard = Some(Ok(result.clone()));
                Ok(result)
            }
            Err(e) => {
                match e {
                    // Only cache non-transient errors
                    BridgeError::GovernanceActionIsNotApproved { .. }
                    | BridgeError::ActionIsNotGovernanceAction(..)
                    | BridgeError::BridgeEventInUnrecognizedHaneulPackage
                    | BridgeError::BridgeEventInUnrecognizedEthContract
                    | BridgeError::BridgeEventNotActionable
                    | BridgeError::NoBridgeEventsInTxPosition => {
                        *guard = Some(Err(e.clone()));
                    }
                    _ => (),
                }
                Err(e)
            }
        }
    }

    #[cfg(test)]
    async fn get_testing_only(
        &mut self,
        key: K,
    ) -> Option<&Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>>> {
        let _ = self.mutex.lock().await;
        self.cache.get(&key)
    }
}

pub struct BridgeRequestHandler {
    haneul_signer_tx: haneullabs_metrics::metered_channel::Sender<(
        (TransactionDigest, u16),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
    eth_signer_tx: haneullabs_metrics::metered_channel::Sender<(
        (TxHash, u16),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
    governance_signer_tx: haneullabs_metrics::metered_channel::Sender<(
        BridgeAction,
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
}

impl BridgeRequestHandler {
    pub fn new<
        SC: HaneulClientInner + Send + Sync + 'static,
        EP: JsonRpcClient + Send + Sync + 'static,
    >(
        signer: BridgeAuthorityKeyPair,
        haneul_client: Arc<HaneulClient<SC>>,
        eth_client: Arc<EthClient<EP>>,
        approved_governance_actions: Vec<BridgeAction>,
    ) -> Self {
        let (haneul_signer_tx, haneul_rx) = haneullabs_metrics::metered_channel::channel(
            1000,
            &haneullabs_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["server_haneul_action_signing_queue"]),
        );
        let (eth_signer_tx, eth_rx) = haneullabs_metrics::metered_channel::channel(
            1000,
            &haneullabs_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["server_eth_action_signing_queue"]),
        );
        let (governance_signer_tx, governance_rx) = haneullabs_metrics::metered_channel::channel(
            1000,
            &haneullabs_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["server_governance_action_signing_queue"]),
        );
        let signer = Arc::new(signer);

        SignerWithCache::new(signer.clone(), HaneulActionVerifier { haneul_client }).spawn(haneul_rx);
        SignerWithCache::new(signer.clone(), EthActionVerifier { eth_client }).spawn(eth_rx);
        SignerWithCache::new(
            signer.clone(),
            GovernanceVerifier::new(approved_governance_actions).unwrap(),
        )
        .spawn(governance_rx);

        Self {
            haneul_signer_tx,
            eth_signer_tx,
            governance_signer_tx,
        }
    }
}

#[async_trait]
impl BridgeRequestHandlerTrait for BridgeRequestHandler {
    #[instrument(level = "info", skip(self))]
    async fn handle_eth_tx_hash(
        &self,
        tx_hash_hex: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        info!("Received handle eth tx request");
        let tx_hash = TxHash::from_str(&tx_hash_hex).map_err(|_| BridgeError::InvalidTxHash)?;

        let (tx, rx) = oneshot::channel();
        self.eth_signer_tx
            .send(((tx_hash, event_idx), tx))
            .await
            .unwrap_or_else(|_| panic!("Server eth signing channel is closed"));
        let signed_action = rx
            .await
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }

    #[instrument(level = "info", skip(self))]
    async fn handle_haneul_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        info!("Received handle haneul tx request");
        let tx_digest = TransactionDigest::from_str(&tx_digest_base58)
            .map_err(|_e| BridgeError::InvalidTxHash)?;
        let (tx, rx) = oneshot::channel();
        self.haneul_signer_tx
            .send(((tx_digest, event_idx), tx))
            .await
            .unwrap_or_else(|_| panic!("Server haneul signing channel is closed"));
        let signed_action = rx
            .await
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }

    async fn handle_governance_action(
        &self,
        action: BridgeAction,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        info!("Received handle governace action request");
        if !action.is_governace_action() {
            return Err(BridgeError::ActionIsNotGovernanceAction(action));
        }
        let (tx, rx) = oneshot::channel();
        self.governance_signer_tx
            .send((action, tx))
            .await
            .unwrap_or_else(|_| panic!("Server governance action signing channel is closed"));
        let signed_action = rx.await.unwrap_or_else(|_| {
            panic!("Server governance action task's oneshot channel is dropped")
        })?;
        Ok(Json(signed_action))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{
        eth_mock_provider::EthMockProvider,
        events::{init_all_struct_tags, MoveTokenDepositedEvent, HaneulToEthTokenBridgeV1},
        haneul_mock_client::HaneulMockClient,
        test_utils::{
            get_test_log_and_action, get_test_haneul_to_eth_bridge_action, mock_last_finalized_block,
        },
        types::{EmergencyAction, EmergencyActionType, LimitUpdateAction},
    };
    use ethers::types::{Address as EthAddress, TransactionReceipt};
    use haneul_json_rpc_types::HaneulEvent;
    use haneul_types::bridge::{BridgeChainId, TOKEN_ID_USDC};
    use haneul_types::{base_types::HaneulAddress, crypto::get_key_pair};

    #[tokio::test]
    async fn test_haneul_signer_with_cache() {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let haneul_client_mock = HaneulMockClient::default();
        let haneul_verifier = HaneulActionVerifier {
            haneul_client: Arc::new(HaneulClient::new_for_testing(haneul_client_mock.clone())),
        };
        let mut haneul_signer_with_cache = SignerWithCache::new(signer.clone(), haneul_verifier);

        // Test `get_cache_entry` creates a new entry if not exist
        let haneul_tx_digest = TransactionDigest::random();
        let haneul_event_idx = 42;
        assert!(haneul_signer_with_cache
            .get_testing_only((haneul_tx_digest, haneul_event_idx))
            .await
            .is_none());
        let entry = haneul_signer_with_cache
            .get_cache_entry((haneul_tx_digest, haneul_event_idx))
            .await;
        let entry_ = haneul_signer_with_cache
            .get_testing_only((haneul_tx_digest, haneul_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        let action = get_test_haneul_to_eth_bridge_action(
            Some(haneul_tx_digest),
            Some(haneul_event_idx),
            None,
            None,
            None,
            None,
            None,
        );
        let sig = BridgeAuthoritySignInfo::new(&action, &signer);
        let signed_action = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig);
        entry.lock().await.replace(Ok(signed_action));
        let entry_ = haneul_signer_with_cache
            .get_testing_only((haneul_tx_digest, haneul_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_some());

        // Test `sign` caches Err result
        let haneul_tx_digest = TransactionDigest::random();
        let haneul_event_idx = 0;

        // Mock an non-cacheable error such as rpc error
        haneul_client_mock.add_events_by_tx_digest_error(haneul_tx_digest);
        haneul_signer_with_cache
            .sign((haneul_tx_digest, haneul_event_idx))
            .await
            .unwrap_err();
        let entry_ = haneul_signer_with_cache
            .get_testing_only((haneul_tx_digest, haneul_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        // Mock a cacheable error such as no bridge events in tx position (empty event list)
        haneul_client_mock.add_events_by_tx_digest(haneul_tx_digest, vec![]);
        assert!(matches!(
            haneul_signer_with_cache
                .sign((haneul_tx_digest, haneul_event_idx))
                .await,
            Err(BridgeError::NoBridgeEventsInTxPosition)
        ));
        let entry_ = haneul_signer_with_cache
            .get_testing_only((haneul_tx_digest, haneul_event_idx))
            .await;
        assert_eq!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition,
        );

        // TODO: test BridgeEventInUnrecognizedHaneulPackage, HaneulBridgeEvent::try_from_haneul_event
        // and BridgeEventNotActionable to be cached

        // Test `sign` caches Ok result
        let emitted_event_1 = MoveTokenDepositedEvent {
            seq_num: 1,
            source_chain: BridgeChainId::HaneulCustom as u8,
            sender_address: HaneulAddress::random_for_testing_only().to_vec(),
            target_chain: BridgeChainId::EthCustom as u8,
            target_address: EthAddress::random().as_bytes().to_vec(),
            token_type: TOKEN_ID_USDC,
            amount_haneul_adjusted: 12345,
        };

        init_all_struct_tags();

        let mut haneul_event_1 = HaneulEvent::random_for_testing();
        haneul_event_1.type_ = HaneulToEthTokenBridgeV1.get().unwrap().clone();
        haneul_event_1.bcs = bcs::to_bytes(&emitted_event_1).unwrap();
        let haneul_tx_digest = haneul_event_1.id.tx_digest;

        let mut haneul_event_2 = HaneulEvent::random_for_testing();
        haneul_event_2.type_ = HaneulToEthTokenBridgeV1.get().unwrap().clone();
        haneul_event_2.bcs = bcs::to_bytes(&emitted_event_1).unwrap();
        let haneul_event_idx_2 = 1;
        haneul_client_mock.add_events_by_tx_digest(haneul_tx_digest, vec![haneul_event_2.clone()]);

        haneul_client_mock.add_events_by_tx_digest(
            haneul_tx_digest,
            vec![haneul_event_1.clone(), haneul_event_2.clone()],
        );
        let signed_1 = haneul_signer_with_cache
            .sign((haneul_tx_digest, haneul_event_idx))
            .await
            .unwrap();
        let signed_2 = haneul_signer_with_cache
            .sign((haneul_tx_digest, haneul_event_idx_2))
            .await
            .unwrap();

        // Because the result is cached now, the verifier should not be called again.
        // Even though we remove the `add_events_by_tx_digest` mock, we will still get the same result.
        haneul_client_mock.add_events_by_tx_digest(haneul_tx_digest, vec![]);
        assert_eq!(
            haneul_signer_with_cache
                .sign((haneul_tx_digest, haneul_event_idx))
                .await
                .unwrap(),
            signed_1
        );
        assert_eq!(
            haneul_signer_with_cache
                .sign((haneul_tx_digest, haneul_event_idx_2))
                .await
                .unwrap(),
            signed_2
        );
    }

    #[tokio::test]
    async fn test_eth_signer_with_cache() {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let eth_mock_provider = EthMockProvider::default();
        let contract_address = EthAddress::random();
        let eth_client = EthClient::new_mocked(
            eth_mock_provider.clone(),
            HashSet::from_iter(vec![contract_address]),
        );
        let eth_verifier = EthActionVerifier {
            eth_client: Arc::new(eth_client),
        };
        let mut eth_signer_with_cache = SignerWithCache::new(signer.clone(), eth_verifier);

        // Test `get_cache_entry` creates a new entry if not exist
        let eth_tx_hash = TxHash::random();
        let eth_event_idx = 42;
        assert!(eth_signer_with_cache
            .get_testing_only((eth_tx_hash, eth_event_idx))
            .await
            .is_none());
        let entry = eth_signer_with_cache
            .get_cache_entry((eth_tx_hash, eth_event_idx))
            .await;
        let entry_ = eth_signer_with_cache
            .get_testing_only((eth_tx_hash, eth_event_idx))
            .await;
        // first unwrap should not pacic because the entry should have been inserted by `get_cache_entry`
        assert!(entry_.unwrap().lock().await.is_none());

        let (_, action) = get_test_log_and_action(contract_address, eth_tx_hash, eth_event_idx);
        let sig = BridgeAuthoritySignInfo::new(&action, &signer);
        let signed_action = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig);
        entry.lock().await.replace(Ok(signed_action.clone()));
        let entry_ = eth_signer_with_cache
            .get_testing_only((eth_tx_hash, eth_event_idx))
            .await;
        assert_eq!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap(),
            signed_action
        );

        // Test `sign` caches Ok result
        let eth_tx_hash = TxHash::random();
        let eth_event_idx = 0;
        let (log, _action) = get_test_log_and_action(contract_address, eth_tx_hash, eth_event_idx);
        eth_mock_provider
            .add_response::<[TxHash; 1], TransactionReceipt, TransactionReceipt>(
                "eth_getTransactionReceipt",
                [log.transaction_hash.unwrap()],
                TransactionReceipt {
                    block_number: log.block_number,
                    logs: vec![log.clone()],
                    ..Default::default()
                },
            )
            .unwrap();
        mock_last_finalized_block(&eth_mock_provider, log.block_number.unwrap().as_u64());

        eth_signer_with_cache
            .sign((eth_tx_hash, eth_event_idx))
            .await
            .unwrap();
        let entry_ = eth_signer_with_cache
            .get_testing_only((eth_tx_hash, eth_event_idx))
            .await;
        entry_.unwrap().lock().await.clone().unwrap().unwrap();
    }

    #[tokio::test]
    async fn test_signer_with_governace_verifier() {
        let action_1 = BridgeAction::EmergencyAction(EmergencyAction {
            chain_id: BridgeChainId::EthCustom,
            nonce: 1,
            action_type: EmergencyActionType::Pause,
        });
        let action_2 = BridgeAction::LimitUpdateAction(LimitUpdateAction {
            chain_id: BridgeChainId::EthCustom,
            sending_chain_id: BridgeChainId::HaneulCustom,
            nonce: 1,
            new_usd_limit: 10000,
        });

        let verifier = GovernanceVerifier::new(vec![action_1.clone(), action_2.clone()]).unwrap();
        assert_eq!(
            verifier.verify(action_1.clone()).await.unwrap(),
            action_1.clone()
        );
        assert_eq!(
            verifier.verify(action_2.clone()).await.unwrap(),
            action_2.clone()
        );

        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let mut signer_with_cache = SignerWithCache::new(signer.clone(), verifier);

        // action_1 is signable
        signer_with_cache.sign(action_1.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_1.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_1
        );

        // alter action_1 to action_3
        let action_3 = BridgeAction::EmergencyAction(EmergencyAction {
            chain_id: BridgeChainId::EthCustom,
            nonce: 1,
            action_type: EmergencyActionType::Unpause,
        });
        // action_3 is not signable
        assert!(matches!(
            signer_with_cache.sign(action_3.clone()).await.unwrap_err(),
            BridgeError::GovernanceActionIsNotApproved { .. }
        ));
        // error is cached
        let entry_ = signer_with_cache.get_testing_only(action_3.clone()).await;
        assert!(matches!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::GovernanceActionIsNotApproved { .. }
        ));

        // Non governace action is not signable
        let action_4 = get_test_haneul_to_eth_bridge_action(None, None, None, None, None, None, None);
        assert!(matches!(
            signer_with_cache.sign(action_4.clone()).await.unwrap_err(),
            BridgeError::ActionIsNotGovernanceAction(..)
        ));
        // error is cached
        let entry_ = signer_with_cache.get_testing_only(action_4.clone()).await;
        assert!(matches!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::ActionIsNotGovernanceAction { .. }
        ));
    }
    // TODO: add tests for BridgeRequestHandler (need to hook up local eth node)
}
