// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use haneul_package_alt::HaneulFlavor;
use move_cli::base::lint;
use move_package_alt_compilation::build_config::BuildConfig;
use std::path::Path;

#[derive(Parser)]
#[group(id = "haneul-move-lint")]
pub struct Lint {
    #[clap(flatten)]
    pub lint: lint::Lint,
}

impl Lint {
    pub async fn execute(
        self,
        path: Option<&Path>,
        build_config: BuildConfig,
        flavor: HaneulFlavor,
    ) -> anyhow::Result<()> {
        self.lint.execute(path, build_config, flavor).await
    }
}
