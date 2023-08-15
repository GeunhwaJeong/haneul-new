// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "haneul-graphql-rpc",
    about = "Haneul GraphQL RPC",
    rename_all = "kebab-case",
    author,
    version
)]
pub enum Command {
    GenerateSchema {
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    StartServer {
        /// URL of the RPC server for data fetching
        #[clap(short, long, default_value = "https://fullnode.testnet.haneul.io:443/")]
        rpc_url: String,
        /// Port to bind the server to
        #[clap(short, long, default_value = "8000")]
        port: u16,
        /// Host to bind the server to
        #[clap(short, long, default_value = "127.0.0.1")]
        host: String,
    },
}
