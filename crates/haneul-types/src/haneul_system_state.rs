// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::base_types::{AuthorityName, ObjectID, HaneulAddress};
use crate::collection_types::{VecMap, VecSet};
use crate::committee::{Committee, CommitteeWithNetAddresses, ProtocolVersion, StakeUnit};
use crate::crypto::{AuthorityPublicKeyBytes, NetworkPublicKey};
use crate::error::HaneulError;
use crate::storage::ObjectStore;
use crate::{balance::Balance, id::UID, HANEUL_FRAMEWORK_ADDRESS, HANEUL_SYSTEM_STATE_OBJECT_ID};
use fastcrypto::traits::ToFromBytes;
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use multiaddr::Multiaddr;
use narwhal_config::{Committee as NarwhalCommittee, WorkerCache, WorkerIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const HANEUL_SYSTEM_STATE_WRAPPER_STRUCT_NAME: &IdentStr = ident_str!("HaneulSystemState");
pub const HANEUL_SYSTEM_MODULE_NAME: &IdentStr = ident_str!("haneul_system");
pub const ADVANCE_EPOCH_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch");
pub const ADVANCE_EPOCH_SAFE_MODE_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch_safe_mode");
pub const CONSENSUS_COMMIT_PROLOGUE_FUNCTION_NAME: &IdentStr =
    ident_str!("consensus_commit_prologue");

/// Rust version of the Move haneul::haneul_system::SystemParameters type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct SystemParameters {
    pub min_validator_stake: u64,
    pub max_validator_candidate_count: u64,
}

/// Rust version of the Move std::option::Option type.
/// Putting it in this file because it's only used here.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct MoveOption<T> {
    pub vec: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct ValidatorMetadata {
    pub haneul_address: HaneulAddress,
    pub pubkey_bytes: Vec<u8>,
    pub network_pubkey_bytes: Vec<u8>,
    pub worker_pubkey_bytes: Vec<u8>,
    pub proof_of_possession_bytes: Vec<u8>,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub project_url: String,
    pub net_address: Vec<u8>,
    pub p2p_address: Vec<u8>,
    pub consensus_address: Vec<u8>,
    pub worker_address: Vec<u8>,
}

/// Rust version of the Move haneul::validator::Validator type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct Validator {
    pub metadata: ValidatorMetadata,
    pub voting_power: u64,
    pub stake_amount: u64,
    pub pending_stake: u64,
    pub pending_withdraw: u64,
    pub gas_price: u64,
    pub delegation_staking_pool: StakingPool,
    pub commission_rate: u64,
    pub next_epoch_stake: u64,
    pub next_epoch_delegation: u64,
    pub next_epoch_gas_price: u64,
    pub next_epoch_commission_rate: u64,
}

impl Validator {
    pub fn to_current_epoch_committee_with_net_addresses(
        &self,
    ) -> (AuthorityName, StakeUnit, Vec<u8>) {
        (
            // TODO: Make sure we are actually verifying this on-chain.
            AuthorityPublicKeyBytes::from_bytes(self.metadata.pubkey_bytes.as_ref())
                .expect("Validity of public key bytes should be verified on-chain"),
            self.voting_power,
            self.metadata.net_address.clone(),
        )
    }
}

/// Rust version of the Move haneul::staking_pool::PendingDelegationEntry type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct PendingDelegationEntry {
    pub delegator: HaneulAddress,
    pub haneul_amount: u64,
    pub staked_haneul_id: ObjectID,
}

/// Rust version of the Move haneul::staking_pool::PendingWithdrawEntry type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct PendingWithdrawEntry {
    delegator: HaneulAddress,
    principal_withdraw_amount: u64,
    withdrawn_pool_tokens: Balance,
}

/// Rust version of the Move haneul::table::Table type. Putting it here since
/// we only use it in haneul_system in the framework.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct TableVec {
    pub contents: Table,
}

impl Default for TableVec {
    fn default() -> Self {
        TableVec {
            contents: Table {
                id: ObjectID::from(HaneulAddress::ZERO),
                size: 0,
            },
        }
    }
}

