// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::MoveTable;
use super::StakeSubsidy;
use super::StakingPool;
use super::StorageFund;
use super::SystemParameters;
use super::SystemState;
use super::Validator;
use super::ValidatorReportRecord;
use super::ValidatorSet;

impl From<haneul_types::haneul_system_state::HaneulSystemState> for SystemState {
    fn from(value: haneul_types::haneul_system_state::HaneulSystemState) -> Self {
        match value {
            haneul_types::haneul_system_state::HaneulSystemState::V1(v1) => v1.into(),
            haneul_types::haneul_system_state::HaneulSystemState::V2(v2) => v2.into(),

            #[allow(unreachable_patterns)]
            _ => Self::default(),
        }
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v1::HaneulSystemStateInnerV1>
    for SystemState
{
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v1::HaneulSystemStateInnerV1 {
            epoch,
            protocol_version,
            system_state_version,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            validator_report_records,
            stake_subsidy,
            safe_mode,
            safe_mode_storage_rewards,
            safe_mode_computation_rewards,
            safe_mode_storage_rebates,
            safe_mode_non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            extra_fields,
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v1::HaneulSystemStateInnerV1,
    ) -> Self {
        let validator_report_records = validator_report_records
            .contents
            .into_iter()
            .map(|entry| ValidatorReportRecord {
                reported: Some(entry.key.to_string()),
                reporters: entry
                    .value
                    .contents
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            })
            .collect();

        Self {
            version: Some(system_state_version),
            epoch: Some(epoch),
            protocol_version: Some(protocol_version),
            validators: Some(validators.into()),
            storage_fund: Some(storage_fund.into()),
            parameters: Some(parameters.into()),
            reference_gas_price: Some(reference_gas_price),
            validator_report_records,
            stake_subsidy: Some(stake_subsidy.into()),
            safe_mode: Some(safe_mode),
            safe_mode_storage_rewards: Some(safe_mode_storage_rewards.value()),
            safe_mode_computation_rewards: Some(safe_mode_computation_rewards.value()),
            safe_mode_storage_rebates: Some(safe_mode_storage_rebates),
            safe_mode_non_refundable_storage_fee: Some(safe_mode_non_refundable_storage_fee),
            epoch_start_timestamp_ms: Some(epoch_start_timestamp_ms),
            extra_fields: Some(extra_fields.into()),
        }
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v2::HaneulSystemStateInnerV2>
    for SystemState
{
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v2::HaneulSystemStateInnerV2 {
            epoch,
            protocol_version,
            system_state_version,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            validator_report_records,
            stake_subsidy,
            safe_mode,
            safe_mode_storage_rewards,
            safe_mode_computation_rewards,
            safe_mode_storage_rebates,
            safe_mode_non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            extra_fields,
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v2::HaneulSystemStateInnerV2,
    ) -> Self {
        let validator_report_records = validator_report_records
            .contents
            .into_iter()
            .map(|entry| ValidatorReportRecord {
                reported: Some(entry.key.to_string()),
                reporters: entry
                    .value
                    .contents
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            })
            .collect();

        Self {
            version: Some(system_state_version),
            epoch: Some(epoch),
            protocol_version: Some(protocol_version),
            validators: Some(validators.into()),
            storage_fund: Some(storage_fund.into()),
            parameters: Some(parameters.into()),
            reference_gas_price: Some(reference_gas_price),
            validator_report_records,
            stake_subsidy: Some(stake_subsidy.into()),
            safe_mode: Some(safe_mode),
            safe_mode_storage_rewards: Some(safe_mode_storage_rewards.value()),
            safe_mode_computation_rewards: Some(safe_mode_computation_rewards.value()),
            safe_mode_storage_rebates: Some(safe_mode_storage_rebates),
            safe_mode_non_refundable_storage_fee: Some(safe_mode_non_refundable_storage_fee),
            epoch_start_timestamp_ms: Some(epoch_start_timestamp_ms),
            extra_fields: Some(extra_fields.into()),
        }
    }
}

impl From<haneul_types::collection_types::Bag> for MoveTable {
    fn from(
        haneul_types::collection_types::Bag { id, size }: haneul_types::collection_types::Bag,
    ) -> Self {
        Self {
            id: Some(id.id.bytes.to_canonical_string(true)),
            size: Some(size),
        }
    }
}

impl From<haneul_types::collection_types::Table> for MoveTable {
    fn from(
        haneul_types::collection_types::Table { id, size }: haneul_types::collection_types::Table,
    ) -> Self {
        Self {
            id: Some(id.to_canonical_string(true)),
            size: Some(size),
        }
    }
}

impl From<haneul_types::collection_types::TableVec> for MoveTable {
    fn from(value: haneul_types::collection_types::TableVec) -> Self {
        value.contents.into()
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v1::StakeSubsidyV1> for StakeSubsidy {
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v1::StakeSubsidyV1 {
            balance,
            distribution_counter,
            current_distribution_amount,
            stake_subsidy_period_length,
            stake_subsidy_decrease_rate,
            extra_fields,
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v1::StakeSubsidyV1,
    ) -> Self {
        Self {
            balance: Some(balance.value()),
            distribution_counter: Some(distribution_counter),
            current_distribution_amount: Some(current_distribution_amount),
            stake_subsidy_period_length: Some(stake_subsidy_period_length),
            stake_subsidy_decrease_rate: Some(stake_subsidy_decrease_rate.into()),
            extra_fields: Some(extra_fields.into()),
        }
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v1::SystemParametersV1>
    for SystemParameters
{
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v1::SystemParametersV1 {
            epoch_duration_ms,
            stake_subsidy_start_epoch,
            max_validator_count,
            min_validator_joining_stake,
            validator_low_stake_threshold,
            validator_very_low_stake_threshold,
            validator_low_stake_grace_period,
            extra_fields,
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v1::SystemParametersV1,
    ) -> Self {
        Self {
            epoch_duration_ms: Some(epoch_duration_ms),
            stake_subsidy_start_epoch: Some(stake_subsidy_start_epoch),
            min_validator_count: None,
            max_validator_count: Some(max_validator_count),
            min_validator_joining_stake: Some(min_validator_joining_stake),
            validator_low_stake_threshold: Some(validator_low_stake_threshold),
            validator_very_low_stake_threshold: Some(validator_very_low_stake_threshold),
            validator_low_stake_grace_period: Some(validator_low_stake_grace_period),
            extra_fields: Some(extra_fields.into()),
        }
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v2::SystemParametersV2>
    for SystemParameters
{
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v2::SystemParametersV2 {
            epoch_duration_ms,
            stake_subsidy_start_epoch,
            min_validator_count,
            max_validator_count,
            min_validator_joining_stake,
            validator_low_stake_threshold,
            validator_very_low_stake_threshold,
            validator_low_stake_grace_period,
            extra_fields,
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v2::SystemParametersV2,
    ) -> Self {
        Self {
            epoch_duration_ms: Some(epoch_duration_ms),
            stake_subsidy_start_epoch: Some(stake_subsidy_start_epoch),
            min_validator_count: Some(min_validator_count),
            max_validator_count: Some(max_validator_count),
            min_validator_joining_stake: Some(min_validator_joining_stake),
            validator_low_stake_threshold: Some(validator_low_stake_threshold),
            validator_very_low_stake_threshold: Some(validator_very_low_stake_threshold),
            validator_low_stake_grace_period: Some(validator_low_stake_grace_period),
            extra_fields: Some(extra_fields.into()),
        }
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v1::StorageFundV1> for StorageFund {
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v1::StorageFundV1 {
            total_object_storage_rebates,
            non_refundable_balance,
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v1::StorageFundV1,
    ) -> Self {
        Self {
            total_object_storage_rebates: Some(total_object_storage_rebates.value()),
            non_refundable_balance: Some(non_refundable_balance.value()),
        }
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v1::ValidatorSetV1> for ValidatorSet {
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v1::ValidatorSetV1 {
            total_stake,
            active_validators,
            pending_active_validators,
            pending_removals,
            staking_pool_mappings,
            inactive_validators,
            validator_candidates,
            at_risk_validators,
            extra_fields,
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v1::ValidatorSetV1,
    ) -> Self {
        let at_risk_validators = at_risk_validators
            .contents
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value))
            .collect();
        Self {
            total_stake: Some(total_stake),
            active_validators: active_validators.into_iter().map(Into::into).collect(),
            pending_active_validators: Some(pending_active_validators.into()),
            pending_removals,
            staking_pool_mappings: Some(staking_pool_mappings.into()),
            inactive_validators: Some(inactive_validators.into()),
            validator_candidates: Some(validator_candidates.into()),
            at_risk_validators,
            extra_fields: Some(extra_fields.into()),
        }
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v1::StakingPoolV1> for StakingPool {
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v1::StakingPoolV1 {
            id,
            activation_epoch,
            deactivation_epoch,
            haneul_balance,
            rewards_pool,
            pool_token_balance,
            exchange_rates,
            pending_stake,
            pending_total_haneul_withdraw,
            pending_pool_token_withdraw,
            extra_fields,
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v1::StakingPoolV1,
    ) -> Self {
        Self {
            id: Some(id.to_canonical_string(true)),
            activation_epoch,
            deactivation_epoch,
            haneul_balance: Some(haneul_balance),
            rewards_pool: Some(rewards_pool.value()),
            pool_token_balance: Some(pool_token_balance),
            exchange_rates: Some(exchange_rates.into()),
            pending_stake: Some(pending_stake),
            pending_total_haneul_withdraw: Some(pending_total_haneul_withdraw),
            pending_pool_token_withdraw: Some(pending_pool_token_withdraw),
            extra_fields: Some(extra_fields.into()),
        }
    }
}

impl From<haneul_types::haneul_system_state::haneul_system_state_inner_v1::ValidatorV1> for Validator {
    fn from(
        haneul_types::haneul_system_state::haneul_system_state_inner_v1::ValidatorV1 {
            metadata:
                haneul_types::haneul_system_state::haneul_system_state_inner_v1::ValidatorMetadataV1 {
                    haneul_address,
                    protocol_pubkey_bytes,
                    network_pubkey_bytes,
                    worker_pubkey_bytes,
                    proof_of_possession_bytes,
                    name,
                    description,
                    image_url,
                    project_url,
                    net_address,
                    p2p_address,
                    primary_address,
                    worker_address,
                    next_epoch_protocol_pubkey_bytes,
                    next_epoch_proof_of_possession,
                    next_epoch_network_pubkey_bytes,
                    next_epoch_worker_pubkey_bytes,
                    next_epoch_net_address,
                    next_epoch_p2p_address,
                    next_epoch_primary_address,
                    next_epoch_worker_address,
                    extra_fields: metadata_extra_fields,
                },
            voting_power,
            operation_cap_id,
            gas_price,
            staking_pool,
            commission_rate,
            next_epoch_stake,
            next_epoch_gas_price,
            next_epoch_commission_rate,
            extra_fields,
            ..
        }: haneul_types::haneul_system_state::haneul_system_state_inner_v1::ValidatorV1,
    ) -> Self {
        Self {
            name: Some(name),
            address: Some(haneul_address.to_string()),
            description: Some(description),
            image_url: Some(image_url),
            project_url: Some(project_url),
            protocol_public_key: Some(protocol_pubkey_bytes.into()),
            proof_of_possession: Some(proof_of_possession_bytes.into()),
            network_public_key: Some(network_pubkey_bytes.into()),
            worker_public_key: Some(worker_pubkey_bytes.into()),
            network_address: Some(net_address),
            p2p_address: Some(p2p_address),
            primary_address: Some(primary_address),
            worker_address: Some(worker_address),
            next_epoch_protocol_public_key: next_epoch_protocol_pubkey_bytes.map(Into::into),
            next_epoch_proof_of_possession: next_epoch_proof_of_possession.map(Into::into),
            next_epoch_network_public_key: next_epoch_network_pubkey_bytes.map(Into::into),
            next_epoch_worker_public_key: next_epoch_worker_pubkey_bytes.map(Into::into),
            next_epoch_network_address: next_epoch_net_address,
            next_epoch_p2p_address,
            next_epoch_primary_address,
            next_epoch_worker_address,
            metadata_extra_fields: Some(metadata_extra_fields.into()),
            voting_power: Some(voting_power),
            operation_cap_id: Some(operation_cap_id.bytes.to_canonical_string(true)),
            gas_price: Some(gas_price),
            staking_pool: Some(staking_pool.into()),
            commission_rate: Some(commission_rate),
            next_epoch_stake: Some(next_epoch_stake),
            next_epoch_gas_price: Some(next_epoch_gas_price),
            next_epoch_commission_rate: Some(next_epoch_commission_rate),
            extra_fields: Some(extra_fields.into()),
        }
    }
}
