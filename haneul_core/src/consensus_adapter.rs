// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use bytes::Bytes;
use futures::SinkExt;
use narwhal_executor::SubscriberResult;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use haneul_network::transport;
use haneul_network::transport::{RwChannel, TcpDataStream};
use haneul_types::committee::Committee;
use haneul_types::error::{HaneulError, HaneulResult};
use haneul_types::messages::ConsensusTransaction;
use haneul_types::serialize::serialize_consensus_transaction;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tracing::debug;

#[cfg(test)]
#[path = "unit_tests/consensus_tests.rs"]
pub mod consensus_tests;

/// A serialized consensus transaction.
type SerializedConsensusTransaction = Vec<u8>;

/// The digest of a consensus transactions
type ConsensusTransactionDigest = u64;

/// The message returned by the consensus to notify that a Haneul certificate has been sequenced
/// and all its shared objects are locked.
type ConsensusOutput = (SubscriberResult<()>, ConsensusTransactionDigest);

/// Channel to notify the called when the Haneul certificate has been sequenced.
type Replier = oneshot::Sender<HaneulResult<()>>;

/// Message to notify the consensus adapter of a new certificate sent to consensus.
#[derive(Debug)]
pub struct ConsensusInput {
    serialized: SerializedConsensusTransaction,
    replier: Replier,
}

/// This module interfaces the consensus with Haneul. It receives certificates input to consensus and
/// notify the called when they are sequenced.
pub struct ConsensusListener {
    /// Receive messages input to the consensus.
    rx_consensus_input: Receiver<ConsensusInput>,
    /// Receive consensus outputs.
    rx_consensus_output: Receiver<ConsensusOutput>,
    /// Keep a map of all consensus inputs that are currently being sequenced.
    pending: HashMap<ConsensusTransactionDigest, VecDeque<Replier>>,
}

impl ConsensusListener {
    /// Spawn a new consensus adapter in a dedicated tokio task.
    pub fn spawn(
        rx_consensus_input: Receiver<ConsensusInput>,
        rx_consensus_output: Receiver<ConsensusOutput>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            Self {
                rx_consensus_input,
                rx_consensus_output,
                pending: HashMap::new(),
            }
            .run()
            .await
        })
    }

    /// Main loop receiving messages input to consensus and notifying the caller once the inputs
    /// are sequenced (of if an error happened).
    async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(consensus_input) = self.rx_consensus_input.recv() => {
                    // Keep track of this certificates so we can notify the user later.
                    let serialized = consensus_input.serialized;
                    let replier = consensus_input.replier;
                    let digest = Self::hash(&serialized);
                    self.pending.entry(digest).or_insert_with(VecDeque::new).push_back(replier);
                },

                Some((result, transaction_digest)) = self.rx_consensus_output.recv() => {
                    // Notify the caller that the transaction has been sequenced.
                    let outcome = result.map_err(HaneulError::from);
                    if let Some(replier) = self.pending.get_mut(&transaction_digest).and_then(|r| r.pop_front()) {
                        if replier.send(outcome).is_err() {
                            debug!("No replier to listen to consensus output {transaction_digest}");
                        }
                    }
                }
            }
        }
    }

    /// Hash serialized consensus transactions. We do not need specific cryptographic properties except
    /// only collision resistance.
    pub fn hash(serialized: &SerializedConsensusTransaction) -> ConsensusTransactionDigest {
        let mut hasher = DefaultHasher::new();
        serialized.hash(&mut hasher);
        hasher.finish()
    }
}

/// Submit Haneul certificates to the consensus.
pub struct ConsensusSubmitter {
    /// The network address of the consensus node.
    consensus_address: SocketAddr,
    /// The network buffer size.
    buffer_size: usize,
    /// The Haneul committee information.
    committee: Committee,
    /// A channel to notify the consensus listener of new transactions.
    tx_consensus_listener: Sender<ConsensusInput>,
}

impl ConsensusSubmitter {
    /// Make a new Consensus submitter instance.
    pub fn new(
        consensus_address: SocketAddr,
        buffer_size: usize,
        committee: Committee,
        tx_consensus_listener: Sender<ConsensusInput>,
    ) -> Self {
        Self {
            consensus_address,
            buffer_size,
            committee,
            tx_consensus_listener,
        }
    }

    /// Attempt to reconnect with a the consensus node.
    async fn reconnect(address: SocketAddr, buffer_size: usize) -> HaneulResult<TcpDataStream> {
        transport::connect(address.to_string(), buffer_size)
            .await
            .map_err(|e| HaneulError::ConsensusConnectionBroken(e.to_string()))
    }

    /// Submit a transaction to consensus, wait for its processing, and notify the caller.
    pub async fn submit(&self, certificate: &ConsensusTransaction) -> HaneulResult<()> {
        // Check the Haneul certificate (submitted by the user).
        certificate.check(&self.committee)?;

        // Send certificate to consensus
        let serialized = serialize_consensus_transaction(certificate);
        let bytes = Bytes::from(serialized.clone());
        // TODO [issue #1452]: We are re-creating a connection every time. This is wasteful but does not
        // require to take self as a mutable reference.
        Self::reconnect(self.consensus_address, self.buffer_size)
            .await?
            .sink()
            .send(bytes.clone())
            .await
            .map_err(|e| HaneulError::ConsensusConnectionBroken(e.to_string()))?;

        // Notify the consensus listener that we are expecting to process this certificate.
        let (sender, receiver) = oneshot::channel();
        let consensus_input = ConsensusInput {
            serialized,
            replier: sender,
        };
        self.tx_consensus_listener
            .send(consensus_input)
            .await
            .expect("Failed to notify consensus listener");

        // Wait for the consensus to sequence the certificate and assign locks to shared objects.
        receiver
            .await
            .expect("Failed to receive reply from consensus listener")
    }
}
