// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IFloodPlain} from "../interfaces/IFloodPlain.sol";

library Duplicates {
    function hasDuplicates(IFloodPlain.Item[] calldata items) internal pure returns (bool) {
        unchecked {
            uint256 itemsLength = items.length;
            if (itemsLength > 1) {
                address token;
                uint256 itemsLengthSubOne = itemsLength - 1;
                for (uint256 i = 0; i < itemsLengthSubOne;) {
                    token = items[i].token;
                    ++i;
                    for (uint256 j = i; j < itemsLength; ++j) {
                        if (token == items[j].token) {
                            return true;
                        }
                    }
                }
            }
            return false;
        }
    }
}
