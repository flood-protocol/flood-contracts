// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {Book, Book__TradeNotFilled, Book__DisputePeriodNotOver} from "src/Book.sol";
import {DisputeFixture} from "./Fixtures.sol";

contract SettlementTest is DisputeFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testSettlement() public {
        uint256 relayerBalanceBeforeSettle = IERC20(testTokenIn).balanceOf(relayer);
        // move to the end of the dispute period
        skipBlocks(book.safeBlockThreshold());
        bytes32 id = _getTradeId(
            testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, tradeIndex, testTrader
        );
        (uint256 filledAtBlock ,,,,)= book.tradesData(id);
        vm.expectEmit(true, true, true, true, address(book));
        emit TradeSettled(relayer, tradeIndex, filledAtBlock, testTrader);
        book.settleTrade(
            testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, tradeIndex, testTrader
        );

        // check that the storage variables have been reset
        (uint256 filledAtBlockInStorageAfter,address filledByInStorageAfter,,,) = book.tradesData(id);
        assertEq(filledByInStorageAfter, address(0), "Filled by should be equal to 0");
        assertEq(filledAtBlockInStorageAfter, 0, "Filled at block should be equal to 0");

        // check that the trade was settled correctly
        uint256 relayerPenalty = (testAmountIn * (100 - testRelayerRefundPct)) / 100;

        assertEq(
            IERC20(testTokenIn).balanceOf(relayer),
            relayerBalanceBeforeSettle + relayerPenalty,
            "The relayer should have received the amount sold by the trader"
        );
        assertEq(IERC20(testTokenOut).balanceOf(address(book)), 0, "Book should be empty");
    }

    function testCannotSettleBeforeThreshold() public {
        vm.expectRevert(abi.encodeWithSelector(Book__DisputePeriodNotOver.selector, testSafeBlockThreashold));
        book.settleTrade(
            testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, tradeIndex, testTrader
        );
    }

    function testCannotSettleIfNotFilled() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                Book__TradeNotFilled.selector,
                _getTradeId(
                    testTokenIn,
                    testTokenOut,
                    testAmountIn + 1,
                    testAmountOutMin,
                    testRecipient,
                    tradeIndex + 1,
                    testTrader
                )
            )
        );
        book.settleTrade(
            testTokenIn, testTokenOut, testAmountIn + 1, testAmountOutMin, testRecipient, tradeIndex + 1, testTrader
        );
    }

    function testCannotSettleIfDisputed() public {
        uint256 bond = (testDisputeBondPct * testAmountIn) / 100;
        deal(testTokenIn, disputer, bond);
        _disputeTrade(testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, tradeIndex, testTrader);
        vm.expectRevert(
            abi.encodeWithSelector(
                Book__TradeNotFilled.selector,
                _getTradeId(
                    testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, tradeIndex, testTrader
                )
            )
        );
        book.settleTrade(
            testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, tradeIndex, testTrader
        );
    }
}
