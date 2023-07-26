// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../IFloodPlain.sol";

interface IFloodPlainDirectFulfiller {
    error IncorrectValueReceived();

    error InsufficientAmountPulled();

    /**
     * @notice Fulfill an order directly by transferring consideration from caller to offerer.
     *
     * @param signedOrder The order to fulfill and the permit2 signature with the order as the
     *                    witness. Note that the offerer must first approve Permit2 contract to
     *                    transfer any relevant tokens on their behalf.
     */
    function fulfillOrder(IFloodPlain.SignedOrder calldata signedOrder) external payable;

    /**
     * @notice Check if the order can be directly fulfilled. This does not check signature validity
     *         and token approvals.
     *
     * @param order  The components of the order.
     * @param caller The address that will call `fulfillOrder`.
     *
     * @return isValid A boolean guaranteeing the order cannot be fulfilled with supplied
     *                 parameters if false.
     */
    function getOrderValidity(IFloodPlain.Order calldata order, address caller) external view returns (bool isValid);
}
