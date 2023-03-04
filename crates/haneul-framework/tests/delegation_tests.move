// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module haneul::delegation_tests {
    use haneul::coin;
    use haneul::test_scenario::{Self, Scenario};
    use haneul::haneul_system::{Self, HaneulSystemState};
    use haneul::staking_pool::{Self, StakedHaneul};
    use haneul::test_utils::assert_eq;
    use haneul::validator_set;


    use haneul::governance_test_utils::{
        Self,
        create_validator_for_testing,
        create_haneul_system_state_for_testing,
        total_haneul_balance,
        undelegate,
    };

    const VALIDATOR_ADDR_1: address = @0x1;
    const VALIDATOR_ADDR_2: address = @0x2;

    const DELEGATOR_ADDR_1: address = @0x42;
    const DELEGATOR_ADDR_2: address = @0x43;

    #[test]
    fun test_add_remove_delegation_flow() {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
            let system_state_mut_ref = &mut system_state;

            let ctx = test_scenario::ctx(scenario);

            // Create a delegation to VALIDATOR_ADDR_1.
            haneul_system::request_add_delegation(
                system_state_mut_ref, coin::mint_for_testing(60, ctx), VALIDATOR_ADDR_1, ctx);

            assert!(haneul_system::validator_stake_amount(system_state_mut_ref, VALIDATOR_ADDR_1) == 100, 101);
            assert!(haneul_system::validator_stake_amount(system_state_mut_ref, VALIDATOR_ADDR_2) == 100, 102);

            test_scenario::return_shared(system_state);
        };

        governance_test_utils::advance_epoch(scenario);

        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {

            let staked_haneul = test_scenario::take_from_sender<StakedHaneul>(scenario);
            assert!(staking_pool::staked_haneul_amount(&staked_haneul) == 60, 105);


            let system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
            let system_state_mut_ref = &mut system_state;

            assert!(haneul_system::validator_stake_amount(system_state_mut_ref, VALIDATOR_ADDR_1) == 160, 103);
            assert!(haneul_system::validator_stake_amount(system_state_mut_ref, VALIDATOR_ADDR_2) == 100, 104);

            let ctx = test_scenario::ctx(scenario);

            // Undelegate from VALIDATOR_ADDR_1
            haneul_system::request_withdraw_delegation(system_state_mut_ref, staked_haneul, ctx);

            assert!(haneul_system::validator_stake_amount(system_state_mut_ref, VALIDATOR_ADDR_1) == 160, 107);
            test_scenario::return_shared(system_state);
        };

        governance_test_utils::advance_epoch(scenario);

        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
            assert!(haneul_system::validator_stake_amount(&mut system_state, VALIDATOR_ADDR_1) == 100, 107);
            test_scenario::return_shared(system_state);
        };
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_remove_delegation_post_active_flow_no_rewards() {
        test_remove_delegation_post_active_flow(false)
    }

    #[test]
    fun test_remove_delegation_post_active_flow_with_rewards() {
        test_remove_delegation_post_active_flow(true)
    }

    fun test_remove_delegation_post_active_flow(should_distribute_rewards: bool) {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);

        governance_test_utils::advance_epoch(scenario);

        governance_test_utils::assert_validator_total_stake_amounts(
            vector[VALIDATOR_ADDR_1, VALIDATOR_ADDR_2],
            vector[200, 100],
            scenario
        );

        if (should_distribute_rewards) {
            // advance the epoch and set rewards at 10 HANEUL for each 100 HANEUL staked.
            governance_test_utils::advance_epoch_with_reward_amounts(0, 40, scenario);
        } else {
            governance_test_utils::advance_epoch(scenario);
        };

        governance_test_utils::remove_validator(VALIDATOR_ADDR_1, scenario);

        governance_test_utils::advance_epoch(scenario);

        // 110 = stake + rewards for that stake
        // 5 = validator rewards
        let reward_amt = if (should_distribute_rewards) 10 else 0;
        let validator_reward_amt = if (should_distribute_rewards) 5 else 0;

        // Make sure delegation withdrawal happens
        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
            let system_state_mut_ref = &mut system_state;

            assert!(!validator_set::is_active_validator_by_haneul_address(
                        haneul_system::validators(system_state_mut_ref),
                        VALIDATOR_ADDR_1
                    ), 0);

            let staked_haneul = test_scenario::take_from_sender<StakedHaneul>(scenario);
            assert_eq(staking_pool::staked_haneul_amount(&staked_haneul), 100);

            // Undelegate from VALIDATOR_ADDR_1
            assert_eq(total_haneul_balance(DELEGATOR_ADDR_1, scenario), 0);
            let ctx = test_scenario::ctx(scenario);
            haneul_system::request_withdraw_delegation(system_state_mut_ref, staked_haneul, ctx);

            // Make sure they have all of their stake.
            assert_eq(total_haneul_balance(DELEGATOR_ADDR_1, scenario), 100 + reward_amt);

            test_scenario::return_shared(system_state);
        };

        // Validator undelegates now.
        assert_eq(total_haneul_balance(VALIDATOR_ADDR_1, scenario), 0);
        undelegate(VALIDATOR_ADDR_1, 0, scenario);
        if (should_distribute_rewards) undelegate(VALIDATOR_ADDR_1, 0, scenario);

        // Make sure have all of their stake. NB there is no epoch change. This is immediate.
        assert_eq(total_haneul_balance(VALIDATOR_ADDR_1, scenario), 100 + reward_amt + validator_reward_amt);

        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = validator_set::ENotAValidator)]
    fun test_add_delegation_post_active_flow() {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);

        governance_test_utils::advance_epoch(scenario);

        governance_test_utils::remove_validator(VALIDATOR_ADDR_1, scenario);

        governance_test_utils::advance_epoch(scenario);

        // Make sure the validator is no longer active.
        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
            let system_state_mut_ref = &mut system_state;

            assert!(!validator_set::is_active_validator_by_haneul_address(
                        haneul_system::validators(system_state_mut_ref),
                        VALIDATOR_ADDR_1
                    ), 0);

            test_scenario::return_shared(system_state);
        };

        // Now try and delegate to the old validator/staking pool. This should fail!
        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 60, scenario);

        test_scenario::end(scenario_val);
    }

    fun set_up_haneul_system_state(scenario: &mut Scenario) {
        let ctx = test_scenario::ctx(scenario);

        let validators = vector[
            create_validator_for_testing(VALIDATOR_ADDR_1, 100, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_2, 100, ctx)
        ];
        create_haneul_system_state_for_testing(validators, 300, 100, ctx);
    }
}
