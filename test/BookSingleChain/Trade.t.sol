// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./Fixtures.sol";

contract TradeTest is TradeFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testRequestTrade(uint256 amount, uint256 feePct) public {
        // We assume amount > 0 and feePct < maxFeePct, since we have separate tests for those.
        vm.assume(amount > 0);
        vm.assume(feePct <= book.maxFeePct());

        uint256 tradeIndex = book.numberOfTrades();
        // Give alice some tokens to trade.
        deal(testTokenIn, alice, amount);
        // Request a trade from Alice.
        uint256 balanceBefore = ERC20(testTokenIn).balanceOf(alice);

        vm.expectEmit(true, true, true, true, address(book));
        emit TradeRequested(
            testTokenIn,
            testTokenOut,
            amount,
            feePct,
            testTo,
            tradeIndex
        );
        _requestTrade(testTokenIn, testTokenOut, amount, feePct, testTo, alice);

        // Check that the balance of Alice of `tokenIn` is reduced by `amount`.
        assertEq(ERC20(testTokenIn).balanceOf(alice), balanceBefore - amount);
    }

    function testCannotTradeIfNoBalance() public {
        vm.expectRevert(bytes("TRANSFER_FROM_FAILED"));
        vm.prank(alice);
        book.requestTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo
        );
    }

    function testCannotTradeNonWhitelistedToken(address token) public {
        // check that the random token is not whitelisted
        vm.assume(!book.whitelistedTokens(token));
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
        book.requestTrade(USDC, USDC, testAmount, testFeePct, testTo);
    }

    function testCannotTradeZeroAmount() public {
        vm.expectRevert(BookSingleChain__ZeroAmount.selector);
        book.requestTrade(testTokenIn, testTokenOut, 0, testFeePct, testTo);
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
            testTo
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

    function testUpdateFee() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = keccak256(
            abi.encode(
                testTokenIn,
                testTokenOut,
                testAmount,
                testFeePct,
                testTo,
                tradeIndex
            )
        );

        uint256 newFeePct = testFeePct + 1;
        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );

        vm.expectEmit(true, true, false, true, address(book));
        emit UpdatedFeeForTrade(alice, tradeId, newFeePct);

        book.updateFeeForTrade(alice, tradeId, newFeePct, aliceSignature);
    }

    function testCannotUpdateFeeWithInvalidSignature() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint128 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = keccak256(
            abi.encode(
                testTokenIn,
                testTokenOut,
                testAmount,
                testFeePct,
                testTo,
                tradeIndex
            )
        );

        uint256 newFeePct = testFeePct + 1;
        // Sign a fee update with Bob's private key.
        bytes memory bobSignature = _signFeeUpdate(BOB_PK, tradeId, newFeePct);
        vm.expectRevert(BookSingleChain__InvalidSignature.selector);
        book.updateFeeForTrade(alice, tradeId, newFeePct, bobSignature);
    }

    function testCannotUpdateFeePastMax() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint128 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = keccak256(
            abi.encode(
                testTokenIn,
                testTokenOut,
                testAmount,
                testFeePct,
                testTo,
                tradeIndex
            )
        );

        uint256 newFeePct = book.maxFeePct() + 1;
        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__FeePctTooHigh.selector,
                newFeePct
            )
        );
        book.updateFeeForTrade(alice, tradeId, newFeePct, aliceSignature);
    }

    function testCannotUpdateFeeForFilledTrade() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint256 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = keccak256(
            abi.encode(
                testTokenIn,
                testTokenOut,
                testAmount,
                testFeePct,
                testTo,
                tradeIndex
            )
        );

        // Artificially fill the trade at the past block.
        stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .checked_write(block.number);

        uint256 newFeePct = testFeePct + 1;

        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeAlreadyFilled.selector,
                tradeId
            )
        );
        book.updateFeeForTrade(alice, tradeId, newFeePct, aliceSignature);
    }

    function testFillTrade(uint256 amountIn, uint256 amountOut) public {
        vm.assume(amountIn > 0);

        vm.prank(alice);
        deal(testTokenIn, alice, amountIn);
        (uint128 tradeIndex, bytes32 tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testFeePct,
            testTo,
            alice
        );

        deal(testTokenOut, bob, amountOut);
        uint256 bobBalanceBefore = ERC20(testTokenOut).balanceOf(bob);
        vm.prank(bob);
        vm.expectEmit(true, true, true, true, address(book));
        emit TradeFilled(bob, tradeId, block.number, testFeePct, amountOut);
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testFeePct,
            testTo,
            tradeIndex,
            amountOut
        );
        // Check bob submitted amountOut tokens
        assertEq(
            ERC20(testTokenOut).balanceOf(bob) + amountOut,
            bobBalanceBefore
        );

        uint256 filledAtInStorage = stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .read_uint();
        assertEq(filledAtInStorage, block.number);

        address filledByInStorage = stdstore
            .target(address(book))
            .sig(book.filledBy.selector)
            .with_key(tradeId)
            .read_address();
        assertEq(filledByInStorage, bob);
        uint256 filledAmountInStorage = stdstore
            .target(address(book))
            .sig(book.filledAmount.selector)
            .with_key(tradeId)
            .read_uint();
        assertEq(filledAmountInStorage, amountOut);
    }

    function testCannotFillIfAlreadyFilled(uint256 amountIn, uint256 amountOut)
        public
    {
        vm.assume(amountIn > 0);
        deal(testTokenIn, alice, amountIn);
        (uint128 tradeIndex, bytes32 tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testFeePct,
            testTo,
            alice
        );
        deal(testTokenOut, bob, amountOut);
        vm.prank(bob);
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testFeePct,
            testTo,
            tradeIndex,
            amountOut
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeAlreadyFilled.selector,
                tradeId
            )
        );
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testFeePct,
            testTo,
            tradeIndex,
            amountOut
        );
    }

    function testCannotFillIfNoTokens(uint256 amountOut) public {
        vm.assume(amountOut > 0);
        vm.prank(bob);
        vm.expectRevert(bytes("TRANSFER_FROM_FAILED"));
        book.fillTrade(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            1,
            amountOut
        );
    }

    function testFillTradeWithUpdatedFee(
        uint256 amountIn,
        uint256 amountOut,
        uint256 newFeePct
    ) public {
        vm.assume(amountIn > 0);
        vm.assume(newFeePct < book.maxFeePct());
        vm.prank(alice);
        deal(testTokenIn, alice, amountIn);
        (uint128 tradeIndex, bytes32 tradeId) = _requestTrade(
            testTokenIn,
            testTokenOut,
            amountIn,
            testFeePct,
            testTo,
            alice
        );

        // Sign a message from Alice updating the fee.
        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );
        deal(testTokenOut, bob, amountOut);
        uint256 bobBalanceBefore = ERC20(testTokenOut).balanceOf(bob);
        vm.prank(bob);
        vm.expectEmit(true, true, true, true, address(book));
        // check the fee is updated
        emit TradeFilled(bob, tradeId, block.number, newFeePct, amountOut);
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            amountIn,
            testFeePct,
            testTo,
            tradeIndex,
            amountOut,
            alice,
            newFeePct,
            aliceSignature
        );
        // Check bob submitted amountOut tokens
        assertEq(
            ERC20(testTokenOut).balanceOf(bob) + amountOut,
            bobBalanceBefore
        );

        uint256 filledAtInStorage = stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .read_uint();
        assertEq(filledAtInStorage, block.number);

        address filledByInStorage = stdstore
            .target(address(book))
            .sig(book.filledBy.selector)
            .with_key(tradeId)
            .read_address();
        assertEq(filledByInStorage, bob);
        uint256 filledAmountInStorage = stdstore
            .target(address(book))
            .sig(book.filledAmount.selector)
            .with_key(tradeId)
            .read_uint();
        assertEq(filledAmountInStorage, amountOut);
    }

    function testCannotFillWithUpdateFeeForFilledTrade() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint128 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = keccak256(
            abi.encode(
                testTokenIn,
                testTokenOut,
                testAmount,
                testFeePct,
                testTo,
                tradeIndex
            )
        );

        // Artificially fill the trade at the past block.
        stdstore
            .target(address(book))
            .sig(book.filledAtBlock.selector)
            .with_key(tradeId)
            .checked_write(block.number);

        // Prepare a message to updates the fee.
        uint256 newFeePct = testFeePct + 1;
        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeAlreadyFilled.selector,
                tradeId
            )
        );
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            tradeIndex,
            1,
            alice,
            newFeePct,
            aliceSignature
        );
    }

    function testCannotFillWithUpdateFeePastMax() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint128 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = keccak256(
            abi.encode(
                testTokenIn,
                testTokenOut,
                testAmount,
                testFeePct,
                testTo,
                tradeIndex
            )
        );

        uint256 newFeePct = book.maxFeePct() + 1;

        bytes memory aliceSignature = _signFeeUpdate(
            ALICE_PK,
            tradeId,
            newFeePct
        );

        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__FeePctTooHigh.selector,
                newFeePct
            )
        );
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            tradeIndex,
            1,
            alice,
            newFeePct,
            aliceSignature
        );
    }

    function testCannotFillTradeWithUpdateFeeWithInvalidSignature() public {
        // Simulate a trade request. We assume that the request is valid and executed correctly.
        uint128 tradeIndex = book.numberOfTrades() + 1;
        bytes32 tradeId = keccak256(
            abi.encode(
                testTokenIn,
                testTokenOut,
                testAmount,
                testFeePct,
                testTo,
                tradeIndex
            )
        );

        uint256 newFeePct = testFeePct + 1;
        // Bob signs the feeUpdate message for Alice trade
        bytes memory bobSignature = _signFeeUpdate(BOB_PK, tradeId, newFeePct);
        vm.expectRevert(BookSingleChain__InvalidSignature.selector);
        book.fillTradeWithUpdatedFee(
            testTokenIn,
            testTokenOut,
            testAmount,
            testFeePct,
            testTo,
            tradeIndex,
            1,
            alice,
            newFeePct,
            bobSignature
        );
    }
}
