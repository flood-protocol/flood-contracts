// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "../FloodPlain.sol";
import {IFloodPlainOnChainOrders} from "./IFloodPlainOnChainOrders.sol";

// Libraries
import {OrderHash} from "../libraries/OrderHash.sol";

// Interfaces
import {IFloodPlain} from "../IFloodPlain.sol";

abstract contract FloodPlainOnChainOrders is FloodPlain, IFloodPlainOnChainOrders {
    using OrderHash for Order;

    OrderWithSignature[] internal _etchedOrders;

    function getEtchedOrder(uint256 etchedOrderId)
        external
        view
        returns (OrderWithSignature memory /* etchedOrder */ )
    {
        return _etchedOrders[etchedOrderId];
    }

    function fulfillEtchedOrder(uint256 orderId, address fulfiller, bytes calldata extraData) external {
        OrderWithSignature memory orderWithSignature = _etchedOrders[orderId];

        // Fulfill the etched order using the standard fulfillment function.
        bytes memory data = abi.encodeWithSelector(
            this.fulfillOrder.selector, orderWithSignature.order, orderWithSignature.signature, fulfiller, extraData
        );
        assembly {
            let result := delegatecall(gas(), address(), add(data, 0x20), mload(data), 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            switch result
            // delegatecall returns 0 on error.
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }

    function etchOrder(OrderWithSignature calldata orderWithSignature) external returns (uint256 orderId) {
        orderId = _etchedOrders.length;
        _etchedOrders.push(orderWithSignature);
        emit OrderEtched({
            orderId: orderId,
            orderHash: orderWithSignature.order.hash(),
            order: orderWithSignature.order,
            signature: orderWithSignature.signature
        });
    }
}
