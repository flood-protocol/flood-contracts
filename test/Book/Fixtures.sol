// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {OracleFixture} from "../AllKnowingOracle/Fixtures.sol";
import {Book, IBookEvents} from "src/Book.sol";

contract BaseBookFixture is IBookEvents, OracleFixture {
    Book internal book;
    uint256 internal testSafeBlockThreashold = 100;
    uint256 testDisputeBondPct = 20;
    uint256 testTradeRebatePct = 20;
    uint256 testRelayerRefundPct = 60;
    uint256 testFeePct = 100;

    function setUp() public virtual override {
        super.setUp();
        book = new Book(
            registry,
            testSafeBlockThreashold,
            testDisputeBondPct,
            testTradeRebatePct,
            testRelayerRefundPct,
            testFeePct
        );
        vm.label(address(book), "Book");
    }
}

contract TradeFixture is BaseBookFixture {
    using stdStorage for StdStorage;

    address internal testTokenIn = WETH;
    address internal testTokenOut = USDC;
    uint256 internal testAmountIn = 1 ether;
    uint256 internal testAmountOutMin = 900 * 10 ** 6;
    address internal testRecipient = alice;
    address internal testTrader = alice;

    function setUp() public virtual override {
        BaseBookFixture.setUp();
        vm.label(USDC, "USDC");
        vm.label(WETH, "WETH");
        registry.whitelistToken(USDC, true);
        registry.whitelistToken(WETH, true);
        vm.startPrank(alice);
        IERC20(USDC).approve(address(book), type(uint256).max);
        IERC20(WETH).approve(address(book), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(bob);
        IERC20(USDC).approve(address(book), type(uint256).max);
        IERC20(WETH).approve(address(book), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(charlie);
        IERC20(USDC).approve(address(book), type(uint256).max);
        IERC20(WETH).approve(address(book), type(uint256).max);
        vm.stopPrank();
    }

    function _requestTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOutMin,
        address recipient,
        address who,
        bool unwrapOutput
    ) internal returns (uint256, bytes32) {
        uint256 tradeIndex = book.numberOfTrades();
        bytes32 tradeId = _getTradeId(tokenIn, tokenOut, amountIn, amountOutMin, recipient, tradeIndex, who);
        vm.prank(who);
        book.requestTrade(tokenIn, tokenOut, amountIn, amountOutMin, recipient, unwrapOutput);
        return (tradeIndex, tradeId);
    }

    function _getTradeId(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        uint256 _amountOutMin,
        address _recipient,
        uint256 _tradeIndex,
        address _trader
    ) internal pure returns (bytes32) {
        return
            keccak256(abi.encodePacked(_tokenIn, _tokenOut, _amountIn, _amountOutMin, _recipient, _tradeIndex, _trader));
    }

    function _fillTrade(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        uint256 _amountOutMin,
        address _recipient,
        uint256 _tradeIndex,
        address _trader,
        uint256 _amountToSend,
        bytes memory _data
    ) internal {
        book.fillTrade(
            _tokenIn, _tokenOut, _amountIn, _amountOutMin, _recipient, _tradeIndex, _trader, _amountToSend, _data
        );
    }

    function _checkFill(bytes32 _tradeId, address _filledBy, int256 _filledAtBlock) internal {
        assertEq(
            _filledBy, stdstore.target(address(book)).sig(book.filledBy.selector).with_key(_tradeId).read_address()
        );
        assertEq(
            _filledAtBlock,
            stdstore.target(address(book)).sig(book.filledAtBlock.selector).with_key(_tradeId).read_int()
        );
    }
}

contract DisputeFixture is TradeFixture {
    uint256 internal tradeIndex;
    bytes32 internal tradeId;
    address internal relayer = bob;
    address internal disputer = charlie;
    uint256 internal testAmountToSend = 2000 * 1e6;

    function setUp() public virtual override {
        super.setUp();

        deal(testTokenIn, alice, testAmountIn);

        (tradeIndex, tradeId) =
            _requestTrade(testTokenIn, testTokenOut, testAmountIn, testAmountOutMin, testRecipient, alice, false);

        deal(testTokenOut, relayer, testAmountToSend);
        vm.prank(relayer);
        _fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            tradeIndex,
            alice,
            testAmountToSend,
            bytes("")
        );
        _checkFill(tradeId, relayer, int256(block.number));

        vm.prank(disputer);
        IERC20(testTokenIn).approve(address(oracle), type(uint256).max);
    }

    function _disputeTrade(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        uint256 _amountOutMin,
        address _recipient,
        uint256 _tradeIndex,
        address _trader
    ) internal {
        vm.prank(disputer);
        book.disputeTrade(_tokenIn, _tokenOut, _amountIn, _amountOutMin, _recipient, _tradeIndex, _trader);
    }
}
