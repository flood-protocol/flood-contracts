// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {AllKnowingOracle} from "./AllKnowingOracle.sol";
import {Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";
import {EnumerableSet} from "@openzeppelin/utils/structs/EnumerableSet.sol";

interface IFloodRegistryEvents {
    event TokenWhitelisted(address indexed token, bool whitelisted);
    event OracleChanged(AllKnowingOracle indexed oracle);
}

error FloodRegistry__TokenAlreadyWhitelisted();
error FloodRegistry__TokenNotWhitelisted();
error FloodRegistry__InvalidInputLength();

/**
 * @title FloodRegistry
 * @notice This contract is the central store of information about the Flood Protocol.
 * Here its possible to query which tokens can be used in the protocol, as well as what is the latest oracle used.
 */
contract FloodRegistry is IFloodRegistryEvents, Ownable2Step {
    using EnumerableSet for EnumerableSet.AddressSet;

    EnumerableSet.AddressSet private _whitelistedTokens;
    // The latest oracle used in the protocol.
    AllKnowingOracle public latestOracle;

    /**
     * @notice Whitelists a token. Reverts if trying to add a token that is already whitelisted or if trying to remove a token that is not whitelisted.
     * @param token The token to whitelist.
     * @param enabled Whether the token should be whitelisted or not.
     */
    function whitelistToken(address token, bool enabled) external onlyOwner {
        _whitelistToken(token, enabled);
    }

    /**
     * @notice whitelists multiple tokens at once. Same rules as `whitelistToken` apply.
     * @param tokens The tokens to whitelist.
     * @param enabled Whether the tokens should be whitelisted or not.
     */
    function batchWhitelistTokens(address[] calldata tokens, bool[] calldata enabled) external onlyOwner {
        if (tokens.length != enabled.length) {
            revert FloodRegistry__InvalidInputLength();
        }

        for (uint256 i = 0; i < tokens.length; i++) {
            _whitelistToken(tokens[i], enabled[i]);
        }
    }

    /**
     * @notice Sets the latest oracle used in the protocol.
     * @param oracle The oracle to set.
     */
    function setOracle(AllKnowingOracle oracle) external onlyOwner {
        latestOracle = oracle;
        emit OracleChanged(oracle);
    }

    /**
     * @notice Checks if a token is whitelisted.
     * @param token The token to check.
     * @return True if the token is whitelisted, false otherwise.
     */
    function isTokenWhitelisted(address token) external view returns (bool) {
        return _whitelistedTokens.contains(token);
    }

    /**
     * @notice Returns the number of whitelisted tokens.
     * @return The number of whitelisted tokens.
     */
    function whitelistedTokensCount() external view returns (uint256) {
        return _whitelistedTokens.length();
    }

    /**
     * @notice Returns all the whitelisted tokens.
     * @dev This function is gas intensive and should be used off-chain or with caution.
     * @return The whitelisted tokens.
     */
    function whitelistedTokens() external view returns (address[] memory) {
        return _whitelistedTokens.values();
    }

    /**
     * @notice Whitelists a token. Reverts if trying to add a token that is already whitelisted or if trying to remove a token that is not whitelisted.
     * @param token The token to whitelist.
     * @param enabled Whether the token should be whitelisted or removed.
     */
    function _whitelistToken(address token, bool enabled) internal {
        if (enabled) {
            bool success = _whitelistedTokens.add(token);
            if (!success) revert FloodRegistry__TokenAlreadyWhitelisted();
        } else {
            bool success = _whitelistedTokens.remove(token);
            if (!success) revert FloodRegistry__TokenNotWhitelisted();
        }
        emit TokenWhitelisted(token, enabled);
    }
}
