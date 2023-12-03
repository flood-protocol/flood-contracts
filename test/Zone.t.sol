// SPDX-License-Identifier: MIT
//pragma solidity ^0.8.17;
//
//import {MockERC20} from "test/flood-plain/utils/MockERC20.sol";
//import {MainZone} from "src/zone/implementations/MainZone.sol";
//import {FloodPlainTestShared} from "test/flood-plain/utils/FloodPlainTestShared.sol";
//import {OrderHash} from "src/flood-plain/libraries/OrderHash.sol";
//import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";
//import {IZone, IZoneDirectFulfiller} from "src/zone/ZoneComplete.sol";
//
//contract MainZoneTest is FloodPlainTestShared {
//    Zone zone;
//
//    function setUp() public override {
//        super.setUp();
//
//        zone = new Zone(account0.addr);
//    }
//
//    function test_ValidateDirectOrder() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertEq(magicValue, IZoneDirectFulfiller.validateOrder.selector);
//    }
//
//    function test_RevertValidateDirectOrderWhenInvalidCallerAndInvalidBook() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
//    }
//
//    function test_RevertValidateDirectOrderWhenInvalidCaller() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
//    }
//
//    function test_RevertValidateDirectOrderWhenInvalidBook() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
//    }
//
//    function test_RevertValidateDirectOrderWhenCancelledOrder() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//
//        bytes32 orderHash = book.getOrderHash(signedOrder.order);
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CANCELLED_ORDERS"), address(uint160(uint256(orderHash))));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(0), // caller
//            orderHash,
//            "" // context
//        );
//
//        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
//    }
//
//    function test_ValidateDirectOrderWithSecondaryZone() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.setSecondaryZone(address(zone));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertEq(magicValue, IZoneDirectFulfiller.validateOrder.selector);
//    }
//
//    function test_RevertValidateDirectOrderWithInvalidSecondaryZone() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.setSecondaryZone(address(zone));
//
//        zone.pause();
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
//    }
//
//    function test_RevertValidateDirectOrderWhenPaused() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.pause();
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
//    }
//
//    function test_ValidateOrder() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertEq(magicValue, IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenInvalidCallerInvalidBookInvalidFulfiller() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenInvalidCaller() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenInvalidBook() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenInvalidFulfiller() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenInvalidFulfillerAndInvalidBook() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenInvalidFulfillerAndInvalidCaller() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenInvalidBookAndInvalidCaller() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenCancelledOrder() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));
//
//        bytes32 orderHash = book.getOrderHash(signedOrder.order);
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CANCELLED_ORDERS"), address(uint160(uint256(orderHash))));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            orderHash,
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_ValidateOrderWithSecondaryZone() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));
//
//        vm.prank(account0.addr);
//        mainZone.setSecondaryZone(address(zone));
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertEq(magicValue, IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWithInvalidSecondaryZone() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));
//
//        vm.prank(account0.addr);
//        mainZone.setSecondaryZone(address(zone));
//        zone.pause();
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_RevertValidateOrderWhenPaused() public {
//        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
//
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
//        vm.prank(account0.addr);
//        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));
//
//        vm.prank(account0.addr);
//        mainZone.pause();
//
//        bytes4 magicValue = mainZone.validateOrder(
//            signedOrder.order,
//            address(1), // book
//            address(2), // fullfiller
//            address(0), // caller
//            book.getOrderHash(signedOrder.order),
//            "" // context
//        );
//
//        assertFalse(magicValue == IZone.validateOrder.selector);
//    }
//
//    function test_FeeUnsetByDefault() public {
//        IFloodPlain.Order memory order = setup_mostBasicOrder().order;
//
//        (address recipient, uint256 fee) = mainZone.fee(order);
//
//        assertEq(recipient, address(0));
//        assertEq(fee, 0);
//    }
//
//    function test_SetFee(uint256 newFee) public {
//        newFee = bound(newFee, 0, 500); // assume fee is 5% or less
//
//        IFloodPlain.Order memory order = setup_mostBasicOrder().order;
//
//        vm.prank(account0.addr);
//        mainZone.setFee(newFee);
//        (address recipient, uint256 fee) = mainZone.fee(order);
//
//        assertEq(recipient, address(0));
//        assertEq(fee, newFee);
//    }
//
//    function test_SetFeeRevertWhenAbove5Percent(uint256 newFee) public {
//        newFee = bound(newFee, 501, type(uint256).max); // assume fee is 5% or less
//
//        IFloodPlain.Order memory order = setup_mostBasicOrder().order;
//
//        vm.prank(account0.addr);
//        vm.expectRevert();
//        mainZone.setFee(newFee);
//
//        (address recipient, uint256 fee) = mainZone.fee(order);
//
//        assertEq(recipient, address(0));
//        assertEq(fee, 0);
//    }
//
//    function test_SetFeeRevertWhenUnprivileged(uint256 newFee) public {
//        newFee = bound(newFee, 0, 500); // assume fee is 5% or less
//
//        IFloodPlain.Order memory order = setup_mostBasicOrder().order;
//
//        vm.prank(account1.addr);
//        vm.expectRevert();
//        mainZone.setFee(newFee);
//
//        (address recipient, uint256 fee) = mainZone.fee(order);
//
//        assertEq(recipient, address(0));
//        assertEq(fee, 0);
//    }
//
//    function test_SetFeeRecipient(address newFeeRecipient) public {
//        IFloodPlain.Order memory order = setup_mostBasicOrder().order;
//
//        vm.prank(account0.addr);
//        mainZone.setFeeRecipient(newFeeRecipient);
//
//        (address recipient, uint256 fee) = mainZone.fee(order);
//
//        assertEq(recipient, newFeeRecipient);
//        assertEq(fee, 0);
//    }
//
//    function test_SetFeeRecipientRevertWhenUnprivileged(address newFeeRecipient) public {
//        IFloodPlain.Order memory order = setup_mostBasicOrder().order;
//
//        vm.prank(account1.addr);
//        vm.expectRevert();
//        mainZone.setFeeRecipient(newFeeRecipient);
//
//        (address recipient, uint256 fee) = mainZone.fee(order);
//
//        assertEq(recipient, address(0));
//        assertEq(fee, 0);
//    }
//}
