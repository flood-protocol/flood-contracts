// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../../flood-plain/IFloodPlain.sol";

interface IZoneWithFees {
    /**
     * @notice Get the fee information.
     *
     * @dev It is up to Fulfiller to respect the fees set in a zone.
     *
     * @param order The components of an order, excluding its signature.
     *
     * @return The address of the fee recipient who should receive the fees.
     * @return The fee cut in BPS that should be taken from the output tokens.
     */
    function fee(IFloodPlain.Order calldata order) external view returns (address, uint256);
}
