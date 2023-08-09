// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::*;

#[derive(Parser)]
#[clap(
    name = "haneul-graphql-rpc",
    about = "Haneul GraphQL RPC",
    rename_all = "kebab-case",
    author,
    version
)]
pub enum Command {
    GenerateSchema,
}
