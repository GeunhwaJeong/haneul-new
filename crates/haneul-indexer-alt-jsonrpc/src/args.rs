// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_pg_db::DbArgs;

use crate::RpcArgs;

#[derive(clap::Parser, Debug, Clone)]
pub struct Args {
    #[command(flatten)]
    pub db_args: DbArgs,

    #[command(flatten)]
    pub rpc_args: RpcArgs,
}
