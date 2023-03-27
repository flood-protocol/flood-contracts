// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Script.sol";
import "forge-std/Test.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {Book} from "src/Book.sol";
import {FloodRegistryScript} from "./FloodRegistry.s.sol";
import {BookScript} from "./Book.s.sol";

contract DeployScript is BookScript, FloodRegistryScript {
    FloodRegistry internal registry;

    function run() public override {
        uint256 feePct = vm.envUint("FEE_PCT");
        address REGISTRY_ADDRESS = vm.envAddress("REGISTRY_ADDRESS");
        address WETH_ADDRESS = vm.envAddress("WETH_ADDRESS");
        vm.startBroadcast();

        if (REGISTRY_ADDRESS == address(0)) registry = _deployRegistry(IWETH9(WETH_ADDRESS));
        else registry = FloodRegistry(REGISTRY_ADDRESS);

        _deployBook(registry, feePct);
        vm.stopBroadcast();
    }
}
