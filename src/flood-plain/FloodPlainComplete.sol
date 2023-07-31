// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "./FloodPlain.sol";
import {FloodPlainOnChainOrders} from "./extensions/FloodPlainOnChainOrders.sol";
import {FloodPlainEncodedCalls} from "./extensions/FloodPlainEncodedCalls.sol";
import {FloodPlainDirectFulfiller} from "./extensions/FloodPlainDirectFulfiller.sol";

contract FloodPlainComplete is FloodPlain, FloodPlainOnChainOrders, FloodPlainEncodedCalls, FloodPlainDirectFulfiller {
    constructor(address permit2, address admin) FloodPlain(permit2) FloodPlainEncodedCalls(admin) {}
}
