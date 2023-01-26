// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulAddress, HANEUL_TYPE_ARG } from "@haneullabs/haneul.js"
import { config } from "../config";

export const PKG = config.VITE_PKG;

/**
 * Generic Coin type.
 * The T can be anything, including the HANEUL Coin.
 */
export const GENERIC_COIN = '0x2::coin::Coin<T>';

/**
 * Just a Coin.
 */
export type Coin = {
    id: HaneulAddress;
    value: bigint;
};

/**
 * Generic Coin with a `0x2::haneul::HANEUL` type parameter.
 */
export const HANEUL_COIN = `0x2::coin::Coin<${HANEUL_TYPE_ARG}>`

/**
 * Goal enum defined in the Assignment
 */
export enum Goal {
    /** Goal: validator finishes in top third by stake */
    Friend = 0,
    /** Goal: validator finishes in middle third by stake */
    Neutral = 1,
    /** Goal: validator finishes in bottom third by stake */
    Enemy = 2
};

/**
 * Assignment object - one per epoch.
 * Received through updating the Scorecard.
 */
export type Assignment = {
    /** Current assignment */
    validator: HaneulAddress;
    /** Goal: Friend, Neutal or Enemy */
    goal: Goal;
    /** Epoch this assignment is for */
    epoch: bigint;
};

export const ASSIGNMENT = `${PKG}::frenemies::Assignment`;

/**
 * Scorecard object.
 * Follows the Move definition.
 * Received through the `register` transaction call.
 */
export type Scorecard = {
    id: HaneulAddress;
    /** Globally unique name of the player */
    name: string;
    /** Current Assignment */
    assignment: Assignment;
    /** Accumulated score across epochs */
    score: number;
    /** Number of epochs for which the player received a score (even 0) */
    participation: number;
    /** Latest epoch for which assignment was recorded; but a score has not yet been assigned */
    epoch: number;
};

export const SCORECARD = `${PKG}::frenemies::Scorecard`;

/**
 * An event emitted when Scorecard was updated.
 * Contains all necessary information to build a table.
 */
export type ScorecardUpdatedEvent = {
    /** Name of the player */
    scorecard: HaneulAddress;
    /** Player's assignment for the epoch */
    assignment: Assignment;
    /** Player's total score after scoring `assignment` */
    totalScore: number;
    /** Score for the epoch. 0 if the player was not successful */
    epochScore: number;
};

export const SCORECARD_UPDATED = `${PKG}::frenemies::ScorecardUpdateEvent`;

/**
 * Leaderboard object holding information about top X (1000) participants.
 */
export type Leaderboard = {
    id: HaneulAddress;
    /** Top SCORE_MAX (1000) scores; sorted in ASC order */
    topScores: Score[];
    /** Validator set sorted by stake in ascending order for each epoch */
    // redundant field as it gives no information directly
    // prev_epoch_stakes: { id: HaneulAddress, size: number }
    /** Current epoch */
    epoch: bigint;
    /** Epoch where the competition began; */
    startEpoch: bigint;
};

export const LEADERBOARD = `${PKG}::leaderboard::Leaderboard`;

/**
 * A single Score record in the Leaderboard.
 */
export type Score = {
    /** Name of the player (unique) */
    name: string;
    /** The score of the player */
    score: number;
    /** Number of epochs the player has participated in */
    participation: number;
};

/**
 * Defined in the haneul::haneul_system
 */
export type HaneulSystem = {
    /** ID - always the same: 0x5 */
    id: HaneulAddress;
    /** Current system epoch */
    epoch: bigint;
    /** Contains information about current validators */
    validators: ValidatorSet;
};

export const HANEUL_SYSTEM = "0x2::haneul_system::HaneulSystem";

/**
 * Event emitted when epoch is advancing.
 * Can be used to get information about current epoch
 * + track next epoch.
 */
export type SystemEpochInfo = {
    epoch: bigint,
    referenceGasPrice: bigint,
    totalStake: bigint,
    storageFundInflows: bigint,
    storageFundOutflows: bigint,
    storageFundBalance: bigint,
    stakeSubsidyAmount: bigint,
    totalGasFees: bigint,
    totalStakeRewards: bigint,
};

export const SYSTEM_EPOCH_INFO = "0x2::haneul_system::SystemEpochInfo";

export type ValidatorSet = {
    /** Total amount of stake from all active validators (not including delegation), at the beginning of the epoch. */
    totalValidatorStake: bigint,
    /** Total amount of stake from delegation, at the beginning of the epoch. */
    totalDelegationStake: bigint,
    /** The current list of active validators. */
    activeValidators: Validator[],
    /** List of new validator candidates added during the current epoch. They will be processed at the end of the epoch. */
    pendingValidators: Validator[],
    /** Removal requests from the validators. Each element is an index pointing to `active_validators`. */
    pendingRemovals: number[],
    /** The metadata of the validator set for the next epoch. This is kept up-to-dated. */
    nextEpochValidators: ValidatorMetadata[],
    /**
     * Delegation switches requested during the current epoch, processed at epoch boundaries
     * so that all the rewards with be added to the new delegation.
     */
    // pendingDelegationSwitches: 'VecMap<ValidatorPair, table::Table>',
};

