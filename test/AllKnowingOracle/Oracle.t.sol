// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/interfaces/IERC20.sol";
import {
    IOptimisticRequester,
    IAllKnowingOracleEvents,
    Request,
    RequestState,
    AllKnowingOracle__NonSettler,
    AllKnowingOracle__NotSettleable,
    AllKnowingOracle__SettlerIsDisputerOrProposer
} from "src/AllKnowingOracle.sol";
import {OracleFixture} from "./Fixtures.sol";

contract MockRequester is IOptimisticRequester {
    bytes32 public id;
    int256 public price = 0;

    function onPriceSettled(bytes32 _id, Request memory _request) external {
        id = _id;
        int256 _price = abi.decode(_request.data, (int256));
        price = _price;
    }
}

contract AllKnowingOracleTest is IAllKnowingOracleEvents, OracleFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testGetId(
        address sender,
        address proposer,
        address disputer,
        address currency,
        uint256 bond,
        uint256 reqIndex
    ) public {
        bytes32 id = keccak256(abi.encodePacked(sender, proposer, disputer, currency, bond, reqIndex));
        assertEq(oracle.getRequestId(sender, proposer, disputer, currency, bond, reqIndex), id);
    }

    function testAsk(uint256 bond) public {
        vm.assume(bond < type(uint256).max / 2);
        // As Charlie is the requester, he will pay the bond for Alice and Bob.
        deal(USDC, charlie, 2 * bond);
        uint256 charlieBalanceBefore = IERC20(USDC).balanceOf(charlie);
        uint256 reqIndex = oracle.requestCount();
        bytes32 id = oracle.getRequestId(charlie, alice, bob, USDC, bond, reqIndex);
        vm.prank(charlie);
        vm.expectEmit(true, true, true, true, address(oracle));
        emit NewRequest(id, alice, bob, USDC, bond, reqIndex);
        oracle.ask(alice, bob, USDC, bond, abi.encode(charlie));

        assertEq(IERC20(USDC).balanceOf(charlie), charlieBalanceBefore - 2 * bond);

        // FIXME: For a bug in foundry, non packed less than 32bytes slots are not found, so we go through the public getter instead. Once fixed move this to reading from storage.
        (
            address storageRequester,
            address storageProposer,
            address storageDisputer,
            IERC20 storageCurrency,
            uint256 storageBond,
            RequestState storageState,
            bool storageAnswer,
            bytes memory storageData
        ) = oracle.requests(id);

        assertEq(storageRequester, charlie);
        assertEq(storageProposer, alice);
        assertEq(storageDisputer, bob);
        assertEq(address(storageCurrency), USDC);
        assertEq(storageBond, bond);
        assertEq(uint256(storageState), uint256(RequestState.Pending));
        assertEq(storageAnswer, false);
        assertEq(storageData, abi.encode(charlie));
    }

    function testCannotAskWithInsufficientBalanceForBond() public {
        vm.prank(charlie);
        vm.expectRevert("ERC20: transfer amount exceeds balance");
        oracle.ask(alice, bob, USDC, 100, "");
    }

    function testCannotAskIfNoAllowance() public {
        // Bob removes the allowance for USDC
        vm.prank(bob);
        IERC20(USDC).approve(address(oracle), 0);
        vm.expectRevert("ERC20: transfer amount exceeds balance");
        vm.prank(charlie);
        oracle.ask(alice, bob, USDC, 100, "");
    }

    function testSettle(bool answer, uint256 bond) public {
        vm.assume(bond < type(uint256).max / 2);
        MockRequester requester = new MockRequester();
        address requesterAddress = address(requester);
        vm.prank(requesterAddress);
        IERC20(USDC).approve(address(oracle), type(uint256).max);
        // requester is sponsoring the bond for alice and bob
        deal(USDC, requesterAddress, 2 * bond);
        vm.prank(requesterAddress);
        oracle.ask(alice, bob, USDC, bond, abi.encode(int256(-42)));

        uint256 reqIndex = oracle.requestCount() - 1;
        bytes32 id = oracle.getRequestId(requesterAddress, alice, bob, USDC, bond, reqIndex);
        (
            address storageRequester,
            address storageProposer,
            address storageDisputer,
            IERC20 storageCurrency,
            uint256 storageBond,
            ,
            ,
            bytes memory storageData
        ) = oracle.requests(id);

        assertEq(storageRequester, requesterAddress);
        assertEq(storageProposer, alice);
        assertEq(storageDisputer, bob);
        assertEq(address(storageCurrency), USDC);
        assertEq(storageBond, bond);
        assertEq(storageData, abi.encode(int256(-42)));

        vm.prank(charlie);
        vm.expectEmit(true, true, true, true, address(oracle));
        emit RequestSettled(id, answer);
        oracle.settle(id, answer);

        assertEq(requester.price(), int256(-42));
        assertEq(requester.id(), id);
    }

    function testCannotSettleIfNotSettler() public {
        vm.expectRevert(AllKnowingOracle__NonSettler.selector);
        vm.prank(alice);
        oracle.settle(bytes32(0), true);
    }

    function testCannotSettleIfProposerOrDisputer() public {
        uint256 bond = 100;
        // As Charlie is the requester, he will pay the bond for Alice and Bob.
        deal(USDC, charlie, 2 * bond);
        uint256 reqIndex = oracle.requestCount();
        bytes32 id = oracle.getRequestId(charlie, alice, bob, USDC, bond, reqIndex);
        vm.prank(charlie);
        oracle.ask(alice, bob, USDC, bond, abi.encode(charlie));

        oracle.whitelistSettler(alice, true);
        vm.prank(alice);
        vm.expectRevert(abi.encodeWithSelector(AllKnowingOracle__SettlerIsDisputerOrProposer.selector, id));
        oracle.settle(id, true);

        oracle.whitelistSettler(bob, true);
        vm.prank(bob);
        vm.expectRevert(abi.encodeWithSelector(AllKnowingOracle__SettlerIsDisputerOrProposer.selector, id));
        oracle.settle(id, true);
    }

    function testCannotSettleIfAlreadySettled() public {
        uint256 bond = 100;
        bool answer = true;
        deal(USDC, charlie, 2 * bond);
        vm.prank(charlie);
        oracle.ask(alice, bob, USDC, bond, "");
        uint256 reqIndex = oracle.requestCount() - 1;
        bytes32 id = oracle.getRequestId(charlie, alice, bob, USDC, bond, reqIndex);

        vm.prank(charlie);
        vm.expectEmit(true, true, true, true, address(oracle));
        emit RequestSettled(id, answer);
        oracle.settle(id, answer);

        vm.prank(charlie);
        vm.expectRevert(abi.encodeWithSelector(AllKnowingOracle__NotSettleable.selector, id));
        oracle.settle(id, answer);
    }

    function testCannotSettleIfUninitialized() public {
        vm.prank(charlie);
        vm.expectRevert(abi.encodeWithSelector(AllKnowingOracle__NotSettleable.selector, bytes32(0)));
        oracle.settle(bytes32(0), true);
    }
}
