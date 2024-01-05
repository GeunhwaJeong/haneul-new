// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::chain_ids {

    use std::vector;

    // Chain IDs
    const HaneulMainnet: u8 = 0;
    const HaneulTestnet: u8 = 1;
    const HaneulDevnet: u8 = 2;

    const EthMainnet: u8 = 10;
    const EthSepolia: u8 = 11;

    struct BridgeRoute has drop {
        source: u8,
        destination: u8,
    }

    public fun haneul_mainnet(): u8 {
        HaneulMainnet
    }

    public fun haneul_testnet(): u8 {
        HaneulTestnet
    }

    public fun haneul_devnet(): u8 {
        HaneulDevnet
    }

    public fun eth_mainnet(): u8 {
        EthMainnet
    }

    public fun eth_sepolia(): u8 {
        EthSepolia
    }

    public fun valid_routes(): vector<BridgeRoute> {
        vector[
            BridgeRoute { source: HaneulMainnet, destination: EthMainnet },
            BridgeRoute { source: HaneulDevnet, destination: EthSepolia },
            BridgeRoute { source: HaneulTestnet, destination: EthSepolia },
            BridgeRoute { source: EthMainnet, destination: HaneulMainnet },
            BridgeRoute { source: EthSepolia, destination: HaneulDevnet },
            BridgeRoute { source: EthSepolia, destination: HaneulTestnet }]
    }

    public fun is_valid_route(source: u8, destination: u8): bool {
        let route = BridgeRoute { source, destination };
        return vector::contains(&valid_routes(), &route)
    }
}
