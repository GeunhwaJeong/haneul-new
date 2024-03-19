// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title IBridgeConfig
/// @dev Interface for the BridgeConfig contract.
interface IBridgeConfig {
    /* ========== STRUCTS ========== */

    /// @notice The data struct for the supported bridge tokens.
    struct Token {
        address tokenAddress;
        uint8 haneulDecimal;
    }

    /* ========== VIEW FUNCTIONS ========== */

    /// @notice Returns the address of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return address of the provided token.
    function getTokenAddress(uint8 tokenID) external view returns (address);

    /// @notice Returns the haneul decimal places of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return amount of haneul decimal places of the provided token.
    function getHaneulDecimal(uint8 tokenID) external view returns (uint8);

    /// @notice Converts the provided token amount to the Haneul decimal adjusted amount.
    /// @param tokenID The ID of the token to convert.
    /// @param amount The ERC20 amount of the tokens to convert to Haneul.
    /// @return Haneul converted amount.
    function convertERC20ToHaneulDecimal(uint8 tokenID, uint256 amount)
        external
        view
        returns (uint64);

    /// @notice Converts the provided token amount to the ERC20 decimal adjusted amount.
    /// @param tokenID The ID of the token to convert.
    /// @param amount The Haneul amount of the tokens to convert to ERC20 amount.
    /// @return ERC20 converted amount.
    function convertHaneulToERC20Decimal(uint8 tokenID, uint64 amount)
        external
        view
        returns (uint256);

    /// @notice Returns the supported status of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return true if the token is supported, false otherwise.
    function isTokenSupported(uint8 tokenID) external view returns (bool);

    /// @notice Returns whether a chain is supported in HaneulBridge with the given ID.
    /// @param chainId The ID of the chain.
    /// @return true if the chain is supported, false otherwise.
    function isChainSupported(uint8 chainId) external view returns (bool);

    /// @notice Returns the chain ID of the bridge.
    function chainID() external view returns (uint8);
}
