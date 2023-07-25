// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../IFloodPlain.sol";

interface IFloodPlainBatchDirectFulfiller {
    struct OrderWithDirectFulfillmentData {
        IFloodPlain.Order order;
        bytes signature;
    }

    function batchFulfillOrder(OrderWithDirectFulfillmentData[] calldata ordersWithData) external;
}
