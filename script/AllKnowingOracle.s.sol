// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.17;

import "forge-std/Script.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {AllKnowingOracle} from "src/AllKnowingOracle.sol";

contract AllKnowingOracleScript is Script {
    function run(FloodRegistry registry) public virtual {
        vm.broadcast();
        _deployOracle(registry);
    }

    function _deployOracle(FloodRegistry registry) internal returns (AllKnowingOracle oracle) {
        oracle = new AllKnowingOracle(registry);
    }

    function setSettlers(AllKnowingOracle oracle, address settler, bool enable) public {
        vm.broadcast();
        oracle.whitelistSettler(settler, enable);
    }
}
