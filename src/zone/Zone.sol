// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IZone} from "./IZone.sol";

// Interfaces
import {IFloodPlain} from "../flood-plain/IFloodPlain.sol";

abstract contract Zone is IZone {
    function validateOrder(
        IFloodPlain.Order calldata order,
        address book,
        address fulfiller,
        address caller,
        bytes32 orderHash,
        bytes calldata context
    ) external view virtual {}
}
