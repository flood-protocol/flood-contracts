// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

/**
 * @title IFloodFillCallback
 * @notice Interface for Relayers to implement to optimistically receive the trade amount in
 */
interface IFloodFillCallback {
    function onFloodFill(bytes calldata data) external returns (uint128);
}
