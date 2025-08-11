// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::RpcService;
use haneul_rpc::proto::haneul::rpc::v2beta2::ledger_service_server::LedgerService;
use haneul_rpc::proto::haneul::rpc::v2beta2::BatchGetObjectsRequest;
use haneul_rpc::proto::haneul::rpc::v2beta2::BatchGetObjectsResponse;
use haneul_rpc::proto::haneul::rpc::v2beta2::BatchGetTransactionsRequest;
use haneul_rpc::proto::haneul::rpc::v2beta2::BatchGetTransactionsResponse;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetCheckpointRequest;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetCheckpointResponse;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetEpochRequest;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetEpochResponse;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetObjectRequest;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetObjectResponse;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetServiceInfoRequest;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetServiceInfoResponse;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetTransactionRequest;
use haneul_rpc::proto::haneul::rpc::v2beta2::GetTransactionResponse;

pub(crate) mod get_checkpoint;
mod get_epoch;
mod get_object;
mod get_service_info;
mod get_transaction;
pub use get_epoch::protocol_config_to_proto;
pub use get_object::validate_get_object_requests;

#[tonic::async_trait]
impl LedgerService for RpcService {
    async fn get_service_info(
        &self,
        _request: tonic::Request<GetServiceInfoRequest>,
    ) -> Result<tonic::Response<GetServiceInfoResponse>, tonic::Status> {
        get_service_info::get_service_info(self)
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_object(
        &self,
        request: tonic::Request<GetObjectRequest>,
    ) -> Result<tonic::Response<GetObjectResponse>, tonic::Status> {
        get_object::get_object(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn batch_get_objects(
        &self,
        request: tonic::Request<BatchGetObjectsRequest>,
    ) -> Result<tonic::Response<BatchGetObjectsResponse>, tonic::Status> {
        get_object::batch_get_objects(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_transaction(
        &self,
        request: tonic::Request<GetTransactionRequest>,
    ) -> Result<tonic::Response<GetTransactionResponse>, tonic::Status> {
        get_transaction::get_transaction(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn batch_get_transactions(
        &self,
        request: tonic::Request<BatchGetTransactionsRequest>,
    ) -> Result<tonic::Response<BatchGetTransactionsResponse>, tonic::Status> {
        get_transaction::batch_get_transactions(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_checkpoint(
        &self,
        request: tonic::Request<GetCheckpointRequest>,
    ) -> Result<tonic::Response<GetCheckpointResponse>, tonic::Status> {
        get_checkpoint::get_checkpoint(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_epoch(
        &self,
        request: tonic::Request<GetEpochRequest>,
    ) -> Result<tonic::Response<GetEpochResponse>, tonic::Status> {
        get_epoch::get_epoch(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }
}
