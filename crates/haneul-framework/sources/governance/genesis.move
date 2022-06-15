// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module haneul::genesis {
    use std::vector;

    use haneul::coin;
    use haneul::haneul;
    use haneul::haneul_system;
    use haneul::tx_context::TxContext;
    use haneul::validator;

    /// The initial amount of HANEUL locked in the storage fund.
    /// 10^14, an arbitrary number.
    const INIT_STORAGE_FUND: u64 = 100000000000000;

    /// Initial value of the lower-bound on the amount of stake required to become a validator.
    const INIT_MIN_VALIDATOR_STAKE: u64 = 100000000000000;

    /// Initial value of the upper-bound on the number of validators.
    const INIT_MAX_VALIDATOR_COUNT: u64 = 100;

    /// This function will be explicitly called once at genesis.
    /// It will create a singleton HaneulSystemState object, which contains
    /// all the information we need in the system.
    fun create(
        validator_pubkeys: vector<vector<u8>>,
        validator_haneul_addresses: vector<address>,
        validator_names: vector<vector<u8>>,
        validator_net_addresses: vector<vector<u8>>,
        validator_stakes: vector<u64>,
        ctx: &mut TxContext,
    ) {
        let treasury_cap = haneul::new(ctx);
        let storage_fund = coin::mint_balance(INIT_STORAGE_FUND, &mut treasury_cap);
        let validators = vector::empty();
        let count = vector::length(&validator_pubkeys);
        assert!(
            vector::length(&validator_haneul_addresses) == count
                && vector::length(&validator_stakes) == count
                && vector::length(&validator_names) == count
                && vector::length(&validator_net_addresses) == count,
            1
        );
        let i = 0;
        while (i < count) {
            let haneul_address = *vector::borrow(&validator_haneul_addresses, i);
            let pubkey = *vector::borrow(&validator_pubkeys, i);
            let name = *vector::borrow(&validator_names, i);
            let net_address = *vector::borrow(&validator_net_addresses, i);
            let stake = *vector::borrow(&validator_stakes, i);
            vector::push_back(&mut validators, validator::new(
                haneul_address,
                pubkey,
                name,
                net_address,
                coin::mint_balance(stake, &mut treasury_cap),
            ));
            i = i + 1;
        };
        haneul_system::create(
            validators,
            treasury_cap,
            storage_fund,
            INIT_MAX_VALIDATOR_COUNT,
            INIT_MIN_VALIDATOR_STAKE,
        );
    }
}
