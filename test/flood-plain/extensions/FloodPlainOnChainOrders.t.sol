// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "test/flood-plain/utils/FloodPlainTestShared.sol";

import {IFloodPlainOnChainOrders} from "src/flood-plain/extensions/IFloodPlainOnChainOrders.sol";

contract FloodPlainOnChainOrdersTest is FloodPlainTestShared {
    function test_fulfillEthOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Etch order.
        book.etchOrder(signedOrder);

        // Fill etched order.
        book.fulfillEtchedOrder(0, address(fulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
    }

    // TODO: Revert works properly, relaying the same error message.

    function test_etchingIncrementsOrderIds() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        uint256 order1 = book.etchOrder(signedOrder);
        assertEq(order1, 0);
        uint256 order2 = book.etchOrder(signedOrder);
        assertEq(order2, 1);
        uint256 order3 = book.etchOrder(signedOrder);
        assertEq(order3, 2);
        uint256 order4 = book.etchOrder(signedOrder);
        assertEq(order4, 3);
    }

    function test_getEtchedOrderDetails() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Etch order.
        book.etchOrder(signedOrder);

        // Get etched order details.
        IFloodPlain.SignedOrder memory retreivedOrderWithSig = book.getEtchedOrder(0);

        assertEq(retreivedOrderWithSig.signature, signedOrder.signature);
        assertEq(retreivedOrderWithSig.order.offerer, signedOrder.order.offerer);
        assertEq(retreivedOrderWithSig.order.zone, signedOrder.order.zone);
        assertEq(retreivedOrderWithSig.order.offer[0].token, signedOrder.order.offer[0].token);
        assertEq(retreivedOrderWithSig.order.offer[0].amount, signedOrder.order.offer[0].amount);
        assertEq(retreivedOrderWithSig.order.consideration[0].token, signedOrder.order.consideration[0].token);
        assertEq(retreivedOrderWithSig.order.consideration[0].amount, signedOrder.order.consideration[0].amount);
        assertEq(retreivedOrderWithSig.order.deadline, signedOrder.order.deadline);
        assertEq(retreivedOrderWithSig.order.nonce, signedOrder.order.nonce);
    }
}
