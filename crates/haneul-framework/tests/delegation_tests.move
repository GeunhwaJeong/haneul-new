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
    use std::vector;

    use haneul::governance_test_utils::{
        Self,
        add_validator,
        add_validator_candidate,
        advance_epoch,
        advance_epoch_safe_mode,
        advance_epoch_with_reward_amounts,
        create_validator_for_testing,
        create_haneul_system_state_for_testing,
        delegate_to,
        remove_validator,
        remove_validator_candidate,
        total_haneul_balance,
        undelegate,
    };

    const VALIDATOR_ADDR_1: address = @0x1;
    const VALIDATOR_ADDR_2: address = @0x2;

    const DELEGATOR_ADDR_1: address = @0x42;
    const DELEGATOR_ADDR_2: address = @0x43;
    const DELEGATOR_ADDR_3: address = @0x44;

    const NEW_VALIDATOR_ADDR: address = @0x1a4623343cd42be47d67314fce0ad042f3c82685544bc91d8c11d24e74ba7357;
    const NEW_VALIDATOR_PUBKEY: vector<u8> = x"99f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a10";
    const NEW_VALIDATOR_POP: vector<u8> = x"8080980b89554e7f03b625ba4104d05d19b523a737e2d09a69d4498a1bcac154fcb29f6334b7e8b99b8f3aa95153232d";

    #[test]
    fun test_split_join_staked_haneul() {
        let scenario_val = test_scenario::begin(DELEGATOR_ADDR_1);
        let scenario = &mut scenario_val;
        // All this is just to generate a dummy StakedHaneul object to split and join later
        set_up_haneul_system_state(scenario);
        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 60, scenario);

        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let staked_haneul = test_scenario::take_from_sender<StakedHaneul>(scenario);
            let ctx = test_scenario::ctx(scenario);
            staking_pool::split_staked_haneul(&mut staked_haneul, 20, ctx);
            test_scenario::return_to_sender(scenario, staked_haneul);
        };

        // Verify the correctness of the split and send the join txn
        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let staked_haneul_ids = test_scenario::ids_for_sender<StakedHaneul>(scenario);
            assert!(vector::length(&staked_haneul_ids) == 2, 101); // staked haneul split to 2 coins

            let part1 = test_scenario::take_from_sender_by_id<StakedHaneul>(scenario, *vector::borrow(&staked_haneul_ids, 0));
            let part2 = test_scenario::take_from_sender_by_id<StakedHaneul>(scenario, *vector::borrow(&staked_haneul_ids, 1));

            let amount1 = staking_pool::staked_haneul_amount(&part1);
            let amount2 = staking_pool::staked_haneul_amount(&part2);
            assert!(amount1 == 20 || amount1 == 40, 102);
            assert!(amount2 == 20 || amount2 == 40, 103);
            assert!(amount1 + amount2 == 60, 104);

            staking_pool::join_staked_haneul(&mut part1, part2);
            assert!(staking_pool::staked_haneul_amount(&part1) == 60, 105);
            test_scenario::return_to_sender(scenario, part1);
        };
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = staking_pool::EIncompatibleStakedHaneul)]
    fun test_join_different_epochs() {
        let scenario_val = test_scenario::begin(DELEGATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);
        // Create two instances of staked haneul w/ different epoch activations
        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 60, scenario);
        governance_test_utils::advance_epoch(scenario);
        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 60, scenario);

        // Verify that these cannot be merged
        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let staked_haneul_ids = test_scenario::ids_for_sender<StakedHaneul>(scenario);
            let part1 = test_scenario::take_from_sender_by_id<StakedHaneul>(scenario, *vector::borrow(&staked_haneul_ids, 0));
            let part2 = test_scenario::take_from_sender_by_id<StakedHaneul>(scenario, *vector::borrow(&staked_haneul_ids, 1));

            staking_pool::join_staked_haneul(&mut part1, part2);

            test_scenario::return_to_sender(scenario, part1);
        };
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = staking_pool::EIncompatibleStakedHaneul)]
    fun test_join_different_locked_coins() {
        let scenario_val = test_scenario::begin(DELEGATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);
        // Create staked haneul w/ locked coin and regular staked haneul
        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 60, scenario);
        governance_test_utils::delegate_locked_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 60, 2, scenario);

        // Verify that these cannot be merged
        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let staked_haneul_ids = test_scenario::ids_for_sender<StakedHaneul>(scenario);
            let part1 = test_scenario::take_from_sender_by_id<StakedHaneul>(scenario, *vector::borrow(&staked_haneul_ids, 0));
            let part2 = test_scenario::take_from_sender_by_id<StakedHaneul>(scenario, *vector::borrow(&staked_haneul_ids, 1));

            staking_pool::join_staked_haneul(&mut part1, part2);

            test_scenario::return_to_sender(scenario, part1);
        };
        test_scenario::end(scenario_val);
    }

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
    fun test_earns_rewards_at_last_epoch() {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);

        advance_epoch(scenario);

        remove_validator(VALIDATOR_ADDR_1, scenario);

        // Add some rewards after the validator requests to leave. Since the validator is still active
        // this epoch, they should get the rewards from this epoch.
        advance_epoch_with_reward_amounts(0, 40, scenario);

        // Each 100 GEUNHWA of stake gets 10 GEUNHWA and validators shares the 10 GEUNHWA from the storage fund
        // so validator gets another 5 GEUNHWA.
        let reward_amt = 10;
        let validator_reward_amt = 5;

        // Make sure delegation withdrawal happens
        test_scenario::next_tx(scenario, DELEGATOR_ADDR_1);
        {
            let system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
            let system_state_mut_ref = &mut system_state;

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
        undelegate(VALIDATOR_ADDR_1, 0, scenario);

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

    #[test]
    fun test_add_preactive_remove_preactive() {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        governance_test_utils::add_validator_candidate(NEW_VALIDATOR_ADDR, NEW_VALIDATOR_PUBKEY, NEW_VALIDATOR_POP, scenario);

        // Delegate 100 GEUNHWA to the preactive validator
        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, NEW_VALIDATOR_ADDR, 100, scenario);

        // Advance epoch twice with some rewards
        advance_epoch_with_reward_amounts(0, 400, scenario);
        advance_epoch_with_reward_amounts(0, 900, scenario);

        // Undelegate from the preactive validator. There should be no rewards earned.
        governance_test_utils::undelegate(DELEGATOR_ADDR_1, 0, scenario);
        assert_eq(total_haneul_balance(DELEGATOR_ADDR_1, scenario), 100);

        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = validator_set::ENotAValidator)]
    fun test_add_preactive_remove_pending_failure() {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        governance_test_utils::add_validator_candidate(NEW_VALIDATOR_ADDR, NEW_VALIDATOR_PUBKEY, NEW_VALIDATOR_POP, scenario);

        governance_test_utils::add_validator(NEW_VALIDATOR_ADDR, scenario);

        // Delegate 100 GEUNHWA to the pending validator. This should fail because pending active validators don't accept
        // new delegations or withdraws.
        governance_test_utils::delegate_to(DELEGATOR_ADDR_1, NEW_VALIDATOR_ADDR, 100, scenario);

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_add_preactive_remove_active() {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        add_validator_candidate(NEW_VALIDATOR_ADDR, NEW_VALIDATOR_PUBKEY, NEW_VALIDATOR_POP, scenario);

        // Delegate 100 GEUNHWA to the preactive validator
        delegate_to(DELEGATOR_ADDR_1, NEW_VALIDATOR_ADDR, 100, scenario);
        advance_epoch_with_reward_amounts(0, 70, scenario);
        delegate_to(DELEGATOR_ADDR_2, NEW_VALIDATOR_ADDR, 300, scenario);
        delegate_to(DELEGATOR_ADDR_3, NEW_VALIDATOR_ADDR, 100, scenario);

        // Now the preactive becomes active
        add_validator(NEW_VALIDATOR_ADDR, scenario);
        advance_epoch(scenario);

        advance_epoch_with_reward_amounts(0, 80, scenario);

        // delegator 1 and 3 undelegate from the validator and earns 9 GEUNHWA.
        // Although they delegate in different epochs, they earn the same rewards as long as they undelegate
        // in the same epoch because the validator was preactive when they delegated.
        undelegate(DELEGATOR_ADDR_1, 0, scenario);
        assert_eq(total_haneul_balance(DELEGATOR_ADDR_1, scenario), 109);
        undelegate(DELEGATOR_ADDR_3, 0, scenario);
        assert_eq(total_haneul_balance(DELEGATOR_ADDR_3, scenario), 109);

        advance_epoch_with_reward_amounts(0, 100, scenario);
        undelegate(DELEGATOR_ADDR_2, 0, scenario);
        // delegator 2 earns about 27 GEUNHWA from the previous epoch and 5/8 of the 100 GEUNHWA.
        assert_eq(total_haneul_balance(DELEGATOR_ADDR_2, scenario), 300 + 27 + 59);

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_add_preactive_remove_post_active() {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        add_validator_candidate(NEW_VALIDATOR_ADDR, NEW_VALIDATOR_PUBKEY, NEW_VALIDATOR_POP, scenario);

        // Delegate 100 GEUNHWA to the preactive validator
        delegate_to(DELEGATOR_ADDR_1, NEW_VALIDATOR_ADDR, 100, scenario);

        // Now the preactive becomes active
        add_validator(NEW_VALIDATOR_ADDR, scenario);
        advance_epoch(scenario);

        advance_epoch_with_reward_amounts(0, 80, scenario); // delegator 1 earns 20 GEUNHWA here.

        // And now the validator leaves the validator set.
        remove_validator(NEW_VALIDATOR_ADDR, scenario);

        advance_epoch(scenario);

        undelegate(DELEGATOR_ADDR_1, 0, scenario);
        assert_eq(total_haneul_balance(DELEGATOR_ADDR_1, scenario), 100 + 20);

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_add_preactive_candidate_drop_out() {
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);

        add_validator_candidate(NEW_VALIDATOR_ADDR, NEW_VALIDATOR_PUBKEY, NEW_VALIDATOR_POP, scenario);

        // Delegate 100 GEUNHWA to the preactive validator
        delegate_to(DELEGATOR_ADDR_1, NEW_VALIDATOR_ADDR, 100, scenario);

        // Advance epoch and give out some rewards. The candidate should get nothing, of course.
        advance_epoch_with_reward_amounts(0, 800, scenario);

        // Now the candidate leaves.
        remove_validator_candidate(NEW_VALIDATOR_ADDR, scenario);

        // Advance epoch a few times.
        advance_epoch(scenario);
        advance_epoch(scenario);
        advance_epoch(scenario);

        // Undelegate now and the delegator should get no rewards.
        undelegate(DELEGATOR_ADDR_1, 0, scenario);
        assert_eq(total_haneul_balance(DELEGATOR_ADDR_1, scenario), 100);

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_delegation_during_safe_mode() {
        // test that delegation and undelegation can work during safe mode too.
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;
        set_up_haneul_system_state(scenario);
        delegate_to(DELEGATOR_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);
        advance_epoch(scenario);
        // The first delegation gets 10 GEUNHWA here.
        advance_epoch_with_reward_amounts(0, 40, scenario);
        advance_epoch_safe_mode(scenario);

        delegate_to(DELEGATOR_ADDR_2, VALIDATOR_ADDR_1, 100, scenario);

        advance_epoch_safe_mode(scenario);
        advance_epoch(scenario);
        // The first delegation gets 10 GEUNHWA and the second one gets 8 here.
        advance_epoch_with_reward_amounts(0, 50, scenario);
        advance_epoch_safe_mode(scenario);

        undelegate(DELEGATOR_ADDR_1, 0, scenario);
        // 100 principal + 20 rewards
        assert_eq(total_haneul_balance(DELEGATOR_ADDR_1, scenario), 120);

        undelegate(DELEGATOR_ADDR_2, 0, scenario);
        // 100 principal + 8 rewards
        assert_eq(total_haneul_balance(DELEGATOR_ADDR_2, scenario), 108);

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
