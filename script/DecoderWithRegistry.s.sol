// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Script, console2 as console} from "forge-std/Script.sol";
import {DecoderWithRegistry, IdToAddress} from "src/decoders/extensions/DecoderWithRegistry.sol";
import {Create2Deploy} from "script/Create2Deploy.sol";

contract DecoderWithRegistryScript is Script, Create2Deploy {
    bytes32 SALT = 0x398f48f5ae577ed837a54bd998bb2e177dc98a037000000000000000000cac39;

    // We could use console.log here, but it won't be catched by expectEmit in tests
    event Duplicated(uint256, address);

    function run() public virtual {
        string memory profile = vm.envString("FOUNDRY_PROFILE");
        require(
            keccak256(bytes(profile)) == keccak256(bytes("deploy")), "DecoderWithRegistryScript: not deploy profile"
        );
        uint256 pk = vm.envUint("DECODER_ADMIN_KEY");
        address decoderAdmin = vm.addr(pk);
        vm.broadcast(pk);
        address decoder = deploy2("DecoderWithRegistry", SALT, abi.encode(decoderAdmin));

        console.log("Decoder deployed at", decoder);
        require(DecoderWithRegistry(decoder).owner() == decoderAdmin, "DecoderWithRegistry: wrong owner");
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
