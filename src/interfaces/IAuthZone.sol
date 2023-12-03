// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import {IZone} from "./IZone.sol";

interface IAuthZone is IZone {
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

    function authorizationFilter(address actor) external view returns (AuthFilter memory);
}
