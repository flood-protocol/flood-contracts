// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IMainZone} from "./IMainZone.sol";
import {Zone} from "../Zone.sol";
import {AccessControl} from "@openzeppelin/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/security/Pausable.sol";

// Interfaces
import {IFloodPlain} from "../../flood-plain/IFloodPlain.sol";

contract MainZone is Zone, IMainZone, AccessControl, Pausable {
    bytes32 public constant CALLER_ROLE = keccak256("CALLER_ROLE");
    bytes32 public constant FULFILLER_ROLE = keccak256("FULFILLER_ROLE");
    bytes32 public constant BOOK_ROLE = keccak256("BOOK_ROLE");
    bytes32 public constant CANCELLED_ORDERS = keccak256("CANCELLED_ORDERS");

    address public secondaryZone;

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
    }

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
    }

    function validateOrder(
        IFloodPlain.Order calldata, /* order */
        address book,
        address fulfiller,
        address caller,
        bytes32 orderHash,
        bytes calldata /* context */
    ) external override view whenNotPaused {
        // Always do basic built-in access control checks.
        _checkRole(CALLER_ROLE, caller);
        _checkRole(FULFILLER_ROLE, fulfiller);
        _checkRole(BOOK_ROLE, book);

        // Check if an order is manually cancelled.
        if (hasRole(CANCELLED_ORDERS, address(uint160(uint256(orderHash))))) {
            revert CancelledOrder(orderHash);
        }

        // Check if an extra zone is set to do additional checks.
        address _secondaryZone = secondaryZone;
        assembly {
            // If secondaryZone is not zero address.
            if _secondaryZone {
                // Move entire calldata to memory starting from offset zero.
                calldatacopy(0, 0, calldatasize())

                // Make the exact same staticcall to the secondary zone.
                let result := staticcall(gas(), _secondaryZone, 0, calldatasize(), 0, 0)

                // Revert with the same message if call fails.
                if iszero(result) {
                    returndatacopy(0, 0, returndatasize())
                    revert(0, returndatasize())
                }
            }
        }
    }
}
