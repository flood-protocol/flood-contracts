// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "../utils/BaseFixture.sol";
import "../utils/TokenFixture.sol";
import "../AllKnowingOracle/Fixtures.sol";
import "src/Book.sol";

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
            address(oracle),
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
    uint256 internal testAmountOutMin = 900 * 10**6;
    address internal testRecipient = alice;

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
        uint256 amountIn,
        uint256 amountOutMin,
        address recipient,
        address who
    ) internal returns (uint256, bytes32) {
        uint256 tradeIndex = book.numberOfTrades();
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            amountOutMin,
            recipient,
            tradeIndex
        );
        vm.prank(who);
        book.requestTrade(tokenIn, tokenOut, amountIn, amountOutMin, recipient);
        return (tradeIndex, tradeId);
    }

    function _getTradeId(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        uint256 _amountOutMin,
        address _recipient,
        uint256 _tradeIndex
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encodePacked(
                    _tokenIn,
                    _tokenOut,
                    _amountIn,
                    _amountOutMin,
                    _recipient,
                    _tradeIndex
                )
            );
    }

    function _fillTrade(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        uint256 _amountOutMin,
        address _to,
        uint256 _tradeIndex,
        uint256 _amountToSend
    ) internal {
        book.fillTrade(
            _tokenIn,
            _tokenOut,
            _amountIn,
            _amountOutMin,
            _to,
            _tradeIndex,
            _amountToSend
        );
    }

    function _checkFill(
        bytes32 _tradeId,
        address _filledBy,
        int256 _filledAtBlock
    ) internal {
        assertEq(
            _filledBy,
            stdstore
                .target(address(book))
                .sig(book.filledBy.selector)
                .with_key(_tradeId)
                .read_address()
        );
        assertEq(
            _filledAtBlock,
            stdstore
                .target(address(book))
                .sig(book.filledAtBlock.selector)
                .with_key(_tradeId)
                .read_int()
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
        oracle.whitelistRequester(address(book), true);
        deal(testTokenIn, alice, testAmountIn);

        (tradeIndex, tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            alice
        );

        deal(testTokenOut, relayer, testAmountToSend);
        vm.prank(relayer);
        _fillTrade(
            testTokenIn,
            testTokenOut,
            testAmountIn,
            testAmountOutMin,
            testRecipient,
            tradeIndex,
            testAmountToSend
        );
        _checkFill(tradeId, relayer, int256(block.number));

        vm.prank(disputer);
        ERC20(testTokenIn).approve(address(oracle), type(uint256).max);
    }

    function _disputeTrade(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        uint256 _amountOutMin,
        address _recipient,
        uint256 _tradeIndex
    ) internal {
        vm.prank(disputer);
        book.disputeTrade(
            _tokenIn,
            _tokenOut,
            _amountIn,
            _amountOutMin,
            _recipient,
            _tradeIndex
        );
    }
}
