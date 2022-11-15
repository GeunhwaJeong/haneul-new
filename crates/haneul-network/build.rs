// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{
    env,
    path::{Path, PathBuf},
};
use tonic_build::manual::{Builder, Method, Service};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let out_dir = if env::var("DUMP_GENERATED_GRPC").is_ok() {
        PathBuf::from("")
    } else {
        PathBuf::from(env::var("OUT_DIR")?)
    };

    let codec_path = "haneullabs_network::codec::BincodeCodec";

    let validator_service = Service::builder()
        .name("Validator")
        .package("haneul.validator")
        .comment("The Validator interface")
        .method(
            Method::builder()
                .name("transaction")
                .route_name("Transaction")
                .input_type("haneul_types::messages::Transaction")
                .output_type("haneul_types::messages::TransactionInfoResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("handle_certificate")
                .route_name("CertifiedTransaction")
                .input_type("haneul_types::messages::CertifiedTransaction")
                .output_type("haneul_types::messages::TransactionInfoResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("account_info")
                .route_name("AccountInfo")
                .input_type("haneul_types::messages::AccountInfoRequest")
                .output_type("haneul_types::messages::AccountInfoResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("object_info")
                .route_name("ObjectInfo")
                .input_type("haneul_types::messages::ObjectInfoRequest")
                .output_type("haneul_types::messages::ObjectInfoResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("transaction_info")
                .route_name("TransactionInfo")
                .input_type("haneul_types::messages::TransactionInfoRequest")
                .output_type("haneul_types::messages::TransactionInfoResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("checkpoint")
                .route_name("Checkpoint")
                .input_type("haneul_types::messages_checkpoint::CheckpointRequest")
                .output_type("haneul_types::messages_checkpoint::CheckpointResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("batch_info")
                .route_name("FollowTxStream")
                .input_type("haneul_types::messages::BatchInfoRequest")
                .output_type("haneul_types::messages::BatchInfoResponseItem")
                .server_streaming()
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("committee_info")
                .route_name("CommitteeInfo")
                .input_type("haneul_types::messages::CommitteeInfoRequest")
                .output_type("haneul_types::messages::CommitteeInfoResponse")
                .codec_path(codec_path)
                .build(),
        )
        .build();

    Builder::new()
        .out_dir(&out_dir)
        .compile(&[validator_service]);

    build_anemo_services(&out_dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DUMP_GENERATED_GRPC");

    Ok(())
}

fn build_anemo_services(out_dir: &Path) {
    let discovery = anemo_build::manual::Service::builder()
        .name("Discovery")
        .package("haneul")
        .method(
            anemo_build::manual::Method::builder()
                .name("get_external_address")
                .route_name("GetExternalAddress")
                .request_type("()")
                .response_type("std::net::SocketAddr")
                .codec_path("anemo::rpc::codec::BincodeCodec")
                .build(),
        )
        .method(
            anemo_build::manual::Method::builder()
                .name("get_known_peers")
                .route_name("GetKnownPeers")
                .request_type("()")
                .response_type("crate::discovery::GetKnownPeersResponse")
                .codec_path("anemo::rpc::codec::BincodeCodec")
                .build(),
        )
        .build();

    anemo_build::manual::Builder::new()
        .out_dir(out_dir)
        .compile(&[discovery]);
}
