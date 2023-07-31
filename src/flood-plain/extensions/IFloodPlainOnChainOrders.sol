// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../IFloodPlain.sol";

interface IFloodPlainOnChainOrders {
    event OrderEtched(uint256 indexed orderId, bytes32 indexed orderHash, IFloodPlain.Order order, bytes signature);

    struct SimpleSignedOrder {
        IFloodPlain.Order order;
        bytes signature;
    }

    /**
     * @notice Get all the details of the etched order corresponding to an order identifier.
     *
     * @param etchedOrderId The identifier of the etched order.
     *
     * @return order All the details of the order, including its signature.
     */
    function getEtchedOrder(uint256 etchedOrderId) external view returns (SimpleSignedOrder memory order);

    /**
     * @notice Fulfill an order with an arbitrary number of items for offer and consideration.
     *
     * @param orderId   The identifier of the etched order to fulfill.
     * @param fulfiller The address that will receive offer items, then source consideration items
     *                  for the offerer.
     * @param zoneData  Extra bytes passed to the Zone.
     * @param swapData  Extra bytes passed to the Fulfiller.
     */
    function fulfillEtchedOrder(
        uint256 orderId,
        address fulfiller,
        bytes calldata zoneData,
        bytes calldata swapData
     ) external;

    /**
     * @notice Record an order on-chain for ease of use by other contracts.
     *
     * @param signedOrder The order and its signature to record.
     *
     * @return orderId Extra bytes passed to the Zone and Fulfiller.
     */
    function etchOrder(SimpleSignedOrder calldata signedOrder) external returns (uint256 orderId);
}
