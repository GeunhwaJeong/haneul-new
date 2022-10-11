// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module haneul::governance_test_utils {
    use haneul::balance;
    use haneul::haneul::HANEUL;
    use haneul::tx_context::{Self, TxContext};
    use haneul::validator::{Self, Validator};
    use haneul::haneul_system::{Self, HaneulSystemState};
    use haneul::test_scenario::{Self, Scenario};
    use std::option;

    public fun create_validator_for_testing(
        addr: address, init_stake_amount: u64, ctx: &mut TxContext
    ): Validator {
        validator::new_for_testing(
            addr,
            x"FF",
            x"FF",
            x"FF",
            b"ValidatorName",
            x"FFFF",
            balance::create_for_testing<HANEUL>(init_stake_amount),
            option::none(),
            1,
            ctx
        )
    }

    public fun create_haneul_system_state_for_testing(
        validators: vector<Validator>, haneul_supply_amount: u64, storage_fund_amount: u64
    ) {
        haneul_system::create(
            validators,
            balance::create_supply_for_testing(haneul_supply_amount), // haneul_supply
            balance::create_for_testing<HANEUL>(storage_fund_amount), // storage_fund
            1024, // max_validator_candidate_count
            0, // min_validator_stake
            1, //storage_gas_price
        )
    }

    public fun advance_epoch(state: &mut HaneulSystemState, scenario: &mut Scenario) {
        let sender = test_scenario::sender(scenario);
        test_scenario::next_epoch(scenario, sender);
        let new_epoch = tx_context::epoch(test_scenario::ctx(scenario));
        haneul_system::advance_epoch(state, new_epoch, 0, 0, &mut tx_context::dummy());
    }
}
