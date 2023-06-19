// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IZone} from "src/zone/IZone.sol";
import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";
import {Pausable} from "@openzeppelin/security/Pausable.sol";

contract MockZone is IZone, Pausable {
    function pause() external {
        _pause();
    }

    function unpause() external {
        _unpause();
    }

    function validateOrder(
        IFloodPlain.Order calldata,
        address,
        address,
        bytes32
    ) external view whenNotPaused {}

    function validateOrder(
        IFloodPlain.Order calldata,
        address,
        address,
        address,
        bytes32,
        bytes calldata
    ) external view whenNotPaused {}
}
