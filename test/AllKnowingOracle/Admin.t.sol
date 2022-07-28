// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "src/AllKnowingOracle.sol";
import "./Fixtures.sol";
import "forge-std/Test.sol";

contract AdminTest is IAllKnowingOracleEvents, OracleFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testWhitelistToken(address token, bool enabled) public {
        vm.expectEmit(true, false, false, true, address(oracle));
        emit TokenWhitelisted(token, enabled);
        oracle.whitelistToken(token, enabled);

        bool storageEnabled = stdstore.target(address(oracle)).sig(
            oracle.whitelistedTokens.selector
        ).with_key(token).read_bool();

        assertEq(storageEnabled, enabled);
    }

    function testCannotWhitelistIfNonOwner() public {
        vm.expectRevert("UNAUTHORIZED");
        vm.prank(alice);
        oracle.whitelistToken(USDC, true);
    }

    function testWhitelistSettler(address settler, bool enabled) public {
        vm.expectEmit(true, false, false, true, address(oracle));
        emit SettlerWhitelisted(settler, enabled);
        oracle.whitelistSettler(settler, enabled);

        bool storageEnabled = stdstore.target(address(oracle)).sig(
            oracle.settlers.selector
        ).with_key(settler).read_bool();

        assertEq(storageEnabled, enabled);
    }

    function testCannotWhitelistSettlerIfNonOwner() public {
        vm.expectRevert("UNAUTHORIZED");
        vm.prank(alice);
        oracle.whitelistSettler(bob, true);
    }
}
