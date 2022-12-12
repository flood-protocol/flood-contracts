// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.17;

import "forge-std/Script.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {AllKnowingOracle} from "src/AllKnowingOracle.sol";

contract FloodRegistryScript is Script {
    function run() public virtual {
        vm.broadcast();
        _deployRegistry();
    }

    function _deployRegistry() internal returns (FloodRegistry registry) {
        registry = new FloodRegistry();
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
