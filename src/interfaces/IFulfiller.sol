// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../interfaces/IFloodPlain.sol";

interface IFulfiller {
    function pullTokens(address to, IFloodPlain.ConsiderationItem[] calldata items) external;

    function sourceConsideration(
        IFloodPlain.Order calldata order,
        address caller,
        bytes calldata context
    ) external;
}
