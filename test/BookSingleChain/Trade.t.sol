// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./Fixtures.sol";

contract TradeTest is TradeFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testRequestTrade(
        uint256 amountIn,
        uint256 amountOutMin,
        uint256 feePct
    ) public {
        // We assume trade fields are valid, since we have separate tests for those.
        vm.assume(amountIn > 0);
        vm.assume(amountOutMin > 0);
        vm.assume(feePct <= MAX_FEE_PCT);

        uint256 tradeIndex = book.numberOfTrades();
        // Give alice some tokens to trade.
        deal(testTokenIn, alice, amountIn);
        // Request a trade from Alice.
        uint256 balanceBefore = ERC20(testTokenIn).balanceOf(alice);

        vm.expectEmit(true, true, true, true, address(book));
        emit TradeRequested(
            testTokenIn,
            testTokenOut,
            amountIn,
            amountOutMin,
            feePct,
            testRecipient,
            tradeIndex
        );
        _requestTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            amountOutMin,
            feePct,
            testRecipient,
            alice
        );

        // Check that the balance of Alice of `tokenIn` is reduced by `amount`.
        assertEq(ERC20(testTokenIn).balanceOf(alice), balanceBefore - amountIn);
    }

    function testCannotTradeIfNoBalance() public {
        vm.expectRevert(bytes("TRANSFER_FROM_FAILED"));
        vm.prank(alice);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient
        );
    }

    function testCannotTradeNonWhitelistedToken(address token) public {
        // check that the random token is not whitelisted
        vm.assume(!book.whitelistedTokens(token));
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__InvalidToken.selector,
                token
            )
        );
        book.requestTrade(
            token,
            testTokenOut,
            testFeePct,
            testAmountIn,
            testAmountOutMin,
            testRecipient
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__InvalidToken.selector,
                token
            )
        );
        book.requestTrade(
            testTokenIn,
            token,
            testFeePct,
            testAmountIn,
            testAmountOutMin,
            testRecipient
        );
    }

    function testCannotTradeSameToken() public {
        vm.expectRevert(BookSingleChain__SameToken.selector);
        book.requestTrade(
            USDC,
            USDC,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient
        );
    }

    function testCannotTradeZeroAmount() public {
        vm.expectRevert(BookSingleChain__ZeroAmount.selector);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            0,
            testAmountOutMin,
            testFeePct,
            testRecipient
        );

        vm.expectRevert(BookSingleChain__ZeroAmount.selector);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            0,
            testFeePct,
            testRecipient
        );
    }

    function testCannotTradeAboveMaxFee() public {
        uint256 maxFeePct = MAX_FEE_PCT + 1;
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__FeePctTooHigh.selector,
                maxFeePct
            )
        );
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            maxFeePct,
            testRecipient
        );
    }

    function testCannotTradeToBlackHole() public {
        address blackHole = address(0);

        vm.expectRevert(BookSingleChain__SentToBlackHole.selector);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            blackHole
        );
    }

    function testUpdateFee(uint256 newFeePct) public {
        vm.assume(newFeePct <= MAX_FEE_PCT);
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );

        vm.expectEmit(true, true, false, true, address(book));
        emit UpdatedFeeForTrade(alice, tradeIndex, newFeePct);

        book.updateFeeForTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            alice,
            newFeePct,
            aliceSignature
        );
    }

    function testCannotUpdateFeeWithInvalidSignature() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        uint256 newFeePct = testFeePct + 1;
        // Sign a fee update with Bob's private key.
        bytes memory bobSignature = _signFeeUpdate(BOB_PK, tradeId, newFeePct);
        vm.expectRevert(BookSingleChain__InvalidSignature.selector);
        book.updateFeeForTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            alice,
            newFeePct,
            bobSignature
        );
    }

    function testCannotUpdateFeePastMax() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        uint256 newFeePct = MAX_FEE_PCT + 1;
        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__FeePctTooHigh.selector,
                newFeePct
            )
        );
        book.updateFeeForTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            alice,
            newFeePct,
            aliceSignature
        );
    }

    function testCannotUpdateFeeForFilledTrade() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        // Artificially fill the trade at the past block.
        stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .checked_write(block.number);

        uint256 newFeePct = testFeePct + 1;

        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeAlreadyFilled.selector,
                tradeId
            )
        );
        book.updateFeeForTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            alice,
            newFeePct,
            aliceSignature
        );
    }

    function testFillTrade(uint256 amountIn, uint256 amountOut) public {
        vm.assume(amountIn > 0);
        vm.assume(amountOut > testAmountOutMin);
        vm.assume(amountIn < type(uint256).max / testRelayerRefundPct - 1);

        deal(testTokenIn, alice, amountIn);
        (uint256 tradeIndex, bytes32 tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            alice
        );

        deal(testTokenOut, bob, amountOut);
        uint256 bobBalanceOutBefore = ERC20(testTokenOut).balanceOf(bob);
        uint256 bobBalanceInBefore = ERC20(testTokenIn).balanceOf(bob);
        uint256 recipientBalanceBefore = ERC20(testTokenOut).balanceOf(
            testRecipient
        );

        vm.prank(bob);
        vm.expectEmit(true, true, true, true, address(book));
        emit TradeFilled(bob, tradeIndex, testFeePct, amountOut);
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            amountOut
        );
        // Check bob submitted amountOut tokens
        assertEq(
            ERC20(testTokenOut).balanceOf(bob) + amountOut,
            bobBalanceOutBefore,
            "bob should have sent amountOut tokens"
        );
        // Check bob got relayerRefundPct * amountIn tokens
        uint256 bobExpectedTokens = (amountIn * testRelayerRefundPct) / 100;

        assertEq(
            ERC20(testTokenIn).balanceOf(bob),
            bobBalanceInBefore + bobExpectedTokens,
            "bob should have received some tokens"
        );
        // Check the recipient received amountOut tokens
        assertEq(
            ERC20(testTokenOut).balanceOf(testRecipient),
            recipientBalanceBefore + amountOut,
            "recipient should have received amountOut tokens"
        );

        uint256 filledAtInStorage = stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .read_uint();
        assertEq(filledAtInStorage, block.number);

        address filledByInStorage = stdstore
            .target(address(book))
            .sig(book.filledBy.selector)
            .with_key(tradeId)
            .read_address();
        assertEq(filledByInStorage, bob);
    }

    function testCannotFillIfAlreadyFilled() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        // Artificially fill&dispute the trade at the past block.
        stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .checked_write(block.number);
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeAlreadyFilled.selector,
                tradeId
            )
        );
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            testAmountOutMin + 1
        );
    }

    function testCannotFillIfDisputed() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        // Artificially fill&dispute the trade at the past block.
        stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .checked_write(uint256(-int256(block.number)));

        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeNotInFilledState.selector,
                tradeId
            )
        );
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            testAmountOutMin + 1
        );
    }

    function testCannotFillIfNoTokens(uint256 amountOut) public {
        vm.assume(amountOut > 0);
        vm.prank(bob);
        vm.expectRevert(bytes("TRANSFER_FROM_FAILED"));
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            0,
            testFeePct,
            testRecipient,
            1,
            amountOut
        );
    }

    function testCannotFillIfAmountOutIsLessThanMin(uint256 minAmountOut)
        public
    {
        vm.assume(minAmountOut > 1);
        uint256 amountOut = minAmountOut - 1;
        vm.prank(bob);
        vm.expectRevert(BookSingleChain__AmountOutTooLow.selector);
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            minAmountOut,
            testFeePct,
            testRecipient,
            1,
            amountOut
        );
    }

    function testFillTradeWithUpdatedFee(uint256 amountIn, uint256 amountOut)
        public
    {
        vm.assume(amountIn > 0);
        vm.assume(amountIn < type(uint256).max / testRelayerRefundPct - 1);
        vm.assume(amountOut > testAmountOutMin);

        deal(testTokenIn, alice, amountIn);
        (uint256 tradeIndex, bytes32 tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            alice
        );

        // Sign a message from Alice updating the fee.
        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            testFeePct + 1
        );
        deal(testTokenOut, bob, amountOut);
        uint256 bobBalanceOutBefore = ERC20(testTokenOut).balanceOf(bob);
        uint256 bobBalanceInBefore = ERC20(testTokenIn).balanceOf(bob);
        uint256 recipientBalanceBefore = ERC20(testTokenOut).balanceOf(
            testRecipient
        );
        vm.prank(bob);
        vm.expectEmit(true, true, true, true, address(book));
        // check the fee is updated
        emit TradeFilled(bob, tradeIndex, testFeePct + 1, amountOut);
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            amountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            amountOut,
            alice,
            testFeePct + 1,
            aliceSignature
        );
        // Check bob submitted amountOut tokens
        assertEq(
            ERC20(testTokenOut).balanceOf(bob) + amountOut,
            bobBalanceOutBefore,
            "bob should have sent amountOut tokens"
        );
        // Check bob got 1 - relayerPenatlyPct amountIn tokens
        uint256 bobExpectedTokens = (amountIn * testRelayerRefundPct) / 100;

        assertEq(
            ERC20(testTokenIn).balanceOf(bob),
            bobBalanceInBefore + bobExpectedTokens,
            "bob should have received some tokens"
        );
        // Check the recipient received amountOut tokens
        assertEq(
            ERC20(testTokenOut).balanceOf(testRecipient),
            recipientBalanceBefore + amountOut,
            "recipient should have received amountOut tokens"
        );

        uint256 filledAtInStorage = stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .read_uint();
        assertEq(filledAtInStorage, block.number);

        address filledByInStorage = stdstore
            .target(address(book))
            .sig(book.filledBy.selector)
            .with_key(tradeId)
            .read_address();
        assertEq(filledByInStorage, bob);
    }

    function testCannotFillWithUpdateFeeForFilledTrade() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        // Artificially fill the trade at the past block.
        stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .checked_write(block.number);

        // Prepare a message to updates the fee.
        uint256 newFeePct = testFeePct + 1;
        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeAlreadyFilled.selector,
                tradeId
            )
        );
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            1,
            alice,
            newFeePct,
            aliceSignature
        );
    }

    function testCannotFillWithUpdateFeeForDisputedTrade() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        // Artificially fill the trade at the past block.
        stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .checked_write(uint256(-int256(block.number)));

        // Prepare a message to updates the fee.
        uint256 newFeePct = testFeePct + 1;
        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeNotInFilledState.selector,
                tradeId
            )
        );
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            1,
            alice,
            newFeePct,
            aliceSignature
        );
    }

    function testCannotFillWithUpdateFeePastMax() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = keccak256(
            abi.encode(
                testTokenIn,
                testTokenOut,
                testAmountIn,
                testAmountOutMin,
                testFeePct,
                testRecipient,
                tradeIndex
            )
        );

        uint256 newFeePct = MAX_FEE_PCT + 1;

        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );

        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__FeePctTooHigh.selector,
                newFeePct
            )
        );
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            1,
            alice,
            newFeePct,
            aliceSignature
        );
    }

    function testCannotFillTradeWithUpdateFeeWithInvalidSignature() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        uint256 newFeePct = testFeePct + 1;
        // Bob signs the feeUpdate message for Alice trade
        bytes memory bobSignature = _signFeeUpdate(BOB_PK, tradeId, newFeePct);
        vm.expectRevert(BookSingleChain__InvalidSignature.selector);
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex,
            1,
            alice,
            newFeePct,
            bobSignature
        );
    }
}
