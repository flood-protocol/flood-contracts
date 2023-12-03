// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "src/libraries/Duplicates.sol";

contract DuplicatesContract {
    function hasDuplicates(IFloodPlain.Item[] calldata items) external pure returns (bool) {
        return Duplicates.hasDuplicates(items);
    }
}

contract DuplicatesTest is Test {
    DuplicatesContract duplicates;

    function setUp() public {
        duplicates = new DuplicatesContract();
    }

    function test_False_zeroItem() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](0);

        assertFalse(duplicates.hasDuplicates(items));
    }

    function test_False_oneItem() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](1);
        items[0].token = address(0xABCD);
        items[0].amount = 500;

        assertFalse(duplicates.hasDuplicates(items));
    }

    function test_True_twoItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](2);
        items[0].token = address(0xABCD);
        items[0].amount = 500;
        items[1].token = address(0xABCD);
        items[1].amount = 0;

        assertTrue(duplicates.hasDuplicates(items));
    }

    function test_False_twoItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](2);
        items[0].token = address(0xABCD);
        items[0].amount = 500;
        items[1].token = address(0xAAAA);
        items[1].amount = 500;

        assertFalse(duplicates.hasDuplicates(items));
    }

    function test_True_threeItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](3);
        items[0].token = address(0xABCD);
        items[0].amount = 500;
        items[1].token = address(0xABCD);
        items[1].amount = 500;
        items[2].token = address(0xABCD);
        items[2].amount = 500;

        assertTrue(duplicates.hasDuplicates(items));
    }

    function test_False_threeItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](3);
        items[0].token = address(0xAAAA);
        items[0].amount = 500;
        items[1].token = address(0xBBBB);
        items[1].amount = 500;
        items[2].token = address(0xCCCC);
        items[2].amount = 500;

        assertFalse(duplicates.hasDuplicates(items));
    }

    function test_True_threeAndTwoItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](5);
        items[0].token = address(0xABCD);
        items[0].amount = 500;
        items[1].token = address(0xABCD);
        items[1].amount = 500;
        items[2].token = address(0xABCD);
        items[2].amount = 500;
        items[3].token = address(0xEFEF);
        items[3].amount = 500;
        items[4].token = address(0xEFEF);
        items[4].amount = 500;

        assertTrue(duplicates.hasDuplicates(items));
    }

    function test_False_fiveItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](5);
        items[0].token = address(0xAAAA);
        items[0].amount = 500;
        items[1].token = address(0xBBBB);
        items[1].amount = 500;
        items[2].token = address(0xCCCC);
        items[2].amount = 500;
        items[3].token = address(0xDDDD);
        items[3].amount = 500;
        items[4].token = address(0xEEEE);
        items[4].amount = 500;

        assertFalse(duplicates.hasDuplicates(items));
    }

    function test_Success_ThreeAndOneAndTwoAndOneItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](7);
        items[0].token = address(0xABCD);
        items[0].amount = 500;
        items[1].token = address(0xABCD);
        items[1].amount = 500;
        items[2].token = address(0xABCD);
        items[2].amount = 500;
        items[3].token = address(0xEFEF);
        items[3].amount = 500;
        items[4].token = address(0xBEEF);
        items[4].amount = 500;
        items[5].token = address(0xBEEF);
        items[5].amount = 500;
        items[6].token = address(0xEFEF);
        items[6].amount = 500;

        assertTrue(duplicates.hasDuplicates(items));
    }
}
