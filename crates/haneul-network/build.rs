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

    let codec_path = "haneullabs_network::codec::BcsCodec";

    let validator_service = Service::builder()
        .name("Validator")
        .package("haneul.validator")
        .comment("The Validator interface")
        .method(
            Method::builder()
                .name("transaction")
                .route_name("Transaction")
                .input_type("haneul_types::messages::Transaction")
                .output_type("haneul_types::messages::HandleTransactionResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("handle_certificate")
                .route_name("CertifiedTransaction")
                .input_type("haneul_types::messages::CertifiedTransaction")
                .output_type("haneul_types::messages::HandleCertificateResponse")
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
                .name("committee_info")
                .route_name("CommitteeInfo")
                .input_type("haneul_types::messages::CommitteeInfoRequest")
                .output_type("haneul_types::messages::CommitteeInfoResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            Method::builder()
                .name("get_system_state_object")
                .route_name("GetSystemStateObject")
                .input_type("haneul_types::messages::SystemStateRequest")
                .output_type("haneul_types::haneul_system_state::HaneulSystemState")
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
    let codec_path = "haneullabs_network::codec::anemo::BcsSnappyCodec";

    let discovery = anemo_build::manual::Service::builder()
        .name("Discovery")
        .package("haneul")
        .method(
            anemo_build::manual::Method::builder()
                .name("get_known_peers")
                .route_name("GetKnownPeers")
                .request_type("()")
                .response_type("crate::discovery::GetKnownPeersResponse")
                .codec_path(codec_path)
                .build(),
        )
        .build();

    let state_sync = anemo_build::manual::Service::builder()
        .name("StateSync")
        .package("haneul")
        .method(
            anemo_build::manual::Method::builder()
                .name("push_checkpoint_summary")
                .route_name("PushCheckpointSummary")
                .request_type("haneul_types::messages_checkpoint::CertifiedCheckpointSummary")
                .response_type("()")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            anemo_build::manual::Method::builder()
                .name("get_checkpoint_summary")
                .route_name("GetCheckpointSummary")
                .request_type("crate::state_sync::GetCheckpointSummaryRequest")
                .response_type("Option<haneul_types::messages_checkpoint::CertifiedCheckpointSummary>")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            anemo_build::manual::Method::builder()
                .name("get_checkpoint_contents")
                .route_name("GetCheckpointContents")
                .request_type("haneul_types::messages_checkpoint::CheckpointContentsDigest")
                .response_type("Option<haneul_types::messages_checkpoint::CheckpointContents>")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            anemo_build::manual::Method::builder()
                .name("get_transaction_and_effects")
                .route_name("GetTransactionAndEffects")
                .request_type("haneul_types::base_types::ExecutionDigests")
                .response_type("Option<(haneul_types::messages::Transaction, haneul_types::messages::TransactionEffects)>")
                .codec_path(codec_path)
                .build(),
        )
        .build();

    anemo_build::manual::Builder::new()
        .out_dir(out_dir)
        .compile(&[discovery, state_sync]);
}
