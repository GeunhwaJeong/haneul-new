// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import "openzeppelin-foundry-upgrades/Options.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "../contracts/HaneulBridge.sol";
import "../contracts/HaneulBridgeV2.sol";
import "../contracts/BridgeCommittee.sol";
import "../contracts/utils/BridgeUtils.sol";

/// @title UpgradeBridgeV2
/// @notice Script to validate and deploy HaneulBridgeV2 implementation for upgrade
/// @dev This script:
///      1. Validates that HaneulBridgeV2 is upgrade-safe from HaneulBridge
///      2. Deploys the HaneulBridgeV2 implementation contract
///      3. Outputs the information needed to create the governance upgrade action
///
/// After running this script, you need to:
///      1. Create an upgrade governance action with the new implementation address
///      2. Collect committee signatures on the upgrade message
///      3. Call bridge.upgradeWithSignatures(signatures, message)
contract UpgradeBridgeV2 is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        // Get the existing HaneulBridge proxy address
        address haneulBridgeProxy = vm.envAddress("HANEUL_BRIDGE_PROXY");

        string memory chainID = Strings.toString(block.chainid);
        console.log("Chain ID:", chainID);
        console.log("HaneulBridge proxy address:", haneulBridgeProxy);

        // Step 1: Validate the upgrade
        console.log("\n=== Validating upgrade from HaneulBridge to HaneulBridgeV2 ===");

        Options memory opts;

        opts.referenceContract = "HaneulBridge.sol";

        Upgrades.validateUpgrade("HaneulBridgeV2.sol", opts);
        console.log("[OK] Upgrade validation passed");

        // Step 2: Deploy the new implementation
        vm.startBroadcast(deployerPrivateKey);

        HaneulBridgeV2 bridgeV2Implementation = new HaneulBridgeV2();
        console.log("\n[Deployed] HaneulBridgeV2 implementation:", address(bridgeV2Implementation));

        vm.stopBroadcast();

        // Step 3: Output the upgrade governance action details
        console.log("\n=== Upgrade Governance Action Details ===");
        console.log("To complete the upgrade, create a governance action with:");
        console.log("  - Proxy address:", haneulBridgeProxy);
        console.log("  - New implementation:", address(bridgeV2Implementation));
        console.log("  - Initializer data: (empty - no initializer needed)");

        // Get current nonce info if proxy is accessible
        try HaneulBridge(haneulBridgeProxy).committee() returns (IBridgeCommittee committeeContract) {
            try BridgeCommittee(address(committeeContract)).nonces(BridgeUtils.UPGRADE) returns (
                uint64 upgradeNonce
            ) {
                console.log("  - Current upgrade nonce:", upgradeNonce);
            } catch {}
            try committeeContract.config() returns (IBridgeConfig configContract) {
                try configContract.chainID() returns (uint8 sourceChainId) {
                    console.log("  - Source chain ID:", sourceChainId);
                } catch {}
            } catch {}
        } catch {}

        console.log("\n=== Upgrade Message Format ===");
        console.log("Create a BridgeUtils.Message with:");
        console.log("  messageType: BridgeUtils.UPGRADE (6)");
        console.log("  version: 1");
        console.log("  nonce: <current upgrade nonce>");
        console.log("  chainID: <source chain ID>");
        console.log("  payload: abi.encode(proxyAddress, implementationAddress, initializerData)");

        console.log("\n=== Next Steps ===");
        console.log("1. Create the upgrade message with the above parameters");
        console.log("2. Have committee members sign the message hash");
        console.log("3. Call bridge.upgradeWithSignatures(signatures, message)");
    }

    // used to ignore for forge coverage
    function testSkip() public {}
}
