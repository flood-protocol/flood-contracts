// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./Admin.t.sol";
import "../TokenFixture.sol";

contract TradeTest is AdminFixture, TokenFixture {
    using stdStorage for StdStorage;

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
        // We want to test the trade request with someone that is not the owner of the contract.
        vm.startPrank(alice);
    }

    function testRequestTrade(uint256 amount, uint256 feePct) public {
        // We assume amount > 0 and feePct < maxFeePct, since we have separate tests for those.
        vm.assume(amount > 0);
        vm.assume(feePct <= book.maxFeePct());

        uint256 tradeIndex = book.numberOfTrades();
        // Request a trade from Alice.
        uint256 balanceBefore = ERC20(testTokenIn).balanceOf(alice);
        if (balanceBefore < amount) {
            vm.expectRevert(bytes("TRANSFER_FROM_FAILED"));
            book.requestTrade(
                testTokenIn,
                testTokenOut,
                amount,
                feePct,
                testTo
            );
            return;
        }
        // Give Alice some tokens to trade.
        deal(testTokenIn, alice, amount, true);
        vm.expectEmit(true, true, true, true, address(book));
        emit TradeRequested(
            testTokenIn,
            testTokenOut,
            amount,
            feePct,
            testTo,
            tradeIndex
        );
        book.requestTrade(testTokenIn, testTokenOut, amount, feePct, testTo);

        // Check that the balance of Alice of `tokenIn` is reduced by `amount`.
        assertEq(ERC20(testTokenIn).balanceOf(alice), balanceBefore - amount);
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

        // Prepare a message to updates the fee.
        uint256 newFeePct = testFeePct + 1;
        bytes32 messageHash = keccak256(
            abi.encode(SIGNATURE_DELIMITER, tradeId, newFeePct)
        );
        bytes32 ethSignedMessageHash = ECDSA.toEthSignedMessageHash(
            messageHash
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(
            ALICE_PK,
            ethSignedMessageHash
        );
        bytes memory aliceSignature = bytes.concat(r, s, bytes1(v));

        vm.expectEmit(true, true, false, true, address(book));
        emit UpdatedFeeForTrade(alice, tradeId, newFeePct);

        book.updateFeeForTrade(alice, tradeId, newFeePct, aliceSignature);
    }

    function testCannotUpdateFeeForOthers() public {
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

        // Prepare a message to updates the fee.
        uint256 newFeePct = testFeePct + 1;
        bytes32 messageHash = keccak256(
            abi.encode(SIGNATURE_DELIMITER, tradeId, newFeePct)
        );
        bytes32 ethSignedMessageHash = ECDSA.toEthSignedMessageHash(
            messageHash
        );

        // Sign the message with Bob's private key.
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(BOB_PK, ethSignedMessageHash);
        bytes memory bobSignature = bytes.concat(r, s, bytes1(v));
        vm.expectRevert(BookSingleChain__InvalidSignature.selector);
        book.updateFeeForTrade(alice, tradeId, newFeePct, bobSignature);
    }

    function testCannotUpdateFeePastMax() public {
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

        // Prepare a message to updates the fee.
        uint256 newFeePct = book.maxFeePct() + 1;
        bytes32 messageHash = keccak256(
            abi.encode(SIGNATURE_DELIMITER, tradeId, newFeePct)
        );
        bytes32 ethSignedMessageHash = ECDSA.toEthSignedMessageHash(
            messageHash
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(
            ALICE_PK,
            ethSignedMessageHash
        );
        bytes memory aliceSignature = bytes.concat(r, s, bytes1(v));
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

        // Prepare a message to updates the fee.
        uint256 newFeePct = testFeePct + 1;
        bytes32 messageHash = keccak256(
            abi.encode(SIGNATURE_DELIMITER, tradeId, newFeePct)
        );
        bytes32 ethSignedMessageHash = ECDSA.toEthSignedMessageHash(
            messageHash
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(
            ALICE_PK,
            ethSignedMessageHash
        );
        bytes memory aliceSignature = bytes.concat(r, s, bytes1(v));
        vm.expectRevert(
            abi.encodeWithSelector(
                BookSingleChain__TradeAlreadyFilled.selector,
                tradeId
            )
        );
        book.updateFeeForTrade(alice, tradeId, newFeePct, aliceSignature);
    }
}
