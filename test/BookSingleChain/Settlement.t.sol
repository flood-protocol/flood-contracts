// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "src/BookSingleChain.sol";
import "forge-std/Test.sol";
import "./Fixtures.sol";

contract SettlementTest is TradeFixture {
    using stdStorage for StdStorage;

    uint256 internal tradeIndex;
    bytes32 internal tradeId;

    function setUp() public override {
        super.setUp();
        deal(testTokenIn, alice, testAmountIn);
        (uint256 _tradeIndex, bytes32 _tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testFeePct,
            testRecipient,
            alice
        );
        tradeIndex = _tradeIndex;
        tradeId = _tradeId;
    }

    function testSettlement() public {}

    function testCannotSettleBeforeThreshold() public {}

    function testCannotSettleIfNotFilled() public {}
}