/// Rust version of the Move haneul::table::Table type. Putting it here since
/// we only use it in haneul_system in the framework.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct Table {
    pub id: ObjectID,
    pub size: u64,
}

impl Default for Table {
    fn default() -> Self {
        Table {
            id: ObjectID::from(HaneulAddress::ZERO),
            size: 0,
        }
    }
}

/// Rust version of the Move haneul::linked_table::LinkedTable type. Putting it here since
/// we only use it in haneul_system in the framework.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct LinkedTable<K> {
    pub id: ObjectID,
    pub size: u64,
    pub head: MoveOption<K>,
    pub tail: MoveOption<K>,
}

impl<K> Default for LinkedTable<K> {
    fn default() -> Self {
        LinkedTable {
            id: ObjectID::from(HaneulAddress::ZERO),
            size: 0,
            head: MoveOption { vec: vec![] },
            tail: MoveOption { vec: vec![] },
        }
    }
}

/// Rust version of the Move haneul::staking_pool::StakingPool type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct StakingPool {
    pub id: ObjectID,
    pub starting_epoch: u64,
    pub haneul_balance: u64,
    pub rewards_pool: Balance,
    pub pool_token_balance: u64,
    pub exchange_rates: Table,
    pub pending_delegation: u64,
    pub pending_withdraws: TableVec,
}

/// Rust version of the Move haneul::validator_set::ValidatorPair type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct ValidatorPair {
    from: HaneulAddress,
    to: HaneulAddress,
}

/// Rust version of the Move haneul::validator_set::ValidatorSet type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct ValidatorSet {
    pub validator_stake: u64,
    pub delegation_stake: u64,
    pub active_validators: Vec<Validator>,
    pub pending_validators: Vec<Validator>,
    pub pending_removals: Vec<u64>,
    pub next_epoch_validators: Vec<ValidatorMetadata>,
    pub staking_pool_mappings: Table,
}

/// Rust version of the Move haneul::haneul_system::HaneulSystemStateInner type
/// We want to keep it named as HaneulSystemState in Rust since this is the primary interface type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct HaneulSystemState {
    pub info: UID,
    pub epoch: u64,
    pub protocol_version: u64,
    pub validators: ValidatorSet,
    pub storage_fund: Balance,
    pub parameters: SystemParameters,
    pub reference_gas_price: u64,
    pub validator_report_records: VecMap<HaneulAddress, VecSet<HaneulAddress>>,
    pub stake_subsidy: StakeSubsidy,
    pub safe_mode: bool,
    pub epoch_start_timestamp_ms: u64,
    // TODO: Use getters instead of all pub.
}

/// Rust version of the Move haneul::haneul_system::HaneulSystemState type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HaneulSystemStateWrapper {
    pub info: UID,
    pub version: u64,
    pub system_state: HaneulSystemState,
}

impl HaneulSystemStateWrapper {
    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            name: HANEUL_SYSTEM_STATE_WRAPPER_STRUCT_NAME.to_owned(),
            module: HANEUL_SYSTEM_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct StakeSubsidy {
    pub epoch_counter: u64,
    pub balance: Balance,
    pub current_epoch_amount: u64,
}

impl HaneulSystemState {
    pub fn get_current_epoch_committee(&self) -> CommitteeWithNetAddresses {
        let mut voting_rights = BTreeMap::new();
        let mut net_addresses = BTreeMap::new();
        for validator in &self.validators.active_validators {
            let (name, voting_stake, net_address) =
                validator.to_current_epoch_committee_with_net_addresses();
            voting_rights.insert(name, voting_stake);
            net_addresses.insert(name, net_address);
        }
        CommitteeWithNetAddresses {
            committee: Committee::new(
                self.epoch,
                ProtocolVersion::new(self.protocol_version),
                voting_rights,
            )
            // unwrap is safe because we should have verified the committee on-chain.
            // TODO: Make sure we actually verify it.
            .unwrap(),
            net_addresses,
        }
    }

