// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../interfaces/IFloodPlain.sol";

interface IFulfiller {
    function sourceConsideration(
        IFloodPlain.Order calldata order,
        IFloodPlain.ConsiderationItem[] calldata requestedItems,
        address caller,
        bytes calldata context
    ) external;
}
