// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IAuthZone {
    event FilterUpdated(address indexed actor, Filter filter);

    struct RangeFilter {
        uint256 gte;
        uint256 lte;
    }

    struct ItemFilter {
        address token;
        RangeFilter amount;
    }

    struct Filter {
        bool initialized;
        address offerer;
        ItemFilter[] offer;
        ItemFilter consideration;
        RangeFilter deadline;
        RangeFilter nonce;
    }

    function setAuthorizationFilter(address actor, Filter calldata filter) external;

    function authorizationFilter(address actor) external view returns (Filter memory);
}
