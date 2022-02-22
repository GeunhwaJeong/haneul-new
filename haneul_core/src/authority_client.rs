// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use haneul_network::network::NetworkClient;
use haneul_types::{error::HaneulError, messages::*, serialize::*};

#[async_trait]
pub trait AuthorityAPI {
    /// Initiate a new transaction to a Haneul or Primary account.
    async fn handle_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<TransactionInfoResponse, HaneulError>;

    /// Confirm a transaction to a Haneul or Primary account.
    async fn handle_confirmation_transaction(
        &self,
        transaction: ConfirmationTransaction,
    ) -> Result<TransactionInfoResponse, HaneulError>;

    /// Handle Account information requests for this account.
    async fn handle_account_info_request(
        &self,
        request: AccountInfoRequest,
    ) -> Result<AccountInfoResponse, HaneulError>;

    /// Handle Object information requests for this account.
    async fn handle_object_info_request(
        &self,
        request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, HaneulError>;

    /// Handle Object information requests for this account.
    async fn handle_transaction_info_request(
        &self,
        request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, HaneulError>;
}

#[derive(Clone)]
pub struct AuthorityClient(NetworkClient);

impl AuthorityClient {
    pub fn new(network_client: NetworkClient) -> Self {
        Self(network_client)
    }
}

#[async_trait]
impl AuthorityAPI for AuthorityClient {
    /// Initiate a new transfer to a Haneul or Primary account.
    async fn handle_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<TransactionInfoResponse, HaneulError> {
        let response = self
            .0
            .send_recv_bytes(serialize_transaction(&transaction))
            .await?;
        deserialize_transaction_info(response)
    }

    /// Confirm a transfer to a Haneul or Primary account.
    async fn handle_confirmation_transaction(
        &self,
        transaction: ConfirmationTransaction,
    ) -> Result<TransactionInfoResponse, HaneulError> {
        let response = self
            .0
            .send_recv_bytes(serialize_cert(&transaction.certificate))
            .await?;
        deserialize_transaction_info(response)
    }

    async fn handle_account_info_request(
        &self,
        request: AccountInfoRequest,
    ) -> Result<AccountInfoResponse, HaneulError> {
        let response = self
            .0
            .send_recv_bytes(serialize_account_info_request(&request))
            .await?;
        deserialize_account_info(response)
    }

    async fn handle_object_info_request(
        &self,
        request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, HaneulError> {
        let response = self
            .0
            .send_recv_bytes(serialize_object_info_request(&request))
            .await?;
        deserialize_object_info(response)
    }

    /// Handle Object information requests for this account.
    async fn handle_transaction_info_request(
        &self,
        request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, HaneulError> {
        let response = self
            .0
            .send_recv_bytes(serialize_transaction_info_request(&request))
            .await?;
        deserialize_transaction_info(response)
    }
}
