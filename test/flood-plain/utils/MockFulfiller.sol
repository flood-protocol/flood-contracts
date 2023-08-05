// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IFulfiller} from "src/fulfiller/IFulfiller.sol";
import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";
import {IERC20, SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";

contract MockFulfiller {
    using SafeERC20 for IERC20;
    using Address for address payable;

    function sourceConsideration(
        IFloodPlain.Order calldata order,
        address, /* caller */
        bytes calldata /* context */
    ) external returns (uint256[] memory) {
        // do nothing. test should just send tokens to this contract

        uint256 length = order.consideration.length;
        uint256[] memory amounts = new uint256[](length);
        IFloodPlain.Item calldata item;
        for (uint256 i = 0; i < length;) {
            item = order.consideration[i];

            if (item.token == address(0)) {
                payable(msg.sender).sendValue(item.amount);
            } else {
                IERC20(item.token).safeIncreaseAllowance(msg.sender, item.amount);
            }

            amounts[i] = item.amount;

            unchecked {
                ++i;
            }
        }

        return amounts;
    }
}
