// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {IAllKnowingOracleEvents} from "src/AllKnowingOracle.sol";
import {OracleFixture} from "./Fixtures.sol";

contract AdminTest is IAllKnowingOracleEvents, OracleFixture {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
    }

    function testWhitelistSettler(address settler, bool enabled) public {
        vm.expectEmit(true, false, false, true, address(oracle));
        emit SettlerWhitelisted(settler, enabled);
        oracle.whitelistSettler(settler, enabled);

        bool storageEnabled =
            stdstore.target(address(oracle)).sig(oracle.settlers.selector).with_key(settler).read_bool();

        assertEq(storageEnabled, enabled);
    }

    function testCannotWhitelistSettlerIfNonOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(alice);
        oracle.whitelistSettler(bob, true);
    }
}
