// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "src/flood-plain/libraries/ItemDeduplicator.sol";

contract ItemDeduplicatorContract {
    using ItemDeduplicator for IFloodPlain.Item[];

    function dedup(IFloodPlain.Item[] calldata items) external pure returns (IFloodPlain.Item[] memory) {
        return items.deduplicate();
    }
}

contract ItemDeduplicatorTest is Test {
    ItemDeduplicatorContract deduplicator;

    function setUp() public {
        deduplicator = new ItemDeduplicatorContract();
    }

    function test_Success_zeroItem() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](0);

        IFloodPlain.Item[] memory deduplicatedItems = deduplicator.dedup(items);

        assertEq(deduplicatedItems.length, 0);
    }

    function test_Success_oneItem() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](1);
        items[0].token = address(0xABCD);
        items[0].amount = 500;

        IFloodPlain.Item[] memory deduplicatedItems = deduplicator.dedup(items);

        assertEq(deduplicatedItems.length, 1);
        assertEq(deduplicatedItems[0].amount, 500);
        assertTrue(deduplicatedItems[0].token == address(0xABCD));
    }

    function test_Success_twoItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](2);
        items[0].token = address(0xABCD);
        items[0].amount = 500;
        items[1].token = address(0xABCD);
        items[1].amount = 500;

        IFloodPlain.Item[] memory deduplicatedItems = deduplicator.dedup(items);

        assertEq(deduplicatedItems.length, 1);
        assertEq(deduplicatedItems[0].amount, 1000);
        assertTrue(deduplicatedItems[0].token == address(0xABCD));
    }

    function test_Success_threeItems() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](3);
        items[0].token = address(0xABCD);
        items[0].amount = 500;
        items[1].token = address(0xABCD);
        items[1].amount = 500;
        items[2].token = address(0xABCD);
        items[2].amount = 500;

        IFloodPlain.Item[] memory deduplicatedItems = deduplicator.dedup(items);

        assertEq(deduplicatedItems.length, 1);
        assertEq(deduplicatedItems[0].amount, 1500);
        assertTrue(deduplicatedItems[0].token == address(0xABCD));
    }

    function test_Success_ThreeAndTwoItems() public {
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

        IFloodPlain.Item[] memory deduplicatedItems = deduplicator.dedup(items);

        assertEq(deduplicatedItems.length, 2);
        assertEq(deduplicatedItems[0].amount, 1500);
        assertEq(deduplicatedItems[1].amount, 1000);
        assertTrue(deduplicatedItems[0].token == address(0xABCD));
        assertTrue(deduplicatedItems[1].token == address(0xEFEF));
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

        IFloodPlain.Item[] memory deduplicatedItems = deduplicator.dedup(items);

        assertEq(deduplicatedItems.length, 3);
        assertEq(deduplicatedItems[0].amount, 1500);
        assertEq(deduplicatedItems[1].amount, 1000);
        assertEq(deduplicatedItems[2].amount, 1000);
        assertTrue(deduplicatedItems[0].token == address(0xABCD));
        assertTrue(deduplicatedItems[1].token == address(0xEFEF));
        assertTrue(deduplicatedItems[2].token == address(0xBEEF));
    }

    function test_RevertWhen_AmountsOverflow() public {
        IFloodPlain.Item[] memory items = new IFloodPlain.Item[](2);
        items[0].token = address(0xABCD);
        items[0].amount = type(uint256).max;
        items[1].token = address(0xABCD);
        items[1].amount = 1;

        vm.expectRevert();
        IFloodPlain.Item[] memory deduplicatedItems = deduplicator.dedup(items);

        assertEq(deduplicatedItems.length, 0);
    }
}
