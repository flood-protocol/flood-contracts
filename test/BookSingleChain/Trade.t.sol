// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./Fixtures.sol";

contract TradeTest is TradeFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testRequestTrade(uint256 amount, uint256 feePct) public {}

    function testCannotTradeIfNoBalance() public {}

    function testCannotTradeNonWhitelistedToken(address token) public {}

    function testCannotTradeSameToken() public {}

    function testCannotTradeZeroAmount() public {}

    function testCannotTradeAboveMaxFee() public {}

    function testCannotTradeToBlackHole() public {}

    function testUpdateFee() public {}

    function testCannotUpdateFeeWithInvalidSignature() public {}

    function testCannotUpdateFeePastMax() public {}

    function testCannotUpdateFeeForFilledTrade() public {}

    function testFillTrade(uint256 amountIn, uint256 amountOut) public {}

    function testCannotFillIfAlreadyFilled(uint256 amountIn, uint256 amountOut)
        public
    {}

    function testCannotFillIfNoTokens(uint256 amountOut) public {}

    function testFillTradeWithUpdatedFee(
        uint256 amountIn,
        uint256 amountOut,
        uint256 newFeePct
    ) public {}

    function testCannotFillWithUpdateFeeForFilledTrade() public {}

    function testCannotFillWithUpdateFeePastMax() public {}

    function testCannotFillTradeWithUpdateFeeWithInvalidSignature() public {}
}
