// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IFulfiller} from "src/interfaces/IFulfiller.sol";
import {IFloodPlain} from "src/interfaces/IFloodPlain.sol";
import {IERC20, SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";

contract MockFulfiller is IFulfiller {
    using SafeERC20 for IERC20;
    using Address for address payable;

    function sourceConsideration(
        IFloodPlain.Order calldata order,
        IFloodPlain.ConsiderationItem[] calldata requestedItems,
        address caller,
        bytes calldata context
    ) external {
        // do nothing. test should just send tokens to this contract

        uint256 length = requestedItems.length;
        address to = order.offerer;
        IFloodPlain.ConsiderationItem calldata item;
        for (uint256 i = 0; i < length;) {
            item = requestedItems[i];

            if (item.isNative) {
                payable(to).sendValue(item.amount);
            } else {
                IERC20(item.token).safeTransfer(to, item.amount);
            }

            unchecked {
                ++i;
            }
        }
    }
}
