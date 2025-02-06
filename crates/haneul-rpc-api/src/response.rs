// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};

use crate::{
    types::{
        X_HANEUL_CHAIN, X_HANEUL_CHAIN_ID, X_HANEUL_CHECKPOINT_HEIGHT, X_HANEUL_EPOCH,
        X_HANEUL_LOWEST_AVAILABLE_CHECKPOINT, X_HANEUL_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS,
        X_HANEUL_TIMESTAMP_MS,
    },
    RpcService,
};

pub async fn append_info_headers(
    State(state): State<RpcService>,
    response: Response,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if let Ok(chain_id) = state.chain_id().to_string().try_into() {
        headers.insert(X_HANEUL_CHAIN_ID, chain_id);
    }

    if let Ok(chain) = state.chain_id().chain().as_str().try_into() {
        headers.insert(X_HANEUL_CHAIN, chain);
    }

    if let Ok(latest_checkpoint) = state.reader.inner().get_latest_checkpoint() {
        headers.insert(X_HANEUL_EPOCH, latest_checkpoint.epoch().into());
        headers.insert(
            X_HANEUL_CHECKPOINT_HEIGHT,
            latest_checkpoint.sequence_number.into(),
        );
        headers.insert(X_HANEUL_TIMESTAMP_MS, latest_checkpoint.timestamp_ms.into());
    }

    if let Ok(lowest_available_checkpoint) = state.reader.inner().get_lowest_available_checkpoint()
    {
        headers.insert(
            X_HANEUL_LOWEST_AVAILABLE_CHECKPOINT,
            lowest_available_checkpoint.into(),
        );
    }

    if let Ok(lowest_available_checkpoint_objects) = state
        .reader
        .inner()
        .get_lowest_available_checkpoint_objects()
    {
        headers.insert(
            X_HANEUL_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS,
            lowest_available_checkpoint_objects.into(),
        );
    }

    (headers, response)
}
