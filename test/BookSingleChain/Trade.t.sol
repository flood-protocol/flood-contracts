// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "./Admin.t.sol";

address constant USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
address constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

contract TradeTest is AdminFixture {
    address internal testTokenIn = WETH;
    address internal testTokenOut = USDC;
    uint256 internal testFeePct = 0.01e18;
    uint256 internal testAmount = 1 ether;
    address internal testTo = bob;

    function setUp() public override {
        super.setUp();
        book.whitelistToken(USDC, true);
        book.whitelistToken(WETH, true);
        vm.label(USDC, "USDC");
        vm.label(WETH, "WETH");
        // We want to test the trade request with someone that is not the owner of the contract.
        vm.startPrank(alice);
    }

    function testCannotTradeNonWhitelistedToken(address token) public {
        vm.assume(token != USDC && token != WETH);
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__InvalidToken.selector,
                token
            )
        );
        book.requestTrade(token, testTokenOut, testFeePct, testAmount, testTo);
    }

    function testCannotTradeSameToken() public {
        vm.expectRevert(BookSingleChain__SameToken.selector);
        book.requestTrade(USDC, USDC, testAmount, testFeePct, bob);
    }

    function testCannotTradeZeroAmount() public {
        vm.expectRevert(BookSingleChain__ZeroAmount.selector);
        book.requestTrade(testTokenIn, testTokenOut, 0, testFeePct, bob);
    }

    function testCannotTradeAboveMaxFee() public {
        uint256 maxFeePct = book.maxFeePct() + 1;
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__FeePctTooHigh.selector,
                maxFeePct
            )
        );
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            maxFeePct,
            bob
        );
    }

    function testCannotTradeToBlackHole() public {
        address blackHole = address(0);
        vm.expectRevert(BookSingleChain__SentToBlackHole.selector);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            blackHole
        );
    }
}
