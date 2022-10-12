// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use bcs::to_bytes;
use haneul_types::balance::{Balance, Supply};
use haneul_types::base_types::HaneulAddress;
use haneul_types::committee::EpochId;
use haneul_types::crypto::{
    get_key_pair, AuthorityPublicKeyBytes, KeypairTraits, NetworkKeyPair, ToFromBytes,
};
use haneul_types::id::UID;
use haneul_types::haneul_system_state::SystemParameters;
use haneul_types::haneul_system_state::{
    StakingPool, HaneulSystemState, Validator, ValidatorMetadata, ValidatorSet,
};
use haneul_types::HANEUL_SYSTEM_STATE_OBJECT_ID;

pub fn test_validatdor_metadata(
    haneul_address: HaneulAddress,
    pubkey_bytes: AuthorityPublicKeyBytes,
    net_address: Vec<u8>,
) -> ValidatorMetadata {
    let network_keypair: NetworkKeyPair = get_key_pair().1;
    ValidatorMetadata {
        haneul_address: haneul_address.into(),
        pubkey_bytes: pubkey_bytes.as_bytes().to_vec(),
        network_pubkey_bytes: network_keypair.public().as_bytes().to_vec(),
        proof_of_possession_bytes: vec![],
        name: to_bytes("zero_commission").unwrap(),
        net_address,
        next_epoch_stake: 1,
        next_epoch_delegation: 1,
        next_epoch_gas_price: 1,
    }
}

pub fn test_staking_pool(haneul_address: HaneulAddress, epoch_starting_haneul_balance: u64) -> StakingPool {
    StakingPool {
        validator_address: haneul_address.into(),
        starting_epoch: 0,
        epoch_starting_haneul_balance,
        haneul_balance: 999,
        rewards_pool: Balance::new(0),
        delegation_token_supply: Supply { value: 0 },
        pending_delegations: vec![],
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
        stake_amount,
        pending_stake: 1,
        pending_withdraw: 1,
        gas_price: 1,
        delegation_staking_pool: test_staking_pool(haneul_address, delegated_amount),
    }
}

pub fn test_haneul_system_state(epoch: EpochId, validators: Vec<Validator>) -> HaneulSystemState {
    let validator_set = ValidatorSet {
        validator_stake: 1,
        delegation_stake: 1,
        quorum_stake_threshold: 1,
        active_validators: validators,
        pending_validators: vec![],
        pending_removals: vec![],
        next_epoch_validators: vec![],
    };
    HaneulSystemState {
        info: UID::new(HANEUL_SYSTEM_STATE_OBJECT_ID),
        epoch,
        validators: validator_set,
        treasury_cap: Supply { value: 0 },
        storage_fund: Balance::new(0),
        parameters: SystemParameters {
            min_validator_stake: 1,
            max_validator_candidate_count: 100,
            storage_gas_price: 1,
        },
        reference_gas_price: 1,
    }
}
