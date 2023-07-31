// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "test/flood-plain/utils/FloodPlainTestShared.sol";

import {IFloodPlainOnChainOrders} from "src/flood-plain/extensions/IFloodPlainOnChainOrders.sol";

contract FloodPlainOnChainOrdersTest is FloodPlainTestShared {
    function test_fulfillEthOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
        IFloodPlainOnChainOrders.SimpleSignedOrder
            memory simpleSignedOrder = IFloodPlainOnChainOrders.SimpleSignedOrder({
                order: signedOrder.order,
                signature: signedOrder.signature
            });

        // Etch order.
        book.etchOrder(simpleSignedOrder);

        // Fill etched order.
        book.fulfillEtchedOrder(0, address(fulfiller), "", "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
    }

    // TODO: Revert works properly, relaying the same error message.

    function test_etchingIncrementsOrderIds() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
        IFloodPlainOnChainOrders.SimpleSignedOrder
            memory simpleSignedOrder = IFloodPlainOnChainOrders.SimpleSignedOrder({
                order: signedOrder.order,
                signature: signedOrder.signature
            });

        uint256 order1 = book.etchOrder(simpleSignedOrder);
        assertEq(order1, 0);
        uint256 order2 = book.etchOrder(simpleSignedOrder);
        assertEq(order2, 1);
        uint256 order3 = book.etchOrder(simpleSignedOrder);
        assertEq(order3, 2);
        uint256 order4 = book.etchOrder(simpleSignedOrder);
        assertEq(order4, 3);
    }

    function test_getEtchedOrderDetails() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
        IFloodPlainOnChainOrders.SimpleSignedOrder
            memory simpleSignedOrder = IFloodPlainOnChainOrders.SimpleSignedOrder({
                order: signedOrder.order,
                signature: signedOrder.signature
            });

        // Etch order.
        book.etchOrder(simpleSignedOrder);

        // Get etched order details.
        IFloodPlainOnChainOrders.SimpleSignedOrder memory retreivedOrder = book.getEtchedOrder(0);

        assertEq(retreivedOrder.signature, signedOrder.signature);
        assertEq(retreivedOrder.order.offerer, signedOrder.order.offerer);
        assertEq(retreivedOrder.order.zone, signedOrder.order.zone);
        assertEq(retreivedOrder.order.offer[0].token, signedOrder.order.offer[0].token);
        assertEq(retreivedOrder.order.offer[0].amount, signedOrder.order.offer[0].amount);
        assertEq(
            retreivedOrder.order.consideration[0].token,
            signedOrder.order.consideration[0].token
        );
        assertEq(
            retreivedOrder.order.consideration[0].amount,
            signedOrder.order.consideration[0].amount
        );
        assertEq(retreivedOrder.order.deadline, signedOrder.order.deadline);
        assertEq(retreivedOrder.order.nonce, signedOrder.order.nonce);
    }
}
