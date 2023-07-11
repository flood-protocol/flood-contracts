// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {DecoderWithRegistryScript} from "script/DecoderWithRegistry.s.sol";
import {Test} from "forge-std/Test.sol";
import {DecoderWithRegistry} from "src/decoders/extensions/DecoderWithRegistry.sol";
import {FloodPlainTestShared, IFloodPlain} from "../utils/FloodPlainTestShared.sol";

struct IdToAddress {
    uint256 id;
    address addr;
}

contract DecoderWithRegistryScriptTest is FloodPlainTestShared, DecoderWithRegistryScript {
    DecoderWithRegistry registryDecoder;

    function setUp() public override {
        super.setUp();
        vm.prank(msg.sender);
        registryDecoder = new DecoderWithRegistry();
    }

    function testAddTokens() public {
        address[] memory tokensToAdd = new address[](2);
        tokensToAdd[0] = address(token0);
        tokensToAdd[1] = address(token1);
        this.addTokens(registryDecoder, tokensToAdd);
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
        this.addTokens(registryDecoder, tokensToAdd);
        vm.expectEmit();
        emit Duplicated(0, address(token0));
        emit Duplicated(1, address(token1));
        this.addTokens(registryDecoder, tokensToAdd);
        assertEq(registryDecoder.tokens().length, 2);
        assertEq(registryDecoder.tokens()[0].addr, address(token0));
        assertEq(registryDecoder.tokens()[1].addr, address(token1));
        assertEq(registryDecoder.tokens()[0].id, 0);
        assertEq(registryDecoder.tokens()[1].id, 1);
    }

    function testAddZones() public {
        address[] memory zonesToAdd = new address[](1);
        zonesToAdd[0] = address(zone);
        this.addZones(registryDecoder, zonesToAdd);
        assertEq(registryDecoder.zones().length, 1);
        assertEq(registryDecoder.zones()[0].addr, address(zone));
        assertEq(registryDecoder.zones()[0].id, 0);
    }

    function testAddDuplicatedZones() public {
        address[] memory zonesToAdd = new address[](1);
        zonesToAdd[0] = address(zone);
        this.addZones(registryDecoder, zonesToAdd);
        vm.expectEmit();
        emit Duplicated(0, address(zone));
        this.addZones(registryDecoder, zonesToAdd);
        assertEq(registryDecoder.zones().length, 1);
        assertEq(registryDecoder.zones()[0].addr, address(zone));
        assertEq(registryDecoder.zones()[0].id, 0);
    }

    function testAddFulfillers() public {
        address[] memory fulfillersToAdd = new address[](1);
        fulfillersToAdd[0] = address(fulfiller);
        this.addFulfillers(registryDecoder, fulfillersToAdd);
        assertEq(registryDecoder.fulfillers().length, 1);
        assertEq(registryDecoder.fulfillers()[0].addr, address(fulfiller));
        assertEq(registryDecoder.fulfillers()[0].id, 0);
    }

    function testAddDuplicatedFulfillers() public {
        address[] memory fulfillersToAdd = new address[](1);
        fulfillersToAdd[0] = address(fulfiller);
        this.addFulfillers(registryDecoder, fulfillersToAdd);
        vm.expectEmit();
        emit Duplicated(0, address(fulfiller));
        this.addFulfillers(registryDecoder, fulfillersToAdd);
        assertEq(registryDecoder.fulfillers().length, 1);
        assertEq(registryDecoder.fulfillers()[0].addr, address(fulfiller));
        assertEq(registryDecoder.fulfillers()[0].id, 0);
    }




} 
