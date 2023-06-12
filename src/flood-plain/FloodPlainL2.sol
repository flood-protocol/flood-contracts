// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "./FloodPlain.sol";
import {FloodPlainOnChainOrders} from "./extensions/FloodPlainOnChainOrders.sol";
import {FloodPlainEncodedCalls} from "./extensions/FloodPlainEncodedCalls.sol";

contract FloodPlainL2 is FloodPlainOnChainOrders, FloodPlainEncodedCalls {
    constructor(address permit2) FloodPlain(permit2) {}
}
