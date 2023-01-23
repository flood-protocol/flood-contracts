// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {FloodRegistry, IFloodRegistryEvents} from "src/FloodRegistry.sol";
import {AllKnowingOracle} from "src/AllKnowingOracle.sol";
import {TokenFixture} from "../utils/TokenFixture.sol";

contract FloodRegistryTest is TokenFixture, IFloodRegistryEvents {
    using stdStorage for StdStorage;

    FloodRegistry internal registry;
    IWETH9 internal testWeth = IWETH9(address(1));

    function setUp() public {
        registry = new FloodRegistry(testWeth);
    }

    function testWhitelistToken() public {
        address token = USDC;
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
        tokens[0] = USDC;
        tokens[1] = WETH;
        tokens[2] = DAI;

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
