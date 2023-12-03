// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {BetterScript, console} from "./BetterScript.s.sol";
import {FloodPlain} from "src/FloodPlain.sol";

contract FloodPlainScript is BetterScript {
    address PERMIT2 = 0x000000000022D473030F116dDEE9F6B43aC78BA3;

    function run() public {
        console.logBytes32(keccak256(bytes.concat(type(FloodPlain).creationCode, abi.encode(PERMIT2))));

        bytes32 SALT = 0x45bddd7a4404868c5a41cb716e01a4006b38bab06c000000000000000001abdd;
        vm.broadcast();
        console.log("FloodPlain deployed at ", deploy2("FloodPlain", SALT, abi.encode(PERMIT2)));
    }

    function addDecoder(FloodPlain flood, address decoder) public {
        vm.broadcast();
        uint256 id = flood.addDecoder(decoder);
        console.log("Decoder", decoder, "added with id", id);
    }
}