export type Validator = {
    /** Summary of the validator. */
    metadata: ValidatorMetadata,
    /** The voting power of this validator, which might be different from its stake amount. */
    votingPower: bigint,
    /** The current active stake amount. This will not change during an epoch. It can only be updated at the end of epoch. */
    stakeAmount: bigint,
    /** Pending stake deposit amount, processed at end of epoch. */
    pendingStake: bigint,
    /** Pending withdraw amount, processed at end of epoch. */
    pendingWithdraw: bigint,
    /** Gas price quote, updated only at end of epoch. */
    gasPrice: bigint,
    /** Staking pool for the stakes delegated to this validator. */
    delegationStakingPool: 'StakingPool',
    /** Commission rate of the validator, in basis point. */
    commissionRate: number,
};

export type ValidatorMetadata = {
    /**
     * The Haneul Address of the validator. This is the sender that created the Validator object
     * and also the address to send validator/coins to during withdraws.
     */
    haneulAddress: HaneulAddress,
    /**
     * The public key bytes corresponding to the private key that the validator
     * holds to sign transactions. For now, this is the same as AuthorityName.
     */
    pubkeyBytes: number[],
    /**
     * The public key bytes corresponding to the private key that the validator
     * uses to establish TLS connections
     */
    networkPubkeyBytes: number[],
    /** The public key bytes correstponding to the Narwhal Worker  */
    workerPubkeyBytes: number[],
    /** This is a proof that the validator has ownership of the private key  */
    proofOfPossession: number[],
    /**A unique human-readable name of this validator.  */
    name: string,
    description: string,
    imageUrl: string,
    projectUrl: string,
    /** The network address of the validator (could also contain extra info such as port, DNS and etc.).  */
    netAddress: number[],
    /** The address of the narwhal primary  */
    consensusAddress: number[],
    /** The address of the narwhal worker  */
    workerAddress: number[],
    /** Total amount of validator stake that would be active in the next epoch.  */
    nextEpochStake: bigint,
    /** Total amount of delegated stake that would be active in the next epoch.  */
    nextEpochDelegation: bigint,
    /** This validator's gas price quote for the next epoch.  */
    nextEpochGasPrice: bigint,
    /** The commission rate of the validator starting the next epoch, in basis point.  */
    nextEpochCommissionRate: bigint,
};

export type StakingPool = {
    /// The haneul address of the validator associated with this pool.
    validatorAddress: HaneulAddress,
    /// The epoch at which this pool started operating. Should be the epoch at which the validator became active.
    starting_epoch: bigint,
    /// The total number of HANEUL tokens in this pool, including the HANEUL in the rewards_pool, as well as in all the principal
    /// in the `Delegation` object, updated at epoch boundaries.
    haneul_balance: bigint,
    /// The epoch delegation rewards will be added here at the end of each epoch.
    rewards_pool: bigint,
    /// The number of delegation pool tokens we have issued so far. This number should equal the sum of
    /// pool token balance in all the `Delegation` objects delegated to this pool. Updated at epoch boundaries.
    delegation_token_supply: bigint,
    /// Delegations requested during the current epoch. We will activate these delegation at the end of current epoch
    /// and distribute staking pool tokens at the end-of-epoch exchange rate after the rewards for the current epoch
    /// have been deposited.
    pending_delegations: any,
    /// Delegation withdraws requested during the current epoch. Similar to new delegation, the withdraws are processed
    /// at epoch boundaries. Rewards are withdrawn and distributed after the rewards for the current epoch have come in.
    pending_withdraws: any,
};

/**
 * Object marking a stake for a Validator.
 */
export type StakedHaneul = {
    id: HaneulAddress,
    /** The validator we are staking with. */
    validatorAddress: HaneulAddress,
    /** The epoch at which the staking pool started operating. */
    poolStartingEpoch: bigint,
    /** The epoch at which the delegation is requested. */
    delegationRequestEpoch: bigint,
    /** The staked HANEUL tokens. */
    staked: bigint,
    /**
     * If the stake comes from a Coin<HANEUL>, this field is None. If it comes from a LockedCoin<HANEUL>,
     * this field will record the original lock expiration epoch, to be used when unstaking.
     */
    haneulTokenLock: { some: bigint } | { none: true }
}

export const STAKED_HANEUL = `0x2::staking_pool::StakedHaneul`;

/**
 * A self-custodial delegation object, serving as evidence that the
 * delegator has delegated to a staking pool.
 *
 * Matches a StakedHaneul object via `stakedHaneulId`.
 */
export type Delegation = {
    id: HaneulAddress,
    /** ID of the StakedHaneul object */
    stakedHaneulId: HaneulAddress,
    /**
     * The pool tokens representing the amount of rewards the delegator
     * can get back when they withdraw from the pool.
     *
     * Move type is: `Balance<DelegationToken>`
     */
    poolTokens: bigint,
    /** Number of HANEUL tokens staked originally */
    principalHaneulAmount: bigint
}

export const DELEGATION = `0x2::staking_pool::Delegation`;
