// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use haneul_json_rpc_types::{
    HaneulObjectData, HaneulObjectDataOptions, HaneulObjectRef, HaneulPastObjectResponse,
};
use haneul_types::{
    base_types::{ObjectID, ObjectType, SequenceNumber},
    digests::ObjectDigest,
    object::Object,
};

use crate::{context::Context, data::objects::VersionedObjectKey, error::RpcError};

/// Fetch the necessary data from the stores in `ctx` and transform it to build a response for a
/// past object identified by its ID and version, according to the response `options`.
pub(super) async fn past_object(
    ctx: &Context,
    object_id: ObjectID,
    version: SequenceNumber,
    options: &HaneulObjectDataOptions,
) -> Result<HaneulPastObjectResponse, RpcError> {
    let Some(stored) = ctx
        .loader()
        .load_one(VersionedObjectKey(object_id, version.value()))
        .await
        .context("Failed to load object from store")?
    else {
        return Ok(HaneulPastObjectResponse::VersionNotFound(object_id, version));
    };

    let Some(bytes) = &stored.serialized_object else {
        return Ok(HaneulPastObjectResponse::ObjectDeleted(HaneulObjectRef {
            object_id,
            version,
            digest: ObjectDigest::OBJECT_DIGEST_DELETED,
        }));
    };

    Ok(HaneulPastObjectResponse::VersionFound(object(
        object_id, version, bytes, options,
    )?))
}

/// Extract a representation of the object from its stored form, according to its response options.
fn object(
    object_id: ObjectID,
    version: SequenceNumber,
    bytes: &[u8],
    options: &HaneulObjectDataOptions,
) -> Result<HaneulObjectData, RpcError> {
    let object: Object = bcs::from_bytes(bytes).context("Failed to deserialize object")?;

    let type_ = options.show_type.then(|| ObjectType::from(&object));
    let owner = options.show_owner.then(|| object.owner().clone());
    let previous_transaction = options
        .show_previous_transaction
        .then(|| object.previous_transaction);
    let storage_rebate = options.show_storage_rebate.then(|| object.storage_rebate);

    Ok(HaneulObjectData {
        object_id,
        version,
        digest: object.digest(),
        type_,
        owner,
        previous_transaction,
        storage_rebate,
        display: None,
        content: None,
        bcs: None,
    })
}
