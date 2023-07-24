// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "../FloodPlain.sol";
import {IFloodPlainBatchFulfiller} from "./IFloodPlainBatchFulfiller.sol";

abstract contract FloodPlainBatchFulfiller is FloodPlain, IFloodPlainBatchFulfiller {
    function batchFulfillOrder(OrderWithFulfillmentData[] calldata ordersWithData) external nonReentrant {
        uint256 i;
        uint256 length = ordersWithData.length;
        OrderWithFulfillmentData calldata orderWithData;
        for (i = 0; i < length; ) {
            orderWithData = ordersWithData[i];

            // Fulfill using internal function.
            _fulfillOrder({
                order: orderWithData.order,
                signature: orderWithData.signature,
                fulfiller: orderWithData.fulfiller,
                extraData: orderWithData.extraData
            });

            unchecked {
                ++i;
            }
        }
    }
}
