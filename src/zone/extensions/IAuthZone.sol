// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface IAuthZone {
    event FilterUpdated(address indexed actor, Filter filter);

    struct RangeFilter {
        uint gte;
        uint lte;
    }

    struct TokenFilter {
        address token;
        RangeFilter amount;
    }

    struct Filter {
        address offerer;
        TokenFilter[] offer;
        TokenFilter consideration;
        RangeFilter deadline;
        RangeFilter nonce;
    }

    function setAuthorizationFilter(address actor, Filter calldata filter) external;

    function authorizationFilter(address actor) external view returns (Filter memory);
}
