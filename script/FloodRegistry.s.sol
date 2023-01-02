// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.17;

import "forge-std/Script.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {AllKnowingOracle} from "src/AllKnowingOracle.sol";

contract FloodRegistryScript is Script {
    function run() public virtual {
        address WETH_ADDRESS = vm.envAddress("WETH_ADDRESS");
        vm.broadcast();
        _deployRegistry(IWETH9(WETH_ADDRESS));
    }

    function _deployRegistry(IWETH9 weth) internal returns (FloodRegistry registry) {
        registry = new FloodRegistry(weth);
    }

    function setOracle(FloodRegistry registry, AllKnowingOracle oracle) public {
        vm.broadcast();
        registry.setOracle(oracle);
    }

    function whitelistToken(FloodRegistry registry, address token, bool isWhitelisted) public {
        vm.broadcast();
        registry.whitelistToken(token, isWhitelisted);
    }

    function batchWhitelistTokens(FloodRegistry registry, address[] memory tokens, bool[] memory isWhitelisted)
        public
    {
        vm.broadcast();
        registry.batchWhitelistTokens(tokens, isWhitelisted);
    }
}
