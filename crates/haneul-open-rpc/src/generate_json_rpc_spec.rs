// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::ArgEnum;
use clap::Parser;
use pretty_assertions::assert_str_eq;
use std::fs::File;
use std::io::Write;
use haneul_json_rpc::api::EventReadApiOpenRpc;
use haneul_json_rpc::transaction_builder_api::FullNodeTransactionBuilderApi;
use haneul_json_rpc::transaction_execution_api::FullNodeTransactionExecutionApi;

use crate::examples::RpcExampleProvider;
use haneul_json_rpc::api::EventStreamingApiOpenRpc;
use haneul_json_rpc::bcs_api::BcsApiImpl;
use haneul_json_rpc::read_api::{FullNodeApi, ReadApi};
use haneul_json_rpc::haneul_rpc_doc;
use haneul_json_rpc::HaneulRpcModule;

mod examples;

#[derive(Debug, Parser, Clone, Copy, ArgEnum)]
enum Action {
    Print,
    Test,
    Record,
}

#[derive(Debug, Parser)]
#[clap(
    name = "Haneul format generator",
    about = "Trace serde (de)serialization to generate format descriptions for Haneul types"
)]
struct Options {
    #[clap(arg_enum, default_value = "Record", ignore_case = true)]
    action: Action,
}

const FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/spec/openrpc.json",);

#[tokio::main]
async fn main() {
    let options = Options::parse();

    let mut open_rpc = haneul_rpc_doc();
    open_rpc.add_module(ReadApi::rpc_doc_module());
    open_rpc.add_module(FullNodeApi::rpc_doc_module());
    open_rpc.add_module(BcsApiImpl::rpc_doc_module());
    open_rpc.add_module(EventStreamingApiOpenRpc::module_doc());
    open_rpc.add_module(EventReadApiOpenRpc::module_doc());
    open_rpc.add_module(FullNodeTransactionExecutionApi::rpc_doc_module());
    open_rpc.add_module(FullNodeTransactionBuilderApi::rpc_doc_module());

    open_rpc.add_examples(RpcExampleProvider::new().examples());

    match options.action {
        Action::Print => {
            let content = serde_json::to_string_pretty(&open_rpc).unwrap();
            println!("{content}");
        }
        Action::Record => {
            let content = serde_json::to_string_pretty(&open_rpc).unwrap();
            let mut f = File::create(FILE_PATH).unwrap();
            writeln!(f, "{content}").unwrap();
        }
        Action::Test => {
            let reference = std::fs::read_to_string(FILE_PATH).unwrap();
            let content = serde_json::to_string_pretty(&open_rpc).unwrap() + "\n";
            assert_str_eq!(&reference, &content);
        }
    }
}
