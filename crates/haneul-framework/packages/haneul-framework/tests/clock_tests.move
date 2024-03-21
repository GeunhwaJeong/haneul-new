// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module haneul::clock_tests {
    use haneul::clock;
    use haneul::tx_context;

    #[test]
    fun creating_a_clock_and_incrementing_it() {
        let mut ctx = tx_context::dummy();
        let mut clock = clock::create_for_testing(&mut ctx);

        clock::increment_for_testing(&mut clock, 42);
        assert!(clock::timestamp_ms(&clock) == 42, 1);

        clock::set_for_testing(&mut clock, 50);
        assert!(clock::timestamp_ms(&clock) == 50, 1);

        clock::destroy_for_testing(clock);
    }
}
