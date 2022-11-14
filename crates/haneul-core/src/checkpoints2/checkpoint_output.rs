// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::authority::StableSyncAuthoritySigner;
use crate::consensus_adapter::SubmitToConsensus;
use async_trait::async_trait;
use haneul_types::base_types::AuthorityName;
use haneul_types::error::HaneulResult;
use haneul_types::messages::ConsensusTransaction;
use haneul_types::messages_checkpoint::{
    CheckpointContents, CheckpointSignatureMessage, CheckpointSummary, SignedCheckpointSummary,
};
use tracing::{debug, info};

#[async_trait]
pub trait CheckpointOutput: Sync + Send + 'static {
    async fn checkpoint_created(
        &self,
        summary: &CheckpointSummary,
        contents: &CheckpointContents,
    ) -> HaneulResult;
}

pub struct SubmitCheckpointToConsensus<T> {
    pub sender: T,
    pub signer: StableSyncAuthoritySigner,
    pub authority: AuthorityName,
}

pub struct LogCheckpointOutput;

impl LogCheckpointOutput {
    pub fn boxed() -> Box<dyn CheckpointOutput> {
        Box::new(Self)
    }
}

#[async_trait]
impl<T: SubmitToConsensus> CheckpointOutput for SubmitCheckpointToConsensus<T> {
    async fn checkpoint_created(
        &self,
        summary: &CheckpointSummary,
        contents: &CheckpointContents,
    ) -> HaneulResult {
        LogCheckpointOutput
            .checkpoint_created(summary, contents)
            .await?;
        let summary = SignedCheckpointSummary::new_from_summary(
            summary.clone(),
            self.authority,
            &*self.signer,
        );
        let message = CheckpointSignatureMessage { summary };
        let transaction = ConsensusTransaction::new_checkpoint_signature_message(message);
        self.sender.submit_to_consensus(&transaction).await
    }
}

#[async_trait]
impl CheckpointOutput for LogCheckpointOutput {
    async fn checkpoint_created(
        &self,
        summary: &CheckpointSummary,
        contents: &CheckpointContents,
    ) -> HaneulResult {
        debug!(
            "Including following transactions in checkpoint {}: {:?}",
            summary.sequence_number, contents
        );
        info!(
            "Creating checkpoint {:?} at sequence {}, previous digest {:?}, transactions count {}, content digest {:?}",
            hex::encode(summary.digest()),
            summary.sequence_number,
            summary.previous_digest,
            contents.size(),
            hex::encode(summary.content_digest),
        );

        Ok(())
    }
}
