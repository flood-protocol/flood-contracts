// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../IFloodPlain.sol";

interface IFloodPlainBatchFulfiller {
    /**
     * @notice Fulfill multiple orders using the same fulfiller.
     *
     * @param signedOrders The list of orders to fulfill and the permit2 signature with the order
     *                     as the witness. Note that the offerers must first approve Permit2
     *                     contract to transfer any relevant tokens on their behalf.
     * @param fulfiller    The address that will receive offer items, then source consideration
     *                     items for the offerers.
     * @param swapData     Extra bytes passed to the Fulfiller.
     */
    function fulfillOrders(IFloodPlain.SignedOrder[] calldata signedOrders, address fulfiller, bytes calldata swapData) external;
}
