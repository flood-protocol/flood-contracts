// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {BetterScript, console} from "./BetterScript.s.sol";
import {Zone, IAuthZone} from "src/Zone.sol";

contract ZoneScript is BetterScript {
    function run() public {
        address zoneAdmin = vm.envAddress("ZONE_ADMIN_ADDRESS");
        bytes memory creationCode = type(Zone).creationCode;

        bytes32 SALT = 0x45bddd7a4404868c5a41cb716e01a4006b38bab05375c9e3e58c000000ca7d4a;
        vm.broadcast(zoneAdmin);
        address zone = deploy3(creationCode, SALT, abi.encode(zoneAdmin));

        console.log("Zone deployed at", zone);
        require(Zone(zone).owner() == zoneAdmin, "Zone: wrong owner");
    }

    function addFulfiller(Zone zone, address fulfiller) public {
        address zoneAdmin = vm.envAddress("ZONE_ADMIN_ADDRESS");
        IAuthZone.AddressFilter memory emptyAddressFilter = IAuthZone.AddressFilter({value: address(0), exclude: false});
        IAuthZone.RangeFilter memory emptyRangeFilter = IAuthZone.RangeFilter({gte: 0, lte: 0});
        IAuthZone.ItemFilter memory emptyItemFilter =
            IAuthZone.ItemFilter({token: emptyAddressFilter, amount: emptyRangeFilter});
        IAuthZone.ItemFilter[] memory emptyItemFilters;

        vm.startBroadcast(zoneAdmin);
        zone.grantRole(zone.FULFILLER_ROLE(), fulfiller);
        zone.setAuthorizationFilter(
            fulfiller,
            IAuthZone.AuthFilter({
                initialized: true,
                offerer: emptyAddressFilter,
                offer: emptyItemFilters,
                consideration: emptyItemFilter,
                deadline: emptyRangeFilter,
                nonce: emptyRangeFilter
            })
        );
        vm.stopBroadcast();
    }
}
