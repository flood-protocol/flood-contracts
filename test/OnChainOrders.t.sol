// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./utils/FloodPlainTestShared.sol";

import {OnChainOrders} from "src/OnChainOrders.sol";
import {IOnChainOrders} from "src/interfaces/IOnChainOrders.sol";

contract OnChainOrdersContract is OnChainOrders {}

contract OnChainOrdersTest is FloodPlainTestShared {
    OnChainOrdersContract onChainOrders;

    function setUp() public override {
        onChainOrders = new OnChainOrdersContract();
        super.setUp();
    }

    function test_etch() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        bytes32 hash = orderSignature.hash(signedOrder.order);

        vm.expectEmit(true, false, false, true, address(onChainOrders));
        emit IOnChainOrders.OrderEtched(hash, signedOrder);

        onChainOrders.etchOrder(signedOrder);
    }
}
