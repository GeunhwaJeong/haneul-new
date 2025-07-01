// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_kvstore::{BigTableClient, KeyValueStoreReader};
use haneul_rpc::field::{FieldMask, FieldMaskTree, FieldMaskUtil};
use haneul_rpc::merge::Merge;
use haneul_rpc_api::{
    proto::{
        google::rpc::bad_request::FieldViolation,
        rpc::v2beta::{get_checkpoint_request::CheckpointId, Checkpoint, GetCheckpointRequest},
    },
    CheckpointNotFoundError, ErrorReason, RpcError,
};
use haneul_types::digests::CheckpointDigest;

pub async fn get_checkpoint(
    mut client: BigTableClient,
    request: GetCheckpointRequest,
) -> Result<Checkpoint, RpcError> {
    let read_mask = {
        let read_mask = request
            .read_mask
            .unwrap_or_else(|| FieldMask::from_str(GetCheckpointRequest::READ_MASK_DEFAULT));
        read_mask.validate::<Checkpoint>().map_err(|path| {
            FieldViolation::new("read_mask")
                .with_description(format!("invalid read_mask path: {path}"))
                .with_reason(ErrorReason::FieldInvalid)
        })?;
        FieldMaskTree::from(read_mask)
    };
    let checkpoint = match request.checkpoint_id {
        Some(CheckpointId::Digest(digest)) => {
            let digest = digest.parse::<CheckpointDigest>().map_err(|e| {
                FieldViolation::new("digest")
                    .with_description(format!("invalid digest: {e}"))
                    .with_reason(ErrorReason::FieldInvalid)
            })?;
            client
                .get_checkpoint_by_digest(digest)
                .await?
                .ok_or(CheckpointNotFoundError::digest(digest.into()))?
        }
        Some(CheckpointId::SequenceNumber(sequence_number)) => client
            .get_checkpoints(&[sequence_number])
            .await?
            .pop()
            .ok_or(CheckpointNotFoundError::sequence_number(sequence_number))?,
        None => {
            let sequence_number = client.get_latest_checkpoint().await?;
            client
                .get_checkpoints(&[sequence_number])
                .await?
                .pop()
                .ok_or(CheckpointNotFoundError::sequence_number(sequence_number))?
        }
    };
    let mut message = Checkpoint::default();
    let summary: haneul_sdk_types::CheckpointSummary = checkpoint.summary.try_into()?;
    let signatures: haneul_sdk_types::ValidatorAggregatedSignature = checkpoint.signatures.into();
    message.merge(&summary, &read_mask);
    message.merge(signatures, &read_mask);

    if read_mask.contains(Checkpoint::CONTENTS_FIELD.name) {
        message.merge(
            haneul_sdk_types::CheckpointContents::try_from(checkpoint.contents)?,
            &read_mask,
        );
    }
    // TODO: handle Checkpoint::TRANSACTIONS_FIELD submask
    Ok(message)
}
