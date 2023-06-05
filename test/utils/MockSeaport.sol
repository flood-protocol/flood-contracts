// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {Consideration} from "seaport-core/lib/Consideration.sol";

contract MockSeaport is Consideration {
    constructor(address conduitController) Consideration(conduitController) {}

    function _name() internal pure override returns (string memory) {
        return _nameString();
    }

    function _nameString() internal pure override returns (string memory) {
        return "Seaport";
    }
}
