// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {
    FloodRegistry,
    IFloodRegistryEvents,
    FloodRegistry__InvalidToken,
    FloodRegistry__TokenAlreadyWhitelisted,
    FloodRegistry__TokenNotWhitelisted,
    FloodRegistry__RelayerAlreadyWhitelisted,
    FloodRegistry__RelayerNotWhitelisted
} from "src/FloodRegistry.sol";
import {TokenFixture} from "../utils/TokenFixture.sol";

contract FloodRegistryTest is TokenFixture, IFloodRegistryEvents {
    using stdStorage for StdStorage;

    FloodRegistry internal registry;

    function setUp() public {
        registry = new FloodRegistry(WETH);
    }

    function testWhitelistToken() public {
        vm.expectEmit(address(registry));
        emit TokenWhitelisted(USDC, true);
        registry.whitelistToken(USDC, true);
        assertTrue(registry.isTokenWhitelisted(USDC));
        vm.expectRevert(FloodRegistry__TokenAlreadyWhitelisted.selector);
        registry.whitelistToken(USDC, true);

        vm.expectEmit(address(registry));
        emit TokenWhitelisted(USDC, false);
        registry.whitelistToken(USDC, false);
        assertFalse(registry.isTokenWhitelisted(USDC));
        vm.expectRevert(FloodRegistry__TokenNotWhitelisted.selector);
        registry.whitelistToken(USDC, false);

        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(address(2));
        registry.whitelistToken(USDC, true);

        vm.expectRevert(FloodRegistry__InvalidToken.selector);
        registry.whitelistToken(IERC20(address(0)), true);
    }

    function testWhitelistRelayer() public {
        vm.expectEmit(address(registry));
        emit RelayerWhitelisted(address(1), true);
        registry.whitelistRelayer(address(1), true);
        assertTrue(registry.isRelayerWhitelisted(address(1)));
        vm.expectRevert(FloodRegistry__RelayerAlreadyWhitelisted.selector);
        registry.whitelistRelayer(address(1), true);

        vm.expectEmit(address(registry));
        emit RelayerWhitelisted(address(1), false);
        registry.whitelistRelayer(address(1), false);
        assertFalse(registry.isRelayerWhitelisted(address(1)));
        vm.expectRevert(FloodRegistry__RelayerNotWhitelisted.selector);
        registry.whitelistRelayer(address(1), false);

        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(address(2));
        registry.whitelistRelayer(address(1), true);
    }

    function testAreTokensWhitelisted() public {
        IERC20[] memory tokens = new IERC20[](3);
        tokens[0] = USDC;
        tokens[1] = USDT;
        tokens[2] = DAI;

        registry.whitelistToken(tokens[1], true);
        assertFalse(registry.areTokensWhitelisted(tokens));

        registry.whitelistToken(tokens[0], true);
        registry.whitelistToken(tokens[2], true);
        assertTrue(registry.areTokensWhitelisted(tokens));
    }

    function testBatchWhitelistTokens() public {
        IERC20[] memory tokens = new IERC20[](3);
        tokens[0] = USDC;
        tokens[1] = USDT;
        tokens[2] = DAI;

        bool[] memory enabled = new bool[](3);
        enabled[0] = true;
        enabled[1] = false;
        enabled[2] = true;

        registry.whitelistToken(tokens[1], true);

        for (uint256 i = 0; i < tokens.length; i++) {
            vm.expectEmit(address(registry));
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
}
