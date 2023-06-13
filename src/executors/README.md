# Executors

An executor contains logic for performing a specific swap operation for a Fulfiller. [This interface](./IExecutor.sol) is designed for the current version of the Fulfiller. There can be different executor interfaces for different Fulfillers.

## Known Issues

There are two known issues with the current design.

### Selector Clash

An executor added might have a callback function that clashes either with the `swap(Swap memory swap)` function, or any other function in the Fulfiller. If such a callback function is required to be added, the swap would need to go through a Router contract. So the execution flow below

    EAO calls FloodPlain.fulfillOrder calls Fulfiller.sourceConsideration delegatecalls Executor.swap calls Pool calls Fulfiller.fallback delegatecalls Executor.callback

would become

    EAO calls FloodPlain.fulfillOrder calls Fulfiller.sourceConsideration delegatecalls Executor.swap calls Router calls Pool calls Router.callback


### Mutable token indices

Current Executor interface gets all the swap information as a Swap struct.

```sol
    struct Swap {
        address pool;
        uint256 tokenInIndex;
        uint256 tokenOutIndex;
        uint256 amountIn;
        uint256 amountOut;
    }
```

Instead of specifying token addresses explicity, token indices are specified. The reasoning behind this is to reduce the L1 storage fees when doing an L2 transaction (i.e.: reducing calldata size). Therefore swap instructions are given as a custom packed data to Fulfiller. The Fulfiller then decodes this data. This allows implicitly providing two token addresses in a single byte instead of explicity giving 40 bytes for two token addresses.

Some pools, such as Balancer pools, have mutable token indices. When new tokens are added or removed from the pool, the indices might change. If a malicious actor controls such a pool, they can frontrun a swap to change token indices, and theoretically steal the operating capital and accumulated fees in Fulfiller.

To prevent such an attack vector, only trusted protocols are whitelisted. And a caller (EAO/bot) must check pool indices, and whether there are any pending multisig or governance proposal to change the pool tokens, right before fulfilling an order.
