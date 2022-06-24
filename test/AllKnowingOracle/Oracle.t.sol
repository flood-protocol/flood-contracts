// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "src/AllKnowingOracle.sol";
import "./Fixtures.sol";
import "forge-std/Test.sol";

contract AllKnowingOracleTest is IAllKnowingOracleEvents, OracleFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testBondForStake(uint256 s) public {
        if (s >= type(uint256).max / 25) {
            vm.expectRevert(stdError.arithmeticError);
            oracle.bondForStake(s);
            return;
        }
        uint256 b = (s * 25) / 100;
        assertEq(oracle.bondForStake(s), b);
    }

    function testAsk(uint256 stake) public {
        // above this, the oracle will reject the request as the math (stake * 25 / 100) in the contract will overflow
        vm.assume(stake < type(uint256).max / 25);
        uint256 bond = oracle.bondForStake(stake);
        bytes32 id = keccak256(abi.encode(alice, bob, USDC, stake, bond));
        // Give alice & bob some tokens
        deal(USDC, alice, stake);
        deal(USDC, bob, bond);

        uint256 aliceBalanceBefore = ERC20(USDC).balanceOf(alice);
        uint256 bobBalanceBefore = ERC20(USDC).balanceOf(bob);
        vm.prank(alice);
        vm.expectEmit(true, true, true, true, address(oracle));
        emit NewRequest(id, alice, bob, USDC, stake, bond);
        bytes32 realId = _ask(alice, bob, USDC, stake);
        assertEq(id, realId);

        // Check that the request is in the oracle's list of pending requests
        (
            address _proposer,
            address _disputer,
            address _bondToken,
            uint256 _stake,
            uint256 _bond,
            bool _answer,
            RequestState _state
        ) = oracle.requests(id);

        assertEq(_proposer, alice);
        assertEq(_disputer, bob);
        assertEq(_bondToken, USDC);
        assertEq(uint256(_state), uint256(RequestState.Pending));
        assertEq(_answer, false);
        assertEq(_bond, bond);
        assertEq(_stake, stake);
        assertEq(ERC20(USDC).balanceOf(alice), aliceBalanceBefore - stake);
        assertEq(ERC20(USDC).balanceOf(bob), bobBalanceBefore - bond);
    }

    function testAskForOtherProposer() public {
        uint256 stake = 100;
        uint256 bond = 25;
        address proposer = charlie;
        bytes32 id = keccak256(abi.encode(proposer, bob, USDC, stake, bond));
        // Give alice & bob some tokens
        deal(USDC, alice, stake);
        deal(USDC, bob, bond);

        uint256 aliceBalanceBefore = ERC20(USDC).balanceOf(alice);
        vm.prank(alice);
        vm.expectEmit(true, true, true, true, address(oracle));
        emit NewRequest(id, proposer, bob, USDC, stake, bond);
        _ask(proposer, bob, USDC, 100);

        // Check that the request is in the oracle's list of pending requests
        (
            address _proposer,
            address _disputer,
            address _bondToken,
            uint256 _stake,
            uint256 _bond,
            bool _answer,
            RequestState _state
        ) = oracle.requests(id);

        assertEq(_proposer, proposer);
        assertEq(_disputer, bob);
        assertEq(_bondToken, USDC);
        assertEq(uint256(_state), uint256(RequestState.Pending));
        assertEq(_answer, false);
        assertEq(_bond, bond);
        assertEq(_stake, stake);
        // It should be Alice to have sponsored the request
        assertEq(ERC20(USDC).balanceOf(alice), aliceBalanceBefore - stake);
    }

    function testCannotAskWithNonWhitelistedToken(address bondToken) public {
        vm.assume(bondToken != USDC);
        vm.assume(bondToken != WETH);
        vm.expectRevert(
            abi.encodeWithSelector(
                AllKnowingOracle__NotWhitelisted.selector,
                bondToken
            )
        );
        vm.prank(alice);
        oracle.ask(alice, bob, bondToken, 100);
    }

    function testCannotAskWithInsufficientBalanceForBond() public {
        vm.prank(alice);
        vm.expectRevert("TRANSFER_FROM_FAILED");
        _ask(alice, bob, USDC, 100);
    }

    function testCannotAskIfNoAllowance() public {
        // Alice removes the allowance for USDC
        vm.prank(alice);
        ERC20(USDC).approve(address(oracle), 0);
        vm.expectRevert("TRANSFER_FROM_FAILED");
        vm.prank(alice);
        oracle.ask(alice, bob, USDC, 100);
    }

    function testSettle(bool answer, uint256 stake) public {
        // above this, the oracle will reject the request as the math (stake * 25 / 100) in the contract will overflow
        vm.assume(stake < type(uint256).max / 25);
        uint256 bond = oracle.bondForStake(stake);

        deal(USDC, alice, stake);
        deal(USDC, bob, bond);

        // lets ask the oracle about this request
        vm.prank(alice);
        bytes32 id = oracle.ask(alice, bob, USDC, stake);

        uint256 aliceBalanceBefore = ERC20(USDC).balanceOf(alice);
        uint256 bobBalanceBefore = ERC20(USDC).balanceOf(bob);
        // charlie settles the request
        vm.prank(charlie);
        oracle.settle(id, answer);

        // Check the request is now settled
        (, , , , , bool _answer, RequestState _state) = oracle.requests(id);

        assertEq(uint256(_state), uint256(RequestState.Settled));
        assertEq(_answer, answer);
        if (answer == true) {
            // The request was settled as true, so the bond + stake should be returned to the proposer
            assertEq(
                ERC20(USDC).balanceOf(alice),
                aliceBalanceBefore + stake + bond
            );
            // no changes for the disputer.
            assertEq(ERC20(USDC).balanceOf(bob), bobBalanceBefore);
        } else {
            // The request was settled as false, so the bond + stake should go to the disputer
            assertEq(ERC20(USDC).balanceOf(alice), aliceBalanceBefore);
            assertEq(
                ERC20(USDC).balanceOf(bob),
                bobBalanceBefore + stake + bond
            );
        }
    }

    function testCannotSettleAsIfNotSettler() public {
        vm.expectRevert(AllKnowingOracle__NonSettler.selector);
        vm.prank(alice);
        _settle(bytes32(0), true);
    }

    function testCannotSettleIfAlreadySettled() public {
        bool answer = true;
        uint256 stake = 100;
        uint256 bond = oracle.bondForStake(stake);
        deal(USDC, alice, stake);
        deal(USDC, bob, bond);
        vm.prank(alice);
        bytes32 id = _ask(alice, bob, USDC, stake);
        vm.prank(oracle.owner());
        _settle(id, answer);
        vm.prank(oracle.owner());
        vm.expectRevert(
            abi.encodeWithSelector(
                AllKnowingOracle__AlreadySettled.selector,
                id
            )
        );
        _settle(id, answer);
    }
}
