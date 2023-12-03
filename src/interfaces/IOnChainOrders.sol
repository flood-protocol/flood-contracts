// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import {IFloodPlain} from "./IFloodPlain.sol";

interface IOnChainOrders {
    event OrderEtched(bytes32 indexed orderHash, IFloodPlain.SignedOrder orderWithSignature);

    /**
     * @notice Record an order on-chain for ease of use by other contracts.
     *
     * @param orderWithSignature The order and its signature to record.
     */
    function etchOrder(IFloodPlain.SignedOrder calldata orderWithSignature) external;
}
