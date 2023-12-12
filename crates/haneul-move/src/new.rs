// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_cli::base::new;
use std::path::PathBuf;

const HANEUL_PKG_NAME: &str = "Haneul";

// Use testnet by default. Probably want to add options to make this configurable later
const HANEUL_PKG_PATH: &str = "{ git = \"https://github.com/GeunhwaJeong/haneul.git\", subdir = \"crates/haneul-framework/packages/haneul-framework\", rev = \"framework/testnet\" }";

#[derive(Parser)]
#[group(id = "haneul-move-new")]
pub struct New {
    #[clap(flatten)]
    pub new: new::New,
}

impl New {
    pub fn execute(self, path: Option<PathBuf>) -> anyhow::Result<()> {
        let name = &self.new.name.to_lowercase();
        self.new
            .execute(path, [(HANEUL_PKG_NAME, HANEUL_PKG_PATH)], [(name, "0x0")], "")?;
        Ok(())
    }
}
