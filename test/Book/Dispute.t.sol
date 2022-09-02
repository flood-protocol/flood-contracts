// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "./Fixtures.sol";

contract DisputeTest is DisputeFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testDispute() public {
        uint256 bond = (testDisputeBondPct * testAmountIn) / 100;
        deal(testTokenIn, disputer, bond);

        uint256 bookBalanceBefore = ERC20(testTokenIn).balanceOf(address(book));
        uint256 disputerBalanceBefore = ERC20(testTokenIn).balanceOf(disputer);
        int256 filledAtBeforeDispute = stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .read_int();

        // check that the request was received by the oracle
        bytes32 reqId = keccak256(
            abi.encodePacked(
                address(book),
                relayer,
                disputer,
                testTokenIn,
                bond
            )
        );

        vm.expectEmit(true, true, true, true, address(book));
        emit TradeDisputed(
            relayer,
            tradeIndex,
            reqId,
            uint256(filledAtBeforeDispute)
        );
        _disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            tradeIndex
        );

        // check that trade variables have been reset
        {
            int256 filledAtAfterDispute = stdstore
                .target(address(book))
                .sig(book.filledAtBlock.selector)
                .with_key(tradeId)
                .read_int();

            address filledByAfterDispute = stdstore
                .target(address(book))
                .sig(book.filledBy.selector)
                .with_key(tradeId)
                .read_address();

            bool isInitializedAfterDispute = stdstore
                .target(address(book))
                .sig(book.isInitialized.selector)
                .with_key(tradeId)
                .read_bool();

            assertEq(filledByAfterDispute, address(0));
            assertEq(filledAtAfterDispute, 0);
            assertEq(isInitializedAfterDispute, false);
        }

        (
            address _reqRequester,
            address _reqProposer,
            address _reqDisputer,
            ERC20 _reqCurrency,
            uint256 _reqBond,
            RequestState _reqState,
            bool _reqAnswer,
            bytes memory _reqData
        ) = oracle.requests(reqId);

        assertEq(_reqRequester, address(book), "Requester should be book");
        assertEq(_reqProposer, relayer, "Proposer should equal relayer");
        assertEq(
            _reqDisputer,
            disputer,
            "Request Disputer should equal disputer"
        );
        assertEq(
            address(_reqCurrency),
            testTokenIn,
            "Request currency should equal tokenIn"
        );
        assertEq(_reqBond, bond, "request bond should equal bond");
        assertEq(_reqAnswer, false, "Answer should be false");
        assertEq(
            uint256(_reqState),
            uint256(RequestState.Pending),
            "State should be Pending"
        );
        assertEq(
            _reqData,
            abi.encode(testAmountIn, testRecipient, tradeIndex),
            "Request Data should equal request data"
        );

        // Check that the tokens have been pulled from the book and the disputer
        assertEq(
            ERC20(testTokenIn).balanceOf(address(book)),
            bookBalanceBefore - bond,
            "Book should have sponsored the proposer bond"
        );
        assertEq(
            ERC20(testTokenIn).balanceOf(disputer),
            disputerBalanceBefore - bond,
            "Disputer should have posted the bond"
        );
    }

    function testOnPriceSettledCallback(bool answer) public {
        uint256 bond = (testDisputeBondPct * testAmountIn) / 100;
        deal(testTokenIn, disputer, bond);

        uint256 bookBalanceBeforeDispute = ERC20(testTokenIn).balanceOf(
            address(book)
        );
        uint256 disputerBalanceBeforeDispute = ERC20(testTokenIn).balanceOf(
            disputer
        );
        uint256 relayerBalanceBeforeDispute = ERC20(testTokenIn).balanceOf(
            relayer
        );
        uint256 recipientBalanceBeforeDispute = ERC20(testTokenIn).balanceOf(
            testRecipient
        );

        bytes32 reqId = keccak256(
            abi.encodePacked(
                address(book),
                relayer,
                disputer,
                testTokenIn,
                bond
            )
        );

        _disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            tradeIndex
        );

        oracle.settle(reqId, answer);

        uint256 rebate = (testTradeRebatePct * testAmountIn) / 100;
        assertEq(
            ERC20(testTokenIn).balanceOf(address(book)),
            bookBalanceBeforeDispute - bond - rebate,
            "Book should no tokenIn balance left"
        );
        // Dispute was wrong
        if (answer) {
            assertEq(
                ERC20(testTokenIn).balanceOf(disputer),
                disputerBalanceBeforeDispute - bond,
                "Disputer should have lost the bond"
            );
            assertEq(
                ERC20(testTokenIn).balanceOf(relayer),
                relayerBalanceBeforeDispute + rebate + 2 * bond,
                "Relayer should have received the rebate, his bond back and the disputer bond."
            );
            assertEq(
                ERC20(testTokenIn).balanceOf(testRecipient),
                recipientBalanceBeforeDispute,
                "Recipient should have received no tokens"
            ); // Disputer was right
        } else {
            assertEq(
                ERC20(testTokenIn).balanceOf(disputer),
                disputerBalanceBeforeDispute + bond,
                "Disputer should have received its bond + the relayer bond"
            );
            assertEq(
                ERC20(testTokenIn).balanceOf(relayer),
                relayerBalanceBeforeDispute,
                "Relayer should have received tokens"
            );
            assertEq(
                ERC20(testTokenIn).balanceOf(testRecipient),
                recipientBalanceBeforeDispute + rebate,
                "Recipient should have received the rebate"
            );
        }
    }

    function testCannotCallOnPriceSettledIfNotOracle(address caller) public {
        vm.assume(caller != address(oracle));
        Request memory fakeRequest = Request({
            requester: address(book),
            proposer: relayer,
            disputer: disputer,
            currency: ERC20(USDC),
            bond: 100,
            state: RequestState.Settled,
            answer: true,
            data: abi.encode(testAmountIn, testRecipient, tradeIndex)
        });
        vm.prank(caller);
        vm.expectRevert(
            abi.encodeWithSelector(Book__MaliciousCaller.selector, caller)
        );
        book.onPriceSettled(keccak256("id"), fakeRequest);
    }

    function testCannotDisputeTwice() public {
        uint256 bond = (testDisputeBondPct * testAmountIn) / 100;

        deal(testTokenIn, disputer, bond);
        vm.prank(disputer);
        book.disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            tradeIndex
        );
        address nextDisputer = generateUser("nextDisputer");
        deal(testTokenIn, nextDisputer, bond);
        vm.prank(nextDisputer);
        ERC20(testTokenIn).approve(address(oracle), type(uint256).max);
        vm.expectRevert(
            abi.encodeWithSelector(Book__TradeNotFilled.selector, tradeId)
        );
        book.disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            tradeIndex
        );
    }

    function testCannotDisputeIfPeriodIsOver() public {
        skipBlocks(testSafeBlockThreashold + 1);
        vm.expectRevert(Book__DisputePeriodOver.selector);
        _disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            tradeIndex
        );
    }

    function testCannotDisputeIfNotFilled() public {
        bytes32 nonExistentTradeId = keccak256(
            abi.encodePacked(
                testTokenIn,
                testTokenOut,
                testAmountIn + 1,
                testAmountOutMin,
                testRecipient,
                tradeIndex
            )
        );

        vm.expectRevert(
            abi.encodeWithSelector(
                Book__TradeNotFilled.selector,
                nonExistentTradeId
            )
        );
        // dispute a trade which was never filled
        book.disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn + 1,
            testAmountOutMin,
            testRecipient,
            tradeIndex
        );
    }
}
