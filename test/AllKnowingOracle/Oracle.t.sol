// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "src/AllKnowingOracle.sol";
import "./Fixtures.sol";
import "forge-std/Test.sol";

contract AllKnowingOracleTest is Test, IAllKnowingOracleEvents, OracleFixture {
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
        bytes32 id = keccak256(abi.encode(alice, bob, USDC, bond, stake));
        // Give alice & bob some tokens
        deal(USDC, alice, stake);
        deal(USDC, bob, bond);

        uint256 aliceBalanceBefore = ERC20(USDC).balanceOf(alice);
        uint256 bobBalanceBefore = ERC20(USDC).balanceOf(bob);
        vm.prank(alice);
        vm.expectEmit(true, true, true, true, address(oracle));
        emit NewRequest(id, alice, bob, USDC, bond, stake);
        bytes32 realId = oracle.ask(alice, bob, USDC, bond, stake);
        assertEq(id, realId);

        // Check that the request is in the oracle's list of pending requests
        (
            address _proposer,
            address _disputer,
            address _bondToken,
            uint256 _bond,
            uint256 _stake,
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

    function testAskForOtherProposer(address proposer) public {
        uint256 stake = 100;
        uint256 bond = 25;
        bytes32 id = keccak256(abi.encode(proposer, bob, USDC, bond, stake));
        // Give alice & bob some tokens
        deal(USDC, alice, stake);
        deal(USDC, bob, bond);

        uint256 aliceBalanceBefore = ERC20(USDC).balanceOf(alice);
        vm.prank(alice);
        vm.expectEmit(true, true, true, true, address(oracle));
        emit NewRequest(id, proposer, bob, USDC, bond, stake);
        _ask(proposer, bob, USDC, 25, 100);

        // Check that the request is in the oracle's list of pending requests
        (
            address _proposer,
            address _disputer,
            address _bondToken,
            uint256 _bond,
            uint256 _stake,
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
        oracle.ask(alice, bob, bondToken, 25, 100);
    }

    function testCannotAskWithInsufficientBond() public {
        vm.expectRevert(AllKnowingOracle__BondTooSmall.selector);
        vm.prank(alice);
        oracle.ask(alice, bob, USDC, 24, 100);
    }

    function testCannotAskIfNoAllowance() public {
        // Alice removes the allowance for USDC
        vm.prank(alice);
        ERC20(USDC).approve(address(oracle), 0);
        vm.expectRevert("TRANSFER_FROM_FAILED");
        vm.prank(alice);
        oracle.ask(alice, bob, USDC, 25, 100);
    }

    function testSettle(bool answer, uint256 stake) public {
        // above this, the oracle will reject the request as the math (stake * 25 / 100) in the contract will overflow
        vm.assume(stake < type(uint256).max / 25);
        uint256 bond = oracle.bondForStake(stake);

        deal(USDC, alice, stake);
        deal(USDC, bob, bond);

        // lets ask the oracle about this request
        vm.prank(alice);
        bytes32 id = oracle.ask(alice, bob, USDC, bond, stake);

        uint256 aliceBalanceBefore = ERC20(USDC).balanceOf(alice);
        uint256 bobBalanceBefore = ERC20(USDC).balanceOf(bob);

        // grab the owner of the oracle, as its the only one that can settle
        address owner = oracle.owner();
        vm.prank(owner);
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

    function testCannotSettleAsNonOwner() public {
        vm.expectRevert("UNAUTHORIZED");
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
        bytes32 id = _ask(alice, bob, USDC, bond, stake);
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
