// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "test/utils/FloodPlainTestShared.sol";

import {PermitHash} from "permit2/src/libraries/PermitHash.sol";
import {OrderHash} from "src/libraries/OrderHash.sol";

import {IFloodPlain} from "src/interfaces/IFloodPlain.sol";

contract FloodPlainTest is FloodPlainTestShared {
    function test_fulfillBasicOrder() public {
        (IFloodPlain.SignedOrder memory signedOrder) = setup_mostBasicOrder();

        // Prechecks.
        assertEq(token0.balanceOf(address(account0.addr)), 500);
        assertEq(token1.balanceOf(address(account0.addr)), 0);
        assertEq(token0.balanceOf(address(fulfiller)), 0);
        assertEq(token1.balanceOf(address(fulfiller)), 500);

        // Fill the order.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
    }

    function test_fulfillOrderWithCustomRecipient() public {
        (IFloodPlain.SignedOrder memory signedOrder) = setup_mostBasicOrder();

        // Set the recipient to account3.
        signedOrder.order.recipient = account3.addr;
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Prechecks.
        assertEq(token0.balanceOf(address(account0.addr)), 500);
        assertEq(token1.balanceOf(address(account0.addr)), 0);
        assertEq(token0.balanceOf(address(fulfiller)), 0);
        assertEq(token1.balanceOf(address(fulfiller)), 500);
        assertEq(token0.balanceOf(address(account3.addr)), 0);
        assertEq(token1.balanceOf(address(account3.addr)), 0);

        // Fill the order.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 0);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
        assertEq(token0.balanceOf(address(account3.addr)), 0);
        assertEq(token1.balanceOf(address(account3.addr)), 500);
    }

    function test_fulfillMultiItemOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_multiItemOrder();

        // Fill the order.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 0);
        assertEq(token2.balanceOf(address(account0.addr)), 0);
        assertEq(token3.balanceOf(address(account0.addr)), 999);
        assertEq(token0.balanceOf(address(fulfiller)), 100);
        assertEq(token1.balanceOf(address(fulfiller)), 200);
        assertEq(token2.balanceOf(address(fulfiller)), 300);
        assertEq(token3.balanceOf(address(fulfiller)), 0);
    }

    function test_revertWhenOfferTokenRepeated(uint256 indexA, uint256 indexB) public {
        indexA = bound(indexA, 0, 2);
        indexB = bound(indexB, 0, 2);
        vm.assume(indexA != indexB);

        IFloodPlain.SignedOrder memory signedOrder = setup_multiItemOrder();

        // Set deadline to one second ago, and sign it.
        signedOrder.order.offer[indexA].token = signedOrder.order.offer[indexB].token;
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Fail fill the order.
        vm.expectRevert(bytes4(keccak256("DuplicateItems()")));
        book.fulfillOrder(signedOrder, address(fulfiller), "");
    }

    function test_RevertWhenInsufficientConsiderationReceived() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(maliciousFulfiller), 500);

        // Filling order fails.
        vm.expectRevert(
            abi.encodeWithSignature("ERC20InsufficientAllowance(address,uint256,uint256)", address(book), 499, 500)
        );
        book.fulfillOrder(signedOrder, address(maliciousFulfiller), "");
    }

    function test_RevertWhenInsufficientConsiderationIndicated() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(maliciousFulfiller2), 500);

        // Filling order fails.
        vm.expectRevert(bytes4(keccak256("InsufficientAmountReceived()")));
        book.fulfillOrder(signedOrder, address(maliciousFulfiller2), "");
    }

    function test_NonceValidity() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        assertTrue(book.getNonceStatus(account0.addr, 0));

        // Fill the order.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        assertFalse(book.getNonceStatus(account0.addr, 0));
    }

    function test_NonceValidityForOrderId1() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Use nonce 1. Using nonce 0 does not pass mutation tests due to bit shift operations by 0
        // being ineffective. So we test with nonce 1 as well.
        signedOrder.order.nonce = 1;
        signedOrder.signature = getSignature(signedOrder.order, account0);

        assertTrue(book.getNonceStatus(account0.addr, 1));

        // Fill the order.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        assertFalse(book.getNonceStatus(account0.addr, 1));
    }

    function test_RevertWhenNonceReuse() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Fill the order.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        deal(address(token0), address(account0.addr), 500);
        deal(address(token1), address(fulfiller), 500);

        // Approve permit2 spending.
        vm.prank(account0.addr);
        token0.approve(address(permit2), 500);

        // Revert due to nonce reuse.
        vm.expectRevert(abi.encodePacked(bytes4(keccak256("InvalidNonce()"))));
        book.fulfillOrder(signedOrder, address(fulfiller), "");
    }

    function test_RevertWhenDeadlineExpire() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Set deadline to one second ago, and sign it.
        signedOrder.order.deadline = block.timestamp - 1;
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Fill the order.
        vm.expectRevert(abi.encodePacked(bytes4(keccak256("SignatureExpired(uint256)")), bytes32(bytes20(address(0)))));
        book.fulfillOrder(signedOrder, address(fulfiller), "");
    }

    function test_OrderPassingThroughZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Set zone to pre deployed zone, and sign the order.
        signedOrder.order.zone = address(zone);
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Fill the order without a problem.
        book.fulfillOrder(signedOrder, address(fulfiller), "");
    }

    function test_RevertWhenZoneReverts() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Set zone to pre deployed zone, and sign the order.
        signedOrder.order.zone = address(zone);
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Make zone revert.
        zone.pause();

        // Fulfillment disabled by zone.
        vm.expectRevert(bytes4(keccak256("ZoneDenied()")));
        book.fulfillOrder(signedOrder, address(fulfiller), "");
    }

    function test_OrderStatus() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Nonce available and deadline not passed.
        assertTrue(book.getOrderStatus(signedOrder.order));

        // Nonce available but deadline passed.
        signedOrder.order.deadline = block.timestamp - 1;
        assertFalse(book.getOrderStatus(signedOrder.order));
        signedOrder.order.deadline = type(uint256).max;

        // Fill the order, disables nonce.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        // Deadline not passed but nonce not available.
        assertFalse(book.getOrderStatus(signedOrder.order));

        // Deadline passed and nonce not available.
        signedOrder.order.deadline = block.timestamp - 1;
        assertFalse(book.getOrderStatus(signedOrder.order));
    }

    function test_OrderStatusWhenDeadlineIsNow() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        // Nonce available and deadline not passed.
        assertTrue(book.getOrderStatus(signedOrder.order));

        // Nonce available and deadline still not passed (per Permit2 contract)
        signedOrder.order.deadline = block.timestamp;
        assertTrue(book.getOrderStatus(signedOrder.order));
        signedOrder.order.deadline = type(uint256).max;

        // Fill the order, disables nonce.
        book.fulfillOrder(signedOrder, address(fulfiller), "");

        // Deadline not passed but nonce not available.
        assertFalse(book.getOrderStatus(signedOrder.order));

        // Deadline still passed and nonce not available.
        signedOrder.order.deadline = block.timestamp;
        assertFalse(book.getOrderStatus(signedOrder.order));
    }

    //function test_fufillThroughDecoder() public {
    //    book.addDecoder(address(decoder));

    //    IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
    //    signedOrder.order.deadline = type(uint32).max;
    //    signedOrder.signature = getSignature(signedOrder.order, account0);

    //    (bytes32 r, bytes32 s) = abi.decode(signedOrder.signature, (bytes32, bytes32));
    //    bytes1 v = signedOrder.signature[64];

    //    bytes memory encodedDataBegin = abi.encodePacked(
    //        bytes1(0x00), // fallback selector
    //        bytes1(0x00), // decoder id
    //        signedOrder.order.offerer,
    //        signedOrder.order.zone,
    //        uint24(signedOrder.order.nonce),
    //        uint32(signedOrder.order.deadline),
    //        address(fulfiller),
    //        v,
    //        r,
    //        s
    //    );

    //    bytes memory encodedDataEnd = abi.encodePacked(
    //        bytes1(0x11), // (0001, 0001)
    //        address(address(token0)),
    //        uint112(500),
    //        address(address(token1)),
    //        uint112(500)
    //    );

    //    bytes memory encodedData = bytes.concat(encodedDataBegin, encodedDataEnd);

    //    address bookAddress = address(book);

    //    bool result;
    //    assembly {
    //        result := call(gas(), bookAddress, 0, add(encodedData, 0x20), encodedData, 0, 0)
    //    }
    //    assertTrue(result);

    //    // Assertions.
    //    assertEq(token0.balanceOf(address(account0.addr)), 0);
    //    assertEq(token1.balanceOf(address(account0.addr)), 500);
    //    assertEq(token0.balanceOf(address(fulfiller)), 500);
    //    assertEq(token1.balanceOf(address(fulfiller)), 0);
    //}

    function test_OrderHash() public {
        IFloodPlain.Item memory item = IFloodPlain.Item({token: address(0), amount: 0});

        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](3);
        offer[0] = item;
        offer[1] = item;
        offer[2] = item;

        IFloodPlain.Hook[] memory preHooks = new IFloodPlain.Hook[](1);
        IFloodPlain.Hook[] memory postHooks = new IFloodPlain.Hook[](2);

        IFloodPlain.Order memory order = IFloodPlain.Order({
            offerer: address(0),
            zone: address(0),
            recipient: address(0),
            offer: offer,
            consideration: item,
            deadline: 0,
            nonce: 0,
            preHooks: preHooks,
            postHooks: postHooks
        });

        bytes32 orderHash = book.getOrderHash(order);

        // Confirmed through EthersV6. See OrderHash.t.sol for details.
        assertEq(orderHash, 0xece53f158244592f601148c3a00ab85c63d4bf4ce04da8375e216dfc40694b32);
    }

    function test_PermitHash() public {
        IFloodPlain.Item memory item = IFloodPlain.Item({token: address(0), amount: 0});

        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](3);
        offer[0] = item;
        offer[1] = item;
        offer[2] = item;

        IFloodPlain.Hook[] memory preHooks = new IFloodPlain.Hook[](1);
        IFloodPlain.Hook[] memory postHooks = new IFloodPlain.Hook[](2);

        IFloodPlain.Order memory order = IFloodPlain.Order({
            offerer: address(0),
            zone: address(0),
            recipient: address(0),
            offer: offer,
            consideration: item,
            deadline: 0,
            nonce: 0,
            preHooks: preHooks,
            postHooks: postHooks
        });

        vm.etch(address(0x420), address(book).code);
        bytes32 permitHash = IFloodPlain(payable(address(0x420))).getPermitHash(order);

        assertEq(permitHash, 0x8aea3ef4ab58e3cfd67a39b948421def10f4424ee4be0b8c1be0bb6c05bb022a);
    }

    /**
     *
     */
    /* DIRECT ORDERS */
    /**
     *
     */
    function test_fulfillBasicDirectOrder() public {
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

    function test_fulfillMultiItemDirectOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_multiItemOrder();

        deal(address(token3), address(this), 999);
        token3.approve(address(book), 999);

        // Fill the order.
        book.fulfillOrder(signedOrder);

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 0);
        assertEq(token2.balanceOf(address(account0.addr)), 0);
        assertEq(token3.balanceOf(address(account0.addr)), 999);
        assertEq(token0.balanceOf(address(this)), 100);
        assertEq(token1.balanceOf(address(this)), 200);
        assertEq(token2.balanceOf(address(this)), 300);
        assertEq(token3.balanceOf(address(this)), 0);
    }

    function test_RevertDirectOrderWhenInsufficientConsiderationApproved() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 500);
        token1.approve(address(book), 499);

        // Filling order fails.
        vm.expectRevert(
            abi.encodeWithSignature("ERC20InsufficientAllowance(address,uint256,uint256)", address(book), 499, 500)
        );
        book.fulfillOrder(signedOrder);
    }

    function test_RevertDirectOrderWhenInsufficientConsiderationBalance() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 499);
        token1.approve(address(book), 500);

        // Filling order fails.
        vm.expectRevert(
            abi.encodeWithSignature("ERC20InsufficientBalance(address,uint256,uint256)", address(this), 499, 500)
        );
        book.fulfillOrder(signedOrder);
    }

    function test_revertDirectOrderWhenOfferTokenRepeated(uint256 indexA, uint256 indexB) public {
        indexA = bound(indexA, 0, 2);
        indexB = bound(indexB, 0, 2);
        vm.assume(indexA != indexB);

        IFloodPlain.SignedOrder memory signedOrder = setup_multiItemOrder();

        // Set deadline to one second ago, and sign it.
        signedOrder.order.offer[indexA].token = signedOrder.order.offer[indexB].token;
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Fail fill the order.
        vm.expectRevert(bytes4(keccak256("DuplicateItems()")));
        book.fulfillOrder{value: 0}(signedOrder);
    }

    function test_DirectOrderPassingThroughZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 500);
        token1.approve(address(book), 500);

        // Set zone to pre deployed zone, and sign the order.
        signedOrder.order.zone = address(zone);
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Fill the order without a problem.
        book.fulfillOrder(signedOrder);
    }

    function test_RevertDirectOrderWhenZoneReverts() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        deal(address(token1), address(this), 500);
        token1.approve(address(book), 500);

        // Set zone to pre deployed zone, and sign the order.
        signedOrder.order.zone = address(zone);
        signedOrder.signature = getSignature(signedOrder.order, account0);

        // Make zone revert.
        zone.pause();

        // Fulfillment disabled by zone.
        vm.expectRevert(bytes4(keccak256("ZoneDenied()")));
        book.fulfillOrder(signedOrder);
    }

    /**
     *
     */
    /* BATCHED ORDERS */
    /**
     *
     */
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

    function test_RevertBatchFillWhenDeadlineExpire() public {
        IFloodPlain.SignedOrder[] memory signedOrders = new IFloodPlain.SignedOrder[](2);
        signedOrders[0] = setup_mostBasicOrder();
        signedOrders[1] = setup_mostBasicOrder();

        // Send tokens to fulfiller to fill the order.
        deal(address(token1), address(batchFulfiller), 1000);

        signedOrders[1].order.nonce = 1;
        signedOrders[1].order.deadline = block.timestamp - 1;
        signedOrders[1].signature = getSignature(signedOrders[1].order, account0);

        // Fill the order.
        vm.expectRevert(abi.encodePacked(bytes4(keccak256("SignatureExpired(uint256)")), bytes32(bytes20(address(0)))));
        book.fulfillOrders(signedOrders, address(batchFulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 1000);
        assertEq(token1.balanceOf(address(account0.addr)), 0);
        assertEq(token0.balanceOf(address(batchFulfiller)), 0);
        assertEq(token1.balanceOf(address(batchFulfiller)), 1000);
    }

    function test_BatchOrderPassingThroughZone() public {
        IFloodPlain.SignedOrder[] memory signedOrders = new IFloodPlain.SignedOrder[](2);
        signedOrders[0] = setup_mostBasicOrder();
        signedOrders[1] = setup_mostBasicOrder();

        // Send tokens to fulfiller to fill the order.
        deal(address(token1), address(batchFulfiller), 1000);

        signedOrders[1].order.nonce = 1;
        signedOrders[1].order.zone = address(zone);
        signedOrders[1].signature = getSignature(signedOrders[1].order, account0);

        // Fill the order.
        book.fulfillOrders(signedOrders, address(batchFulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 1000);
        assertEq(token0.balanceOf(address(batchFulfiller)), 1000);
        assertEq(token1.balanceOf(address(batchFulfiller)), 0);
    }

    function test_RevertBatchFillWhenZoneReverts() public {
        IFloodPlain.SignedOrder[] memory signedOrders = new IFloodPlain.SignedOrder[](2);
        signedOrders[0] = setup_mostBasicOrder();
        signedOrders[1] = setup_mostBasicOrder();

        // Send tokens to fulfiller to fill the order.
        deal(address(token1), address(batchFulfiller), 1000);

        signedOrders[1].order.nonce = 1;
        signedOrders[1].order.zone = address(zone);
        signedOrders[1].signature = getSignature(signedOrders[1].order, account0);

        // Make zone revert.
        zone.pause();

        // Fulfillment disabled by zone.
        vm.expectRevert(bytes4(keccak256("ZoneDenied()")));
        book.fulfillOrders(signedOrders, address(batchFulfiller), "");

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 1000);
        assertEq(token1.balanceOf(address(account0.addr)), 0);
        assertEq(token0.balanceOf(address(batchFulfiller)), 0);
        assertEq(token1.balanceOf(address(batchFulfiller)), 1000);
    }
}
