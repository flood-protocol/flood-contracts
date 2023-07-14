// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

interface Create2Factory {
    function safeCreate2(bytes32 salt, bytes memory initializationCode) external returns (address deploymentAddress);
}

contract Create2Deploy is Script {
    Create2Factory factory = Create2Factory(0x0000000000FFe8B47B3e2130213B802212439497);

    function deploy2(string memory name, bytes32 salt, bytes memory args) internal returns (address) {
        return factory.safeCreate2(salt, bytes.concat(vm.getCode(string.concat(name, ".sol")), args));
    }
}
