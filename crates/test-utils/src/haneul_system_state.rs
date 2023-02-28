// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_types::balance::Balance;
use haneul_types::base_types::{ObjectID, HaneulAddress};
use haneul_types::collection_types::VecMap;
use haneul_types::committee::{EpochId, ProtocolVersion};
use haneul_types::crypto::{
    get_key_pair, AuthorityPublicKeyBytes, KeypairTraits, NetworkKeyPair, ToFromBytes,
};
use haneul_types::id::UID;
use haneul_types::haneul_system_state::SystemParameters;
use haneul_types::haneul_system_state::{
    StakeSubsidy, StakingPool, HaneulSystemState, Table, TableVec, Validator, ValidatorMetadata,
    ValidatorSet,
};
use haneul_types::HANEUL_SYSTEM_STATE_OBJECT_ID;

pub fn test_validatdor_metadata(
    haneul_address: HaneulAddress,
    pubkey_bytes: AuthorityPublicKeyBytes,
    net_address: Vec<u8>,
) -> ValidatorMetadata {
    let network_keypair: NetworkKeyPair = get_key_pair().1;
    ValidatorMetadata {
        haneul_address,
        pubkey_bytes: pubkey_bytes.as_bytes().to_vec(),
        network_pubkey_bytes: network_keypair.public().as_bytes().to_vec(),
        worker_pubkey_bytes: vec![],
        proof_of_possession_bytes: vec![],
        name: "zero_commission".to_string(),
        description: "".to_string(),
        image_url: "".to_string(),
        project_url: "".to_string(),
        net_address,
        p2p_address: vec![],
        consensus_address: vec![],
        worker_address: vec![],
    }
}

pub fn test_staking_pool(haneul_balance: u64) -> StakingPool {
    StakingPool {
        id: ObjectID::from(HaneulAddress::ZERO),
        starting_epoch: 0,
        haneul_balance,
        rewards_pool: Balance::new(0),
        pool_token_balance: 0,
        exchange_rates: Table::default(),
        pending_delegation: 0,
        pending_withdraws: TableVec::default(),
    }
}

pub fn test_validator(
    pubkey_bytes: AuthorityPublicKeyBytes,
    net_address: Vec<u8>,
    stake_amount: u64,
    delegated_amount: u64,
) -> Validator {
    let haneul_address = HaneulAddress::from(&pubkey_bytes);
    Validator {
        metadata: test_validatdor_metadata(haneul_address, pubkey_bytes, net_address),
        voting_power: stake_amount,
        stake_amount,
        pending_stake: 1,
        pending_withdraw: 1,
        gas_price: 1,
        delegation_staking_pool: test_staking_pool(delegated_amount),
        commission_rate: 0,
        next_epoch_stake: 1,
        next_epoch_delegation: 1,
        next_epoch_gas_price: 1,
        next_epoch_commission_rate: 0,
    }
}

pub fn test_haneul_system_state(epoch: EpochId, validators: Vec<Validator>) -> HaneulSystemState {
    let validator_set = ValidatorSet {
        validator_stake: 1,
        delegation_stake: 1,
        active_validators: validators,
        pending_validators: vec![],
        pending_removals: vec![],
        next_epoch_validators: vec![],
        staking_pool_mappings: Table::default(),
    };
    HaneulSystemState {
        info: UID::new(HANEUL_SYSTEM_STATE_OBJECT_ID),
        epoch,
        protocol_version: ProtocolVersion::MAX.as_u64(),
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
