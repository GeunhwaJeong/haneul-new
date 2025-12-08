// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_cli::base::coverage;
use move_package_alt_compilation::build_config::BuildConfig;
use std::path::Path;
use haneul_package_alt::HaneulFlavor;

#[derive(Parser)]
#[group(id = "haneul-move-coverage")]
pub struct Coverage {
    #[clap(flatten)]
    pub coverage: coverage::Coverage,
}

impl Coverage {
    pub async fn execute(
        self,
        path: Option<&Path>,
        build_config: BuildConfig,
    ) -> anyhow::Result<()> {
        self.coverage
            .execute::<HaneulFlavor>(path, build_config)
            .await?;
        Ok(())
    }
}
