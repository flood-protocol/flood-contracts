// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "./Fixtures.sol";

contract DisputeTest is TradeFixture {
    using stdStorage for StdStorage;

    uint256 internal tradeIndex;
    bytes32 internal tradeId;
    address internal relayer = bob;
    address internal disputer = charlie;
    uint256 internal testAmountToSend = 2000 * 1e6;

    function setUp() public override {
        super.setUp();
        oracle.whitelistRequester(address(book), true);
        deal(testTokenIn, alice, testAmountIn);

        (tradeIndex, tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            alice
        );

        deal(testTokenOut, relayer, testAmountToSend);
        vm.prank(relayer);
        _fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            testAmountToSend
        );
        _checkFill(tradeId, relayer, int256(block.number));

        vm.prank(disputer);
        ERC20(testTokenIn).approve(address(oracle), type(uint256).max);
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

        vm.prank(disputer);
        vm.expectEmit(true, true, true, true, address(book));
        emit TradeDisputed(relayer, tradeIndex, reqId);
        book.disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        int256 filledAtAfterDispute = stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .read_int();

        assertEq(filledAtAfterDispute, -filledAtBeforeDispute);

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

        vm.prank(disputer);
        book.disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
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
            );
            // Disputer was right
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
            abi.encodeWithSelector(
                BookSingleChain__MaliciousCaller.selector,
                caller
            )
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
            testFeePct,
            testRecipient,
            tradeIndex
        );
        address nextDisputer = generateUser("nextDisputer");
        deal(testTokenIn, nextDisputer, bond);
        vm.prank(nextDisputer);
        ERC20(testTokenIn).approve(address(oracle), type(uint256).max);
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeNotInFilledState.selector,
                tradeId
            )
        );
        book.disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );
    }

    function testCannotDisputeIfPeriodIsOver() public {
        skipBlocks(testSafeBlockThreashold + 1);
        vm.expectRevert(BookSingleChain__DisputePeriodOver.selector);
        book.disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
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
                testFeePct,
                testRecipient,
                tradeIndex
            )
        );

        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeNotInFilledState.selector,
                nonExistentTradeId
            )
        );
        // dispute a trade which was never filled
        book.disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn + 1,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );
    }
}
