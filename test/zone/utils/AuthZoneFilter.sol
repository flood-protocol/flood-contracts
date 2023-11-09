// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IAuthZone} from "src/zone/extensions/IAuthZone.sol";

library AuthZoneFilter {
    function isFilterEqual(IAuthZone.Filter memory a, IAuthZone.Filter memory b) public pure returns (bool) {
        if (a.initialized != b.initialized) {
            return false;
        }
        if (!isAddressFilterEqual(a.offerer, b.offerer)) {
            return false;
        }
        if (a.offer.length != b.offer.length) {
            return false;
        }
        for (uint256 i = 0; i < a.offer.length; i++) {
            if (!isItemFilterEqual(a.offer[i], b.offer[i])) {
                return false;
            }
        }
        if (!isItemFilterEqual(a.consideration, b.consideration)) {
            return false;
        }
        if (!isRangeFilterEqual(a.deadline, b.deadline)) {
            return false;
        }
        if (!isRangeFilterEqual(a.nonce, b.nonce)) {
            return false;
        }
        return true;
    }

    function isItemFilterEqual(IAuthZone.ItemFilter memory a, IAuthZone.ItemFilter memory b)
        public
        pure
        returns (bool)
    {
        return isAddressFilterEqual(a.token, b.token) && isRangeFilterEqual(a.amount, b.amount);
    }

    function isRangeFilterEqual(IAuthZone.RangeFilter memory a, IAuthZone.RangeFilter memory b)
        public
        pure
        returns (bool)
    {
        return a.gte == b.gte && a.lte == b.lte;
    }

    function isAddressFilterEqual(IAuthZone.AddressFilter memory a, IAuthZone.AddressFilter memory b)
        public
        pure
        returns (bool)
    {
        return a.value == b.value && a.exclude == b.exclude;
    }

    function allowNone() public pure returns (IAuthZone.Filter memory) {
        IAuthZone.AddressFilter memory emptyAddressFilter = IAuthZone.AddressFilter({value: address(0), exclude: false});

        IAuthZone.RangeFilter memory emptyRangeFilter = IAuthZone.RangeFilter({gte: 0, lte: 0});

        IAuthZone.ItemFilter memory emptyItemFilter =
            IAuthZone.ItemFilter({token: emptyAddressFilter, amount: emptyRangeFilter});

        IAuthZone.ItemFilter[] memory emptyItemFilters;

        return IAuthZone.Filter({
            initialized: false,
            offerer: emptyAddressFilter,
            offer: emptyItemFilters,
            consideration: emptyItemFilter,
            deadline: emptyRangeFilter,
            nonce: emptyRangeFilter
        });
    }

    function allowAll() public pure returns (IAuthZone.Filter memory) {
        IAuthZone.Filter memory filter = allowNone();
        filter.initialized = true;
        return filter;
    }
}
