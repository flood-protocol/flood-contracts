// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "../FloodPlain.sol";
import {IFloodPlainDirectFulfiller} from "./IFloodPlainDirectFulfiller.sol";

// Libraries
import {OrderHash} from "../libraries/OrderHash.sol";
import {Duplicates} from "../libraries/Duplicates.sol";
import {Address} from "@openzeppelin/utils/Address.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";

// Interfaces
import {IZoneDirectFulfiller} from "../../zone/extensions/IZoneDirectFulfiller.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

abstract contract FloodPlainDirectFulfiller is FloodPlain, IFloodPlainDirectFulfiller {
    using Address for address payable;
    using SafeERC20 for IERC20;
    using OrderHash for Order;
    using Duplicates for Item[];

    function fulfillOrder(SignedOrder calldata signedOrder) external payable nonReentrant {
        // Get order component from calldata.
        Order calldata order = signedOrder.order;

        // Retrieve the order hash for order.
        bytes32 orderHash = order.hash();

        // Check zone restrictions.
        address zone = order.zone;
        if (zone != address(0)) {
            if (
                IZoneDirectFulfiller(zone).validateOrder({
                    order: order,
                    book: address(this),
                    caller: msg.sender,
                    orderHash: orderHash,
                    context: signedOrder.zoneData
                }) != IZoneDirectFulfiller.validateOrder.selector
            ) {
                revert ZoneDenied();
            }
        }

        // Transfer each offer item to msg.sender using Permit2.
        _permitTransferOffer({
            order: order,
            signature: signedOrder.signature,
            orderHash: orderHash,
            to: msg.sender
        });

        // Transfer consideration items from fulfiller to offerer.
        _transferConsideration({order: order, fulfiller: msg.sender});

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, order.offerer, msg.sender);
    }

    function getOrderValidity(
        Order calldata order,
        address caller,
        bytes calldata zoneData
    ) external view returns (bool /* isValid */) {
        if (!getOrderStatus({order: order})) {
            return false;
        }

        if (order.zone == address(0)) {
            return true;
        }

        if (
            IZoneDirectFulfiller(order.zone).validateOrder({
                book: address(this),
                order: order,
                caller: caller,
                orderHash: order.hash(),
                context: zoneData
            }) == IZoneDirectFulfiller.validateOrder.selector
        ) {
            return true;
        }

        return false;
    }

    function _transferConsideration(Order calldata order, address fulfiller) internal {
        // Get consideration items, and their count.
        Item[] calldata consideration = order.consideration;
        uint256 length = consideration.length;

        // Do not accept consideration with duplicate items.
        if (consideration.hasDuplicates()) {
            revert DuplicateItems();
        }

        // Move offerer to the stack.
        address to = order.offerer;

        // Create stack elements outside the loop.
        Item calldata item;
        uint256 amount;
        address token;

        for (uint256 i; i < length;) {
            item = consideration[i];
            token = item.token;
            amount = item.amount;

            if (token == address(0)) {
                // Expect caller to have sent Ether when sourceConsideration was called. No
                // msg.value check is made to ensure caller sent correct amount. Because FloodPlain
                // is not supposed to hold any Ether, and if contract has not received sufficient
                // Ether, below transfer will revert.
                payable(to).sendValue(amount);
            } else {
                // Simply transferFrom, without using Permit2.
                IERC20(token).safeTransferFrom(fulfiller, to, amount);
            }

            unchecked {
                ++i;
            }
        }
    }
}
