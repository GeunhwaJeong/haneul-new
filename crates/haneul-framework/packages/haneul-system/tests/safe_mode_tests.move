// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module haneul::safe_mode_tests {
    use haneul::test_scenario;
    use haneul::balance;
    use haneul::test_utils;

    use haneul_system::governance_test_utils::{
        create_validator_for_testing, create_haneul_system_state_for_testing,
        advance_epoch_safe_mode_with_reward_amounts, advance_epoch_with_reward_amounts_return_rebate,
    };
    use haneul_system::haneul_system;
    use haneul_system::haneul_system::HaneulSystemState;

    const GEUNHWA_PER_HANEUL: u64 = 1_000_000_000;

    #[test]
    fun test_safe_mode_gas_accumulation() {
        let scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        {
            // First, set up the system with 4 validators.
            let ctx = test_scenario::ctx(scenario);
            create_haneul_system_state_for_testing(
                vector[
                    create_validator_for_testing(@0x1, 1, ctx),
                    create_validator_for_testing(@0x2, 1, ctx),
                    create_validator_for_testing(@0x3, 1, ctx),
                    create_validator_for_testing(@0x4, 1, ctx),
                ],
                1000,
                1000,
                ctx,
            );
        };
        {
            test_scenario::next_tx(scenario, @0x1);
            let system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
            test_utils::assert_eq(haneul_system::validator_stake_amount(&mut system_state, @0x1), 1 * GEUNHWA_PER_HANEUL);
            test_utils::assert_eq(haneul_system::get_storage_fund_total_balance(&mut system_state), 1000 * GEUNHWA_PER_HANEUL);
            test_scenario::return_shared(system_state);
        };

        advance_epoch_safe_mode_with_reward_amounts(
            48,
            20,
            30,
            2,
            scenario,
        );
        let rebates = advance_epoch_with_reward_amounts_return_rebate(32, 40, 30, 6, scenario);

        // 30 from safe mode epoch and 30 from current epoch.
        test_utils::assert_eq(balance::value(&rebates), 60);
        test_utils::destroy(rebates);

        let system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
        // For each validator, total computation reward is 20 / 4  + 40 / 4 = 15
        // However due to integer division, each validator is getting 1 less.
        test_utils::assert_eq(haneul_system::validator_stake_amount(&mut system_state, @0x1), 14 + 1 * GEUNHWA_PER_HANEUL);
        test_utils::assert_eq(haneul_system::validator_stake_amount(&mut system_state, @0x2), 14 + 1 * GEUNHWA_PER_HANEUL);
        test_utils::assert_eq(haneul_system::validator_stake_amount(&mut system_state, @0x3), 14 + 1 * GEUNHWA_PER_HANEUL);
        test_utils::assert_eq(haneul_system::validator_stake_amount(&mut system_state, @0x4), 14 + 1 * GEUNHWA_PER_HANEUL);

        // Storage fund delta is 48 + 32 - 30 - 30 = 20. 4 leftover due to integer division.
        test_utils::assert_eq(haneul_system::get_storage_fund_total_balance(&mut system_state), 1000 * GEUNHWA_PER_HANEUL + 24);

        // Storage charges are deposited and storage rebates are taken out of the object rebate
        // portion of the fund so its balance is 48 + 32 - 30 - 30 - 2 - 6 = 12.
        test_utils::assert_eq(haneul_system::get_storage_fund_object_rebates(&mut system_state), 12);

        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }
}
