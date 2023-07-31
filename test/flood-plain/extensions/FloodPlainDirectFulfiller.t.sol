// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "test/flood-plain/utils/FloodPlainTestShared.sol";

import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";

contract FloodPlainDirectFulfillerTest is FloodPlainTestShared {
    function test_fulfillBasicOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 500);
        token1.approve(address(book), 500);

        // Fill the order.
        book.fulfillOrder(signedOrder);

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(this)), 500);
        assertEq(token1.balanceOf(address(this)), 0);
    }

    function test_fulfillEthOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        signedOrder.order.consideration[0].token = address(0);

        signedOrder.signature = getSignature(signedOrder.order, account0);

        uint256 balanceBefore = address(this).balance;

        // Fill the order.
        book.fulfillOrder{value: 500}(signedOrder);

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(address(account0.addr).balance, 500);
        assertEq(token0.balanceOf(address(this)), 500);
        assertEq(address(this).balance, balanceBefore - 500);
    }

    function test_fulfillMultiItemOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_multiItemOrder();

        deal(address(token3), address(this), 400);
        token3.approve(address(book), 400);
        deal(address(token4), address(this), 500);
        token4.approve(address(book), 500);
        deal(address(token5), address(this), 600);
        token5.approve(address(book), 600);

        // Fill the order.
        book.fulfillOrder(signedOrder);

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 0);
        assertEq(token2.balanceOf(address(account0.addr)), 0);
        assertEq(token3.balanceOf(address(account0.addr)), 400);
        assertEq(token4.balanceOf(address(account0.addr)), 500);
        assertEq(token5.balanceOf(address(account0.addr)), 600);
        assertEq(token0.balanceOf(address(this)), 100);
        assertEq(token1.balanceOf(address(this)), 200);
        assertEq(token2.balanceOf(address(this)), 300);
        assertEq(token3.balanceOf(address(this)), 0);
        assertEq(token4.balanceOf(address(this)), 0);
        assertEq(token5.balanceOf(address(this)), 0);
    }

    function test_RevertWhenInsufficientConsiderationApproved() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 500);
        token1.approve(address(book), 499);

        // Filling order fails.
        vm.expectRevert("ERC20: insufficient allowance");
        book.fulfillOrder(signedOrder);
    }

    function test_RevertWhenInsufficientConsiderationBalance() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 499);
        token1.approve(address(book), 500);

        // Filling order fails.
        vm.expectRevert("ERC20: transfer amount exceeds balance");
        book.fulfillOrder(signedOrder);
    }

    function test_RevertWhenInsufficientEthConsiderationReceived() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        signedOrder.order.consideration[0].token = address(0);

        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Filling order fails.
        vm.expectRevert(bytes4(keccak256("IncorrectValueReceived()")));
        book.fulfillOrder{value: 499}(signedOrder);
    }

    function test_RevertWhenTransferTaxTokens() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        signedOrder.order.consideration[0].token = address(token6); // fee-on-transfer token
        signedOrder.signature = getSignature(signedOrder.order, account0);

        deal(address(token6), address(this), 500);
        token6.approve(address(book), 500);

        // Filling order fails.
        vm.expectRevert(bytes4(keccak256("InsufficientAmountPulled()")));
        book.fulfillOrder(signedOrder);
    }

    function test_OrderPassingThroughZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 500);
        token1.approve(address(book), 500);

        // Set zone to pre deployed zone, and sign the order.
        signedOrder.order.zone = address(zone);
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Fill the order without a problem.
        book.fulfillOrder(signedOrder);
    }

    function test_RevertWhenZoneReverts() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 500);
        token1.approve(address(book), 500);

        // Set zone to pre deployed zone, and sign the order.
        signedOrder.order.zone = address(zone);
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Make zone revert.
        zone.pause();

        // Fulfillment disabled by zone.
        vm.expectRevert(IFloodPlain.ZoneDenied.selector);
        book.fulfillOrder(signedOrder);
    }

    function test_OrderValidityWithoutZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Zone is not set.

        // Nonce available and deadline not passed.
        assertTrue(book.getOrderValidity(signedOrder.order, address(this), ""));

        // Nonce available but deadline passed.
        signedOrder.order.deadline = block.timestamp - 1;
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));
        signedOrder.order.deadline = type(uint256).max;

        // Fill the order, disables nonce.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        // Deadline not passed but nonce not available.
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));

        // Deadline passed and nonce not available.
        signedOrder.order.deadline = block.timestamp - 1;
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));
    }

    function test_OrderValidityWithUnpausedZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Zone is set but does not revert.

        signedOrder.order.zone = address(zone);
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Nonce available and deadline not passed.
        assertTrue(book.getOrderValidity(signedOrder.order, address(this), ""));

        // Nonce available but deadline passed.
        signedOrder.order.deadline = block.timestamp - 1;
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));
        signedOrder.order.deadline = type(uint256).max;

        // Fill the order, disables nonce.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        // Deadline not passed but nonce not available.
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));

        // Deadline passed and nonce not available.
        signedOrder.order.deadline = block.timestamp - 1;
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));
    }

    function test_OrderValidityWithPausedZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Zone is set and reverts. Must always fail.

        signedOrder.order.zone = address(zone);
        signedOrder.signature = getSignature(signedOrder.order, account0);
        zone.pause();

        // Nonce available and deadline not passed.
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));

        // Nonce available but deadline passed.
        signedOrder.order.deadline = block.timestamp - 1;
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));
        signedOrder.order.deadline = type(uint256).max;

        // Fill the order, disables nonce.
        zone.unpause();
        book.fulfillOrder(signedOrder, address(fulfiller), "");
        zone.pause();

        // Deadline not passed but nonce not available.
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));

        // Deadline passed and nonce not available.
        signedOrder.order.deadline = block.timestamp - 1;
        assertFalse(book.getOrderValidity(signedOrder.order, address(this), ""));
    }
}
