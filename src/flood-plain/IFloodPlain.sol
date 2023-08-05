// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";

interface IFloodPlain {
    error InsufficientAmountReceived();

    error NotAContract();

    error ZoneDenied();

    error DuplicateItems();

    error ArrayLengthMismatch();

    event OrderFulfilled(bytes32 indexed orderHash, address indexed offerer, address indexed fulfiller);

    struct SignedOrder {
        Order order;
        bytes signature;
        bytes zoneData;
    }

    struct Order {
        address offerer;
        address zone;
        Item[] offer;
        Item[] consideration;
        uint256 deadline;
        uint256 nonce;
    }

    struct Item {
        address token;
        uint256 amount;
    }

    /**
     * @notice Get Permit2 SignatureTransfer contract that is used in verifying orders.
     */
    function PERMIT2() external view returns (ISignatureTransfer);

    /**
     * @notice Fulfill an order with an arbitrary number of items for offer and consideration.
     *
     * @param signedOrder The order to fulfill and the permit2 signature with the order as the
     *                    witness. Note that the offerer must first approve Permit2 contract to
     *                    transfer any relevant tokens on their behalf.
     * @param fulfiller   The address that will receive offer items, then source consideration
     *                    items for the offerer.
     * @param swapData    Extra bytes passed to the Fulfiller.
     */
    function fulfillOrder(SignedOrder calldata signedOrder, address fulfiller, bytes calldata swapData)
        external;

    /**
     * @notice Retrieve the permit2 hash for a given order.
     *
     * @param order The components of the order.
     *
     * @return permitHash The permit2 hash as order as the witness.
     */
    function getPermitHash(Order calldata order) external view returns (bytes32 permitHash);

    /**
     * @notice Retrieve the order hash for a given order.
     *
     * @param order The components of the order.
     *
     * @return orderHash The order hash.
     */
    function getOrderHash(Order calldata order) external view returns (bytes32 orderHash);

    /**
     * @notice Check if the order can be fulfilled with the provided parameters. This does not
     *         check signature validity and token approvals.
     *
     * @param order     The components of the order.
     * @param fulfiller The address that will fulfill the order.
     * @param caller    The address that will call `fulfillOrder`.
     * @param zoneData  Extra bytes that will be passed to Zone.
     *
     * @return isValid A boolean guaranteeing the order cannot be fulfilled with supplied
     *                 parameters if false.
     */
    function getOrderValidity(Order calldata order, address fulfiller, address caller, bytes calldata zoneData)
        external
        view
        returns (bool isValid);

    /**
     * @notice Check if the order deadline has passed or its nonce has been cancelled or used.
     *
     * @param order The components of the order.
     *
     * @return isValid A boolean guaranteeing the order cannot be fulfilled if false.
     */
    function getOrderStatus(Order calldata order) external view returns (bool isValid);

    /**
     * @notice Check if the nonce of a user is available.
     *
     * @param user  The address of the user to check the nonce of.
     * @param nonce The nonce of the user to check.
     *
     * @return isValid A boolean returning true if the nonce is not flipped.
     */
    function getNonceStatus(address user, uint256 nonce) external view returns (bool isValid);

    /**
     * @notice Allow receiving ether from Fulfiller. No checks are made to ensure the ether is
     *         sent from a Fulfiller. If you send ether directly, someone else can steal it.
     */
    receive() external payable;
}
