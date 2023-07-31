// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IZone} from "./IZone.sol";
import {IZoneDirectFulfiller} from "./extensions/IZoneDirectFulfiller.sol";

// Interfaces
import {IFloodPlain} from "../flood-plain/IFloodPlain.sol";

abstract contract ZoneFull is IZone, IZoneDirectFulfiller {
    function validateOrder(
        IFloodPlain.Order calldata order,
        address book,
        address caller,
        bytes32 orderHash,
        bytes calldata context
    ) external virtual view returns (bytes4) {}

    function validateOrder(
        IFloodPlain.Order calldata order,
        address book,
        address fulfiller,
        address caller,
        bytes32 orderHash,
        bytes calldata context
    ) external virtual view returns (bytes4) {}
}
