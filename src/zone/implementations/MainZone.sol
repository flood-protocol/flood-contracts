// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IMainZone} from "./IMainZone.sol";
import {ZoneComplete} from "../ZoneComplete.sol";
import {AccessControlDefaultAdminRules} from "@openzeppelin/access/AccessControlDefaultAdminRules.sol";
import {Pausable} from "@openzeppelin/security/Pausable.sol";

// Interfaces
import {IFloodPlain} from "../../flood-plain/IFloodPlain.sol";
import {IZone, IZoneDirectFulfiller} from "../ZoneComplete.sol";

contract MainZone is ZoneComplete, IMainZone, AccessControlDefaultAdminRules, Pausable {
    bytes32 public constant CALLER_ROLE = keccak256("CALLER_ROLE");
    bytes32 public constant FULFILLER_ROLE = keccak256("FULFILLER_ROLE");
    bytes32 public constant BOOK_ROLE = keccak256("BOOK_ROLE");
    bytes32 public constant CANCELLED_ORDERS = keccak256("CANCELLED_ORDERS");

    address public secondaryZone;

    constructor(address admin) AccessControlDefaultAdminRules(2 days, admin) {}

    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    function setSecondaryZone(address newSecondaryZone) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newSecondaryZone != address(0)) {
            require(newSecondaryZone.code.length != 0);
        }

        secondaryZone = newSecondaryZone; // Zero address is to unset.

        emit SecondaryZoneSet(newSecondaryZone);
    }

    function validateOrder(
        IFloodPlain.Order calldata,/* order */
        address book,
        address caller,
        bytes32 orderHash,
        bytes calldata /* context */
    ) external view override whenNotPaused returns (bytes4) {
        // Do basic built-in access control checks and check if an order is manually cancelled.
        if (
            hasRole(CALLER_ROLE, caller) &&
                hasRole(BOOK_ROLE, book) &&
                !hasRole(CANCELLED_ORDERS, address(uint160(uint256(orderHash))))
        ) {
            address _secondaryZone = secondaryZone;
            if (_secondaryZone == address(0)) {
                return IZoneDirectFulfiller.validateOrder.selector;
            } else {
                return _checkSecondaryZone(_secondaryZone);
            }
        } else {
            return ~IZoneDirectFulfiller.validateOrder.selector;
        }
    }

    function validateOrder(
        IFloodPlain.Order calldata, /* order */
        address book,
        address fulfiller,
        address caller,
        bytes32 orderHash,
        bytes calldata /* context */
    ) external view override whenNotPaused returns (bytes4) {
        // Do basic built-in access control checks and check if an order is manually cancelled.
        if (
            hasRole(CALLER_ROLE, caller) &&
                hasRole(FULFILLER_ROLE, fulfiller) &&
                hasRole(BOOK_ROLE, book) &&
                !hasRole(CANCELLED_ORDERS, address(uint160(uint256(orderHash))))
        ) {
            address _secondaryZone = secondaryZone;
            if (_secondaryZone == address(0)) {
                return IZone.validateOrder.selector;
            } else {
                return _checkSecondaryZone(_secondaryZone);
            }
        } else {
            return ~IZone.validateOrder.selector;
        }
    }

    function _checkSecondaryZone(address _secondaryZone) internal view returns (bytes4 selector) {
        assembly {
            // Get free memory pointer.
            let fmp := mload(0x40)

            // Move entire calldata to memory starting from the free memory pointer.
            calldatacopy(fmp, 0, calldatasize())

            // Make the exact same staticcall to the secondary zone.
            let result := staticcall(gas(), _secondaryZone, fmp, calldatasize(), fmp, 4)

            // Optimistically move selector to stack.
            selector := mload(fmp)

            // Revert with the same message if call fails.
            if iszero(result) {
                returndatacopy(0, 0, returndatasize())
                revert(0, returndatasize())
            }
        }
    }
}
