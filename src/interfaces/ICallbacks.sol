// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

/**
 * @title IFloodFillCallback
 * @notice Interface for Relayers to implement to optimistically receive the trade amount in
 */
interface IFloodFillCallback {
    function onFloodFill(bytes calldata data) external returns (uint128);
}

/**
 * @title IFloodRecipient
 * @notice Interface for Recipients to implement to be notified of a trade
 */
interface IFloodRecipient {
    function onTradeFilled(
        address trader,
        IERC20[] calldata tokens,
        uint128[] calldata amounts,
        uint128 amountReceived,
        bytes32 tradeId
    ) external;
}
