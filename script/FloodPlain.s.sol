// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {BetterScript, console} from "./BetterScript.s.sol";
import {FloodPlain} from "src/FloodPlain.sol";

contract FloodPlainScript is BetterScript {
    address PERMIT2 = 0x000000000022D473030F116dDEE9F6B43aC78BA3;

    function run() public {
        bytes memory creationCode = type(FloodPlain).creationCode;
        console.logBytes32(keccak256(bytes.concat(creationCode, abi.encode(PERMIT2))));

        bytes32 SALT = 0xb58000000000000000000000c883f10d58199e71d6d980ab76441ae57a58f581;
        vm.broadcast();
        console.log("FloodPlain deployed at ", deploy3(creationCode, SALT, abi.encode(PERMIT2)));
    }

    function addDecoder(FloodPlain flood, address decoder) public {
        uint256 key = vm.envUint("ALT_ADMIN_KEY");
        vm.broadcast(key);
        uint256 id = flood.addDecoder(decoder);
        console.log("Decoder", decoder, "added with id", id);
    }
}
