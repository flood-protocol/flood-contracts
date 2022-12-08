// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.17;

import "forge-std/Script.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {AllKnowingOracle} from "src/AllKnowingOracle.sol";
import {Book} from "src/Book.sol";

contract BookScript is Script {
    function run(FloodRegistry registry, uint256 safeBlockThreshold,
        uint256 disputeBondPct,
        uint256 tradeRebatePct,
        uint256 relayerRefundPct,
        uint256 feePct) public virtual {
        vm.broadcast();
        _deployBook(registry, safeBlockThreshold, disputeBondPct, tradeRebatePct, relayerRefundPct, feePct);
    }

    function _deployBook(
        FloodRegistry registry,
        uint256 safeBlockThreshold,
        uint256 disputeBondPct,
        uint256 tradeRebatePct,
        uint256 relayerRefundPct,
        uint256 feePct) internal returns (Book book) {
        book = new Book(registry, safeBlockThreshold, disputeBondPct, tradeRebatePct, relayerRefundPct, feePct);
    }
}
