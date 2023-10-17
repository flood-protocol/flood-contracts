// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {DecoderWithRegistryScript} from "script/DecoderWithRegistry.s.sol";
import {Test} from "forge-std/Test.sol";
import {DecoderWithRegistry} from "src/decoders/extensions/DecoderWithRegistry.sol";
import {MockZone} from "../utils/MockZone.sol";
import {MockERC20} from "../utils/MockERC20.sol";
import {MockFulfiller} from "../utils/MockFulfiller.sol";

struct IdToAddress {
    uint256 id;
    address addr;
}

contract DecoderWithRegistryScriptTest is DecoderWithRegistryScript, Test {
    DecoderWithRegistry registryDecoder;
    MockZone zone = new MockZone();
    MockFulfiller fulfiller;
    MockERC20 token0;
    MockERC20 token1;

    function setUp() public {
        registryDecoder = new DecoderWithRegistry(msg.sender);
        zone = new MockZone();
        fulfiller = new MockFulfiller();
        token0 = new MockERC20();
        token1 = new MockERC20();
    }

    function run() public override(DecoderWithRegistryScript) {
        super.run();
    }

    function testAddTokens() public {
        address[] memory tokensToAdd = new address[](2);
        tokensToAdd[0] = address(token0);
        tokensToAdd[1] = address(token1);
        addTokens(registryDecoder, tokensToAdd);
        assertEq(registryDecoder.tokens().length, 2);
        assertEq(registryDecoder.tokens()[0].addr, address(token0));
        assertEq(registryDecoder.tokens()[1].addr, address(token1));
        assertEq(registryDecoder.tokens()[0].id, 0);
        assertEq(registryDecoder.tokens()[1].id, 1);
    }

    function testAddDuplicatedTokens() public {
        address[] memory tokensToAdd = new address[](2);
        tokensToAdd[0] = address(token0);
        tokensToAdd[1] = address(token1);
        addTokens(registryDecoder, tokensToAdd);
        vm.expectEmit();
        emit Duplicated(0, address(token0));
        emit Duplicated(1, address(token1));
        addTokens(registryDecoder, tokensToAdd);
        assertEq(registryDecoder.tokens().length, 2);
        assertEq(registryDecoder.tokens()[0].addr, address(token0));
        assertEq(registryDecoder.tokens()[1].addr, address(token1));
        assertEq(registryDecoder.tokens()[0].id, 0);
        assertEq(registryDecoder.tokens()[1].id, 1);
    }

    function testAddZones() public {
        address[] memory zonesToAdd = new address[](1);
        zonesToAdd[0] = address(zone);
        addZones(registryDecoder, zonesToAdd);
        assertEq(registryDecoder.zones().length, 1);
        assertEq(registryDecoder.zones()[0].addr, address(zone));
        assertEq(registryDecoder.zones()[0].id, 0);
    }

    function testAddDuplicatedZones() public {
        address[] memory zonesToAdd = new address[](1);
        zonesToAdd[0] = address(zone);
        addZones(registryDecoder, zonesToAdd);
        vm.expectEmit();
        emit Duplicated(0, address(zone));
        addZones(registryDecoder, zonesToAdd);
        assertEq(registryDecoder.zones().length, 1);
        assertEq(registryDecoder.zones()[0].addr, address(zone));
        assertEq(registryDecoder.zones()[0].id, 0);
    }

    function testAddFulfillers() public {
        address[] memory fulfillersToAdd = new address[](1);
        fulfillersToAdd[0] = address(fulfiller);
        addFulfillers(registryDecoder, fulfillersToAdd);
        assertEq(registryDecoder.fulfillers().length, 1);
        assertEq(registryDecoder.fulfillers()[0].addr, address(fulfiller));
        assertEq(registryDecoder.fulfillers()[0].id, 0);
    }

    function testAddDuplicatedFulfillers() public {
        address[] memory fulfillersToAdd = new address[](1);
        fulfillersToAdd[0] = address(fulfiller);
        addFulfillers(registryDecoder, fulfillersToAdd);
        vm.expectEmit();
        emit Duplicated(0, address(fulfiller));
        addFulfillers(registryDecoder, fulfillersToAdd);
        assertEq(registryDecoder.fulfillers().length, 1);
        assertEq(registryDecoder.fulfillers()[0].addr, address(fulfiller));
        assertEq(registryDecoder.fulfillers()[0].id, 0);
    }
}
