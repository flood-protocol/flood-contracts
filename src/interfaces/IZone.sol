// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "./IFloodPlain.sol";

interface IZone {
    function validateOrder(
        IFloodPlain.Order calldata order,
        address fulfiller,
        address caller,
        bytes32 orderHash,
        bytes calldata context
    ) external view;
}
