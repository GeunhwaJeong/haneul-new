// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_cli::base::new;
use std::path::PathBuf;

const HANEUL_PKG_NAME: &str = "Haneul";
const HANEUL_PKG_PATH: &str = "{ git = \"https://github.com/GeunhwaJeong/haneul.git\", subdir = \"crates/haneul-framework\", rev = \"main\" }";

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
            [(name, "0x0")],
        )?;
        Ok(())
    }
}
