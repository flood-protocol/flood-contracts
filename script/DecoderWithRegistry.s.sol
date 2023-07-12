// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Script, console2 as console} from "forge-std/Script.sol";
import {DecoderWithRegistry, IdToAddress} from "src/decoders/extensions/DecoderWithRegistry.sol";

contract DecoderWithRegistryScript is Script {

    event Duplicated(uint256, address);

    function run() public virtual {
        vm.broadcast();
        DecoderWithRegistry decoder = new DecoderWithRegistry();
        console.log("Deployed DecoderWithRegistry at", address(decoder));
    }

    function addTokens(DecoderWithRegistry decoder, address[] memory tokens) public {
        vm.startBroadcast();
        _addTokens(decoder, tokens);
        vm.stopBroadcast();
    }

    function addZones(DecoderWithRegistry decoder, address[] memory zones) public {
        vm.startBroadcast();
        _addZones(decoder, zones);
        vm.stopBroadcast();
    }

    function addFulfillers(DecoderWithRegistry decoder, address[] memory fulfillers) public {
        vm.startBroadcast();
        _addFulfillers(decoder, fulfillers);
        vm.stopBroadcast();
    }

function _addTokens(DecoderWithRegistry decoder, address[] memory tokensToAdd) internal {
    IdToAddress[] memory tokens = decoder.tokens();
    uint8 tokensLength = uint8(tokens.length);
    uint256 tokensToAddLength = tokensToAdd.length;
    for (uint8 i; i < tokensToAddLength;) {
        bool isDuplicate;
        for (uint256 j; j < tokensLength;) {
            if (tokens[j].addr == tokensToAdd[i]) {
                isDuplicate = true;
                emit Duplicated(j, tokens[j].addr);
                break;
            }
            unchecked {
                ++j;
            }
        }
        if (!isDuplicate) {
            decoder.setToken(tokensLength + i, tokensToAdd[i]);
        }
        unchecked {
            ++i;
        }
    }
}

function _addZones(DecoderWithRegistry decoder, address[] memory zonesToAdd) internal {
    IdToAddress[] memory zones = decoder.zones();
    uint8 zonesLength = uint8(zones.length);
    uint256 zonesToAddLength = zonesToAdd.length;
    for (uint8 i; i < zonesToAddLength;) {
        bool isDuplicate;
        for (uint256 j; j < zonesLength;) {
            if (zones[j].addr == zonesToAdd[i]) {
                isDuplicate = true;
                emit Duplicated(j, zones[j].addr);
                break;
            }
            unchecked {
                ++j;
            }
        }
        if (!isDuplicate) {
            decoder.setZone(zonesLength + i, zonesToAdd[i]);
        }
        unchecked {
            ++i;
        }
    }
}

function _addFulfillers(DecoderWithRegistry decoder, address[] memory fulfillersToAdd) internal {
    IdToAddress[] memory fulfillers = decoder.fulfillers();
    uint8 fulfillersLength = uint8(fulfillers.length);
    uint256 fulfillersToAddLength = fulfillersToAdd.length;
    for (uint8 i; i < fulfillersToAddLength;) {
        bool isDuplicate;
        for (uint256 j; j < fulfillersLength;) {
            if (fulfillers[j].addr == fulfillersToAdd[i]) {
                isDuplicate = true;
                emit Duplicated(j, fulfillers[j].addr);
                break;
            }
            unchecked {
                ++j;
            }
        }
        if (!isDuplicate) {
            decoder.setFulfiller(fulfillersLength + i, fulfillersToAdd[i]);
        }
        unchecked {
            ++i;
        }
    }
}


}
