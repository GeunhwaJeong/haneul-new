// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_config::{haneul_config_dir, HANEUL_CLIENT_CONFIG};
use haneul_sdk::wallet_context::WalletContext;
use haneul_source_validation_service::{initialize, serve};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = haneul_config_dir()?.join(HANEUL_CLIENT_CONFIG);
    let context = WalletContext::new(&config, None, None).await?;
    let package_paths = vec![];
    initialize(&context, package_paths).await?;
    serve()?.await.map_err(anyhow::Error::from)
}
