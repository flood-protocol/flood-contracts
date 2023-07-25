// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "./FloodPlain.sol";
import {FloodPlainBatchFulfiller} from "./extensions/FloodPlainBatchFulfiller.sol";
import {FloodPlainOnChainOrders} from "./extensions/FloodPlainOnChainOrders.sol";
import {FloodPlainEncodedCalls} from "./extensions/FloodPlainEncodedCalls.sol";
import {FloodPlainDirectFulfiller} from "./extensions/FloodPlainDirectFulfiller.sol";
import {FloodPlainBatchDirectFulfiller} from "./extensions/FloodPlainBatchDirectFulfiller.sol";

contract FloodPlainL2 is
    FloodPlain,
    FloodPlainBatchFulfiller,
    FloodPlainOnChainOrders,
    FloodPlainEncodedCalls,
    FloodPlainDirectFulfiller,
    FloodPlainBatchDirectFulfiller
{
    constructor(address permit2, address admin) FloodPlain(permit2) FloodPlainEncodedCalls(admin) {}
}
