// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "./IFloodPlain.sol";

interface IFulfiller {
    function pullToken(address token, address to, uint256 amount) external;
    function pullNativeToken(address to, uint256 amount) external;
    function sourceConsideration(
        IFloodPlain.OrderParameters calldata order,
        address fulfiller,
        address caller,
        bytes32 orderHash,
        bytes calldata context
    ) external;
}
