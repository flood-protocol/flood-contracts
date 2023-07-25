// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlainDirectFulfiller} from "./FloodPlainDirectFulfiller.sol";
import {IFloodPlainBatchDirectFulfiller} from "./IFloodPlainBatchDirectFulfiller.sol";

abstract contract FloodPlainBatchDirectFulfiller is FloodPlainDirectFulfiller, IFloodPlainBatchDirectFulfiller {
    function batchFulfillOrder(OrderWithDirectFulfillmentData[] calldata ordersWithData) external nonReentrant {
        uint256 i;
        uint256 length = ordersWithData.length;
        OrderWithDirectFulfillmentData calldata orderWithData;
        for (i = 0; i < length; ) {
            orderWithData = ordersWithData[i];

            // Fulfill using internal function.
            _fulfillOrder({
                order: orderWithData.order,
                signature: orderWithData.signature
            });

            unchecked {
                ++i;
            }
        }
    }
}

