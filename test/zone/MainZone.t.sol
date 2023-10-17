// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {MockERC20} from "test/flood-plain/utils/MockERC20.sol";
import {MainZone} from "src/zone/implementations/MainZone.sol";
import {FloodPlainTestShared} from "test/flood-plain/utils/FloodPlainTestShared.sol";
import {OrderHash} from "src/flood-plain/libraries/OrderHash.sol";
import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";
import {IZone, IZoneDirectFulfiller} from "src/zone/ZoneComplete.sol";

contract MainZoneTest is FloodPlainTestShared {
    MainZone mainZone;

    function setUp() public override {
        super.setUp();

        mainZone = new MainZone(account0.addr);
    }

    function test_Pause() public {
        assertFalse(mainZone.paused());

        vm.prank(account0.addr);
        mainZone.pause();

        assertTrue(mainZone.paused());
    }

    function test_Unpause() public {
        assertFalse(mainZone.paused());

        vm.prank(account0.addr);
        mainZone.pause();

        assertTrue(mainZone.paused());

        vm.prank(account0.addr);
        mainZone.unpause();

        assertFalse(mainZone.paused());
    }

    function test_RevertPauseWhenUnprivilegedCaller() public {
        assertFalse(mainZone.paused());

        vm.expectRevert();
        mainZone.pause();

        assertFalse(mainZone.paused());
    }

    function test_RevertUnpauseWhenUnprivilegedCaller() public {
        assertFalse(mainZone.paused());

        vm.prank(account0.addr);
        mainZone.pause();

        assertTrue(mainZone.paused());

        vm.expectRevert();
        mainZone.unpause();

        assertTrue(mainZone.paused());
    }

    function test_SetSecondaryZone() public {
        assertEq(mainZone.secondaryZone(), address(0));

        vm.prank(account0.addr);
        mainZone.setSecondaryZone(address(zone));

        assertEq(mainZone.secondaryZone(), address(zone));
    }

    function test_SetSecondaryZoneRevertWhenNonContract() public {
        assertEq(mainZone.secondaryZone(), address(0));

        vm.prank(account0.addr);
        vm.expectRevert();
        mainZone.setSecondaryZone(account0.addr);
    }

    function test_SetSecondaryZoneRevertWhenUnprivileged() public {
        assertEq(mainZone.secondaryZone(), address(0));

        vm.prank(account1.addr);
        vm.expectRevert();
        mainZone.setSecondaryZone(address(zone));
    }

    function test_UnsetSecondaryZone() public {
        assertEq(mainZone.secondaryZone(), address(0));

        vm.prank(account0.addr);
        mainZone.setSecondaryZone(address(zone));

        assertEq(mainZone.secondaryZone(), address(zone));

        vm.prank(account0.addr);
        mainZone.setSecondaryZone(address(0));

        assertEq(mainZone.secondaryZone(), address(0));
    }

    function test_UnsetSecondaryZoneRevertWhenUnprivileged() public {
        assertEq(mainZone.secondaryZone(), address(0));

        vm.prank(account0.addr);
        mainZone.setSecondaryZone(address(zone));

        assertEq(mainZone.secondaryZone(), address(zone));

        vm.prank(account1.addr);
        vm.expectRevert();
        mainZone.setSecondaryZone(address(0));
    }

    function test_ValidateDirectOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertEq(magicValue, IZoneDirectFulfiller.validateOrder.selector);
    }

    function test_RevertValidateDirectOrderWhenInvalidCallerAndInvalidBook() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
    }

    function test_RevertValidateDirectOrderWhenInvalidCaller() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
    }

    function test_RevertValidateDirectOrderWhenInvalidBook() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
    }

    function test_RevertValidateDirectOrderWhenCancelledOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));

        bytes32 orderHash = book.getOrderHash(signedOrder.order);

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CANCELLED_ORDERS"), address(uint160(uint256(orderHash))));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(0), // caller
            orderHash,
            "" // context
        );

        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
    }

    function test_ValidateDirectOrderWithSecondaryZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.setSecondaryZone(address(zone));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertEq(magicValue, IZoneDirectFulfiller.validateOrder.selector);
    }

    function test_RevertValidateDirectOrderWithInvalidSecondaryZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.setSecondaryZone(address(zone));

        zone.pause();

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
    }

    function test_RevertValidateDirectOrderWhenPaused() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.pause();

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZoneDirectFulfiller.validateOrder.selector);
    }

    function test_ValidateOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertEq(magicValue, IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenInvalidCallerInvalidBookInvalidFulfiller() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenInvalidCaller() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenInvalidBook() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenInvalidFulfiller() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenInvalidFulfillerAndInvalidBook() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenInvalidFulfillerAndInvalidCaller() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenInvalidBookAndInvalidCaller() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenCancelledOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));

        bytes32 orderHash = book.getOrderHash(signedOrder.order);

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CANCELLED_ORDERS"), address(uint160(uint256(orderHash))));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            orderHash,
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_ValidateOrderWithSecondaryZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));

        vm.prank(account0.addr);
        mainZone.setSecondaryZone(address(zone));

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertEq(magicValue, IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWithInvalidSecondaryZone() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));

        vm.prank(account0.addr);
        mainZone.setSecondaryZone(address(zone));
        zone.pause();

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }

    function test_RevertValidateOrderWhenPaused() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("CALLER_ROLE"), address(0));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("BOOK_ROLE"), address(1));
        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(2));

        vm.prank(account0.addr);
        mainZone.pause();

        bytes4 magicValue = mainZone.validateOrder(
            signedOrder.order,
            address(1), // book
            address(2), // fullfiller
            address(0), // caller
            book.getOrderHash(signedOrder.order),
            "" // context
        );

        assertFalse(magicValue == IZone.validateOrder.selector);
    }
}
