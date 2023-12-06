// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {Script, console2 as console} from "forge-std/Script.sol";

interface Create2Factory {
    function safeCreate2(bytes32 salt, bytes memory initializationCode) external returns (address deploymentAddress);
}

contract BetterScript is Script {
    Create2Factory internal factory = Create2Factory(0x0000000000FFe8B47B3e2130213B802212439497);

    modifier broadcast(uint256 key) {
        vm.startBroadcast(key);
        _;
        vm.stopBroadcast();
    }

    modifier broadcastAddr(address who) {
        vm.startBroadcast(who);
        _;
        vm.stopBroadcast();
    }

    function deploy2(bytes memory creationCode, bytes32 salt, bytes memory abiEncodedArgs) internal returns (address) {
        require(
            keccak256(abi.encodePacked(vm.envString("FOUNDRY_PROFILE"))) == keccak256(abi.encodePacked("deploy")),
            "Deploy profile not used"
        );
        return factory.safeCreate2(salt, bytes.concat(creationCode, abiEncodedArgs));
    }

    function initCodeHash(bytes memory initCode, bytes memory abiEncodedArgs) internal pure returns (bytes32) {
        return keccak256(bytes.concat(initCode, abiEncodedArgs));
    }
}
