// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../IFloodPlain.sol";

interface IFloodPlainBatchFulfiller {
    struct OrderWithFulfillmentData {
        IFloodPlain.Order order;
        bytes signature;
        address fulfiller;
        bytes extraData;
    }

    function batchFulfillOrder(OrderWithFulfillmentData[] calldata ordersWithData) external;
}
