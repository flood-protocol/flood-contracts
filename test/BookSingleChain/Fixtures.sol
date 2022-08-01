// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "../utils/BaseFixture.sol";
import "../utils/TokenFixture.sol";
import "../AllKnowingOracle/Fixtures.sol";
import "src/BookSingleChain.sol";

contract BaseBookFixture is IBookSingleChainEvents, OracleFixture {
    BookSingleChain internal book;
    uint256 internal testSafeBlockThreashold = 100;
    uint256 testDisputeBondPct = 20;
    uint256 testTradeRebatePct = 20;
    uint256 testRelayerPenaltyPct = 60;

    function setUp() public virtual override {
        super.setUp();
        book = new BookSingleChain(
            address(oracle),
            testSafeBlockThreashold,
            testDisputeBondPct,
            testTradeRebatePct,
            testRelayerPenaltyPct
        );
        vm.label(address(book), "Book");
    }
}

contract TradeFixture is BaseBookFixture {
    using stdStorage for StdStorage;
    address internal testTokenIn = WETH;
    address internal testTokenOut = USDC;
    uint256 internal testFeePct = 0.01e18;
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
        uint256 feePct,
        address recipient,
        address who
    ) internal returns (uint256, bytes32) {
        uint256 tradeIndex = book.numberOfTrades();
        bytes32 tradeId = keccak256(
            abi.encodePacked(
                tokenIn,
                tokenOut,
                amountIn,
                amountOutMin,
                feePct,
                recipient,
                tradeIndex
            )
        );
        vm.prank(who);
        book.requestTrade(
            tokenIn,
            tokenOut,
            amountIn,
            amountOutMin,
            feePct,
            recipient
        );
        return (tradeIndex, tradeId);
    }

    function _signFeeUpdate(
        uint256 pk,
        bytes32 tradeId,
        uint256 newFeePct
    ) internal returns (bytes memory) {
        bytes32 messageHash = keccak256(
            abi.encodePacked(SIGNATURE_DELIMITER, tradeId, newFeePct)
        );
        bytes32 ethSignedMessageHash = ECDSA.toEthSignedMessageHash(
            messageHash
        );
        return sign(pk, ethSignedMessageHash);
    }

    function _fillTrade(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        uint256 _amountOutMin,
        uint256 _feePct,
        address _to,
        uint256 _tradeIndex,
        uint256 _amountToSend
    ) internal {
        book.fillTrade(
            _tokenIn,
            _tokenOut,
            _amountIn,
            _amountOutMin,
            _feePct,
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
