// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import {IFloodPlain} from "./IFloodPlain.sol";

interface IOnChainOrders {
    event OrderEtched(bytes32 indexed orderHash, IFloodPlain.SignedOrder signedOrder);

    /**
     * @notice Record an order on-chain for ease of use by other contracts.
     *
     * @param signedOrder The order and its signature to record.
     */
    function etchOrder(IFloodPlain.SignedOrder calldata signedOrder) external;
}
