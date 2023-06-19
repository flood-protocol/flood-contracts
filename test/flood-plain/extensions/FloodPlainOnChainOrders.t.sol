// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "test/flood-plain/utils/FloodPlainTestShared.sol";

import {IFloodPlainOnChainOrders} from "src/flood-plain/extensions/IFloodPlainOnChainOrders.sol";

contract FloodPlainOnChainOrdersTest is FloodPlainTestShared {
    function test_fulfillEthOrder() public {
        (IFloodPlain.Order memory order, bytes memory sig) = setup_mostBasicOrder();

        IFloodPlainOnChainOrders.OrderWithSignature memory orderWithSig = IFloodPlainOnChainOrders.OrderWithSignature({
            order: order,
            signature: sig
        });

        // Etch order.
        book.etchOrder(orderWithSig);

        // Fill etched order.
        book.fulfillEtchedOrder(0, address(fulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
    }

    function test_etchingIncrementsOrderIds() public {
        (IFloodPlain.Order memory order, bytes memory sig) = setup_mostBasicOrder();

        IFloodPlainOnChainOrders.OrderWithSignature memory orderWithSig = IFloodPlainOnChainOrders.OrderWithSignature({
            order: order,
            signature: sig
        });

        uint256 order1 = book.etchOrder(orderWithSig);
        assertEq(order1, 0);
        uint256 order2 = book.etchOrder(orderWithSig);
        assertEq(order2, 1);
        uint256 order3 = book.etchOrder(orderWithSig);
        assertEq(order3, 2);
        uint256 order4 = book.etchOrder(orderWithSig);
        assertEq(order4, 3);
    }

    function test_getEtchedOrderDetails() public {
        (IFloodPlain.Order memory order, bytes memory sig) = setup_mostBasicOrder();

        IFloodPlainOnChainOrders.OrderWithSignature memory orderWithSig = IFloodPlainOnChainOrders.OrderWithSignature({
            order: order,
            signature: sig
        });

        // Etch order.
        book.etchOrder(orderWithSig);

        // Get etched order details.
        IFloodPlainOnChainOrders.OrderWithSignature memory retreivedOrderWithSig = book.getEtchedOrder(0);

        assertEq(retreivedOrderWithSig.signature, orderWithSig.signature);
        assertEq(retreivedOrderWithSig.order.offerer, orderWithSig.order.offerer);
        assertEq(retreivedOrderWithSig.order.zone, orderWithSig.order.zone);
        assertEq(retreivedOrderWithSig.order.offer[0].token, orderWithSig.order.offer[0].token);
        assertEq(retreivedOrderWithSig.order.offer[0].amount, orderWithSig.order.offer[0].amount);
        assertEq(retreivedOrderWithSig.order.consideration[0].token, orderWithSig.order.consideration[0].token);
        assertEq(retreivedOrderWithSig.order.consideration[0].amount, orderWithSig.order.consideration[0].amount);
        assertEq(retreivedOrderWithSig.order.deadline, orderWithSig.order.deadline);
        assertEq(retreivedOrderWithSig.order.nonce, orderWithSig.order.nonce);
    }
}
