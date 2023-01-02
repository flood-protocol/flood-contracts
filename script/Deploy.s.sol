// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Script.sol";
import "forge-std/Test.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {AllKnowingOracle} from "src/AllKnowingOracle.sol";
import {Book} from "src/Book.sol";
import {FloodRegistryScript} from "./FloodRegistry.s.sol";
import {AllKnowingOracleScript} from "./AllKnowingOracle.s.sol";
import {BookScript} from "./Book.s.sol";

contract DeployScript is BookScript, AllKnowingOracleScript, FloodRegistryScript, Test {
    AllKnowingOracle internal oracle;
    FloodRegistry internal registry;

    function run() public override {
        uint256 safeBlockThreshold = vm.envUint("SAFE_BLOCK_THRESHOLD");
        uint256 disputeBondPct = vm.envUint("DISPUTE_BOND_PCT");
        uint256 tradeRebatePct = vm.envUint("TRADE_REBATE_PCT");
        uint256 relayerRefundPct = vm.envUint("RELAYER_REFUND_PCT");
        uint256 feePct = vm.envUint("FEE_PCT");
        address REGISTRY_ADDRESS = vm.envAddress("REGISTRY_ADDRESS");
        address ORACLE_ADDRESS = vm.envAddress("ORACLE_ADDRESS");
        address WETH_ADDRESS = vm.envAddress("WETH_ADDRESS");
        vm.startBroadcast();

        if (REGISTRY_ADDRESS == address(0)) registry = _deployRegistry(IWETH9(WETH_ADDRESS));
        else registry = FloodRegistry(REGISTRY_ADDRESS);

        if (ORACLE_ADDRESS == address(0)) {
            oracle = _deployOracle(registry);
        } else {
            oracle = AllKnowingOracle(ORACLE_ADDRESS);
        }
        registry.setOracle(oracle);
        _deployBook(registry, safeBlockThreshold, disputeBondPct, tradeRebatePct, relayerRefundPct, feePct);
        vm.stopBroadcast();
    }
}
