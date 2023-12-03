// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import {IFloodPlain} from "./IFloodPlain.sol";

interface IZone {
    struct FeeInfo {
        address recipient;
        uint64 bps;
    }

    /**
     * @notice Check if a fulfiller belongs to the zone.
     *
     * @dev Fulfiller must still ensure that
     *      - msg.sender is a the BOOK.
     *      - Book caller is authorized.
     *
     * @param order Order being fulfilled.
     * @param fulfiller The address that will fulfill the order by supplying consideration items.
     *
     * @return True if fulfiller is enabled, false if fulfiller is not enabled.
     */
    function validate(IFloodPlain.Order calldata order, address fulfiller) external view returns (bool);

    /**
     * @notice Get the fee information.
     *
     * @dev It is up to Fulfiller to respect the fees set in a zone.
     *
     * @return The address of the fee recipient who should receive the fees.
     *         The fee cut in BPS that should be taken from the output tokens.
     */
    function fee(IFloodPlain.Order calldata order, address fulfiller) external view returns (FeeInfo memory);
}
