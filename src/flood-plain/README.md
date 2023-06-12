# FloodPlain (Book)

FloodPlain is an order book which allows fulfillment of orders through arbitrary execution.

A FloodPlain order has an arbitrary amount of ERC20 offer items and consideration items. Offer items are what the offerer (order maker) wants to give, and the consideration items are what the offerer wants to receive in return.

An order is a single Permit2 signature which includes necessary token approvals and the order data. The signater verification and order cancellation features are both relegated to Permit2.

To create an order, offerer has to sign `PermitBatchWitnessTransferFrom` struct of Permit2 based on the EIP712 standard. The details of this struct is below.

```sol
    struct PermitBatchWitnessTransferFrom {
        TokenPermissions[] permitted;
        address spender;
        uint256 nonce;
        uint256 deadline;
        Order witness;
    }

    struct Item {
        address token;
        uint256 amount;
    }

    struct Order {
        address offerer;
        address zone;
        Item[] offer;
        Item[] consideration;
        uint256 deadline;
        uint256 nonce;
    }

    struct TokenPermissions {
        address token;
        uint256 amount;
    }
```

Note that in this structure, `witness.nonce == nonce`, `witness.deadline == deadline`, and `witness.offer == permitted`. This redundancy is for simplicity, because it allows Order struct to contain all the information related to an order. The order also has a Zone. Zone is used to enforce how and by whom an order should be filled.

After the order is signed, the order and its signature can be stored using standard Web2 solutions, or using IPFS.

Fulfilling an order requires calling `FloodPlain.fulfillOrder`. The caller also provides additional information as seen below in the function header.

```sol
    function fulfillOrder(Order calldata order, bytes calldata signature, address fulfiller, bytes calldata extraData) external;
```

The `fulfiller` address is what makes FloodPlain unique. It is a special contract that takes offer items, and use those to source consideration items in whichever way it wants. The `extraData` can be passed to the fulfiller to instruct how it should source the consideration items. Below is the execution flow of `fulfillOrder`.

1. EIP712 hash of the order is calculated
2. All the arguments of the fulfillOrder along with the order hash is passed to the zone of the order. The Zone can employ any arbitrary restrictions. It is mainly intended to be used for access control (i.e.: who can fulfill an order using which fulfiller contract).
3. Order data and the Permit2 signature is passed to Permit2 contract.
4. Permit2 performs signature validation, nonce and deadline checks, and transfers offer tokens from the offerer to the fulfiller contract.
5. Consideration items are deduplicated, then `Fulfiller.sourceConsideration` is called with order details and extra arbitrary bytes as swap instructions. Balances of consideration items of the offerer is checked before and after calling the fulfiller to ensure FloodPlain guarantees the proper fulfillment of user intent regardless of which fulfiller is used.

## Extensions

There are two extensions for FloodPlain.

### FloodPlainEncodedCalls

`FloodPlainEncodedCalls` extension adds a fallback function that allows decoding of non-standard calldata. This is to minimize calldata sizes when fulfilling or etching orders. This is desired on L2 chains, because on L2s, calldata is the primary driver of transaction costs.

This extension adds an owner to the FloodPlain contract. The owner can add 256 decoders to the contract. Once a decoder is added, it cannot be changed or removed. A decoder's identifier is its array index. So a valid decoder has an identifier between 0 to 255 inclusive. When a user wants to fulfill an order, instead of calling `fulfillOrder`, they would pass a custom calldata to execute fallback function. The first byte of this custom calldata must be a unique byte that no other function signature in the contract starts with. It is guaranteed that such a unique byte exists if there are less than 256 external functions in the contract. Inside fallback, first byte is discarded, and the second byte is used as the identifier of the decoder contract. The rest of the calldata is passed unmodified to the decoder contract as a staticcall. The returned data from the decoder is delegatecalled to the FloodPlain contract itself. This is logically the same as making a call with that decoded data directly.

### FloodPlainOnChainOrders

`FloodPlainOnChainOrders` extension allows recording ("etching") a signature order on chain. This could make it easier for contracts to integrate. An etched order is not checked for any type of validity. The validity of an order is always checked during the fulfillment. Fulfilling an etched order reuses the fulfillOrder function by delegatecalling itself with the recorded on chain order data. So to fulfill an order, a user would only need to pass the order identifier, which is then used to construct fulfillOrder parameter. It is also possible to use encoded calls to etch an order to reduce calldata size. There would be a separate encoder for this purpose.
