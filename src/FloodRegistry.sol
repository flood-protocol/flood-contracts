// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";
import {EnumerableSet} from "@openzeppelin/utils/structs/EnumerableSet.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {IWETH9} from "./interfaces/IWETH9.sol";

interface IFloodRegistryEvents {
    event TokenWhitelisted(IERC20 indexed token, bool whitelisted);
    event RelayerWhitelisted(address indexed relayer, bool whitelisted);
}

error FloodRegistry__InvalidToken();
error FloodRegistry__TokenAlreadyWhitelisted();
error FloodRegistry__RelayerAlreadyWhitelisted();
error FloodRegistry__TokenNotWhitelisted();
error FloodRegistry__RelayerNotWhitelisted();
error FloodRegistry__InvalidInputLength();

/**
 * @title FloodRegistry
 * @notice This contract is the central store of information about the Flood Protocol.
 * Here its possible to query which tokens can be used in the protocol, as well as what is the latest oracle used.
 */
contract FloodRegistry is IFloodRegistryEvents, Ownable2Step {
    using EnumerableSet for EnumerableSet.AddressSet;

    EnumerableSet.AddressSet private _whitelistedTokens;
    EnumerableSet.AddressSet private _whitelistedRelayers;
    // The address of the WETH token on the current network.
    IWETH9 public immutable WETH;

    /////////////////////////////////
    //    CONSTRUCTOR             //
    ///////////////////////////////
    constructor(IWETH9 weth) {
        WETH = weth;
        // Whitelist WETH and ETH
        _whitelistToken(weth, true);
        _whitelistToken(IERC20(address(0)), true);
    }

    /////////////////////////////////
    //    ADMIN FUNCTIONS         //
    ///////////////////////////////

    /**
     * @notice Whitelists a relayer, allowing it to fill quotes. Reverts if trying to add a relayer that is already whitelisted or if trying to remove a relayer that is not whitelisted.
     * @param relayer The relayer to whitelist.
     * @param enabled Wether the relayer should be whitelisted or not.
     */
    function whitelistRelayer(address relayer, bool enabled) external onlyOwner {
        _whitelistRelayer(relayer, enabled);
    }

    /**
     * @notice Whitelists a token. Reverts if trying to add a token that is already whitelisted or if trying to remove a token that is not whitelisted.
     * @param token The token to whitelist.
     * @param enabled Whether the token should be whitelisted or not.
     */
    function whitelistToken(IERC20 token, bool enabled) external onlyOwner {
        if (address(token).code.length == 0) {
            revert FloodRegistry__InvalidToken();
        }
        _whitelistToken(token, enabled);
    }

    /**
     * @notice whitelists multiple tokens at once. Same rules as `whitelistToken` apply.
     * @param tokens The tokens to whitelist.
     * @param enabled Whether the tokens should be whitelisted or not.
     */
    function batchWhitelistTokens(IERC20[] calldata tokens, bool[] calldata enabled) external onlyOwner {
        if (tokens.length != enabled.length) {
            revert FloodRegistry__InvalidInputLength();
        }

        for (uint256 i = 0; i < tokens.length; i++) {
            if (address(tokens[i]).code.length == 0) {
                revert FloodRegistry__InvalidToken();
            }
            _whitelistToken(tokens[i], enabled[i]);
        }
    }

    /////////////////////////////////
    //    EXTERNAL FUNCTIONS      //
    ///////////////////////////////

    /**
     * @notice Checks if a token is whitelisted.
     * @param token The token to check.
     * @return True if the token is whitelisted, false otherwise.
     */
    function isTokenWhitelisted(IERC20 token) external view returns (bool) {
        return _whitelistedTokens.contains(address(token));
    }

    /**
     * @notice Checks if a list of tokens is whitelisted.
     * @param tokens The tokens to check.
     * @return True if all the tokens are whitelisted, false otherwise.
     */
    function areTokensWhitelisted(IERC20[] calldata tokens) external view returns (bool) {
        for (uint256 i = 0; i < tokens.length; i++) {
            if (!_whitelistedTokens.contains(address(tokens[i]))) {
                return false;
            }
        }
        return true;
    }

    /**
     * @notice Checks if a relayer is whitelisted.
     * @param relayer The relayer to check.
     * @return True if the relayer is whitelisted, false otherwise.
     */
    function isRelayerWhitelisted(address relayer) external view returns (bool) {
        return _whitelistedRelayers.contains(relayer);
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
     * @notice Returns all the whitelisted relayers.
     * @return The number of whitelisted relayers.
     */
    function whitelistedRelayers() external view returns (address[] memory) {
        return _whitelistedRelayers.values();
    }

    /////////////////////////////////
    //    INTERNAL FUNCTIONS      //
    ///////////////////////////////

    /**
     * @notice Whitelists a token. Reverts if trying to add a token that is already whitelisted or if trying to remove a token that is not whitelisted.
     * @param token The token to whitelist.
     * @param enabled Whether the token should be whitelisted or removed.
     */
    function _whitelistToken(IERC20 token, bool enabled) internal {
        if (enabled) {
            bool success = _whitelistedTokens.add(address(token));
            if (!success) revert FloodRegistry__TokenAlreadyWhitelisted();
        } else {
            bool success = _whitelistedTokens.remove(address(token));
            if (!success) revert FloodRegistry__TokenNotWhitelisted();
        }
        emit TokenWhitelisted(token, enabled);
    }

    /**
     * @notice Whitelists a relayer. Reverts if trying to whitelist an address that is already whitelisted.
     * @param relayer The address of the relayer to whitelist.
     * @param enabled Whether the relayer should be whitelisted or removed.
     */
    function _whitelistRelayer(address relayer, bool enabled) internal {
        if (enabled) {
            bool success = _whitelistedRelayers.add(relayer);
            if (!success) revert FloodRegistry__RelayerAlreadyWhitelisted();
        } else {
            bool success = _whitelistedRelayers.remove(relayer);
            if (!success) revert FloodRegistry__RelayerNotWhitelisted();
        }
        emit RelayerWhitelisted(relayer, enabled);
    }
}
