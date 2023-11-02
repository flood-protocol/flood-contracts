// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IMainZone {
    error CancelledOrder(bytes32 orderHash);

    event SecondaryZoneSet(address indexed newSecondayZone);

    /**
     * @notice Get the secondary zone address that performs additional validation for this zone.
     *
     * @return zone The address of the secondary zone.
     */
    function secondaryZone() external view returns (address zone);

    /**
     * @notice Restricted function to set a secondary zone that can perform additional checks.
     *
     * @dev The added zone will also employ Zone interface.
     * @dev Setting secondary zone to zero address will unset it, meaning that there will be no
     *      call made to the secondary zone.
     * @dev Changing secondary zone is dangerous, and should be performed with care. If a current
     *      secondary zone performs important validation, and it is changed to another secondary
     *      zone that lacks these checks, that can potentially lead to loss of fulfiller funds.
     *
     * @param newSecondaryZone The address of the new secondary zone.
     */
    function setSecondaryZone(address newSecondaryZone) external;

    /**
     * @notice Restricted function to block all validation. This means that no order using this
     *         zone can be fulfilled.
     */
    function pause() external;

    /**
     * @notice Restricted function to resume regular validation functionality.
     */
    function unpause() external;
}
