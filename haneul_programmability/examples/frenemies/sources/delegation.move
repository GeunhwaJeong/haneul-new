// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Module that helps handling multiple delegation / withdrawal
/// requests. Helps doing programmable batches with the HaneulSystem
/// object avoiding the transaction batch limitation (can't use the
/// same object twice).
module frenemies::delegation {
    use haneul::haneul_system::{Self, HaneulSystemState};
    use haneul::staking_pool::{Delegation, StakedHaneul};
    use haneul::tx_context::TxContext;
    use std::vector;

    /// For when there's a mismatch between Delegation and StakedHaneul vectors.
    const EVecArgumentLengthMismatch: u64 = 0;

    /// Switch multiple delegations into a single Validator account.
    /// Vector of Delegations must match the vector of StakedHaneul objects.
    ///
    /// Aborts if there's a vector length mismatch.
    public entry fun switch_into_one(
        self: &mut HaneulSystemState,
        delegations: vector<Delegation>,
        staked_haneuls: vector<StakedHaneul>, // don't mind if I do
        new_validator_address: address,
        ctx: &mut TxContext
    ) {
        let len = vector::length(&delegations);
        assert!(len == vector::length(&staked_haneuls), EVecArgumentLengthMismatch);

        while (len > 0) {
            let (staked_haneul, delegation) = (
                vector::pop_back(&mut staked_haneuls),
                vector::pop_back(&mut delegations)
            );

            haneul_system::request_switch_delegation(self, delegation, staked_haneul, new_validator_address, ctx);
            len = len - 1;
        };

        vector::destroy_empty(delegations);
        vector::destroy_empty(staked_haneuls);
    }

    /// Request multiple withdraws at once.
    /// Vector of Delegations must match the vector of StakedHaneul objects.
    ///
    /// Aborts if there's a vector length mismatch.
    public entry fun request_withdraw_mul(
        self: &mut HaneulSystemState,
        delegations: vector<Delegation>,
        staked_haneuls: vector<StakedHaneul>,
        ctx: &mut TxContext
    ) {
        let len = vector::length(&delegations);
        assert!(len == vector::length(&staked_haneuls), EVecArgumentLengthMismatch);

        while (len > 0) {
            let (staked_haneul, delegation) = (
                vector::pop_back(&mut staked_haneuls),
                vector::pop_back(&mut delegations)
            );

            haneul_system::request_withdraw_delegation(self, delegation, staked_haneul, ctx);
            len = len - 1;
        };

        vector::destroy_empty(delegations);
        vector::destroy_empty(staked_haneuls);
    }
}
