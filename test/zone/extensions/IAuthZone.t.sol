// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";
import {MockERC20} from "test/flood-plain/utils/MockERC20.sol";
import {MainZone} from "src/zone/extensions/MainZone.sol";
import {IAuthZone} from "src/zone/extensions/IAuthZone.sol";
import {AuthZoneFilter} from "test/zone/utils/AuthZoneFilter.sol";

contract ZoneTest is Test {
    Account account0;
    Account account1;
    MainZone zone;
    MockERC20 token0;

    event FilterUpdated(address indexed actor, IAuthZone.Filter filter);

    function setUp() public virtual {
        account0 = makeAccount("a");
        account1 = makeAccount("b");
        zone = new MainZone(account0.addr);
        token0 = new MockERC20();
    }

    function test_auth() public {
        MainZone.RangeFilter memory rangeFilter = IAuthZone.RangeFilter({gte: 0, lte: 100});

        IAuthZone.ItemFilter memory tokenFilter = IAuthZone.ItemFilter({token: address(token0), amount: rangeFilter});

        IAuthZone.ItemFilter[] memory filters = new IAuthZone.ItemFilter[](1);
        filters[0] = tokenFilter;

        IAuthZone.Filter memory filter = IAuthZone.Filter({
            initialized: true,
            offerer: account0.addr,
            offer: filters,
            consideration: tokenFilter,
            deadline: rangeFilter,
            nonce: rangeFilter
        });

        assertTrue(AuthZoneFilter.isFilterEqual(zone.authorizationFilter(account1.addr), AuthZoneFilter.allowNone()));

        vm.startPrank(account0.addr);
        vm.expectEmit();
        emit FilterUpdated(account1.addr, filter);
        zone.setAuthorizationFilter(account1.addr, filter);

        assertTrue(AuthZoneFilter.isFilterEqual(zone.authorizationFilter(account1.addr), filter));
    }
}
