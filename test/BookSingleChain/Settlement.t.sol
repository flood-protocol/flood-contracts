// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "src/BookSingleChain.sol";
import "forge-std/Test.sol";
import "./Fixtures.sol";

contract SettlementTest is TradeFixture {
    using stdStorage for StdStorage;

    uint128 internal tradeIndex;
    bytes32 internal tradeId;

    function setUp() public override {
        super.setUp();
        deal(testTokenIn, alice, testAmount);
        (uint128 _tradeIndex, bytes32 _tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            alice
        );
        tradeIndex = _tradeIndex;
        tradeId = _tradeId;
    }

    function testSettlement() public {
        // Alice requests the trade, bob accepts it
        address relayer = bob;

        uint256 amountToSend = 2000 * 1e6;
        deal(testTokenOut, bob, amountToSend);
        vm.prank(relayer);
        _fillTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            tradeIndex,
            amountToSend
        );

        // lets check the trade was filled correctly and storage variables are set
        uint256 filledAmountInStorageBefore = stdstore
            .target(address(book))
            .sig(book.filledAmount.selector)
            .with_key(tradeId)
            .read_uint();

        assertEq(
            filledAmountInStorageBefore,
            amountToSend,
            "Filled amount should be equal to the amount sent"
        );
        address filledByInStorageBefore = stdstore
            .target(address(book))
            .sig(book.filledBy.selector)
            .with_key(tradeId)
            .read_address();

        assertEq(
            filledByInStorageBefore,
            bob,
            "Filled by should be equal to the address of the relayer"
        );

        uint256 filledAtBlockInStorageBefore = stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .read_uint();

        assertEq(
            filledAtBlockInStorageBefore,
            block.number,
            "Filled at block should be equal to the current block number"
        );

        uint256 filledAmount = filledAmountInStorageBefore;

        uint256 relayerBalanceBefore = ERC20(testTokenIn).balanceOf(relayer);
        uint256 recipientBalanceBefore = ERC20(testTokenOut).balanceOf(testTo);

        // move to the end of the dispute period
        skipBlocks(book.safeBlockThreshold());

        vm.expectEmit(true, true, true, true, address(book));
        emit TradeSettled(relayer, tradeId, filledAmount, testFeePct);
        book.settleTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            tradeIndex
        );

        // check that the storage variables have been reset
        uint256 filledAmountInStorageAfter = stdstore
            .target(address(book))
            .sig(book.filledAmount.selector)
            .with_key(tradeId)
            .read_uint();

        assertEq(
            filledAmountInStorageAfter,
            0,
            "Filled amount should be equal to 0"
        );
        address filledByInStorageAfter = stdstore
            .target(address(book))
            .sig(book.filledBy.selector)
            .with_key(tradeId)
            .read_address();

        assertEq(
            filledByInStorageAfter,
            address(0),
            "Filled by should be equal to 0"
        );
        uint256 filledAtBlockInStorageAfter = stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .read_uint();

        assertEq(
            filledAtBlockInStorageAfter,
            0,
            "Filled at block should be equal to 0"
        );

        // check that the trade was settled correctly
        assertEq(
            ERC20(testTokenIn).balanceOf(relayer),
            relayerBalanceBefore + testAmount,
            "Bob (Relayer) should have received the amount sold by the trader"
        );
        assertEq(
            ERC20(testTokenOut).balanceOf(testTo),
            recipientBalanceBefore + amountToSend,
            "The recipient of the trade should have received the amount sent by the relayer"
        );
    }

    function testCannotSettleBeforeThreshold() public {
        uint256 amountToSend = 2000_10e6;
        deal(testTokenOut, bob, amountToSend);
        vm.prank(bob);
        _fillTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            tradeIndex,
            amountToSend
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__DisputePeriodNotOver.selector,
                testSafeBlockThreashold
            )
        );
        book.settleTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            tradeIndex
        );
    }

    function testCannotSettleIfNotFilled() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeNotFilled.selector,
                tradeId
            )
        );
        book.settleTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            tradeIndex
        );
    }
}
