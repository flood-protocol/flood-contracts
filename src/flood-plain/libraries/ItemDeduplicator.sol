// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IFloodPlain} from "../IFloodPlain.sol";

library ItemDeduplicator {
    function deduplicate(IFloodPlain.Item[] calldata items) internal pure returns (IFloodPlain.Item[] memory) {
        uint256 itemsLength = items.length;
        IFloodPlain.Item memory deduplicatedItem;
        IFloodPlain.Item calldata item;
        IFloodPlain.Item[] memory deduplicatedItems = new IFloodPlain.Item[](itemsLength);
        uint256 dedupCount;
        // For each item...
        for (uint256 i; i < itemsLength;) {
            item = items[i];
            bool isFound;
            // Check if it is the same as a previous consideration item in the array.
            for (uint256 j; j < dedupCount;) {
                deduplicatedItem = deduplicatedItems[j];
                if (deduplicatedItem.token == item.token) {
                    // And add the amounts of the two consideration items with identical tokens.
                    deduplicatedItem.amount += item.amount;
                    isFound = true;
                    break;
                }

                unchecked {
                    ++j;
                }
            }

            // If the consideration item is encountered the first time...
            if (!isFound) {
                // Add it to the deduplicated consideration items array.
                deduplicatedItems[dedupCount] = item;

                // And increase the actual length of the array.
                unchecked {
                    ++dedupCount;
                }
            }

            unchecked {
                ++i;
            }
        }

        assembly {
            // Trim the array to its actual size.
            mstore(deduplicatedItems, dedupCount)
        }

        return deduplicatedItems;
    }
}
