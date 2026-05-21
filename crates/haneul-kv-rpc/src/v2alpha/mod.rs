// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::KvRpcServer;
use crate::operation::OperationSpec;
use haneul_rpc::proto::haneul::rpc::v2alpha::ListCheckpointsRequest;
use haneul_rpc::proto::haneul::rpc::v2alpha::ListCheckpointsResponse;
use haneul_rpc::proto::haneul::rpc::v2alpha::ListEventsRequest;
use haneul_rpc::proto::haneul::rpc::v2alpha::ListEventsResponse;
use haneul_rpc::proto::haneul::rpc::v2alpha::ListTransactionsRequest;
use haneul_rpc::proto::haneul::rpc::v2alpha::ListTransactionsResponse;
use haneul_rpc::proto::haneul::rpc::v2alpha::ledger_service_server::LedgerService;
use tonic::codegen::BoxStream;

mod list_checkpoints;
mod list_events;
mod list_transactions;

// Per-RPC hard request timeout (from `LedgerHistoryConfig`). The outer
// `operation::with_deadline` wrapper drops the response stream with
// `DeadlineExceeded` when this fires; debounced intermediate `Watermark` frames
// let the client resume from wherever it got to.
#[tonic::async_trait]
impl LedgerService for KvRpcServer {
    async fn list_checkpoints(
        &self,
        request: tonic::Request<ListCheckpointsRequest>,
    ) -> Result<tonic::Response<BoxStream<ListCheckpointsResponse>>, tonic::Status> {
        self.serve_query_stream(
            OperationSpec::new(
                "list_checkpoints",
                self.ledger_history.list_checkpoints().timeout,
            ),
            request,
            list_checkpoints::list_checkpoints,
        )
        .await
    }

    async fn list_transactions(
        &self,
        request: tonic::Request<ListTransactionsRequest>,
    ) -> Result<tonic::Response<BoxStream<ListTransactionsResponse>>, tonic::Status> {
        self.serve_query_stream(
            OperationSpec::new(
                "list_transactions",
                self.ledger_history.list_transactions().timeout,
            ),
            request,
            list_transactions::list_transactions,
        )
        .await
    }

    async fn list_events(
        &self,
        request: tonic::Request<ListEventsRequest>,
    ) -> Result<tonic::Response<BoxStream<ListEventsResponse>>, tonic::Status> {
        self.serve_query_stream(
            OperationSpec::new("list_events", self.ledger_history.list_events().timeout),
            request,
            list_events::list_events,
        )
        .await
    }
}
