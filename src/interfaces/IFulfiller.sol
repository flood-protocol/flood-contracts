// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IFloodPlain} from "../interfaces/IFloodPlain.sol";

interface IFulfiller {
    error FallbackNotThroughExecutor();

    error InvalidBook();

    error InvalidZone();

    error DisabledExecutor();

    error CallbackNotByPool();

    error NotAContract();

    event ExecutorAdded(uint256 indexed executorId, address indexed executor);

    event ExecutorEnabled(uint256 indexed executorId);

    event ExecutorDisabled(uint256 indexed executorId);

    event BookEnabled(address indexed book);

    event BookDisabled(address indexed book);

    struct ExecutorInfo {
        address executor;
        bool hasCallback;
        bool isEnabled;
    }

    struct CallbackInfo {
        bool alwaysTrue;
        bool expectingCallback;
        uint64 activeExecutorId;
        address callbackSource;
    }

    /**
     * @notice Get the zone address that the Fulfiller requires all orders to go through.
     *
     * @return zone The zone address.
     */
    function ZONE() external view returns (address zone);

    /**
     * @notice Get the executor that corresponds to an identifier.
     *
     * @dev An executor performs a specific swap to source consideration items.
     *
     * @param executorId The array index of the executor.
     *
     * @return executorInfo The details of the executor.
     */
    function getExecutor(uint256 executorId) external view returns (ExecutorInfo memory executorInfo);

    /**
     * @notice Restricted function to disable `sourceConsiderations` function.
     */
    function pause() external;

    /**
     * @notice Restricted function to enable `sourceConsiderations` function.
     */
    function unpause() external;

    /**
     * @notice Restricted function to add an executor to the executors array.
     *
     * @dev An executor extends the functionality of Fulfiller by way of a delegatecall. An
     *      improperly written executor can result in the entire funds in the contract to be lost.
     *
     * @param executor The address of the executor to be added.
     *
     * @return executorId The array index of the executor added.
     */
    function addExecutor(address executor) external returns (uint256 executorId);

    /*
     * @notice Restricted function to disable using an executor.
     *
     * @dev An executor is enabled by default.
     *
     * @param executorId The array index of the executor.
     */
    function disableExecutor(uint256 executorId) external;

    /**
     * @notice Restricted function to enable using an executor.
     *
     * @dev An executor is enabled by default.
     *
     * @param executorId The array index of the executor.
     */
    function enableExecutor(uint256 executorId) external;

    /*
     * @notice Restricted function to disable `sourceConsideration` calls for an address.
     *
     * @dev `sourceConsideration` calls are disabled to everyone by default.
     *
     * @param book The address that is prevented from calling `sourceConsideration`.
     */
    function disableBook(address book) external;

    /**
     * @notice Restricted function to enable `sourceConsideration` calls for an address.
     *
     * @dev `sourceConsideration` calls are disabled to everyone by default. Enabling a book allows
     *       book from requesting tokens from fulfiller. A properly created book first sends offer
     *       tokens to the fulfiller. More importantly, the book calls the zone of this fulfiller
     *       which will enforce access control. Without proper access control, operating capital
     *       and accumulated fees can be stolen by anyone. Therefore new books must be added with
     *       extreme care and detail.
     *
     * @param book The address that can now call `sourceConsideration`.
     */
    function enableBook(address book) external;

    /**
     * @notice Restricted function to withdraw tokens from the contract.
     *
     * @dev A token with zero address is for withdrawing native gas token.
     *
     * @param items A list of tokens with amounts to withdraw.
     */
    function batchWithdraw(IFloodPlain.Item[] calldata items) external;

    /**
     * @notice Restricted function to send native tokens to the contract.
     */
    function pay() external payable;

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

    /**
     * @notice Fallback function to redirect callbacks from pools that executors call.
     *
     * @dev When a swap is going through the executor, a call made the Fulfiller is actually a
     *      callback to the executor. So we delegatecall to the active executor to continue
     *      completing the swap. Note that if a callback function has a signature clash with an
     *      existing function in this contract, the fallback will not be reached. To prevent that
     *      a fulfiller can employ a custom function dispatcher that redirects all calls to the
     *      executor if an active executor is set.
     */
    fallback() external payable;

    /**
     * @notice Receive function to redirect callbacks from pools that executors call.
     *
     * @dev When a swap is going through the executor, a call made the Fulfiller is actually a
     *      callback to the executor. So we delegatecall to the active executor to continue
     *      completing the swap.
     */
    receive() external payable;
}
