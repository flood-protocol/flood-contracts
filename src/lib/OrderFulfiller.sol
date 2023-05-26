// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {CounterManager} from "seaport-core/src/lib/CounterManager.sol";
import {ReentrancyGuard} from "seaport-core/src/lib/ReentrancyGuard.sol";

import {OrderParameters, Order} from "./OrderBookStructs.sol";

contract OrderFulfiller is CounterManager, ReentrancyGuard {
    function _validateAndFulfillOrder(Order calldata order, address caller) internal returns (bool) {
        // Set strict reentrancy guard that does not accept NATIVE tokens.
        _setReentrancyGuard({ acceptNativeTokens: false} );

        // Validate order and update status.
        bytes32 orderHash = _validateOrderAndUpdateStatus({ order: Order });

        // Retrieve the order parameters.
        OrderParameters calldata orderParameters = order.parameters;

        // Check zone restrictions.
        address zone = orderParameters.zone;
        if (zone != address(0)) {
            IZone(zone).validateOrder({
                order: orderParameters,
                fulfiller: fulfiller,
                caller: caller,
                orderHash: orderHash
            });
        }

        // Transfer each offer item to fulfiller.
        _transferItemsFrom({ from: orderParameters.offerer, to: fulfiller, items: orderParameters.offer });

        // Call fulfiller with order data and the caller address to source consideration items.
        // Contracts implementing Fulfiller interface could get all their tokens drained, hence
        // they should not hold any tokens.
        IFulfiller(fulfiller).sourceConsideration({
            items: orderParameters.consideration,
            caller: caller,
            context: orderParameters.extraData
        });

        // Transfer consideration items from fulfiller to offerer.
        _transferItemsFrom({ from: fulfiller, to: offerer, items: orderParameters.consideration });

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, orderParameters.offerer, orderParameters.fulfiller);

        // Clear the reentrancy guard.
        _clearReentrancyGuard();

        return true;
    }
}
