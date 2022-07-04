// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "./Fixtures.sol";

contract DisputeTest is TradeFixture {
    using stdStorage for StdStorage;

    uint256 internal tradeIndex;
    address internal relayer = bob;
    address internal disputer = charlie;
    uint256 internal testAmountToSend = 2000 * 1e6;

    function setUp() public override {
        super.setUp();
        deal(testTokenIn, alice, testAmount);
        uint256 _tradeIndex = _requestTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            alice
        );
        tradeIndex = _tradeIndex;
        tradeIndex = _tradeIndex;

        deal(testTokenOut, relayer, testAmountToSend);
        vm.prank(relayer);
        _fillTrade(testTokenOut, testFeePct, tradeIndex, testAmountToSend);
    }

    function testDispute() public {
        // Lets check the trade was filled correctly and storage variables are set
        uint256 filledAmountInStorageBefore = stdstore
            .target(address(book))
            .sig(book.filledTrades.selector)
            .with_key(tradeIndex)
            .depth(1)
            .read_uint();

        assertEq(
            filledAmountInStorageBefore,
            testAmountToSend,
            "Filled amount should be equal to the amount sent"
        );
        address filledByInStorageBefore = stdstore
            .target(address(book))
            .sig(book.filledTrades.selector)
            .with_key(tradeIndex)
            .depth(0)
            .read_address();

        assertEq(
            filledByInStorageBefore,
            relayer,
            "Filled by should be equal to the relayer"
        );

        uint256 filleAtBlockInStorageBefore = stdstore
            .target(address(book))
            .sig(book.filledTrades.selector)
            .with_key(tradeIndex)
            .depth(2)
            .read_uint();

        assertEq(
            filleAtBlockInStorageBefore,
            block.number,
            "Filled at block should be equal to the current block number"
        );

        uint256 bond = oracle.bondForStake(testAmountToSend);
        deal(testTokenOut, disputer, bond);

        uint256 bookBalanceBefore = ERC20(testTokenOut).balanceOf(
            address(book)
        );
        uint256 disputerBalanceBefore = ERC20(testTokenOut).balanceOf(disputer);

        // check that the request was received by the oracle
        bytes32 reqId = keccak256(
            abi.encode(relayer, disputer, testTokenOut, testAmountToSend, bond)
        );

        vm.prank(disputer);
        ERC20(testTokenOut).approve(address(oracle), type(uint256).max);
        vm.prank(disputer);
        vm.expectEmit(true, true, true, true, address(book));
        emit TradeDisputed(
            relayer,
            tradeIndex,
            reqId,
            testAmountToSend,
            testFeePct
        );
        book.disputeTrade(testTokenOut, testFeePct, tradeIndex);

        (
            address _reqProposer,
            address _reqDisputer,
            address _reqBondToken,
            uint256 _stake,
            uint256 _bond,
            bool _answer,
            RequestState _state
        ) = oracle.requests(reqId);

        assertEq(
            _reqProposer,
            relayer,
            "Proposer should be equal to the relayer"
        );
        assertEq(
            _reqDisputer,
            disputer,
            "Disputer should be equal to the disputer"
        );
        assertEq(
            _reqBondToken,
            testTokenOut,
            "Bond token should be equal to the test token out"
        );
        assertEq(
            _stake,
            testAmountToSend,
            "Stake should be equal to the amount sent"
        );
        assertEq(_bond, bond, "Bond should be equal to the bond");
        assertEq(_answer, false, "Answer should be equal to false");
        assertEq(
            uint256(_state),
            uint256(RequestState.Pending),
            "State should be equal to Pending"
        );

        // Check that the tokens have been pulled from the book and the disputer
        assertEq(
            ERC20(testTokenOut).balanceOf(address(book)),
            bookBalanceBefore - testAmountToSend,
            "Book should have sponsored the proposal"
        );
        assertEq(
            ERC20(testTokenOut).balanceOf(disputer),
            disputerBalanceBefore - bond,
            "Disputer should have posted the bond"
        );

        // check that the storage variables have been unset
        uint256 filledAmountInStorageAfter = stdstore
            .target(address(book))
            .sig(book.filledTrades.selector)
            .with_key(tradeIndex)
            .depth(1)
            .read_uint();

        assertEq(
            filledAmountInStorageAfter,
            0,
            "Filled amount should be unset"
        );

        address filledByInStorageAfter = stdstore
            .target(address(book))
            .sig(book.filledTrades.selector)
            .with_key(tradeIndex)
            .depth(0)
            .read_address();

        assertEq(
            filledByInStorageAfter,
            address(0),
            "Filled by should be unset"
        );

        uint256 filleAtBlockInStorageAfter = stdstore
            .target(address(book))
            .sig(book.filledTrades.selector)
            .with_key(tradeIndex)
            .depth(2)
            .read_uint();

        assertEq(
            filleAtBlockInStorageAfter,
            0,
            "Filled at block should be unset"
        );
    }

    function testCannotDisputeIfPeriodIsOver() public {
        skipBlocks(testSafeBlockThreashold + 1);
        vm.expectRevert(BookSingleChain__DisputePeriodOver.selector);
        book.disputeTrade(testTokenOut, testFeePct, tradeIndex);
    }

    function testCannotDisputeIfNotFilled() public {
        uint256 nonExistingTradeIndex = type(uint256).max - 1;
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeNotFilled.selector,
                nonExistingTradeIndex
            )
        );
        // dispute a trade which was never filled
        book.disputeTrade(testTokenOut, testFeePct, nonExistingTradeIndex);
    }
}
