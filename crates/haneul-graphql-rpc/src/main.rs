// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use haneul_graphql_rpc::commands::Command;
use haneul_graphql_rpc::schema_sdl_export;

fn main() {
    let cmd: Command = Command::parse();
    match cmd {
        Command::GenerateSchema => {
            println!("{}", &schema_sdl_export());
        }
    }
}
