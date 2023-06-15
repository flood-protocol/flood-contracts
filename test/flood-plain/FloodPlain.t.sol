// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";
import {DeployPermit2} from "permit2/test/utils/DeployPermit2.sol";

import {OrderHash} from "src/flood-plain/libraries/OrderHash.sol";
import {FloodPlain} from "src/flood-plain/FloodPlain.sol";
import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";
import {MockERC20} from "test/flood-plain/utils/MockERC20.sol";
import {MockFulfiller} from "test/flood-plain/utils/MockFulfiller.sol";
import {OrderSignature} from "test/flood-plain/utils/OrderSignature.sol";
import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";
import {EIP712} from "permit2/src/EIP712.sol";

import {PermitHash} from "permit2/src/libraries/PermitHash.sol";

contract MyTest is Test, DeployPermit2 {
    ISignatureTransfer permit2;
    FloodPlain book;
    MockFulfiller fulfiller;
    OrderSignature orderSignature;
    MockERC20 token0;
    MockERC20 token1;
    MockERC20 token2;
    MockERC20 token3;
    MockERC20 token4;
    MockERC20 token5;
    MockERC20 token6;
    Account account0;
    Account account1;
    Account account2;
    Account account3;

    function setUp() public {
        permit2 = ISignatureTransfer(deployPermit2());
        book = new FloodPlain(address(permit2));
        fulfiller = new MockFulfiller();
        orderSignature = new OrderSignature();
        token0 = new MockERC20();
        token1 = new MockERC20();
        token2 = new MockERC20();
        token3 = new MockERC20();
        token4 = new MockERC20();
        token5 = new MockERC20();
        token6 = new MockERC20();
        account0 = makeAccount("a");
        account1 = makeAccount("b");
        account2 = makeAccount("c");
        account3 = makeAccount("d");
    }

    function setup_mostBasicOrder() internal returns (IFloodPlain.Order memory order, bytes memory sig) {
        deal(address(token0), address(account0.addr), 500);
        deal(address(token1), address(fulfiller), 500);

        // Set offer item.
        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](1);
        offer[0].token = address(token0);
        offer[0].amount = 500;

        // Approve permit2 spending.
        vm.prank(account0.addr);
        token0.approve(address(permit2), 500);

        // Set consideration item.
        IFloodPlain.Item[] memory consideration = new IFloodPlain.Item[](1);
        consideration[0].token = address(token1);
        consideration[0].amount = 500;

        // Construct order.
        order = IFloodPlain.Order({
            offerer: address(account0.addr),
            zone: address(0),
            offer: offer,
            consideration: consideration,
            deadline: type(uint256).max,
            nonce: 0
        });

        // Sign the order.
        sig = orderSignature.getSignature(order, account0.key, EIP712(address(permit2)).DOMAIN_SEPARATOR(), address(book));

        return (order, sig);
    }

    function test_fulfillBasicOrder() public {
        (IFloodPlain.Order memory order, bytes memory sig) = setup_mostBasicOrder();

        // Fill the order.
        book.fulfillOrder(order, sig, address(fulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
    }

    function test_NonceValidity() public {
        (IFloodPlain.Order memory order, bytes memory sig) = setup_mostBasicOrder();

        assertTrue(book.getNonceStatus(account0.addr, 0));

        // Fill the order.
        book.fulfillOrder(order, sig, address(fulfiller), "");

        assertFalse(book.getNonceStatus(account0.addr, 0));
    }

    function test_RevertWhenNonceReuse() public {
        (IFloodPlain.Order memory order, bytes memory sig) = setup_mostBasicOrder();

        // Fill the order.
        book.fulfillOrder(order, sig, address(fulfiller), "");

        deal(address(token0), address(account0.addr), 500);
        deal(address(token1), address(fulfiller), 500);

        // Approve permit2 spending.
        vm.prank(account0.addr);
        token0.approve(address(permit2), 500);

        // Revert due to nonce reuse.
        vm.expectRevert();
        book.fulfillOrder(order, sig, address(fulfiller), "");
    }
}
