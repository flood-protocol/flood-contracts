// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {OrderFulfiller} from "./OrderFulfiller.sol";

contract OrderBook is OrderFulfiller {
    /**
     * @notice Fulfill an order with an arbitrary number of items for offer and
     *         consideration.
     *
     * @param order     The order to fulfill. Note that the offerer must first
     *                  approve this contract to transfer any relevant tokens
     *                  on their behalf. And the fulfiller must approve this
     *                  contract after receiving offer items.
     * @param fulfiller The address that will receive offer items, then source
     *                  consideration items.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillOrder(Order calldata order, address fulfiller) external override returns (bool /* fulfilled */) {
        return _validateAndFulfillOrder({ order: order, caller: msg.sender });
    }

    /**
     * @notice Cancel an arbitrary number of orders. Note that only the offerer
     *         of a given order may cancel it. Callers should ensure that the
     *         intended order was cancelled by calling `getOrderStatus` and
     *         confirming that `isCancelled` returns `true`.
     *
     * @param orders The orders to cancel.
     *
     * @return cancelled A boolean indicating whether the supplied orders have
     *                   been successfully cancelled.
     */
    function cancel(OrderComponents[] calldata order) external override returns (bool /* cancelled */) {
        return _cancel(order);
    }

    /**
     * @notice Validate an arbitrary number of orders, thereby registering their
     *         signatures as valid and allowing the fulfiller to skip signature
     *         verification on fulfillment. Note that validated orders may still
     *         be unfulfillable due to invalid item amounts or other factors;
     *         callers should determine whether validated orders are fulfillable
     *         by simulating the fulfillment call prior to execution. Also note
     *         that anyone can validate a signed order, but only the offerer can
     *         validate an order without supplying a signature.
     *
     * @param orders The orders to validate.
     *
     * @return validated A boolean indicating whether the supplied orders have
     *                   been successfully validated.
     */
    function validate(Order[] calldata orders) external override returns (bool /* validated */ ) {
        return _validate(orders);
    }

    /**
     * @notice Cancel all orders from a given offerer with a given zone in bulk
     *         by incrementing a counter. Note that only the offerer may
     *         increment the counter.
     *
     * @return newCounter The new counter.
     */
    function incrementCounter() external override returns (uint256 /* newCounter */) {
        return _incrementCounter();
    }

    /**
     * @notice Retrieve the order hash for a given order.
     *
     * @param order The components of the order.
     *
     * @return orderHash The order hash.
     */
    function getOrderHash(OrderComponents calldata order) external view override returns (bytes32 /* orderHash */) {
        // Derive order hash by supplying order parameters along with counter.
        return _deriveOrderHash(order);
    }

    /**
     * @notice Retrieve the status of a given order by hash, including whether
     *         the order has been cancelled or validated. Note that this
     *         function is susceptible to view reentrancy and so should be used
     *         with care when calling from other contracts.
     *
     * @param orderHash The order hash in question.
     *
     * @return isValidated A boolean indicating whether the order in question
     *                     has been validated (i.e. previously approved or
     *                     partially filled).
     * @return isCancelled A boolean indicating whether the order in question
     *                     has been cancelled.
     */
    function getOrderStatus(bytes32 orderHash) external view override returns (bool /* isValidated */, bool /* isCancelled */) {
        // Retrieve the order status using the order hash.
        return _getOrderStatus(orderHash);
    }

    /**
     * @notice Retrieve the current counter for a given offerer.
     *
     * @param offerer The offerer in question.
     *
     * @return counter The current counter.
     */
    function getCounter(address offerer) external view override returns (uint256 /* counter */) {
        // Return the counter for the supplied offerer.
        return _getCounter(offerer);
    }

    /**
     * @notice Retrieve configuration information for this contract.
     *
     * @return version           The contract version.
     * @return domainSeparator   The domain separator for this contract.
     */
    function information() external view override returns (string memory /* version */, bytes32 /* domainSeparator */) {
        // Return the information for this contract.
        return _information();
    }

    /**
     * @notice Retrieve the name of this contract.
     *
     * @return contractName The name of this contract.
     */
    function name() external pure override returns (string memory /* contractName */ ) {
        // Return the name of the contract.
        return _name();
    }
}
