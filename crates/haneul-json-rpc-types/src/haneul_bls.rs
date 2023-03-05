// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{self, Display, Formatter, Write};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use haneul_types::base_types::{EpochId, TransactionEffectsDigest};
use haneul_types::crypto::HaneulAuthorityStrongQuorumSignInfo;
use haneul_types::messages::EffectsFinalityInfo;
use haneul_types::messages_checkpoint::CheckpointSequenceNumber;

use crate::HaneulTransactionEffects;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "EffectsFinalityInfo", rename_all = "camelCase")]
pub enum HaneulEffectsFinalityInfo {
    Certified(HaneulAuthorityStrongQuorumSignInfo),
    Checkpointed(EpochId, CheckpointSequenceNumber),
}

impl From<EffectsFinalityInfo> for HaneulEffectsFinalityInfo {
    fn from(info: EffectsFinalityInfo) -> Self {
        match info {
            EffectsFinalityInfo::Certified(cert) => {
                Self::Certified(HaneulAuthorityStrongQuorumSignInfo::from(&cert))
            }
            EffectsFinalityInfo::Checkpointed(epoch, checkpoint) => {
                Self::Checkpointed(epoch, checkpoint)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "FinalizedEffects", rename_all = "camelCase")]
pub struct HaneulFinalizedEffects {
    pub transaction_effects_digest: TransactionEffectsDigest,
    pub effects: HaneulTransactionEffects,
    pub finality_info: HaneulEffectsFinalityInfo,
}

impl Display for HaneulFinalizedEffects {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        writeln!(
            writer,
            "Transaction Effects Digest: {:?}",
            self.transaction_effects_digest
        )?;
        writeln!(writer, "Transaction Effects: {:?}", self.effects)?;
        match &self.finality_info {
            HaneulEffectsFinalityInfo::Certified(cert) => {
                writeln!(writer, "Signed Authorities Bitmap: {:?}", cert.signers_map)?;
            }
            HaneulEffectsFinalityInfo::Checkpointed(epoch, checkpoint) => {
                writeln!(
                    writer,
                    "Finalized at epoch {:?}, checkpoint {:?}",
                    epoch, checkpoint
                )?;
            }
        }

        write!(f, "{}", writer)
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum HaneulTBlsSignObjectCommitmentType {
    /// Check that the object is committed by the consensus.
    ConsensusCommitted,
    /// Check that the object is committed using the effects certificate.
    FastPathCommitted(HaneulFinalizedEffects),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct HaneulTBlsSignRandomnessObjectResponse {
    pub signature: fastcrypto_tbls::types::RawSignature,
}
