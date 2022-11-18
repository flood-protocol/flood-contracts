// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "src/FloodRegistry.sol";

contract FloodRegistryTest is Test, IFloodRegistryEvents {
    using stdStorage for StdStorage;

    FloodRegistry internal registry;

    function setUp() public {
        registry = new FloodRegistry();
    }

    function testWhitelistToken(address token) public {
        vm.expectEmit(true, false, false, true, address(registry));
        emit TokenWhitelisted(token, true);
        registry.whitelistToken(token, true);
        assertTrue(registry.isTokenWhitelisted(token));
        assertEq(stdstore.target(address(registry)).sig(registry.tokenIndexes.selector).with_key(token).read_uint(), 1);
        vm.expectEmit(true, false, false, true, address(registry));
        emit TokenWhitelisted(token, false);
        registry.whitelistToken(token, false);
        assertFalse(registry.isTokenWhitelisted(token));
        vm.expectRevert("UNAUTHORIZED");
        vm.prank(address(2));
        registry.whitelistToken(token, true);
    }

    function testSetOracle(AllKnowingOracle oracle) public {
        vm.expectEmit(true, false, false, true, address(registry));
        emit OracleChanged(oracle);
        registry.setOracle(oracle);
        assertEq(address(registry.latestOracle()), address(oracle));
        vm.expectRevert("UNAUTHORIZED");
        vm.prank(address(2));
        registry.setOracle(oracle);
    }
}