    #[allow(clippy::mutable_key_type)]
    pub fn get_current_epoch_narwhal_committee(&self) -> NarwhalCommittee {
        let narwhal_committee = self
            .validators
            .active_validators
            .iter()
            .map(|validator| {
                let name = narwhal_crypto::PublicKey::from_bytes(&validator.metadata.pubkey_bytes)
                    .expect("Can't get narwhal public key");
                let network_key = narwhal_crypto::NetworkPublicKey::from_bytes(
                    &validator.metadata.network_pubkey_bytes,
                )
                .expect("Can't get narwhal network key");
                let primary_address =
                    Multiaddr::try_from(validator.metadata.consensus_address.clone())
                        .expect("Can't get narwhal primary address");
                let authority = narwhal_config::Authority {
                    stake: validator.voting_power as narwhal_config::Stake,
                    primary_address,
                    network_key,
                };
                (name, authority)
            })
            .collect();

        narwhal_config::Committee {
            authorities: narwhal_committee,
            epoch: self.epoch as narwhal_config::Epoch,
        }
    }

    #[allow(clippy::mutable_key_type)]
    pub fn get_current_epoch_narwhal_worker_cache(
        &self,
        transactions_address: &Multiaddr,
    ) -> WorkerCache {
        let workers: BTreeMap<narwhal_crypto::PublicKey, WorkerIndex> = self
            .validators
            .active_validators
            .iter()
            .map(|validator| {
                let name = narwhal_crypto::PublicKey::from_bytes(&validator.metadata.pubkey_bytes)
                    .expect("Can't get narwhal public key");
                let worker_address = Multiaddr::try_from(validator.metadata.worker_address.clone())
                    .expect("Can't get worker address");
                let workers = [(
                    0,
                    narwhal_config::WorkerInfo {
                        name: NetworkPublicKey::from_bytes(&validator.metadata.worker_pubkey_bytes)
                            .expect("Can't get worker key"),
                        transactions: transactions_address.clone(),
                        worker_address,
                    },
                )]
                .into_iter()
                .collect();
                let worker_index = WorkerIndex(workers);

                (name, worker_index)
            })
            .collect();
        WorkerCache {
            workers,
            epoch: self.epoch,
        }
    }
}

// The default implementation for tests
impl Default for HaneulSystemState {
    fn default() -> Self {
        let validator_set = ValidatorSet {
            validator_stake: 1,
            delegation_stake: 1,
            active_validators: vec![],
            pending_validators: vec![],
            pending_removals: vec![],
            next_epoch_validators: vec![],
            staking_pool_mappings: Table::default(),
        };
        HaneulSystemState {
            info: UID::new(HANEUL_SYSTEM_STATE_OBJECT_ID),
            epoch: 0,
            protocol_version: ProtocolVersion::MIN.as_u64(),
            validators: validator_set,
            storage_fund: Balance::new(0),
            parameters: SystemParameters {
                min_validator_stake: 1,
                max_validator_candidate_count: 100,
            },
            reference_gas_price: 1,
            validator_report_records: VecMap { contents: vec![] },
            stake_subsidy: StakeSubsidy {
                epoch_counter: 0,
                balance: Balance::new(0),
                current_epoch_amount: 0,
            },
            safe_mode: false,
            epoch_start_timestamp_ms: 0,
        }
    }
}

pub fn get_haneul_system_state_wrapper<S>(object_store: S) -> Result<HaneulSystemStateWrapper, HaneulError>
where
    S: ObjectStore,
{
    let haneul_system_object = object_store
        .get_object(&HANEUL_SYSTEM_STATE_OBJECT_ID)?
        .ok_or(HaneulError::HaneulSystemStateNotFound)?;
    let move_object = haneul_system_object
        .data
        .try_as_move()
        .ok_or(HaneulError::HaneulSystemStateNotFound)?;
    let result = bcs::from_bytes::<HaneulSystemStateWrapper>(move_object.contents())
        .expect("Haneul System State object deserialization cannot fail");
    Ok(result)
}

pub fn get_haneul_system_state<S>(object_store: S) -> Result<HaneulSystemState, HaneulError>
where
    S: ObjectStore,
{
    let wrapper = get_haneul_system_state_wrapper(object_store)?;
    Ok(wrapper.system_state)
}
