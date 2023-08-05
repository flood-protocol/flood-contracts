// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../flood-plain/IFloodPlain.sol";

interface IFulfiller {
    /**
     * @notice External function to perform arbitrary swaps to source requested items.
     *
     * @param order All the details of a book order.
     * @param caller The address that called the book.
     * @param context All the swap data, or any other arbitrary data.
     *
     * @return The amount of consideration items that were sourced and approved for Book to
     *         transfer. These amounts might be higher than requested amounts.
     */
    function sourceConsideration(
        IFloodPlain.Order calldata order,
        address caller,
        bytes calldata context
    ) external returns (uint256[] memory);

    /**
     * @notice External function to fill multiple orders in the same transaction.
     *
     * @param orders An array of orders that will be batch filled.
     * @param caller The address that called the book.
     * @param context All the swap data, or any other arbitrary data.
     *
     * @return The amount of consideration items, for each order, that were sourced and approved
     *         for Book to transfer. These amounts might be higher than requested amounts.
     */
    function sourceConsiderations(
        IFloodPlain.Order[] calldata orders,
        address caller,
        bytes calldata context
    ) external returns (uint256[][] memory);
}
