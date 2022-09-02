// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./Fixtures.sol";

contract TradeTest is TradeFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testRequestTrade(uint256 amountIn, uint256 amountOutMin) public {
        // We assume trade fields are valid, since we have separate tests for those.
        vm.assume(amountIn > 0);
        vm.assume(amountOutMin > 0);

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
            testRecipient,
            tradeIndex
        );
        (, bytes32 id) = _requestTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            amountOutMin,
            testRecipient,
            alice
        );

        // Check that the balance of Alice of `tokenIn` is reduced by `amount`.
        assertEq(ERC20(testTokenIn).balanceOf(alice), balanceBefore - amountIn);

        // Check that the trade has been initialized
        bool isInitializedInStorage = stdstore
            .target(address(book))
            .sig(book.isInitialized.selector)
            .with_key(id)
            .read_bool();
        assertTrue(isInitializedInStorage, "Trade not initialized");

        // Check that the trade number has been increased
        uint256 numberOfTradesInStorage = stdstore
            .target(address(book))
            .sig(book.numberOfTrades.selector)
            .read_uint();
        assertEq(numberOfTradesInStorage, tradeIndex + 1);
    }

    function testCannotTradeIfNoBalance() public {
        vm.expectRevert(bytes("TRANSFER_FROM_FAILED"));
        vm.prank(alice);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient
        );
    }

    function testCannotTradeNonWhitelistedToken(address token) public {
        // check that the random token is not whitelisted
        vm.assume(!book.whitelistedTokens(token));
        vm.expectRevert(
            abi.encodeWithSelector(Book__InvalidToken.selector, token)
        );
        book.requestTrade(
            token,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient
        );
        vm.expectRevert(
            abi.encodeWithSelector(Book__InvalidToken.selector, token)
        );
        book.requestTrade(
            testTokenIn,
            token,
            testAmountIn,
            testAmountOutMin,
            testRecipient
        );
    }

    function testCannotTradeSameToken() public {
        vm.expectRevert(Book__SameToken.selector);
        book.requestTrade(
            USDC,
            USDC,
            testAmountIn,
            testAmountOutMin,
            testRecipient
        );
    }

    function testCannotTradeZeroAmount() public {
        vm.expectRevert(Book__ZeroAmount.selector);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            0,
            testAmountOutMin,
            testRecipient
        );

        vm.expectRevert(Book__ZeroAmount.selector);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            0,
            testRecipient
        );
    }

    function testCannotTradeToBlackHole() public {
        address blackHole = address(0);

        vm.expectRevert(Book__SentToBlackHole.selector);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            blackHole
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
        emit TradeFilled(bob, tradeIndex, amountOut);
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testAmountOutMin,
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

    function testCannotFillIfNotRequested() public {
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            1
        );
        // This should fail as the trade has not been requested.
        vm.expectRevert(
            abi.encodeWithSelector(
                Book__TradeNotInFillableState.selector,
                tradeId
            )
        );
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            1,
            1
        );
    }

    function testCannotFillIfAlreadyFilled() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
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
                Book__TradeNotInFillableState.selector,
                tradeId
            )
        );
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
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
            testRecipient,
            tradeIndex
        );
        stdstore
            .target(address(book))
            .sig(book.isInitialized.selector)
            .with_key(tradeId)
            .checked_write(true);

        // Artificially fill&dispute the trade at the past block.

        stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .checked_write(uint256(-int256(block.number)));

        vm.expectRevert(
            abi.encodeWithSelector(
                Book__TradeNotInFillableState.selector,
                tradeId
            )
        );
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            tradeIndex,
            testAmountOutMin + 1
        );
    }

    function testCannotFillIfNoTokens(uint256 amountOut) public {
        vm.assume(amountOut > 0);
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            0,
            testRecipient,
            1
        );
        // simulate a request
        stdstore
            .target(address(book))
            .sig(book.isInitialized.selector)
            .with_key(tradeId)
            .checked_write(true);
        vm.prank(bob);
        vm.expectRevert(bytes("TRANSFER_FROM_FAILED"));
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            0,
            testRecipient,
            1,
            amountOut
        );
    }

    function testCannotFillIfAmountOutIsLessThanMin(uint256 minAmountOut)
        public
    {
        vm.assume(minAmountOut > 1);
        bytes32 tradeId = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            minAmountOut,
            testRecipient,
            1
        );
        // simulate a request
        stdstore
            .target(address(book))
            .sig(book.isInitialized.selector)
            .with_key(tradeId)
            .checked_write(true);
        uint256 amountOut = minAmountOut - 1;
        vm.prank(bob);
        vm.expectRevert(Book__AmountOutTooLow.selector);
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            minAmountOut,
            testRecipient,
            1,
            amountOut
        );
    }
}
