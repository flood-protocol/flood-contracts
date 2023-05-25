// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "forge-std/Test.sol";
import {IBookEvents} from "src/interfaces/IBook.sol";
import {TradeStatus, Book__TradeNotCancelable} from "src/Book.sol";
import {TradeFixture} from "./Fixture.t.sol";

contract CancelTest is TradeFixture, IBookEvents {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
        vm.label(address(this), "CancelTest");
    }

    function testCancel() public {
        vm.prank(testTrader);
        vm.expectEmit(address(book));
        emit TradeCancelled(testTradeIndex, testTradeId, testTrader);
        book.cancelTrade(testBasket, testAmounts, testRecipient, testTradeIndex);

        // Check that the trade was cancelled
        (TradeStatus s, bool wI, bool uO) = book.tradesData(testTradeId);

        assertEq(uint256(s), uint256(TradeStatus.UNINITIALIZED));
        assertEq(wI, false);
        assertEq(uO, false);

        // Check that the trader's balances were restored. We don't check the last one as its the tokenOut
        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            assertEq(testBasket[i].balanceOf(testTrader), balancesBefore[i]);
        }
    }

    function testCannotCancelForSomeoneElse() public {
        vm.prank(bob);
        vm.expectRevert(Book__TradeNotCancelable.selector);
        book.cancelTrade(testBasket, testAmounts, testRecipient, testTradeIndex);
    }

    function testCannotCancelIfNotRequested() public {
        vm.prank(testTrader);
        vm.expectRevert(Book__TradeNotCancelable.selector);
        // This does not exists as testTradeIndex + 1 has not been requested
        book.cancelTrade(testBasket, testAmounts, testRecipient, testTradeIndex + 1);
    }

    function testCannotCancelIfFilled() public {
        // We artificially fill the trade
        stdstore.target(address(book)).sig(book.tradesData.selector).with_key(testTradeId).depth(0).checked_write(
            uint256(TradeStatus.FILLED)
        );
        (TradeStatus s,,) = book.tradesData(testTradeId);

        assertEq(uint256(s), uint256(TradeStatus.FILLED), "Trade should be filled");

        vm.prank(testTrader);
        vm.expectRevert(Book__TradeNotCancelable.selector);
        book.cancelTrade(testBasket, testAmounts, testRecipient, testTradeIndex);
    }
}
