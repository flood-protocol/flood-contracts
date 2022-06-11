// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "../utils/BaseFixture.sol";
import "../utils/TokenFixture.sol";
import "src/BookSingleChain.sol";

interface IEvents {
    event SafeBlockThresholdChanged(uint256 newSafeBlockThreshold);
    event MaxFeePctChanged(uint128 newMaxFeePct);
    event TokenWhitelisted(address indexed token, bool whitelisted);
    event TradeRequested(
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 feePct,
        uint256 amount,
        address to,
        uint256 indexed tradeIndex
    );
    event UpdatedFeeForTrade(
        address indexed trader,
        bytes32 indexed tradeId,
        uint256 newFeePct
    );
    event TradeSettled(
        address indexed relayer,
        bytes32 indexed tradeId,
        uint256 indexed filledAmount,
        uint256 feePct
    );
}

contract BaseBookFixture is BaseFixture, IEvents {
    BookSingleChain internal book;
    uint256 internal testSafeBlockThreashold = 100;

    function setUp() public virtual override {
        super.setUp();
        book = new BookSingleChain(testSafeBlockThreashold);
        vm.label(address(book), "Book");
    }
}

contract TradeFixture is BaseBookFixture, TokenFixture {
    address internal testTokenIn = WETH;
    address internal testTokenOut = USDC;
    uint256 internal testFeePct = 0.01e18;
    uint256 internal testAmount = 1 ether;
    address internal testTo = charlie;

    function setUp() public virtual override {
        BaseBookFixture.setUp();

        book.whitelistToken(USDC, true);
        book.whitelistToken(WETH, true);
        vm.label(USDC, "USDC");
        vm.label(WETH, "WETH");
        vm.startPrank(alice);
        ERC20(USDC).approve(address(book), type(uint256).max);
        ERC20(WETH).approve(address(book), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(bob);
        ERC20(USDC).approve(address(book), type(uint256).max);
        ERC20(WETH).approve(address(book), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(charlie);
        ERC20(USDC).approve(address(book), type(uint256).max);
        ERC20(WETH).approve(address(book), type(uint256).max);
        vm.stopPrank();
    }

    function _requestTrade(
        address tokenIn,
        address tokenOut,
        uint256 amount,
        uint256 feePct,
        address to,
        address who
    ) internal returns (uint128, bytes32) {
        uint128 tradeIndex = book.numberOfTrades();
        bytes32 tradeId = keccak256(
            abi.encode(tokenIn, tokenOut, amount, feePct, to, tradeIndex)
        );
        vm.prank(who);
        book.requestTrade(tokenIn, tokenOut, amount, feePct, to);
        return (tradeIndex, tradeId);
    }
}
