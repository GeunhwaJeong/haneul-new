// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use haneul_config::ValidatorConfig;
use haneul_core::authority_server::AuthorityServerHandle;
use tracing::info;

pub struct HaneulNode {
    authority_server: AuthorityServerHandle,
}

impl HaneulNode {
    pub async fn start(config: &ValidatorConfig) -> Result<()> {
        let server = haneul_core::make::make_server(config).await?.spawn().await?;

        info!(node =? config.public_key(),
            "Initializing haneul-node listening on {}", config.network_address
        );

        let node = HaneulNode {
            authority_server: server,
        };

        node.authority_server.join().await?;

        Ok(())
    }
}
