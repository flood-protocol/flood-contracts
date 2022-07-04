// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "../utils/BaseFixture.sol";
import "../utils/TokenFixture.sol";
import "../AllKnowingOracle/Fixtures.sol";
import "src/BookSingleChain.sol";

interface IBookSingleChainEvents {
    event SafeBlockThresholdChanged(uint256 newSafeBlockThreshold);
    event MaxFeePctChanged(uint256 newMaxFeePct);
    event TokenWhitelisted(address indexed token, bool whitelisted);
    event TradeRequested(
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 feePct,
        address to,
        uint256 indexed tradeIndex
    );
    event UpdatedFeeForTrade(
        address indexed trader,
        uint256 indexed tradeIndex,
        uint256 newFeePct
    );
    event TradeFilled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        uint256 indexed filledAtBlock,
        uint256 feePct,
        uint256 amountOut
    );
    event TradeSettled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        uint256 indexed filledAmount,
        uint256 feePct
    );
    event TradeDisputed(
        address indexed relayer,
        uint256 indexed tradeIndex,
        bytes32 indexed disputeId,
        uint256 filledAmount,
        uint256 feePct
    );
}

contract BaseBookFixture is IBookSingleChainEvents, OracleFixture {
    BookSingleChain internal book;
    uint256 internal testSafeBlockThreashold = 100;

    function setUp() public virtual override {
        super.setUp();
        book = new BookSingleChain(testSafeBlockThreashold, address(oracle));
        vm.label(address(book), "Book");
    }
}

contract TradeFixture is BaseBookFixture {
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
    ) internal returns (uint256) {
        uint256 tradeIndex = book.numberOfTrades();

        vm.prank(who);
        book.requestTrade(tokenIn, tokenOut, amount, feePct, to);
        return tradeIndex;
    }

    function _signFeeUpdate(
        uint256 pk,
        uint256 tradeIndex,
        uint256 newFeePct
    ) internal returns (bytes memory) {
        bytes32 messageHash = keccak256(
            abi.encode(SIGNATURE_DELIMITER, tradeIndex, newFeePct)
        );
        bytes32 ethSignedMessageHash = ECDSA.toEthSignedMessageHash(
            messageHash
        );
        return sign(pk, ethSignedMessageHash);
    }

    function _fillTrade(
        address _tokenOut,
        uint256 _feePct,
        uint256 _tradeIndex,
        uint256 _amountToSend
    ) internal {
        book.fillTrade(_tokenOut, _feePct, _tradeIndex, _amountToSend);
    }
}
