// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../flood-plain/IFloodPlain.sol";

interface IFulfiller {
    /**
     * @notice External function to perform arbitrary swaps to source requested items.
     *
     * @param order All the details of a book order.
     * @param requestedItems Deduplicated consideration items in an order.
     * @param caller The address that called the book.
     * @param context All the swap data, or any other arbitrary data.
     */
    function sourceConsideration(
        IFloodPlain.Order calldata order,
        IFloodPlain.Item[] calldata requestedItems,
        address caller,
        bytes calldata context
    ) external;
}
