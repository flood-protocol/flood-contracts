// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {MockERC20} from "test/utils/MockERC20.sol";
import {Zone} from "src/Zone.sol";

import {FloodPlainTestShared} from "test/utils/FloodPlainTestShared.sol";
import {OrderHash} from "src/libraries/OrderHash.sol";
import {IFloodPlain} from "src/interfaces/IFloodPlain.sol";
import {IZone} from "src/interfaces/IZone.sol";
import {IAuthZone} from "src/interfaces/IAuthZone.sol";

contract ZoneTest is FloodPlainTestShared {
    Zone mainZone;

    function setUp() public override {
        super.setUp();

        mainZone = new Zone(account0.addr);

        vm.prank(account0.addr);
        mainZone.grantRole(keccak256("FULFILLER_ROLE"), address(0xabcd));
    }

    function test_ValidateOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
        assertTrue(mainZone.validate(signedOrder.order, address(0xabcd)));
    }

    function test_InvalidateOrder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
        assertFalse(mainZone.validate(signedOrder.order, address(0xaaaa)));
    }

    function test_Pause() public {
        assertFalse(mainZone.paused());

        vm.prank(account0.addr);
        mainZone.pause();

        assertTrue(mainZone.paused());
    }

    function test_PauseUnprivileged() public {
        assertFalse(mainZone.paused());

        vm.expectRevert();
        mainZone.pause();

        assertFalse(mainZone.paused());
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

    function test_UnpauseUnprivileged() public {
        assertFalse(mainZone.paused());

        vm.prank(account0.addr);
        mainZone.pause();

        assertTrue(mainZone.paused());

        vm.expectRevert();
        mainZone.unpause();

        assertTrue(mainZone.paused());
    }

    function test_InvalidateOrderWhenPaused() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();

        vm.prank(account0.addr);
        mainZone.pause();

        assertFalse(mainZone.validate(signedOrder.order, address(0xabcd)));
    }

    function test_FeeUnsetByDefault() public {
        IFloodPlain.Order memory order = setup_mostBasicOrder().order;

        IZone.FeeInfo memory fee = mainZone.fee(order, address(0xabcd));

        assertEq(fee.recipient, address(0));
        assertEq(fee.bps, 0);
    }

    function test_SetFee(uint256 bps, address recipient) public {
        bps = bound(bps, 0, 500); // assume fee is 5% or less

        IFloodPlain.Order memory order = setup_mostBasicOrder().order;

        IZone.FeeInfo memory fee;
        fee.recipient = recipient;
        fee.bps = uint64(bps);

        vm.prank(account0.addr);
        mainZone.setFee(fee);

        assertEq(abi.encode(fee), abi.encode(mainZone.fee(order, address(0xabcd))));
    }

    function test_RevertSetFeeUnprivilegedCalled(uint256 bps, address recipient) public {
        bps = bound(bps, 0, 500); // assume fee is 5% or less

        IZone.FeeInfo memory fee;
        fee.recipient = recipient;
        fee.bps = uint64(bps);

        vm.expectRevert(
            abi.encodeWithSignature("AccessControlUnauthorizedAccount(address,bytes32)", address(this), bytes32(0))
        );
        mainZone.setFee(fee);
    }

    function test_setAuthFilter(address actor, IAuthZone.AuthFilter calldata filter) public {
        vm.prank(account0.addr);
        mainZone.setAuthorizationFilter(actor, filter);
        assertEq(abi.encode(filter), abi.encode(mainZone.authorizationFilter(actor)));
    }

    function test_setAuthFilterUnprivileged(address actor, IAuthZone.AuthFilter calldata filter) public {
        vm.expectRevert();
        mainZone.setAuthorizationFilter(actor, filter);
    }
}
