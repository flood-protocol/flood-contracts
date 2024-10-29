// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {SELECTOR_EXTENSION} from "src/libraries/Hooks.sol";
import {IFulfiller} from "src/interfaces/IFulfiller.sol";
import {IFloodPlain} from "src/interfaces/IFloodPlain.sol";
import {IERC20, SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";

contract MaliciousFulfiller2 {
    using SafeERC20 for IERC20;
    using Address for address payable;

    function sourceConsideration(
        bytes28 selectorExtension,
        IFloodPlain.Order calldata order,
        address, /* caller */
        bytes calldata /* context */
    ) external returns (uint256) {
        // do nothing. test should just send tokens to this contract

        require(selectorExtension == SELECTOR_EXTENSION);

        IFloodPlain.Item calldata item = order.consideration;
        if (item.token == address(0)) payable(msg.sender).sendValue(item.amount);
        else IERC20(item.token).safeIncreaseAllowance(msg.sender, item.amount);

        // indicate one wei less to try trick fulfiller
        return item.amount - 1;
    }
}
