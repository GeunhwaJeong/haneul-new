// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_cli::package::cli::create_move_package;
use std::path::Path;

pub fn execute(path: &Path, name: &String) -> anyhow::Result<()> {
    create_move_package(path,
                        name,
                        "0.0.1",
                        "Haneul",
                        "{ git = \"https://github.com/GeunhwaJeong/haneul.git\", subdir = \"crates/haneul-framework\", rev = \"main\" }",
                        &name.to_lowercase(),
                        "0x0")?;
    Ok(())
}
