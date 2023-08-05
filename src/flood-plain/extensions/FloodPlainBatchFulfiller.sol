// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "../FloodPlain.sol";
import {IFloodPlainBatchFulfiller} from "./IFloodPlainBatchFulfiller.sol";

// Libraries
import {OrderHash} from "../libraries/OrderHash.sol";
import {Duplicates} from "../libraries/Duplicates.sol";
import {Address} from "@openzeppelin/utils/Address.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";

// Interfaces
import {IFulfiller} from "../../fulfiller/IFulfiller.sol";
import {IZone} from "../../zone/IZone.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

abstract contract FloodPlainBatchFulfiller is FloodPlain, IFloodPlainBatchFulfiller {
    using Address for address payable;
    using SafeERC20 for IERC20;
    using OrderHash for Order;
    using Duplicates for Item[];

    function fulfillOrders(SignedOrder[] calldata signedOrders, address fulfiller, bytes calldata swapData)
        external
        nonReentrant
    {
        uint256 length = signedOrders.length;
        SignedOrder calldata signedOrder;
        Order calldata order;
        bytes32 orderHash;
        address zone;
        Order[] memory orders;

        for (uint256 i; i < length; ) {
            signedOrder = signedOrders[i];

            // Get order component from calldata.
            order = signedOrder.order;

            // Retrieve the order hash for order.
            orderHash = order.hash();

            // Check zone restrictions.
            zone = order.zone;
            if (zone != address(0)) {
                IZone(zone).validateOrder({
                    order: order,
                    book: address(this),
                    fulfiller: fulfiller,
                    caller: msg.sender,
                    orderHash: orderHash,
                    context: signedOrder.zoneData
                });
            }

            // Transfer each offer item to fulfiller using Permit2.
            _permitTransferOffer({order: order, signature: signedOrder.signature, orderHash: orderHash, to: fulfiller});

            // Emit an event signifying that the order will be fulfilled within this transaction.
            emit OrderFulfilled(orderHash, order.offerer, fulfiller);

            // Move order to memory for later use.
            orders[i] = order;

            unchecked {
                ++i;
            }
        }

        // Transfer consideration items from fulfiller to offerer.
        _transferConsiderations({
            signedOrders: signedOrders,
            orders: orders,
            fulfiller: fulfiller,
            swapData: swapData
        });
    }

    function _transferConsiderations(
        SignedOrder[] calldata signedOrders,
        Order[] memory orders,
        address fulfiller,
        bytes calldata swapData
    ) internal {
        // Call fulfiller with order data and the caller address to source consideration items.
        // Contracts implementing Fulfiller interface could get all their tokens drained, hence
        // they should restrict who can call them directly or indirectly.
        uint256[][] memory allReturnedAmounts = IFulfiller(payable(fulfiller)).sourceConsiderations({
            orders: orders,
            caller: msg.sender,
            context: swapData
        });

        uint256 ordersLength = signedOrders.length;
        if (ordersLength != allReturnedAmounts.length) {
            revert ArrayLengthMismatch();
        }

        // Define pointers set in outer loop.
        Order calldata order;
        address offerer;
        Item[] calldata consideration;
        uint256[] memory returnedAmounts;
        uint256 itemsLength;

        // Define pointers set in inner loop.
        Item calldata item;
        address token;
        uint256 returnedAmount;

        for (uint256 i; i < ordersLength;) {
            order = signedOrders[i].order;
            offerer = order.offerer;
            consideration = order.consideration;
            returnedAmounts = allReturnedAmounts[i];
            itemsLength = consideration.length;

            if (itemsLength != returnedAmounts.length) {
                revert ArrayLengthMismatch();
            }

            // Do not accept consideration with duplicate items.
            if (consideration.hasDuplicates()) {
                revert DuplicateItems();
            }

            // Pull the returned amounts from the fulfiller.
            for (uint256 j; j < itemsLength;) {
                item = consideration[j];
                token = item.token;
                returnedAmount = returnedAmounts[j];

                if (returnedAmount < item.amount) {
                    revert InsufficientAmountReceived();
                }

                if (token == address(0)) {
                    // Expect Fulfiller to have sent Ether when sourceConsideration was called. No
                    // check is made that Fulfiller sent correct amount. Because FloodPlain is not
                    // supposed to hold any Ether, and if less than the returnedAmount indicated by
                    // Fulfiller is received, below transfer will revert.
                    payable(offerer).sendValue(returnedAmount);
                } else {
                    IERC20(token).safeTransferFrom(fulfiller, offerer, returnedAmount);
                }

                unchecked {
                    ++j;
                }
            }

            unchecked {
                ++i;
            }
        }
    }
}
