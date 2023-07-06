// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "../FloodPlain.sol";
import {IFloodPlainDirectFulfiller} from "./IFloodPlainDirectFulfiller.sol";

// Libraries
import {OrderHash} from "../libraries/OrderHash.sol";
import {ItemDeduplicator} from "../libraries/ItemDeduplicator.sol";
import {Address} from "@openzeppelin/utils/Address.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";

// Interfaces
import {IZone} from "../../zone/IZone.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

abstract contract FloodPlainDirectFulfiller is FloodPlain, IFloodPlainDirectFulfiller {
    using Address for address payable;
    using SafeERC20 for IERC20;
    using OrderHash for Order;
    using ItemDeduplicator for Item[];

    function fulfillOrder(Order calldata order, bytes calldata signature) external payable nonReentrant {
        // Retrieve the order hash for order.
        bytes32 orderHash = order.hash();

        // Check zone restrictions.
        address zone = order.zone;
        if (zone != address(0)) {
            IZone(zone).validateOrder({order: order, book: address(this), caller: msg.sender, orderHash: orderHash});
        }

        // Transfer each offer item to msg.sender using Permit2.
        _permitTransferOffer({order: order, signature: signature, orderHash: orderHash, to: msg.sender});

        // Transfer consideration items from fulfiller to offerer.
        _transferConsideration({order: order, fulfiller: msg.sender});

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, order.offerer, msg.sender);
    }

    function getOrderValidity(Order calldata order, address caller) external view returns (bool /* isValid */ ) {
        if (!getOrderStatus({order: order})) {
            return false;
        }

        if (order.zone == address(0)) {
            return true;
        } else {
            try IZone(order.zone).validateOrder({
                book: address(this),
                order: order,
                caller: caller,
                orderHash: order.hash()
            }) {
                return true;
            } catch {
                return false;
            }
        }
    }

    function _transferConsideration(Order calldata order, address fulfiller) internal {
        // Define pointers once outside the loops.
        Item memory item;
        uint256 requiredAmount;
        uint256 amount;
        address token;

        // Deduplicate consideration items, and move the length to stack.
        Item[] memory deduplicatedItems = order.consideration.deduplicate();
        uint256 dedupCount = deduplicatedItems.length;

        address to = order.offerer;
        for (uint256 i; i < dedupCount;) {
            item = deduplicatedItems[i];
            token = item.token;
            amount = item.amount;
            if (token == address(0)) {
                requiredAmount = amount + to.balance;

                if (msg.value != amount) {
                    revert IncorrectValueReceived();
                }

                payable(to).sendValue(amount);
            } else {
                requiredAmount = amount + IERC20(token).balanceOf(to);

                IERC20(token).safeTransferFrom(fulfiller, to, amount);

                if (IERC20(token).balanceOf(to) < requiredAmount) {
                    revert InsufficientAmountPulled();
                }
            }

            unchecked {
                ++i;
            }
        }
    }
}
