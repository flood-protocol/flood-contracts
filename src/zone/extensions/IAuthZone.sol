// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IAuthZone {
    event FilterUpdated(address indexed actor, AuthFilter filter);

    struct AddressFilter {
        address value;
        bool exclude;
    }

    struct RangeFilter {
        uint256 gte;
        uint256 lte;
    }

    struct ItemFilter {
        AddressFilter token;
        RangeFilter amount;
    }

    struct AuthFilter {
        bool initialized;
        AddressFilter offerer;
        ItemFilter[] offer;
        ItemFilter consideration;
        RangeFilter deadline;
        RangeFilter nonce;
    }

    function setAuthorizationFilter(address actor, AuthFilter calldata filter) external;

    function authorizationFilter(address actor) external view returns (AuthFilter memory);
}
