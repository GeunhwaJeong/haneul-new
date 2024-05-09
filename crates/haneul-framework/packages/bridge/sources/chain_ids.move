// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::chain_ids {

    // Chain IDs
    const HaneulMainnet: u8 = 0;
    const HaneulTestnet: u8 = 1;
    const HaneulCustom: u8 = 2;

    const EthMainnet: u8 = 10;
    const EthSepolia: u8 = 11;
    const EthCustom: u8 = 12;

    const EInvalidBridgeRoute: u64 = 0;

    //////////////////////////////////////////////////////
    // Types
    //

    public struct BridgeRoute has copy, drop, store {
        source: u8,
        destination: u8,
    }

    //////////////////////////////////////////////////////
    // Public functions
    //

    public fun haneul_mainnet(): u8 { HaneulMainnet }
    public fun haneul_testnet(): u8 { HaneulTestnet }
    public fun haneul_custom(): u8 { HaneulCustom }

    public fun eth_mainnet(): u8 { EthMainnet }
    public fun eth_sepolia(): u8 { EthSepolia }
    public fun eth_custom(): u8 { EthCustom }

    public use fun route_source as BridgeRoute.source;
    public fun route_source(route: &BridgeRoute): &u8 {
        &route.source
    }

    public use fun route_destination as BridgeRoute.destination;
    public fun route_destination(route: &BridgeRoute): &u8 {
        &route.destination
    }

    public fun assert_valid_chain_id(id: u8) {
        assert!(
            id == HaneulMainnet ||
            id == HaneulTestnet ||
            id == HaneulCustom ||
            id == EthMainnet ||
            id == EthSepolia ||
            id == EthCustom,
            EInvalidBridgeRoute
        )
    }

    public fun valid_routes(): vector<BridgeRoute> {
        vector[
            BridgeRoute { source: HaneulMainnet, destination: EthMainnet },
            BridgeRoute { source: EthMainnet, destination: HaneulMainnet },

            BridgeRoute { source: HaneulTestnet, destination: EthSepolia },
            BridgeRoute { source: HaneulTestnet, destination: EthCustom },
            BridgeRoute { source: HaneulCustom, destination: EthCustom },
            BridgeRoute { source: HaneulCustom, destination: EthSepolia },
            BridgeRoute { source: EthSepolia, destination: HaneulTestnet },
            BridgeRoute { source: EthSepolia, destination: HaneulCustom },
            BridgeRoute { source: EthCustom, destination: HaneulTestnet },
            BridgeRoute { source: EthCustom, destination: HaneulCustom }
        ]
    }

    public fun is_valid_route(source: u8, destination: u8): bool {
        let route = BridgeRoute { source, destination };
        valid_routes().contains(&route)
    }

    // Checks and return BridgeRoute if the route is supported by the bridge.
    public fun get_route(source: u8, destination: u8): BridgeRoute {
        let route = BridgeRoute { source, destination };
        assert!(valid_routes().contains(&route), EInvalidBridgeRoute);
        route
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test]
    fun test_chains_ok() {
        assert_valid_chain_id(HaneulMainnet);
        assert_valid_chain_id(HaneulTestnet);
        assert_valid_chain_id(HaneulCustom);
        assert_valid_chain_id(EthMainnet);
        assert_valid_chain_id(EthSepolia);
        assert_valid_chain_id(EthCustom);        
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_chains_error() {
        assert_valid_chain_id(100);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_haneul_chains_error() {
        // this will break if we add one more haneul chain id and should be corrected
        assert_valid_chain_id(4); 
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_eth_chains_error() {
        // this will break if we add one more eth chain id and should be corrected
        assert_valid_chain_id(13); 
    }

    #[test]
    fun test_routes() {
        let valid_routes = vector[
            BridgeRoute { source: HaneulMainnet, destination: EthMainnet },
            BridgeRoute { source: EthMainnet, destination: HaneulMainnet },

            BridgeRoute { source: HaneulTestnet, destination: EthSepolia },
            BridgeRoute { source: HaneulTestnet, destination: EthCustom },
            BridgeRoute { source: HaneulCustom, destination: EthCustom },
            BridgeRoute { source: HaneulCustom, destination: EthSepolia },
            BridgeRoute { source: EthSepolia, destination: HaneulTestnet },
            BridgeRoute { source: EthSepolia, destination: HaneulCustom },
            BridgeRoute { source: EthCustom, destination: HaneulTestnet },
            BridgeRoute { source: EthCustom, destination: HaneulCustom }
        ];
        let mut size = valid_routes.length();
        while (size > 0) {
            size = size - 1;
            let route = valid_routes[size];
            assert!(is_valid_route(route.source, route.destination), 100); // sould not assert
        }
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_haneul_1() {
        get_route(HaneulMainnet, HaneulMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_haneul_2() {
        get_route(HaneulMainnet, HaneulTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_haneul_3() {
        get_route(HaneulMainnet, EthSepolia);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_haneul_4() {
        get_route(HaneulMainnet, EthCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_1() {
        get_route(EthMainnet, EthMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_2() {
        get_route(EthMainnet, EthCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_3() {
        get_route(EthMainnet, HaneulCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_4() {
        get_route(EthMainnet, HaneulTestnet);
    }
}
