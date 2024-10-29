// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {IZone, IFloodPlain} from "src/interfaces/IZone.sol";
import {Ownable, Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";
import {Pausable} from "@openzeppelin/utils/Pausable.sol";

contract MockZone is IZone, Ownable2Step, Pausable {
    mapping(address fulfiller => bool enabled) private _fulfillers;
    FeeInfo private _fee;

    event FeeUpdated(FeeInfo indexed newFee);
    event FulfillerUpdated(address indexed fulfiller, bool indexed valid);

    constructor(address admin) Ownable(admin) {}

    function fee(IFloodPlain.Order calldata, address) external view returns (FeeInfo memory) {
        return _fee;
    }

    function setFee(FeeInfo calldata newFee) external onlyOwner {
        require(newFee.bps <= 500); // %5 max zone fee.
        _fee.bps = newFee.bps;
        _fee.recipient = newFee.recipient;
        emit FeeUpdated(newFee);
    }

    function setFulfiller(address fulfiller, bool valid) external onlyOwner {
        _fulfillers[fulfiller] = valid;
        emit FulfillerUpdated(fulfiller, valid);
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }

    function validate(IFloodPlain.Order calldata, address fulfiller) external view returns (bool) {
        return _fulfillers[fulfiller] && !paused();
    }
}
