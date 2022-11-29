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
        vm.expectEmit(true, false, false, true, address(registry));
        emit TokenWhitelisted(token, false);
        registry.whitelistToken(token, false);
        assertFalse(registry.isTokenWhitelisted(token));
        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(address(2));
        registry.whitelistToken(token, true);
    }

    function testBatchWhitelistTokens() public {
        address[] memory tokens = new address[](3);
        tokens[0] = address(1);
        tokens[1] = address(2);
        tokens[2] = address(3);

        bool[] memory enabled = new bool[](3);
        enabled[0] = true;
        enabled[1] = false;
        enabled[2] = true;

        registry.whitelistToken(tokens[1], true);

        for (uint256 i = 0; i < tokens.length; i++) {
            vm.expectEmit(true, false, false, true, address(registry));
            emit TokenWhitelisted(tokens[i], enabled[i]);
        }
        registry.batchWhitelistTokens(tokens, enabled);
        for (uint256 i = 0; i < tokens.length; i++) {
            assertEq(registry.isTokenWhitelisted(tokens[i]), enabled[i]);
        }

        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(address(2));
        registry.batchWhitelistTokens(tokens, enabled);
    }

    function testSetOracle(AllKnowingOracle oracle) public {
        vm.expectEmit(true, false, false, true, address(registry));
        emit OracleChanged(oracle);
        registry.setOracle(oracle);
        assertEq(address(registry.latestOracle()), address(oracle));
        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(address(2));
        registry.setOracle(oracle);
    }
}
