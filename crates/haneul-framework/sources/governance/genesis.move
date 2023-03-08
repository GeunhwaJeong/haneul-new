// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module haneul::genesis {
    use std::vector;

    use haneul::balance;
    use haneul::coin;
    use haneul::clock;
    use haneul::haneul;
    use haneul::haneul_system;
    use haneul::tx_context::TxContext;
    use haneul::validator;
    use std::option;

    /// Initial value of the lower-bound on the amount of stake required to become a validator.
    const INIT_MIN_VALIDATOR_STAKE: u64 = 25_000_000_000_000_000;

    /// Initial value of the upper-bound on the number of validators.
    const INIT_MAX_VALIDATOR_COUNT: u64 = 100;

    /// Stake subisidy to be given out in the very first epoch. Placeholder value.
    const INIT_STAKE_SUBSIDY_AMOUNT: u64 = 1000000;

    /// The initial balance of the Subsidy fund in Geunhwa (1 Billion * 10^9)
    const INIT_STAKE_SUBSIDY_FUND_BALANCE: u64 = 1_000_000_000_000_000_000;

    /// This function will be explicitly called once at genesis.
    /// It will create a singleton HaneulSystemState object, which contains
    /// all the information we need in the system.
    fun create(
        initial_haneul_custody_account_address: address,
        initial_validator_stake_geunhwa: u64,
        governance_start_epoch: u64,
        validator_pubkeys: vector<vector<u8>>,
        validator_network_pubkeys: vector<vector<u8>>,
        validator_worker_pubkeys: vector<vector<u8>>,
        validator_proof_of_possessions: vector<vector<u8>>,
        validator_haneul_addresses: vector<address>,
        validator_names: vector<vector<u8>>,
        validator_descriptions: vector<vector<u8>>,
        validator_image_urls: vector<vector<u8>>,
        validator_project_urls: vector<vector<u8>>,
        validator_net_addresses: vector<vector<u8>>,
        validator_p2p_addresses: vector<vector<u8>>,
        validator_primary_addresses: vector<vector<u8>>,
        validator_worker_addresses: vector<vector<u8>>,
        validator_gas_prices: vector<u64>,
        validator_commission_rates: vector<u64>,
        protocol_version: u64,
        system_state_version: u64,
        epoch_start_timestamp_ms: u64,
        ctx: &mut TxContext,
    ) {
        let haneul_supply = haneul::new(ctx);
        let subsidy_fund = balance::split(&mut haneul_supply, INIT_STAKE_SUBSIDY_FUND_BALANCE);
        let storage_fund = balance::zero();
        let validators = vector::empty();
        let count = vector::length(&validator_pubkeys);
        assert!(
            vector::length(&validator_haneul_addresses) == count
                && vector::length(&validator_names) == count
                && vector::length(&validator_descriptions) == count
                && vector::length(&validator_image_urls) == count
                && vector::length(&validator_project_urls) == count
                && vector::length(&validator_net_addresses) == count
                && vector::length(&validator_p2p_addresses) == count
                && vector::length(&validator_primary_addresses) == count
                && vector::length(&validator_worker_addresses) == count
                && vector::length(&validator_gas_prices) == count
                && vector::length(&validator_commission_rates) == count,
            1
        );
        let i = 0;
        while (i < count) {
            let haneul_address = *vector::borrow(&validator_haneul_addresses, i);
            let pubkey = *vector::borrow(&validator_pubkeys, i);
            let network_pubkey = *vector::borrow(&validator_network_pubkeys, i);
            let worker_pubkey = *vector::borrow(&validator_worker_pubkeys, i);
            let proof_of_possession = *vector::borrow(&validator_proof_of_possessions, i);
            let name = *vector::borrow(&validator_names, i);
            let description = *vector::borrow(&validator_descriptions, i);
            let image_url = *vector::borrow(&validator_image_urls, i);
            let project_url = *vector::borrow(&validator_project_urls, i);
            let net_address = *vector::borrow(&validator_net_addresses, i);
            let p2p_address = *vector::borrow(&validator_p2p_addresses, i);
            let primary_address = *vector::borrow(&validator_primary_addresses, i);
            let worker_address = *vector::borrow(&validator_worker_addresses, i);
            let gas_price = *vector::borrow(&validator_gas_prices, i);
            let commission_rate = *vector::borrow(&validator_commission_rates, i);
            vector::push_back(&mut validators, validator::new(
                haneul_address,
                pubkey,
                network_pubkey,
                worker_pubkey,
                proof_of_possession,
                name,
                description,
                image_url,
                project_url,
                net_address,
                p2p_address,
                primary_address,
                worker_address,
                // Initialize all validators with uniform stake taken from the subsidy fund.
                option::some(balance::split(&mut subsidy_fund, initial_validator_stake_geunhwa)),
                option::none(),
                gas_price,
                commission_rate,
                true, // validator is active right away
                ctx
            ));
            i = i + 1;
        };

        haneul_system::create(
            validators,
            subsidy_fund,
            storage_fund,
            INIT_MAX_VALIDATOR_COUNT,
            INIT_MIN_VALIDATOR_STAKE,
            governance_start_epoch,
            INIT_STAKE_SUBSIDY_AMOUNT,
            protocol_version,
            system_state_version,
            epoch_start_timestamp_ms,
            ctx,
        );

        clock::create();

        // Transfer the remaining balance of haneul's supply to the initial account
        haneul::transfer(coin::from_balance(haneul_supply, ctx), initial_haneul_custody_account_address);
    }
}
