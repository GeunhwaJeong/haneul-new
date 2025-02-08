// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use futures::future::OptionFuture;
use move_core_types::annotated_value::MoveTypeLayout;
use haneul_json_rpc_types::{
    HaneulData, HaneulObjectData, HaneulObjectDataOptions, HaneulObjectRef, HaneulObjectResponse, HaneulParsedData,
    HaneulPastObjectResponse, HaneulRawData,
};
use haneul_types::{
    base_types::{ObjectID, ObjectType, SequenceNumber},
    digests::ObjectDigest,
    error::HaneulObjectResponseError,
    object::{Data, Object},
    TypeTag,
};
use tokio::join;

use crate::{
    context::Context,
    data::{
        object_info::LatestObjectInfoKey,
        objects::{load_latest, VersionedObjectKey},
    },
    error::{internal_error, rpc_bail, RpcError},
};

/// Fetch the necessary data from the stores in `ctx` and transform it to build a response for a
/// the latest version of an object, identified by its ID, according to the response `options`.
pub(super) async fn live_object(
    ctx: &Context,
    object_id: ObjectID,
    options: &HaneulObjectDataOptions,
) -> Result<HaneulObjectResponse, RpcError> {
    let Some(info) = ctx
        .loader()
        .load_one(LatestObjectInfoKey(object_id))
        .await
        .context("Failed to load object ownership information from store")?
    else {
        return Ok(HaneulObjectResponse::new_with_error(
            HaneulObjectResponseError::NotExists { object_id },
        ));
    };

    // This means that the latest ownership record shows the object has been deleted, but our
    // schema doesn't include the version the object was deleted at, and once the deletion moves
    // out of the available range, this record will be deleted too, so we return a `NotExists`
    // error instead of a `Deleted` error for consistency with the above error case.
    if info.owner_kind.is_none() {
        return Ok(HaneulObjectResponse::new_with_error(
            HaneulObjectResponseError::NotExists { object_id },
        ));
    }

    latest_object(ctx, object_id, options).await
}

/// Assuming the latest version of this object exists, fetch it from the database and convert it
/// into a response. This is intended to be used after checking with `obj_info` that the object is
/// live.
pub(super) async fn latest_object(
    ctx: &Context,
    object_id: ObjectID,
    options: &HaneulObjectDataOptions,
) -> Result<HaneulObjectResponse, RpcError> {
    // The fact that we found an `obj_info` record above means that the latest version of the
    // object does exist, so the following calls should find a valid latest version for the object,
    // and that version is expected to have content, so if either of those things don't happen,
    // it's an internal error.
    let o = load_latest(ctx.loader(), object_id)
        .await
        .context("Failed to load latest object")?
        .ok_or_else(|| internal_error!("Could not find latest content for live object"))?;

    Ok(HaneulObjectResponse::new_with_data(
        object(ctx, o, options).await?,
    ))
}

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

    let o: Object = bcs::from_bytes(&bytes).context("Failed to deserialize object")?;

    Ok(HaneulPastObjectResponse::VersionFound(
        object(ctx, o, options).await?,
    ))
}

/// Extract a representation of the object from its stored form, according to its response options.
pub(crate) async fn object(
    ctx: &Context,
    object: Object,
    options: &HaneulObjectDataOptions,
) -> Result<HaneulObjectData, RpcError> {
    let type_ = options.show_type.then(|| ObjectType::from(&object));
    let owner = options.show_owner.then(|| object.owner().clone());
    let previous_transaction = options
        .show_previous_transaction
        .then(|| object.previous_transaction);
    let storage_rebate = options.show_storage_rebate.then(|| object.storage_rebate);

    let content: OptionFuture<_> = options
        .show_content
        .then(|| object_data::<HaneulParsedData>(ctx, &object))
        .into();

    let bcs: OptionFuture<_> = options
        .show_bcs
        .then(|| object_data::<HaneulRawData>(ctx, &object))
        .into();

    let (content, bcs) = join!(content, bcs);

    let content = content
        .transpose()
        .context("Failed to deserialize object content")?;

    let bcs = bcs
        .transpose()
        .context("Failed to deserialize object to BCS")?;

    Ok(HaneulObjectData {
        object_id: object.id(),
        version: object.version(),
        digest: object.digest(),
        type_,
        owner,
        previous_transaction,
        storage_rebate,
        display: None,
        content,
        bcs,
    })
}

/// Extract the contents of an object, in a format chosen by the `D` type parameter.
/// This operaton can fail if it's not possible to get the type layout for the object's type.
async fn object_data<D: HaneulData>(ctx: &Context, object: &Object) -> Result<D, RpcError> {
    Ok(match object.data.clone() {
        Data::Package(move_package) => D::try_from_package(move_package)?,

        Data::Move(move_object) => {
            let type_: TypeTag = move_object.type_().clone().into();
            let MoveTypeLayout::Struct(layout) = ctx
                .package_resolver()
                .type_layout(type_.clone())
                .await
                .with_context(|| {
                    format!(
                        "Failed to resolve type layout for {}",
                        type_.to_canonical_display(/*with_prefix */ true)
                    )
                })?
            else {
                rpc_bail!(
                    "Type {} is not a struct",
                    type_.to_canonical_display(/*with_prefix */ true)
                );
            };

            D::try_from_object(move_object, *layout)?
        }
    })
}
