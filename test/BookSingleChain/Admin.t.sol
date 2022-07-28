// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "src/BookSingleChain.sol";
import "forge-std/Test.sol";
import "./Fixtures.sol";

contract AdminTest is BaseBookFixture {
    using stdStorage for StdStorage;

    function testTokenWhitelist(address token, bool enabled) public {
        // pre whitelist the token in the oracle
        if (enabled) {
            oracle.whitelistToken(token, enabled);
        }

        // check that the whitelisted mapping starts as false
        bool whitelistedBefore = stdstore.target(address(book)).sig(
            book.whitelistedTokens.selector
        ).with_key(token).read_bool();
        assertFalse(whitelistedBefore);

        vm.expectEmit(true, false, false, true, address(book));
        emit TokenWhitelisted(token, enabled);
        book.whitelistToken(token, enabled);
        // check that the whitelisted mapping has correctly been updated
        bool realWhitelisted = stdstore.target(address(book)).sig(
            book.whitelistedTokens.selector
        ).with_key(token).read_bool();
        assertEq(realWhitelisted, enabled, "Token should be whitelisted");
    }

    function testCannotWhitelistAsNonOwner(address token, bool enabled)
        public
    {
        vm.expectRevert(bytes("UNAUTHORIZED"));
        vm.prank(alice);
        book.whitelistToken(token, enabled);
    }

    function testCannotWhitelistUnsafeToken(address token) public {
        vm.assume(oracle.whitelistedTokens(token) == false);
        vm.expectRevert(
            abi.encodeWithSelector(BookSingleChain__UnsafeTokenToWhitelist.selector, token)
        );
        book.whitelistToken(token, true);
    }

    function testThresholdChange(uint256 newThreshold) public {
        // should fail if not called by the owner
        vm.expectRevert(bytes("UNAUTHORIZED"));
        vm.prank(alice);
        book.setSafeBlockThreshold(newThreshold);

        vm.expectEmit(true, false, false, true, address(book));
        emit SafeBlockThresholdChanged(newThreshold);
        book.setSafeBlockThreshold(newThreshold);
        uint256 storageThreshold = stdstore.target(address(book)).sig(
            book.safeBlockThreshold.selector
        ).read_uint();
        assertEq(storageThreshold, newThreshold, "Threashold should be updated");
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
            stdstore.target(address(book)).sig(book.maxFeePct.selector).read_uint()
        );
        assertEq(storageFeePct, newFee, "Max fee should be updated");
    }
}
