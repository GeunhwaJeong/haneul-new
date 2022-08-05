// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::authority::AuthorityState;
use async_trait::async_trait;
use futures::{stream::BoxStream, TryStreamExt};
use multiaddr::Multiaddr;
use haneullabs_network::config::Config;
use narwhal_crypto::traits::ToFromBytes;
use std::collections::BTreeMap;
use std::sync::Arc;
use haneul_config::genesis::Genesis;
use haneul_network::{api::ValidatorClient, tonic};
use haneul_types::crypto::AuthorityPublicKeyBytes;
use haneul_types::messages_checkpoint::{CheckpointRequest, CheckpointResponse};
use haneul_types::haneul_system_state::HaneulSystemState;
use haneul_types::{error::HaneulError, messages::*};

#[cfg(test)]
use haneul_types::{
    base_types::ObjectID, committee::Committee, crypto::AuthorityKeyPair, object::Object,
};

use crate::epoch::reconfiguration::Reconfigurable;
use haneul_network::tonic::transport::Channel;

#[async_trait]
pub trait AuthorityAPI {
    /// Initiate a new transaction to a Haneul or Primary account.
    async fn handle_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<TransactionInfoResponse, HaneulError>;

    /// Execute a certificate.
    async fn handle_certificate(
        &self,
        certificate: CertifiedTransaction,
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

    async fn handle_batch_stream(
        &self,
        request: BatchInfoRequest,
    ) -> Result<BatchInfoResponseItemStream, HaneulError>;

    async fn handle_checkpoint(
        &self,
        request: CheckpointRequest,
    ) -> Result<CheckpointResponse, HaneulError>;

    async fn handle_epoch(&self, request: EpochRequest) -> Result<EpochResponse, HaneulError>;
}

pub type BatchInfoResponseItemStream = BoxStream<'static, Result<BatchInfoResponseItem, HaneulError>>;

#[derive(Clone)]
pub struct NetworkAuthorityClient {
    client: ValidatorClient<tonic::transport::Channel>,
}

impl NetworkAuthorityClient {
    pub async fn connect(address: &Multiaddr) -> anyhow::Result<Self> {
        let channel = haneullabs_network::client::connect(address).await?;
        Ok(Self::new(channel))
    }

    pub fn connect_lazy(address: &Multiaddr) -> anyhow::Result<Self> {
        let channel = haneullabs_network::client::connect_lazy(address)?;
        Ok(Self::new(channel))
    }

    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            client: ValidatorClient::new(channel),
        }
    }

    fn client(&self) -> ValidatorClient<tonic::transport::Channel> {
        self.client.clone()
    }
}

#[async_trait]
impl Reconfigurable for NetworkAuthorityClient {
    fn needs_network_recreation() -> bool {
        true
    }

    fn recreate(channel: tonic::transport::Channel) -> Self {
        NetworkAuthorityClient::new(channel)
    }
}

