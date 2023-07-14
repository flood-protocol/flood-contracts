// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Script, console2 as console} from "forge-std/Script.sol";
import {MainZone} from "src/zone/extensions/MainZone.sol";
import {Create2Deploy} from "./Create2Deploy.sol";

contract MainZoneScript is Script, Create2Deploy {
    function run() public {
        address zoneAdmin = vm.envAddress("ZONE_ADMIN_ADDRESS");
        console.logBytes32(keccak256(bytes.concat(type(MainZone).creationCode, abi.encode(zoneAdmin))));
        string memory profile = vm.envString("FOUNDRY_PROFILE");
        require(
            keccak256(bytes(profile)) == keccak256(bytes("deploy")), " not deploy profile"
        );
        SALT = 0x45bddd7a4404868c5a41cb716e01a4006b38bab06c000000000000000001abdd;
        vm.broadcast(zoneAdmin);
        address zone = deploy2("MainZone", SALT, abi.encode(zoneAdmin));

        console.log("MainZone deployed at", zone);
        require(MainZone(zone).owner() == zoneAdmin, "MainZone: wrong owner");
    }
}
