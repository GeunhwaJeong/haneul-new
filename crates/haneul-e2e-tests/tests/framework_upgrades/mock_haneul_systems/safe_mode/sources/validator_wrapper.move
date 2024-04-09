// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module haneul_system::validator_wrapper {
    use haneul::versioned::Versioned;

    /* friend haneul_system::haneul_system_state_inner; */

    public struct ValidatorWrapper has store {
        inner: Versioned
    }
}
