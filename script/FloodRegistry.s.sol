// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "forge-std/Script.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";

contract FloodRegistryScript is Script {
    function run() public virtual {
        address WETH_ADDRESS = vm.envAddress("WETH_ADDRESS");
        vm.broadcast();
        _deployRegistry(IWETH9(WETH_ADDRESS));
    }

    function _deployRegistry(IWETH9 weth) internal returns (FloodRegistry registry) {
        registry = new FloodRegistry(weth);
    }

    function whitelistToken(FloodRegistry registry, IERC20 token, bool isWhitelisted) public {
        vm.broadcast();
        registry.whitelistToken(token, isWhitelisted);
    }

    function batchWhitelistTokens(FloodRegistry registry, IERC20[] memory tokens, bool[] memory isWhitelisted) public {
        vm.broadcast();
        registry.batchWhitelistTokens(tokens, isWhitelisted);
    }
}
