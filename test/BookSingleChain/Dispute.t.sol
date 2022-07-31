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

    function setUp() public override {}

    function testDispute() public {}

    function testCannotDisputeIfNotFilled() public {}
}
