// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {IAuthZone} from "./interfaces/IAuthZone.sol";
import {IFloodPlain} from "./interfaces/IFloodPlain.sol";
import {AccessControlDefaultAdminRules} from "@openzeppelin/access/extensions/AccessControlDefaultAdminRules.sol";
import {Pausable} from "@openzeppelin/utils/Pausable.sol";

contract Zone is IAuthZone, AccessControlDefaultAdminRules, Pausable {
    bytes32 public constant FULFILLER_ROLE = keccak256("FULFILLER_ROLE");
    mapping(address => AuthFilter) private _filters;

    FeeInfo private _fee;

    event FeeUpdated(FeeInfo indexed newFee);
    event FulfillerUpdated(address indexed fulfiller, bool indexed valid);
    event FilterUpdated(address indexed actor, AuthFilter filter);

    constructor(address admin) AccessControlDefaultAdminRules(1 hours, admin) {}

    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    function setFee(FeeInfo calldata newFee) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _fee.bps = newFee.bps;
        _fee.recipient = newFee.recipient;
        emit FeeUpdated(newFee);
    }

    function setAuthorizationFilter(address actor, AuthFilter calldata filter) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _filters[actor] = filter;
        emit FilterUpdated(actor, filter);
    }

    function fee(IFloodPlain.Order calldata, address /* fulfiller */ ) external view returns (FeeInfo memory) {
        return _fee;
    }

    function validate(IFloodPlain.Order calldata, /* order */ address fulfiller) external view returns (bool) {
        return hasRole(FULFILLER_ROLE, fulfiller) && !paused();
    }

    function authorizationFilter(address actor) external view returns (AuthFilter memory) {
        return _filters[actor];
    }
}
