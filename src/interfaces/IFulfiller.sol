// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import {IFloodPlain} from "./IFloodPlain.sol";

interface IFulfiller {
    /**
     * @notice External function to perform arbitrary swaps to source requested items.
     *
     * @dev Book must have sent offer items prior to this call. The swap data is ought to use the
     *      received offer items. If a fulfiller has operating capital and does not have any checks
     *      to ensure not more than the received offer items are spent, it must depend on trusted
     *      callers supplying honest swap data. Zone can enforce the access control.
     *
     * @param selectorExtension Additional bytes to prevent hooks from calling Fulfiller.
     * @param order All the details of a book order.
     * @param caller The address that called the book.
     * @param context All the swap data, or any other arbitrary data.
     *
     * @return The amount of consideration that was sourced and approved for Book to
     *         transfer. These amounts might be higher than requested amounts.
     */
    function sourceConsideration(
        bytes28 selectorExtension,
        IFloodPlain.Order calldata order,
        address caller,
        bytes calldata context
    ) external returns (uint256);

    /**
     * @notice External function to fill multiple orders in the same transaction.
     *
     * @param selectorExtension Additional bytes to prevent hooks from calling Fulfiller.
     * @param orders An array of orders that will be batch filled.
     * @param caller The address that called the book.
     * @param context All the swap data, or any other arbitrary data.
     *
     * @return The amount of consideration items, for each order, that were sourced and approved
     *         for Book to transfer. These amounts might be higher than requested amounts.
     */
    function sourceConsiderations(
        bytes28 selectorExtension,
        IFloodPlain.Order[] calldata orders,
        address caller,
        bytes calldata context
    ) external returns (uint256[] memory);
}
