// SPDX-License-Identifier: Unlincensed
pragma solidity ^0.8.15;

import "src/AllKnowingOracle.sol";
import "src/BookSingleChain.sol";
import "forge-std/Script.sol";

contract DeployScript is Script {
    function run() public {
        // A bit more than 2hrs on mainnet
        uint256 safeThreshold = 500;
        vm.startBroadcast();
        AllKnowingOracle oracle = new AllKnowingOracle();
        new BookSingleChain(safeThreshold, address(oracle));
        vm.stopBroadcast();
    }
}
