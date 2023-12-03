// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import "forge-std/Test.sol";

/**
 * @notice Generic base fixture for testing
 */
contract BaseFixture is Test {
    uint256 internal constant ALICE_PK = uint256(keccak256("Alice"));
    uint256 internal constant BOB_PK = uint256(keccak256("Bob"));
    uint256 internal constant CHARLIE_PK = uint256(keccak256("Charlie"));
    address payable internal alice = payable(vm.addr(ALICE_PK));
    address payable internal bob = payable(vm.addr(BOB_PK));
    address payable internal charlie = payable(vm.addr(CHARLIE_PK));

    function setUp() public virtual {
        vm.label(alice, "Alice");
        vm.label(bob, "Bob");
        vm.label(charlie, "Charlie");
        vm.label(address(this), "You");
    }

    function generateUser(bytes memory _name) internal pure returns (address payable) {
        return payable(vm.addr(uint256(keccak256(_name))));
    }

    //move block.number forward by a given number of blocks
    function skipBlocks(uint256 numBlocks) internal {
        uint256 targetBlock = block.number + numBlocks;
        vm.roll(targetBlock);
    }

    // signs a message with the given private key
    function sign(uint256 pk, bytes32 messageHash) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, messageHash);
        return bytes.concat(r, s, bytes1(v));
    }
}
