// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Script, console2 as console} from "forge-std/Script.sol";
import {FloodPlainFull} from "src/flood-plain/FloodPlainFull.sol";
import {Create2Deploy} from "./Create2Deploy.sol";

contract FloodPlainFullScript is Script, Create2Deploy {
    address internal PERMIT2 = 0x000000000022D473030F116dDEE9F6B43aC78BA3;

    function run() public {
        address floodPlainAdmin = vm.envAddress("FLOOD_PLAIN_ADMIN_ADDRESS");
        console.logBytes32(
            keccak256(bytes.concat(type(FloodPlainFull).creationCode, abi.encode(PERMIT2, floodPlainAdmin)))
        );
        string memory profile = vm.envString("FOUNDRY_PROFILE");
        require(keccak256(bytes(profile)) == keccak256(bytes("deploy")), " not deploy profile");
        SALT = 0x45bddd7a4404868c5a41cb716e01a4006b38bab02400000000000000020efcf5;
        vm.broadcast(floodPlainAdmin);
        address floodPlain = deploy2("FloodPlainFull", SALT, abi.encode(PERMIT2, floodPlainAdmin));

        console.log("FloodPlain deployed at", floodPlain);
        require(FloodPlainFull(payable(floodPlain)).owner() == floodPlainAdmin, "FloodPlain: wrong owner");
    }
}
