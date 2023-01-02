// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

/**
 * @title IRelayer
 * @notice Interface for Relayers to implement to optimistically receive the trade rebate pct
 */
interface IFloodFillCallback {
    function onFloodFill(bytes calldata data) external;
}
