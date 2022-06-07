// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

/**
 * @notice Generic base fixture for testing
 */
contract BaseFixture is Test {
    address payable constant alice =
        payable(address(uint160(uint256(keccak256("Alice")))));
    address payable constant bob =
        payable(address(uint160(uint256(keccak256("Bob")))));
    address payable constant charlie =
        payable(address(uint160(uint256(keccak256("Charlie")))));

    function setUp() public virtual {
        vm.label(alice, "Alice");
        vm.label(bob, "Bob");
        vm.label(charlie, "Charlie");
        vm.label(address(this), "You");
    }

    function generateUser(bytes memory _name)
        internal
        pure
        returns (address payable)
    {
        return payable(address(uint160(uint256(keccak256(_name)))));
    }

    //move block.number forward by a given number of blocks
    function skipBlocks(uint256 numBlocks) internal {
        uint256 targetBlock = block.number + numBlocks;
        vm.roll(targetBlock);
    }
}
