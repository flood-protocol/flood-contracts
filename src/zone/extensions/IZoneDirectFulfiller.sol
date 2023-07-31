// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../../flood-plain/IFloodPlain.sol";

interface IZoneDirectFulfiller {
    /**
     * @notice Check if a direct fill order is valid.
     *
     * @param order     The components of an order, excluding its signature.
     * @param book      The address of the book where the order is created (e.g.: FloodPlain).
     * @param caller    The address that is fulfilling the order by calling the book and supplying
     *                  all the fulfillment parameters.
     * @param orderHash The EIP712 hash of the order components.
     * @param context   The arbitrary extra data supplied by the caller to be checked by the Zone.
     *
     * @return Its four byte function selector if order is valid, anything else if invalid.
     */
    function validateOrder(
        IFloodPlain.Order calldata order,
        address book,
        address caller,
        bytes32 orderHash,
        bytes calldata context
    ) external view returns (bytes4);
}
