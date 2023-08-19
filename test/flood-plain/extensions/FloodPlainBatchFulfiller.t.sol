// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "test/flood-plain/utils/FloodPlainTestShared.sol";

import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";

contract FloodPlaiBatchFulfillerTest is FloodPlainTestShared {
    function test_fulfillBasicBatchOrder() public {
        IFloodPlain.SignedOrder[] memory signedOrders = new IFloodPlain.SignedOrder[](2);
        signedOrders[0] = setup_mostBasicOrder();
        signedOrders[1] = setup_mostBasicOrder();

        // Send tokens to fulfiller to fill the order.
        deal(address(token1), address(batchFulfiller), 1000);

        // Change second order nonce.
        signedOrders[1].order.nonce = 1;
        signedOrders[1].signature = getSignature(signedOrders[1].order, account0);

        // Fill the order.
        book.fulfillOrders(signedOrders, address(batchFulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 1000);
        assertEq(token0.balanceOf(address(batchFulfiller)), 1000);
        assertEq(token1.balanceOf(address(batchFulfiller)), 0);
    }
}
