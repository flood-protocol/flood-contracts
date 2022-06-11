// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

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

    function generateUser(bytes memory _name)
        internal
        returns (address payable)
    {
        return payable(vm.addr(uint256(keccak256(_name))));
    }

    //move block.number forward by a given number of blocks
    function skipBlocks(uint256 numBlocks) internal {
        uint256 targetBlock = block.number + numBlocks;
        vm.roll(targetBlock);
    }
}
