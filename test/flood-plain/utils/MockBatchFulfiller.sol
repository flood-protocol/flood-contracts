// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IFulfiller} from "src/fulfiller/IFulfiller.sol";
import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";
import {IERC20, SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";

contract MockBatchFulfiller {
    using SafeERC20 for IERC20;
    using Address for address payable;

    function sourceConsiderations(
        IFloodPlain.Order[] calldata orders,
        address, /* caller */
        bytes calldata /* context */
    ) external returns (uint256[][] memory) {
        // do nothing. test should just send tokens to this contract

        uint256 ordersLength = orders.length;
        uint256[][] memory allAmounts = new uint256[][](ordersLength);

        for (uint256 i = 0; i < ordersLength; ++i) {
            IFloodPlain.Item[] memory consideration = orders[i].consideration;
            uint256 considerationLength = consideration.length;
            uint256[] memory amounts = new uint256[](considerationLength);
            for (uint256 j; j < considerationLength; ++j) {
                IFloodPlain.Item memory item = consideration[j];
                if (item.token == address(0)) {
                    payable(msg.sender).sendValue(item.amount);
                } else {
                    IERC20(item.token).safeIncreaseAllowance(msg.sender, item.amount);
                }
                amounts[j] = item.amount;
            }
            allAmounts[i] = amounts;
        }

        return allAmounts;
    }
}
