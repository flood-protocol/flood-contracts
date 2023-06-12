// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../IFloodPlain.sol";

interface IFloodPlainOnChainOrders {
    event OrderEtched(uint256 indexed orderId, bytes32 indexed orderHash, IFloodPlain.Order order, bytes signature);

    struct OrderWithSignature {
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
    function getEtchedOrder(uint256 etchedOrderId) external view returns (OrderWithSignature memory order);

    /**
     * @notice Fulfill an order with an arbitrary number of items for offer and consideration.
     *
     * @param orderId   The identifier of the etched order to fulfill.
     * @param fulfiller The address that will receive offer items, then source consideration items
     *                  for the offerer.
     * @param extraData Extra bytes passed to the Zone and Fulfiller.
     */
    function fulfillEtchedOrder(uint256 orderId, address fulfiller, bytes calldata extraData) external;

    /**
     * @notice Record an order on-chain for ease of use by other contracts.
     *
     * @param orderWithSignature The order and its signature to record.
     *
     * @return orderId Extra bytes passed to the Zone and Fulfiller.
     */
    function etchOrder(OrderWithSignature calldata orderWithSignature) external returns (uint256 orderId);
}
