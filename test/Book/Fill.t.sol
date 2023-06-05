// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/token/ERC20/IERC20.sol";
import {TradeFixture} from "./Fixture.t.sol";
import {Book_UnauthorizedRelayer, Book__AmountOutTooLow, Book__TradeNotFillable, TradeStatus} from "src/Book.sol";
import {IBook, IBookEvents} from "src/interfaces/IBook.sol";
import {IFloodFillCallback, IFloodRecipient} from "src/interfaces/ICallbacks.sol";

contract MockFloodFillCallee is IFloodFillCallback {
    function onFloodFill(bytes calldata data) external pure returns (uint128) {
        uint128 amountToSend = abi.decode(data, (uint128));
        return amountToSend;
    }
}

contract MockFloodRecipient is IFloodRecipient {
    function onTradeFilled(
        address trader,
        IERC20[] calldata tokens,
        uint128[] calldata amounts,
        uint128 amountReceived,
        bytes32 tradeId
    ) external {
        // Do nothing
    }
}

contract FillTest is TradeFixture, IBookEvents, MockFloodFillCallee, MockFloodRecipient {
    address internal relayer = bob;

    function setUp() public virtual override {
        super.setUp();
        vm.label(relayer, "Relayer");
        registry.whitelistRelayer(relayer, true);
        vm.startPrank(relayer);
        WETH.approve(address(book), type(uint256).max);
        testBasket[testBasket.length - 1].approve(address(book), type(uint256).max);
        vm.stopPrank();
    }

    function testFillTrade() public {
        // Give the relayer some tokens
        uint128 amountOut = testAmounts[testBasket.length - 1] * 2;
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            tradeIndex: testTradeIndex,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });
        deal(address(testBasket[testBasket.length - 1]), relayer, amountOut);

        vm.prank(relayer);
        vm.expectEmit(address(book));
        emit TradeFilled(relayer, trade.tradeIndex, trade.amountOut, trade.trader);
        book.fillTrade(trade);

        assertEq(
            testBasket[testBasket.length - 1].balanceOf(trade.recipient),
            amountOut,
            "Recipient should have received tokens"
        );
        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            assertEq(testBasket[i].balanceOf(address(book)), 0, "Book should have no tokens");
            assertEq(testBasket[i].balanceOf(relayer), testAmounts[i], "Relayer should have received tokens");
        }
    }

    function testCannotFillIfNotAuthorized() public {
        registry.whitelistRelayer(relayer, false);
        uint128 amountOut = testAmounts[testBasket.length - 1] * 2;
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            tradeIndex: testTradeIndex,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });

        vm.prank(relayer);
        vm.expectRevert(abi.encodeWithSelector(Book_UnauthorizedRelayer.selector, relayer));
        book.fillTrade(trade);
    }

    function testCannotFillIfNoTokens() public {
        // Give the relayer some tokens
        uint128 amountOut = testAmounts[testBasket.length - 1] * 2;
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            tradeIndex: testTradeIndex,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });

        vm.prank(relayer);
        vm.expectRevert();
        book.fillTrade(trade);
    }

    function testCannotFillIfLessThanMinAmount() public {
        // Give the relayer some tokens
        uint128 amountOut = testAmounts[testBasket.length - 1] - 1;
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            tradeIndex: testTradeIndex,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });

        deal(address(testBasket[testBasket.length - 1]), relayer, amountOut);
        vm.prank(relayer);
        vm.expectRevert(Book__AmountOutTooLow.selector);
        book.fillTrade(trade);
    }

    function testCannotFillIfNotRequested() public {
        // Give the relayer some tokens
        uint128 amountOut = testAmounts[testBasket.length - 1] * 2;
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            // We know this trade index is not requested
            tradeIndex: testTradeIndex + 1,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });

        deal(address(testBasket[testBasket.length - 1]), relayer, amountOut);
        vm.prank(relayer);
        vm.expectRevert(Book__TradeNotFillable.selector);
        book.fillTrade(trade);
    }

    function testCannotFillTwice() public {
        // Give the relayer some tokens
        uint128 amountOut = testAmounts[testBasket.length - 1] * 2;
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            tradeIndex: testTradeIndex,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });

        deal(address(testBasket[testBasket.length - 1]), relayer, 2 * amountOut);
        vm.prank(relayer);
        book.fillTrade(trade);

        vm.expectRevert(Book__TradeNotFillable.selector);
        vm.prank(relayer);
        book.fillTrade(trade);
    }

    function testFillReceivesETH() public {
        uint256[] memory bookBalanceBefore = new uint[](testBasket.length);
        // We set token out to be WETH
        testBasket[testBasket.length - 1] = WETH;
        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            deal(address(testBasket[i]), testTrader, testAmounts[i]);
            bookBalanceBefore[i] = testBasket[i].balanceOf(address(book));
        }
        testUnwrapOutput = true;
        vm.prank(testTrader);
        book.requestTrade(testBasket, testAmounts, testRecipient, testUnwrapOutput);

        // Give the relayer some tokens
        uint128 amountOut = testAmounts[testBasket.length - 1];
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            // This trade is right after the one in the setup.
            tradeIndex: testTradeIndex + 1,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });

        deal(address(testBasket[testBasket.length - 1]), relayer, amountOut);
        vm.prank(relayer);
        book.fillTrade(trade);

        assertEq(testRecipient.balance, amountOut, "Recipient should have received ETH");

        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            assertEq(testBasket[i].balanceOf(address(book)), bookBalanceBefore[i], "Book should have no tokens");
            assertEq(testBasket[i].balanceOf(relayer), testAmounts[i], "Relayer should have received tokens");
        }
    }

    function testRelayerReceivesWETHEvenIfUserSentETH() public {
        uint256[] memory bookBalanceBefore = new uint[](testBasket.length);
        // We set token out to be WETH
        testBasket[0] = IERC20(address(0));
        // Give ETH to the trader
        deal(testTrader, testAmounts[0]);
        for (uint256 i = 1; i < testBasket.length - 1; i++) {
            deal(address(testBasket[i]), testTrader, testAmounts[i]);
            bookBalanceBefore[i] = testBasket[i].balanceOf(address(book));
        }
        vm.prank(testTrader);
        book.requestTrade{value: testAmounts[0]}(testBasket, testAmounts, testRecipient, testUnwrapOutput);

        // Give the relayer some tokens
        uint128 amountOut = testAmounts[testBasket.length - 1];
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            // This trade is right after the one in the setup.
            tradeIndex: testTradeIndex + 1,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });

        deal(address(testBasket[testBasket.length - 1]), relayer, amountOut);
        vm.prank(relayer);
        book.fillTrade(trade);

        assertEq(
            testBasket[testBasket.length - 1].balanceOf(testRecipient),
            amountOut,
            "Recipient should have received tokens"
        );

        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            if (address(testBasket[i]) == address(0)) {
                assertEq(address(book).balance, 0, "Book should have no ETH");
                assertEq(WETH.balanceOf(relayer), testAmounts[i], "Relayer should have received WETH");
            } else {
                assertEq(testBasket[i].balanceOf(address(book)), bookBalanceBefore[i], "Book should have no tokens");
                assertEq(testBasket[i].balanceOf(relayer), testAmounts[i], "Relayer should have received tokens");
            }
        }
    }

    function testFillWithCallback() public {
        // Give the relayer some tokens
        uint128 realAmountOut = testAmounts[testBasket.length - 1] * 2;

        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            tradeIndex: testTradeIndex,
            trader: testTrader,
            // Set this to 0 so we can check that the callback is called
            amountOut: 0,
            callbackData: abi.encode(realAmountOut)
        });
        deal(address(testBasket[testBasket.length - 1]), relayer, realAmountOut);

        registry.whitelistRelayer(address(this), true);
        deal(address(testBasket[testBasket.length - 1]), address(this), realAmountOut);
        testBasket[testBasket.length - 1].approve(address(book), realAmountOut);
        vm.expectCall(address(this), abi.encodeCall(IFloodFillCallback.onFloodFill, (abi.encode(realAmountOut))));
        vm.expectEmit(address(book));
        emit TradeFilled(address(this), trade.tradeIndex, realAmountOut, trade.trader);
        book.fillTrade(trade);

        assertEq(
            testBasket[testBasket.length - 1].balanceOf(testRecipient),
            realAmountOut,
            "Recipient should have received tokens"
        );
        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            assertEq(testBasket[i].balanceOf(address(book)), 0, "Book should have no tokens");
            assertEq(testBasket[i].balanceOf(address(this)), testAmounts[i], "Relayer should have received tokens");
        }
    }

    function testRecipientGetsNotified() public {
        testRecipient = address(this);
        testTradeId = book.getTradeId(testBasket, testAmounts, testRecipient, testTradeIndex + 1, testTrader);
        uint256[] memory bookBalanceBefore = new uint[](testBasket.length);

        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            deal(address(testBasket[i]), testTrader, testAmounts[i]);
            bookBalanceBefore[i] = testBasket[i].balanceOf(address(book));
        }

        vm.prank(testTrader);
        book.requestTrade(testBasket, testAmounts, testRecipient, testUnwrapOutput);

        (TradeStatus s,,) = book.tradesData(testTradeId);
        assertEq(uint256(s), uint256(TradeStatus.REQUESTED), "Trade should be requested");

        uint128 amountOut = testAmounts[testBasket.length - 1] * 2;
        IBook.FillTradeArgs memory trade = IBook.FillTradeArgs({
            tokens: testBasket,
            amounts: testAmounts,
            recipient: testRecipient,
            tradeIndex: testTradeIndex + 1,
            trader: testTrader,
            amountOut: amountOut,
            callbackData: bytes("")
        });
        deal(address(testBasket[testBasket.length - 1]), relayer, amountOut);

        vm.prank(relayer);
        vm.expectCall(
            address(this),
            abi.encodeCall(IFloodRecipient.onTradeFilled, (testTrader, testBasket, testAmounts, amountOut, testTradeId))
        );
        vm.expectEmit(address(book));
        emit TradeFilled(relayer, trade.tradeIndex, trade.amountOut, trade.trader);
        book.fillTrade(trade);

        assertEq(
            testBasket[testBasket.length - 1].balanceOf(trade.recipient),
            amountOut,
            "Recipient should have received tokens"
        );
        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            assertEq(testBasket[i].balanceOf(address(book)), bookBalanceBefore[i], "Book should have no tokens");
            assertEq(testBasket[i].balanceOf(relayer), testAmounts[i], "Relayer should have received tokens");
        }
    }
}
