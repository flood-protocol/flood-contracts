// SPDX-License-Identifier: Unlicense
pragma solidity 0.8.17;

import "forge-std/Script.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {Book} from "src/Book.sol";

contract BookScript is Script {
    function run(FloodRegistry registry, uint256 feePct) public virtual {
        vm.broadcast();
        _deployBook(registry, feePct);
    }

    function _deployBook(FloodRegistry registry, uint256 feePct) internal returns (Book book) {}
}
