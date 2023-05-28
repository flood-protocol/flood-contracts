// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IFloodPlain {
    struct OrderComponents {
        address offerer;
        address zone;
        Item[] offer;
        Item[] consideration;
        uint256 deadline;
        uint256 salt;
        uint256 counter;
    }

    struct Item {
        bool isNative;
        address token;
        uint256 amount;
    }

    struct OrderParameters {
        address offerer;
        address zone;
        Item[] offer;
        Item[] consideration;
        uint256 deadline;
        uint256 salt;
    }

    struct Order {
        OrderParameters parameters;
        bytes signature;
    }

    struct OrderStatus {
        bool isValidated;
        bool isCancelled;
        bool isFilled;
    }

    event OrderFulfilled(bytes32 indexed orderHash, address indexed offerer, address indexed fulfiller);
    event OrderCancelled(bytes32 indexed orderHash, address indexed offerer, address indexed zone);
    event OrderValidated(bytes32 indexed orderHash, OrderParameters orderParameters);
    event CounterIncremented(uint256 newCounter, address indexed zone);

    error CrossEntrancy();
    error InvalidSignature();
    error DeadlinePassed();
    error InvalidCaller();
    error InsufficientAmountPulled();
    error OrderIsFilledOrCancelled(bytes32 orderHash);

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
     * @param extraData Extra bytes passed to the zone and FloodGate.
     */
    function fulfillOrder(Order calldata order, address fulfiller, bytes calldata extraData) external;

    /**
     * @notice Cancel an arbitrary number of orders. Note that only the offerer
     *         of a given order may cancel it. Callers should ensure that the
     *         intended order was cancelled by calling `getOrderStatus` and
     *         confirming that `isCancelled` returns `true`.
     *
     * @param orders The orders to cancel.
     */
    function cancel(OrderComponents[] calldata orders) external;

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
     */
    function validate(Order[] calldata orders) external;

    /**
     * @notice Cancel all orders from a given offerer with a given zone in bulk
     *         by incrementing a counter. Note that only the offerer may
     *         increment the counter.
     *
     * @return newCounter The new counter.
     */
    function incrementCounter() external returns (uint256 newCounter);

    /**
     * @notice Retrieve the order hash for a given order.
     *
     * @param order The components of the order.
     *
     * @return orderHash The order hash.
     */
    function getOrderHash(OrderComponents calldata order) external view returns (bytes32 orderHash);

    /**
     * @notice Retrieve the status of a given order by hash, including whether
     *         the order has been cancelled or validated. Note that this
     *         function is susceptible to view reentrancy and so should be used
     *         with care when calling from other contracts.
     *
     * @param orderHash The order hash in question.
     *
     * @return orderStatus Three boolean values indicating whether the order in
     *                     question has been validated, filled, and/or cancelled.
     */
    function getOrderStatus(bytes32 orderHash) external view returns (OrderStatus memory orderStatus);

    /**
     * @notice Retrieve the current counter for a given offerer.
     *
     * @param offerer The offerer in question.
     *
     * @return counter The current counter.
     */
    function getCounter(address offerer) external view returns (uint256 counter);
}
