// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_macros::sim_test;
use haneul_rpc_api::proto::rpc::v2alpha::live_data_service_client::LiveDataServiceClient;
use haneul_rpc_api::proto::rpc::v2alpha::GetCoinInfoRequest;
use haneul_rpc_api::proto::rpc::v2alpha::GetCoinInfoResponse;
use haneul_sdk_types::TypeTag;
use test_cluster::TestClusterBuilder;

#[sim_test]
async fn get_coin_info() {
    let test_cluster = TestClusterBuilder::new().build().await;

    let mut grpc_client = LiveDataServiceClient::connect(test_cluster.rpc_url().to_owned())
        .await
        .unwrap();

    let coin_type_sdk: TypeTag = "0x2::haneul::HANEUL".parse().unwrap();
    let request = GetCoinInfoRequest {
        coin_type: Some(coin_type_sdk.to_string()),
    };

    let GetCoinInfoResponse {
        coin_type,
        metadata,
        treasury,
    } = grpc_client
        .get_coin_info(request)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(coin_type, Some(coin_type_sdk.to_string()));
    assert_eq!(metadata.unwrap().symbol, Some("HANEUL".to_owned()));
    assert_eq!(
        treasury.unwrap().total_supply,
        Some(haneul_types::gas_coin::TOTAL_SUPPLY_GEUNHWA)
    );
}
