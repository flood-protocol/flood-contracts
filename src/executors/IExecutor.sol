// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/**
 * @title IExecutor
 * @notice A generic contract executing swaps.
 *
 * @dev This contract is used to execute swaps in the Fulfiller contract.
 * As each executor is called by the Fulfiller in delegatecall, a few invariants must be respected:
 * 1. The executor must not have any storage variables, and must be stateless. If storage is needed, consider using immutable variables instead.
 * 2. Callback signatures MUST be reviewed for collisions. This is especially important when dealing with forks.
 * 3. Fee on transfer tokens MUST not be send by the Fulfiller.
 * 4. Use slither upgradeability checks (even if the executor is not a proxy)
 */
interface IExecutor {
    struct Swap {
        address pool;
        uint256 tokenInIndex;
        uint256 tokenOutIndex;
        uint256 amountIn;
        uint256 amountOut;
    }

    /**
     * @notice Return true if this executor has a callback.
     */
    function hasCallback() external view returns (bool);

    /**
     * @notice Get the address that might do a callback during a swap.
     *
     * @param swap The details of the swap that will be made.
     *
     * @dev There must only one valid callback source for a given swap info.
     * @dev If the executor does not have any callbacks, this function must revert.
     *
     * @return callbackSource The address that might do a callback.
     */
    function getCallbackSource(Swap memory swap) external view returns (address callbackSource);

    /**
     * @notice Swap tokens based on the swap argument.
     *
     * @param swap The details of the swap.
     */
    function swap(Swap memory swap) external;
}
