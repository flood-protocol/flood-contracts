// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IFulfiller} from "src/fulfiller/IFulfiller.sol";
import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";
import {IERC20, SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";

contract MaliciousFulfiller {
    using SafeERC20 for IERC20;
    using Address for address payable;

    function sourceConsideration(
        IFloodPlain.Order calldata order,
        address, /* caller */
        bytes calldata /* context */
    ) external returns (uint256[] memory) {
        IFloodPlain.Item[] calldata consideration = order.consideration;
        uint256 length = consideration.length;
        uint256[] memory amounts = new uint256[](length);
        for (uint256 i = 0; i < length;) {
            IFloodPlain.Item calldata item = consideration[i];

            // send one wei less for the first item to try trick the book.
            uint256 amount = i == 0 ? item.amount - 1 : item.amount;

            if (item.token == address(0)) {
                payable(msg.sender).sendValue(amount);
            } else {
                IERC20(item.token).safeIncreaseAllowance(msg.sender, amount);
            }

            amounts[i] = item.amount;

            unchecked {
                ++i;
            }
        }

        return amounts;
    }
}
