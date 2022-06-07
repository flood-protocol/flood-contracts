// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "src/BookSingleChain.sol";
import "forge-std/Test.sol";
import "./BaseFixture.sol";

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
}

contract BookSingleChainAdminTest is BaseFixture, IEvents {
    using stdStorage for StdStorage;

    BookSingleChain internal book;
    uint256 internal testSafeBlockThreashold = 100;

    function setUp() public override {
        super.setUp();
        book = new BookSingleChain(testSafeBlockThreashold);
        vm.label(address(book), "Book");
    }

    function testTokenWhitelist(address token) public {
        // check that the whitelisted mapping starts as false
        bool whitelistedBefore = stdstore
            .target(address(book))
            .sig(book.whitelistedTokens.selector)
            .with_key(token)
            .read_bool();
        assertFalse(whitelistedBefore);

        // should fail if not called by the owner
        vm.expectRevert(bytes("UNAUTHORIZED"));
        vm.prank(alice);
        book.whitelistToken(token, true);

        vm.expectEmit(true, false, false, true, address(book));
        emit TokenWhitelisted(token, true);
        book.whitelistToken(token, true);
        // check that the whitelisted mapping has correctly been updated
        bool realWhitelisted = stdstore
            .target(address(book))
            .sig(book.whitelistedTokens.selector)
            .with_key(token)
            .read_bool();
        assertTrue(realWhitelisted, "Token should be whitelisted");
    }

    function testThresholdChange(uint256 newThreshold) public {
        // should fail if not called by the owner
        vm.expectRevert(bytes("UNAUTHORIZED"));
        vm.prank(alice);
        book.setSafeBlockThreshold(newThreshold);

        vm.expectEmit(true, false, false, true, address(book));
        emit SafeBlockThresholdChanged(newThreshold);
        book.setSafeBlockThreshold(newThreshold);
        uint256 storageThreshold = stdstore
            .target(address(book))
            .sig(book.safeBlockThreshold.selector)
            .read_uint();
        assertEq(
            storageThreshold,
            newThreshold,
            "Threashold should be updated"
        );
    }

    function testMaxFeePctChange(uint128 newFee) public {
        // should fail if not called by the owner
        vm.expectRevert(bytes("UNAUTHORIZED"));
        vm.prank(alice);
        book.setMaxFeePct(newFee);
        if (newFee >= 1e18) {
            vm.expectRevert(BookSingleChain__NewFeePctTooHigh.selector);
            book.setMaxFeePct(newFee);
            return;
        }
        vm.expectEmit(true, false, false, true, address(book));
        emit MaxFeePctChanged(newFee);
        book.setMaxFeePct(newFee);
        uint128 storageFeePct = uint128(
            stdstore
                .target(address(book))
                .sig(book.maxFeePct.selector)
                .read_uint()
        );
        assertEq(storageFeePct, newFee, "Max fee should be updated");
    }
}
