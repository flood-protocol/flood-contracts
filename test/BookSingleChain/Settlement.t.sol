// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "src/BookSingleChain.sol";
import "forge-std/Test.sol";
import "./Fixtures.sol";

contract SettlementTest is DisputeFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testSettlement() public {
        uint256 relayerBalanceBeforeSettle =
            ERC20(testTokenIn).balanceOf(relayer);
        // move to the end of the dispute period
        skipBlocks(book.safeBlockThreshold());
        bytes32 id = _getTradeId(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );
        int256 filledAtBlock = book.filledAtBlock(id);
        vm.expectEmit(true, true, true, true, address(book));
        emit TradeSettled(relayer, tradeIndex, uint256(filledAtBlock));
        book.settleTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );

        // check that the storage variables have been reset
        address filledByInStorageAfter = stdstore.target(address(book)).sig(
            book.filledBy.selector
        ).with_key(tradeId).read_address();

        assertEq(
            filledByInStorageAfter, address(0), "Filled by should be equal to 0"
        );
        uint256 filledAtBlockInStorageAfter = stdstore.target(address(book)).sig(
            book.filledAtBlock.selector
        ).with_key(tradeId).read_uint();

        assertEq(
            filledAtBlockInStorageAfter, 0, "Filled at block should be equal to 0"
        );

        // check that the trade was settled correctly
        uint256 relayerPenalty =
            (testAmountIn * (100 - testRelayerRefundPct)) / 100;

        assertEq(
            ERC20(testTokenIn).balanceOf(relayer),
            relayerBalanceBeforeSettle + relayerPenalty,
            "The relayer should have received the amount sold by the trader"
        );
        assertEq(
            ERC20(testTokenOut).balanceOf(address(book)),
            0,
            "Book should be empty"
        );
    }

    function testCannotSettleBeforeThreshold() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__DisputePeriodNotOver.selector, testSafeBlockThreashold
            )
        );
        book.settleTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );
    }

    function testCannotSettleIfNotFilled() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeNotInFilledState.selector,
                _getTradeId(
                    testTokenIn,
                    testTokenOut,
                    testAmountIn + 1,
                    testAmountOutMin,
                    testFeePct,
                    testRecipient,
                    tradeIndex + 1
                )
            )
        );
        book.settleTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn + 1,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex + 1
        );
    }

    function testCannotSettleIfDisputed() public {
        uint256 bond = (testDisputeBondPct * testAmountIn) / 100;
        deal(testTokenIn, disputer, bond);
        _disputeTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeNotInFilledState.selector,
                _getTradeId(
                    testTokenIn,
                    testTokenOut,
                    testAmountIn,
                    testAmountOutMin,
                    testFeePct,
                    testRecipient,
                    tradeIndex
                )
            )
        );
        book.settleTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            tradeIndex
        );
    }
}
