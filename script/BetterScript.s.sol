// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {Script, console2 as console} from "forge-std/Script.sol";

interface ICREATE3Factory {
    function deploy(bytes32 salt, bytes memory creationCode) external payable returns (address deployed);

    function getDeployed(address deployer, bytes32 salt) external view returns (address deployed);
}

contract BetterScript is Script {
    ICREATE3Factory internal factory = ICREATE3Factory(0x2Dfcc7415D89af828cbef005F0d072D8b3F23183);

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

    function deploy3(bytes memory creationCode, bytes32 salt, bytes memory abiEncodedArgs) internal returns (address) {
        require(
            keccak256(abi.encodePacked(vm.envString("FOUNDRY_PROFILE"))) == keccak256(abi.encodePacked("deploy")),
            "Deploy profile not used"
        );
        return factory.deploy(salt, bytes.concat(creationCode, abiEncodedArgs));
    }
}
