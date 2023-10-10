// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use async_trait::async_trait;
use haneullabs_network::config::Config;
use std::collections::BTreeMap;
use std::time::Duration;
use haneul_network::{api::ValidatorClient, tonic};
use haneul_types::base_types::AuthorityName;
use haneul_types::committee::CommitteeWithNetworkMetadata;
use haneul_types::messages_checkpoint::{CheckpointRequest, CheckpointResponse};
use haneul_types::multiaddr::Multiaddr;
use haneul_types::haneul_system_state::HaneulSystemState;
use haneul_types::{error::HaneulError, transaction::*};

use haneul_network::tonic::transport::Channel;
use haneul_types::messages_grpc::{
    HandleCertificateResponse, HandleCertificateResponseV2, HandleTransactionResponse,
    ObjectInfoRequest, ObjectInfoResponse, SystemStateRequest, TransactionInfoRequest,
    TransactionInfoResponse,
};

#[async_trait]
pub trait AuthorityAPI {
    /// Initiate a new transaction to a Haneul or Primary account.
    async fn handle_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<HandleTransactionResponse, HaneulError>;

    /// Execute a certificate.
    async fn handle_certificate(
        &self,
        certificate: CertifiedTransaction,
    ) -> Result<HandleCertificateResponse, HaneulError>;

    /// Execute a certificate.
    async fn handle_certificate_v2(
        &self,
        certificate: CertifiedTransaction,
    ) -> Result<HandleCertificateResponseV2, HaneulError>;

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

    async fn handle_checkpoint(
        &self,
        request: CheckpointRequest,
    ) -> Result<CheckpointResponse, HaneulError>;

    // This API is exclusively used by the benchmark code.
    // Hence it's OK to return a fixed system state type.
    async fn handle_system_state_object(
        &self,
        request: SystemStateRequest,
    ) -> Result<HaneulSystemState, HaneulError>;
}

#[derive(Clone)]
pub struct NetworkAuthorityClient {
    client: ValidatorClient<Channel>,
}

impl NetworkAuthorityClient {
    pub async fn connect(address: &Multiaddr) -> anyhow::Result<Self> {
        let channel = haneullabs_network::client::connect(address)
            .await
            .map_err(|err| anyhow!(err.to_string()))?;
        Ok(Self::new(channel))
    }

    pub fn connect_lazy(address: &Multiaddr) -> anyhow::Result<Self> {
        let channel = haneullabs_network::client::connect_lazy(address)
            .map_err(|err| anyhow!(err.to_string()))?;
        Ok(Self::new(channel))
    }

    pub fn new(channel: Channel) -> Self {
        Self {
            client: ValidatorClient::new(channel),
        }
    }

    fn client(&self) -> ValidatorClient<Channel> {
        self.client.clone()
    }
}

#[async_trait]
impl AuthorityAPI for NetworkAuthorityClient {
    /// Initiate a new transfer to a Haneul or Primary account.
    async fn handle_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<HandleTransactionResponse, HaneulError> {
        self.client()
            .transaction(transaction)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    /// Execute a certificate.
    async fn handle_certificate(
        &self,
        certificate: CertifiedTransaction,
    ) -> Result<HandleCertificateResponse, HaneulError> {
        self.client()
            .handle_certificate(certificate)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    /// Execute a certificate.
    async fn handle_certificate_v2(
        &self,
        certificate: CertifiedTransaction,
    ) -> Result<HandleCertificateResponseV2, HaneulError> {
        let response = self
            .client()
            .handle_certificate_v2(certificate.clone())
            .await
            .map(tonic::Response::into_inner);

        if response.is_ok() {
            return response.map_err(Into::into);
        }
        // TODO: remove this once all validators upgrade
        if response.as_ref().err().unwrap().code() == tonic::Code::Unimplemented {
            let response = self
                .client()
                .handle_certificate(certificate)
                .await
                .map(tonic::Response::into_inner)
                .map_err(HaneulError::from)?;
            return Ok(HandleCertificateResponseV2 {
                signed_effects: response.signed_effects,
                events: response.events,
                fastpath_input_objects: vec![], // unused field
            });
        }
        response.map_err(Into::into)
    }

    async fn handle_object_info_request(
        &self,
        request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, HaneulError> {
        self.client()
            .object_info(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    /// Handle Object information requests for this account.
    async fn handle_transaction_info_request(
        &self,
        request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, HaneulError> {
        self.client()
            .transaction_info(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    /// Handle Object information requests for this account.
    async fn handle_checkpoint(
        &self,
        request: CheckpointRequest,
    ) -> Result<CheckpointResponse, HaneulError> {
        self.client()
            .checkpoint(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    async fn handle_system_state_object(
        &self,
        request: SystemStateRequest,
    ) -> Result<HaneulSystemState, HaneulError> {
        self.client()
            .get_system_state_object(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }
}

pub fn make_network_authority_clients_with_network_config(
    committee: &CommitteeWithNetworkMetadata,
    network_config: &Config,
) -> anyhow::Result<BTreeMap<AuthorityName, NetworkAuthorityClient>> {
    let mut authority_clients = BTreeMap::new();
    for (name, _stakes) in &committee.committee.voting_rights {
        let address = &committee
            .network_metadata
            .get(name)
            .ok_or_else(|| {
                HaneulError::from("Missing network metadata in CommitteeWithNetworkMetadata")
            })?
            .network_address;
        let channel = network_config
            .connect_lazy(address)
            .map_err(|err| anyhow!(err.to_string()))?;
        let client = NetworkAuthorityClient::new(channel);
        authority_clients.insert(*name, client);
    }
    Ok(authority_clients)
}

pub fn make_authority_clients_with_timeout_config(
    committee: &CommitteeWithNetworkMetadata,
    connect_timeout: Duration,
    request_timeout: Duration,
) -> anyhow::Result<BTreeMap<AuthorityName, NetworkAuthorityClient>> {
    let mut network_config = haneullabs_network::config::Config::new();
    network_config.connect_timeout = Some(connect_timeout);
    network_config.request_timeout = Some(request_timeout);
    make_network_authority_clients_with_network_config(committee, &network_config)
}
