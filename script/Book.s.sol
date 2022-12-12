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
        require(disputeBondPct + tradeRebatePct + relayerRefundPct == 100, "invalid fee combination");
        require(feePct < 2500);
        require(disputeBondPct > 0);
        require(tradeRebatePct > 0);
        require(relayerRefundPct > 0);
        require(safeBlockThreshold > 0);

        book = new Book(registry, safeBlockThreshold, disputeBondPct, tradeRebatePct, relayerRefundPct, feePct);
    }
}