#[async_trait]
impl AuthorityAPI for NetworkAuthorityClient {
    /// Initiate a new transfer to a Haneul or Primary account.
    async fn handle_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<TransactionInfoResponse, HaneulError> {
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
    ) -> Result<TransactionInfoResponse, HaneulError> {
        self.client()
            .handle_certificate(certificate)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    async fn handle_account_info_request(
        &self,
        request: AccountInfoRequest,
    ) -> Result<AccountInfoResponse, HaneulError> {
        self.client()
            .account_info(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
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

    /// Handle Batch information requests for this authority.
    async fn handle_batch_stream(
        &self,
        request: BatchInfoRequest,
    ) -> Result<BatchInfoResponseItemStream, HaneulError> {
        let stream = self
            .client()
            .batch_info(request)
            .await
            .map(tonic::Response::into_inner)?
            .map_err(Into::into);

        Ok(Box::pin(stream))
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

    async fn handle_epoch(&self, request: EpochRequest) -> Result<EpochResponse, HaneulError> {
        self.client()
            .epoch_info(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }
}

pub fn make_network_authority_client_sets_from_system_state(
    haneul_system_state: &HaneulSystemState,
    network_config: &Config,
) -> anyhow::Result<BTreeMap<AuthorityPublicKeyBytes, NetworkAuthorityClient>> {
    let mut authority_clients = BTreeMap::new();
    for validator in &haneul_system_state.validators.active_validators {
        let address = Multiaddr::try_from(validator.metadata.net_address.clone())?;
        let channel = network_config.connect_lazy(&address)?;
        let client = NetworkAuthorityClient::new(channel);
        let name: &[u8] = &validator.metadata.name;
        let public_key_bytes = AuthorityPublicKeyBytes::from_bytes(name)?;
        authority_clients.insert(public_key_bytes, client);
    }
    Ok(authority_clients)
}

pub fn make_network_authority_client_sets_from_genesis(
    genesis: &Genesis,
    network_config: &Config,
) -> anyhow::Result<BTreeMap<AuthorityPublicKeyBytes, NetworkAuthorityClient>> {
    let mut authority_clients = BTreeMap::new();
    for validator in genesis.validator_set() {
        let channel = network_config.connect_lazy(validator.network_address())?;
        let client = NetworkAuthorityClient::new(channel);
        authority_clients.insert(validator.public_key(), client);
    }
    Ok(authority_clients)
}

#[derive(Clone, Copy, Default)]
pub struct LocalAuthorityClientFaultConfig {
    pub fail_before_handle_transaction: bool,
    pub fail_after_handle_transaction: bool,
    pub fail_before_handle_confirmation: bool,
    pub fail_after_handle_confirmation: bool,
}

impl LocalAuthorityClientFaultConfig {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone)]
pub struct LocalAuthorityClient {
    pub state: Arc<AuthorityState>,
    pub fault_config: LocalAuthorityClientFaultConfig,
}

impl Reconfigurable for LocalAuthorityClient {
    fn needs_network_recreation() -> bool {
        false
    }

    fn recreate(_channel: Channel) -> Self {
        unreachable!(); // this function should not get called because the above function returns false
    }
}

#[async_trait]
impl AuthorityAPI for LocalAuthorityClient {
    async fn handle_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<TransactionInfoResponse, HaneulError> {
        if self.fault_config.fail_before_handle_transaction {
            return Err(HaneulError::from("Mock error before handle_transaction"));
        }
        let state = self.state.clone();
        let result = state.handle_transaction(transaction).await;
        if self.fault_config.fail_after_handle_transaction {
            return Err(HaneulError::GenericAuthorityError {
                error: "Mock error after handle_transaction".to_owned(),
            });
        }
        result
    }

    async fn handle_certificate(
        &self,
        certificate: CertifiedTransaction,
    ) -> Result<TransactionInfoResponse, HaneulError> {
        if self.fault_config.fail_before_handle_confirmation {
            return Err(HaneulError::GenericAuthorityError {
                error: "Mock error before handle_confirmation_transaction".to_owned(),
            });
        }
        let state = self.state.clone();
        let result = state.handle_certificate(certificate).await;
        if self.fault_config.fail_after_handle_confirmation {
            return Err(HaneulError::GenericAuthorityError {
                error: "Mock error after handle_confirmation_transaction".to_owned(),
            });
        }
        result
    }

    async fn handle_account_info_request(
        &self,
        request: AccountInfoRequest,
    ) -> Result<AccountInfoResponse, HaneulError> {
        let state = self.state.clone();
        state.handle_account_info_request(request).await
    }

    async fn handle_object_info_request(
        &self,
        request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, HaneulError> {
        let state = self.state.clone();
        state.handle_object_info_request(request).await
    }

    /// Handle Object information requests for this account.
    async fn handle_transaction_info_request(
        &self,
        request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, HaneulError> {
        let state = self.state.clone();
        state.handle_transaction_info_request(request).await
    }

    /// Handle Batch information requests for this authority.
    async fn handle_batch_stream(
        &self,
        request: BatchInfoRequest,
    ) -> Result<BatchInfoResponseItemStream, HaneulError> {
        let state = self.state.clone();

        let update_items = state.handle_batch_streaming(request).await?;
        Ok(Box::pin(update_items))
    }

    async fn handle_checkpoint(
        &self,
        request: CheckpointRequest,
    ) -> Result<CheckpointResponse, HaneulError> {
        let state = self.state.clone();

        state.handle_checkpoint_request(&request)
    }

    async fn handle_epoch(&self, request: EpochRequest) -> Result<EpochResponse, HaneulError> {
        let state = self.state.clone();

        state.handle_epoch_request(&request)
    }
}

impl LocalAuthorityClient {
    #[cfg(test)]
    pub async fn new(
        committee: Committee,
        address: AuthorityPublicKeyBytes,
        secret: AuthorityKeyPair,
        genesis: &Genesis,
    ) -> Self {
        use crate::authority::AuthorityStore;
        use crate::checkpoints::CheckpointStore;
        use parking_lot::Mutex;
        use std::{env, fs};

        // Random directory
        let dir = env::temp_dir();
        let path = dir.join(format!("DB_{:?}", ObjectID::random()));
        fs::create_dir(&path).unwrap();

        let secret = Arc::pin(secret);

        let mut store_path = path.clone();
        store_path.push("store");
        let store = Arc::new(AuthorityStore::open(&store_path, None));
        let mut checkpoints_path = path.clone();
        checkpoints_path.push("checkpoints");
        let checkpoints = CheckpointStore::open(
            &checkpoints_path,
            None,
            committee.epoch,
            address,
            secret.clone(),
        )
        .expect("Should not fail to open local checkpoint DB");

        let state = AuthorityState::new(
            committee.clone(),
            address,
            secret.clone(),
            store,
            None,
            None,
            Some(Arc::new(Mutex::new(checkpoints))),
            genesis,
            &prometheus::Registry::new(),
        )
        .await;
        Self {
            state: Arc::new(state),
            fault_config: LocalAuthorityClientFaultConfig::default(),
        }
    }

    #[cfg(test)]
    pub async fn new_with_objects(
        committee: Committee,
        address: AuthorityPublicKeyBytes,
        secret: AuthorityKeyPair,
        objects: Vec<Object>,
        genesis: &Genesis,
    ) -> Self {
        let client = Self::new(committee, address, secret, genesis).await;

        for object in objects {
            client.state.insert_genesis_object(object).await;
        }

        client
    }

    pub fn new_from_authority(state: Arc<AuthorityState>) -> Self {
        Self {
            state,
            fault_config: LocalAuthorityClientFaultConfig::default(),
        }
    }
}
