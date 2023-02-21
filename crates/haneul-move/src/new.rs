// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_cli::base::new;
use std::path::PathBuf;
use haneul_types::HANEUL_FRAMEWORK_ADDRESS;

const HANEUL_PKG_NAME: &str = "Haneul";

// Use devnet by default. Probably want to add options to make this configurable later
const HANEUL_PKG_PATH: &str = "{ git = \"https://github.com/GeunhwaJeong/haneul.git\", subdir = \"crates/haneul-framework\", rev = \"devnet\" }";

#[derive(Parser)]
pub struct New {
    #[clap(flatten)]
    pub new: new::New,
}

impl New {
    pub fn execute(self, path: Option<PathBuf>) -> anyhow::Result<()> {
        let name = &self.new.name.to_lowercase();
        self.new.execute(
            path,
            "0.0.1",
            [(HANEUL_PKG_NAME, HANEUL_PKG_PATH)],
            [
                (name, "0x0"),
                (
                    &HANEUL_PKG_NAME.to_lowercase(),
                    &HANEUL_FRAMEWORK_ADDRESS.to_string(),
                ),
            ],
            "",
        )?;
        Ok(())
    }
}
