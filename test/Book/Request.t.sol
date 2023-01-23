// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {
    Book__SameToken,
    Book__ZeroAmount,
    Book__SentToBlackHole,
    Book__InvalidToken,
    Book__TradeNotInFillableState,
    Book__NotTrader,
    Book__TradeNotCancelable,
    Book__AmountOutTooLow,
    Book__NotWeth,
    Book__InvalidValue,
    TradeStatus
} from "src/Book.sol";
import {TradeFixture} from "./Fixtures.sol";

contract RequestTest is TradeFixture {
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
        uint256 balanceBefore = IERC20(testTokenIn).balanceOf(alice);

        vm.expectEmit(true, true, true, true, address(book));
        emit TradeRequested(testTokenIn, testTokenOut, amountIn, amountOutMin, testRecipient, tradeIndex, alice);
        (, bytes32 id) = _requestTrade(testTokenIn, testTokenOut, amountIn, amountOutMin, testRecipient, alice, false);

        // Check that the balance of Alice of `tokenIn` is reduced by `amount`.
        assertEq(IERC20(testTokenIn).balanceOf(alice), balanceBefore - amountIn);

        // Check that the trade has been initialized
        uint256 statusInStorage = stdstore.target(address(book)).sig(book.status.selector).with_key(id).read_uint();
        assertEq(statusInStorage, uint256(TradeStatus.REQUESTED), "Trade not initialized");

        // Check that the trade number has been increased
        uint256 numberOfTradesInStorage = stdstore.target(address(book)).sig(book.numberOfTrades.selector).read_uint();
        assertEq(numberOfTradesInStorage, tradeIndex + 1);
    }

    function testCannotTradeIfNoBalance() public {
        vm.expectRevert(bytes("ERC20: transfer amount exceeds balance"));
        vm.prank(alice);
        book.requestTrade(testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, false);
    }

    function testCannotTradeNonWhitelistedToken(address token) public {
        // check that the random token is not whitelisted
        vm.assume(token != testTokenIn);
        vm.assume(token != testTokenOut);
        vm.assume(!registry.isTokenWhitelisted(token));
        vm.expectRevert(abi.encodeWithSelector(Book__InvalidToken.selector, token));
        book.requestTrade(token, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, false);
        vm.expectRevert(abi.encodeWithSelector(Book__InvalidToken.selector, token));
        book.requestTrade(testTokenIn, token, testAmountIn, testAmountOutMin, testRecipient, false);
    }

    function testCannotTradeSameToken() public {
        vm.expectRevert(Book__SameToken.selector);
        book.requestTrade(USDC, USDC, testAmountIn, testAmountOutMin, testRecipient, false);
    }

    function testCannotTradeZeroAmount() public {
        vm.expectRevert(Book__ZeroAmount.selector);
        book.requestTrade(testTokenIn, testTokenOut, 0, testAmountOutMin, testRecipient, false);

        vm.expectRevert(Book__ZeroAmount.selector);
        book.requestTrade(testTokenIn, testTokenOut, testAmountIn, 0, testRecipient, false);
    }

    function testCannotTradeToBlackHole() public {
        address blackHole = address(0);

        vm.expectRevert(Book__SentToBlackHole.selector);
        book.requestTrade(testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, blackHole, false);
    }

    function testRequestTradeWithUnwrap() public {
        uint256 amountIn = 1e6;
        address tokenIn = USDC;
        address tokenOut = WETH;
        uint256 tradeIndex = book.numberOfTrades();
        // Give alice some tokens to trade.
        deal(tokenIn, alice, amountIn);
        // Request a trade from Alice.
        uint256 balanceBefore = IERC20(tokenIn).balanceOf(alice);

        vm.expectEmit(true, true, true, true, address(book));
        emit TradeRequested(tokenIn, tokenOut, amountIn, testAmountOutMin, testRecipient, tradeIndex, alice);
        (, bytes32 id) = _requestTrade(tokenIn, tokenOut, amountIn, testAmountOutMin, testRecipient, alice, true);

        // Check that the balance of Alice of `tokenIn` is reduced by `amount`.
        assertEq(IERC20(tokenIn).balanceOf(alice), balanceBefore - amountIn);

        // Check that the trade has been initialized
        uint256 statusInStorage = stdstore.target(address(book)).sig(book.status.selector).with_key(id).read_uint();
        assertEq(statusInStorage, uint256(TradeStatus.REQUESTED), "Trade not initialized");

        // Check that the trade number has been increased
        uint256 numberOfTradesInStorage = stdstore.target(address(book)).sig(book.numberOfTrades.selector).read_uint();
        assertEq(numberOfTradesInStorage, tradeIndex + 1);

        // Check that the unwrap preference is set to true
        bool unwrapInStorage = stdstore.target(address(book)).sig(book.unwrapOutput.selector).with_key(id).read_bool();
        assertTrue(unwrapInStorage, "Unwrap preference not set to true");
    }

    function testCannotRequestUnwrapIfNotReceivingWETH() public {
        vm.expectRevert(Book__NotWeth.selector);
        book.requestTrade(WETH, USDC, testAmountIn, testAmountOutMin, testRecipient, true);
    }

    function testRequestTradeWithETH() public {
        uint256 tradeIndex = book.numberOfTrades();
        // Give alice some tokens to trade.
        deal(alice, testAmountIn);
        // Request a trade from Alice.
        uint256 balanceBefore = alice.balance;

        vm.expectEmit(true, true, true, true, address(book));
        emit TradeRequested(testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, tradeIndex, alice);
        bytes32 id =
            _getTradeId(testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, tradeIndex, alice);
        vm.prank(alice);
        book.requestTrade{value: testAmountIn}(
            testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, false
        );

        // Check that the balance of Alice of `tokenIn` is reduced by `amount`.
        assertEq(alice.balance, balanceBefore - testAmountIn);

        // Check that the trade has been initialized
        uint256 statusInStorage = stdstore.target(address(book)).sig(book.status.selector).with_key(id).read_uint();
        assertEq(statusInStorage, uint256(TradeStatus.REQUESTED), "Trade not initialized");

        // Check that the trade number has been increased
        uint256 numberOfTradesInStorage = stdstore.target(address(book)).sig(book.numberOfTrades.selector).read_uint();
        assertEq(numberOfTradesInStorage, tradeIndex + 1);
    }

    function testCannotRequestWithInvalidValue() public {
        deal(alice, testAmountIn);
        vm.expectRevert(Book__InvalidValue.selector);
        vm.prank(alice);
        book.requestTrade{value: testAmountIn - 1}(
            testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, false
        );
    }

    function testCannotRequestETHIfTokenInIsNotWeth() public {
        deal(alice, testAmountIn);
        vm.expectRevert(Book__NotWeth.selector);
        vm.prank(alice);
        book.requestTrade{value: 1}(USDC, WETH, testAmountIn, testAmountOutMin, testRecipient, false);
    }

        function testCancelTrade() public {
        deal(testTokenIn, testTrader, testAmountIn);
        uint256 balanceBefore = IERC20(testTokenIn).balanceOf(testTrader);
        uint256 bookBalanceBefore = IERC20(testTokenIn).balanceOf(address(book));
        // make a request
        (uint256 tradeIndex, bytes32 tradeId) =
            _requestTrade(testTokenIn, testTokenOut, testAmountIn, 1, testRecipient, testTrader, false);

        vm.prank(testTrader);
        vm.expectEmit(true, true, true, true, address(book));
        emit TradeCancelled(tradeIndex, tradeId, testTrader);
        book.cancelTrade(testTokenIn, testTokenOut, testAmountIn, 1, testRecipient, tradeIndex);

        uint256 statusAfter = stdstore.target(address(book)).sig(book.status.selector).with_key(tradeId).read_uint();

        uint256 balanceAfter = IERC20(testTokenIn).balanceOf(testTrader);
        uint256 bookBalanceAfter = IERC20(testTokenIn).balanceOf(address(book));
        assertEq(balanceAfter, balanceBefore, "trader balance should be unchanged");
        assertEq(bookBalanceAfter, bookBalanceBefore, "book balance should be unchanged");
        assertEq(statusAfter, uint256(TradeStatus.UNINITIALIZED));
    }

    function testCannotCancelTradeIfFilled() public {
        deal(testTokenIn, testTrader, testAmountIn);
        // simulate a request
        (uint256 tradeIndex, bytes32 tradeId) =
            _requestTrade(testTokenIn, testTokenOut, testAmountIn, 1, testRecipient, testTrader, false);
        // simulate a fill
        stdstore.target(address(book)).sig(book.status.selector).with_key(tradeId).checked_write(
            uint256(TradeStatus.FILLED)
        );
        vm.expectRevert(abi.encodeWithSelector(Book__TradeNotCancelable.selector, tradeId));
        vm.prank(testTrader);
        book.cancelTrade(testTokenIn, testTokenOut, testAmountIn, 1, testRecipient, tradeIndex);
    }

    function testCannotCancelIfUninitialized() public {
        deal(testTokenIn, testTrader, testAmountIn);
        // take the hash of an uninitialized trade
        bytes32 tradeId = _getTradeId(testTokenIn, testTokenOut, testAmountIn, 1, testRecipient, 1, testTrader);
        vm.expectRevert(abi.encodeWithSelector(Book__TradeNotCancelable.selector, tradeId));
        vm.prank(testTrader);
        book.cancelTrade(testTokenIn, testTokenOut, testAmountIn, 1, testRecipient, 1);
    }
}
