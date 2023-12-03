// SPDX-License-Identifier: MIT
pragma solidity 0.8.23;

// Inheritances
import {IOnChainOrders} from "./interfaces/IOnChainOrders.sol";
import {IFloodPlain} from "./interfaces/IFloodPlain.sol";

// Libraries
import {OrderHash} from "./libraries/OrderHash.sol";

abstract contract OnChainOrders is IOnChainOrders {
    using OrderHash for IFloodPlain.Order;

    function etchOrder(IFloodPlain.SignedOrder calldata orderWithSignature) external {
        bytes32 orderHash = orderWithSignature.order.hash();
        emit IOnChainOrders.OrderEtched(orderHash, orderWithSignature);
    }
}
