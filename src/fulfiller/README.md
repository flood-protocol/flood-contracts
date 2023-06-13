# Fulfiller

FloodPlain requires a contract to fulfill its orders. The fulfiller must implement `sourceConsideration` function.

```sol
    /**
     * @notice External function to perform arbitrary swaps to source requested items.
     *
     * @param order All the details of a book order.
     * @param requestedItems Deduplicated consideration items in an order.
     * @param caller The address that called the book.
     * @param context All the swap data, or any other arbitrary data.
     */
    function sourceConsideration(
        IFloodPlain.Order calldata order,
        IFloodPlain.Item[] calldata requestedItems,
        address caller,
        bytes calldata context
    ) external;
```

Fulfiller must check the `msg.sender` is a valid Book contract (FloodPlain). A valid Book contract must send `order.offer` items to the Fulfiller before calling its `sourceConsideration` function. Fulfiller must send `requestedItems` to `order.offerer`. `requestedItems` is deduplicated `order.consideration`. Fulfiller can also implement access control by checking `caller`. Alternatively, Fulfiller can ensure an order is made through a specific zone by checking `order.zone`, then let the zone handle the access control. This is useful to not have the same allowlist on separate Fulfillers.

The current Fulfiller holds operating capital and accumulated fees. Callers can steal all the funds in this contract by specifying arbitrary swaps. Therefore access control is a must in this version. However, it is also possible to have a Fulfiller that does not hold any funds hence cannot be drained. Such Fulfillers might skip access control checks.
