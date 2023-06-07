// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IFulfiller} from "./interfaces/IFulfiller.sol";
import {Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";
import {Pausable} from "@openzeppelin/security/Pausable.sol";
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";

// Libraries
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";
import {BitMaps} from "@openzeppelin/utils/structs/BitMaps.sol";

// Interfaces
import {IFloodPlain} from "./interfaces/IFloodPlain.sol";
import {IExecutor} from "./interfaces/IExecutor.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

contract Fulfiller is IFulfiller, Ownable2Step, Pausable, ReentrancyGuard {
    using SafeERC20 for IERC20;
    using Address for address payable;
    using BitMaps for BitMaps.BitMap;

    address public immutable ZONE;

    address internal constant _LOGICAL_ZERO_ADDRESS = address(0xdead);
    address public activeExecutor = _LOGICAL_ZERO_ADDRESS;
    address public activePool = _LOGICAL_ZERO_ADDRESS;

    address[] internal _executors;
    BitMaps.BitMap internal _disabledExecutors;

    mapping(address => bool) internal _books;

    constructor(address zone) {
        if (zone.code.length == 0) {
            revert NotAContract();
        }

        ZONE = zone;
    }

    function getExecutor(uint256 executorId) external view returns (address, /* executor */ bool /* enabled */ ) {
        return (_executors[executorId], !_disabledExecutors.get(executorId));
    }

    function getBookValidity(address book) external view returns (bool /* enabled */ ) {
        return _books[book];
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }

    function addExecutor(address executor) external onlyOwner returns (uint256 executorId) {
        if (executor.code.length == 0) {
            revert NotAContract();
        }

        executorId = _executors.length;
        _executors.push(executor);

        emit ExecutorAdded(executorId, executor);
    }

    function disableExecutor(uint256 executorId) external onlyOwner {
        _disabledExecutors.set(executorId);

        emit ExecutorDisabled(executorId);
    }

    function enableExecutor(uint256 executorId) external onlyOwner {
        _disabledExecutors.unset(executorId);

        emit ExecutorEnabled(executorId);
    }

    function disableBook(address book) external onlyOwner {
        _books[book] = false;

        emit BookDisabled(book);
    }

    function enableBook(address book) external onlyOwner {
        _books[book] = true;

        emit BookEnabled(book);
    }

    function batchWithdraw(IFloodPlain.Item[] calldata items) external onlyOwner {
        uint256 length = items.length;
        IFloodPlain.Item calldata item;

        for (uint256 i = 0; i < length;) {
            item = items[i];

            if (item.token == address(0)) {
                payable(msg.sender).sendValue(item.amount);
            } else {
                IERC20(item.token).safeTransfer(msg.sender, item.amount);
            }

            unchecked {
                ++i;
            }
        }
    }

    function pay() external payable onlyOwner {}

    function sourceConsideration(
        IFloodPlain.Order calldata order,
        IFloodPlain.Item[] calldata requestedItems,
        address, /* caller */
        bytes calldata context
    ) external whenNotPaused nonReentrant {
        if (!_books[msg.sender]) {
            revert InvalidBook();
        }
        if (order.zone != ZONE) {
            revert InvalidZone();
        }

        // Book must have sent offer items prior to this call. The swap data is ought to use the
        // received offer items. However, this fulfiller will not have any checks to ensure not
        // more than the received offer items are spent. It will therefore trust `caller` to supply
        // honest swap data. The caller is a trusted address and the access control is enforced
        // through the zone.

        // Execute swaps based on the swap instructions provided.
        _executeSwaps(context);

        // Send requested items to the offerer.
        uint256 itemsLength = requestedItems.length;
        address to = order.offerer;
        IFloodPlain.Item calldata item;
        for (uint256 i = 0; i < itemsLength;) {
            item = requestedItems[i];

            if (item.token == address(0)) {
                payable(to).sendValue(item.amount);
            } else {
                IERC20(item.token).safeTransfer(to, item.amount);
            }

            unchecked {
                ++i;
            }
        }
    }

    fallback() external payable {
        _fallback();
    }

    receive() external payable {
        _fallback();
    }

    function _executeSwaps(bytes calldata swaps) internal {
        // Pointer is incremented with each loop.
        uint256 ptr = 0;

        // End is the condition set at end of each iteration to break the loop.
        bool end = false;

        // Variables reset at each iteration, defined outside the loop to save gas.
        IExecutor.Swap memory swap;
        bytes memory swapData;
        address executor;

        // Execute a swap with each iteration.
        while(end) {
            // Decode first swap instructions from ptr, ensuring executor is not disabled.
            (ptr, executor, swap) = _decodeSwap(ptr, swaps);

            // Set active executor and pool before delegating execution to the executor.
            activeExecutor = executor;
            activePool = swap.pool;

            // Construct calldata to pass to the executor.
            swapData = abi.encodeWithSelector(IExecutor.swap.selector, swap);
            assembly {
                // Delegate swap execution to the executor.
                let result := delegatecall(gas(), executor, add(swapData, 0x20), mload(swapData), 0, 0)

                // Copy the returned data.
                returndatacopy(0, 0, returndatasize())

                if iszero(result) {
                    revert(0, returndatasize())
                }

                // Break the loop if next word is empty.
                end := iszero(calldataload(ptr))
            }

            // Unset active executor and pool after the swap is completed.
            activeExecutor = _LOGICAL_ZERO_ADDRESS;
            activePool = _LOGICAL_ZERO_ADDRESS;
        }
    }

    // Swaps are series of packets which follow the following scheme:
    //
    // * 1 byte - executor Id
    // * 1 byte - bytes required for amountIn and amountOut
    //       1 nibble - bytes required for amountIn, subtracted by one
    //       1 nibble - bytes required for amountOut, subtracted by one
    // * n bytes - amount in (this fulfiller uses max-in-exact-out slippage control scheme)
    // * m bytes - amount out
    // * 20 bytes - pool address to perform the swap
    // * 1 byte - indices of token in & token out in the pool
    //       1 nibble - index of token in
    //       1 nibble - index of token out
    //
    // Caveat: Token indices in a pool can change when tokens are added or removed from a
    //         multi-token pool. This can theoretically be abused by the pool owner to steal funds
    //         from the fulfiller by frontrunning a swap. This is an accepted risk.
    function _decodeSwap(uint256 ptr, bytes calldata swaps) internal view returns (uint256 endPtr, address executor, IExecutor.Swap memory swap) {
            unchecked {
                // Decode instructions based on the above-described scheme.
                uint256 executorId = abi.decode(swaps[ptr:++ptr], (uint256));
                uint256 amountsSizes = abi.decode(swaps[ptr:++ptr], (uint256));
                uint256 amountInSize = (amountsSizes >> 4) + 1;
                uint256 amountOutSize = (amountsSizes & 0x0f) + 1;
                swap.amountIn = abi.decode(swaps[ptr:ptr += amountInSize], (uint256));
                swap.amountOut = abi.decode(swaps[ptr:ptr += amountOutSize], (uint256));
                swap.pool = abi.decode(swaps[ptr:ptr += 20], (address));
                uint256 tokenIndices = abi.decode(swaps[ptr:endPtr = ptr + 1], (uint256));
                swap.tokenInIndex = tokenIndices >> 4;
                swap.tokenOutIndex = amountsSizes & 0x0f;

                // Ensure executor is not disabled.
                if (_disabledExecutors.get(executorId)) {
                    revert DisabledExecutor();
                }

                // Get executor address from the executor index.
                executor = _executors[executorId];
            }
    }

    function _fallback() internal {
        address executor = activeExecutor;
        if (executor == _LOGICAL_ZERO_ADDRESS) {
            revert FallbackNotThroughExecutor();
        } else {
            // Ensure only the pool can callback.
            if (msg.sender != activePool) {
                revert CallbackNotByPool();
            }
            // When a swap is going through the executor, a call made the Fulfiller is actually
            // a callback to the executor. So we delegatecall to the active executor to continue
            // completing the swap. Note that if a callback function has a signature clash with an
            // existing function in this contract, the fallback will not be reached. Instead of
            // having a custom function dispatcher to prevent this issue, we have accepted the risk
            // of potentially having an incompatible pool with this fulfiller. If we encounter such
            // a pool, we can then think of writing another fulfiller with a workaround. As long as
            // there is no security risks currently, then it should be fine.
            assembly {
                // Copy msg.data. We take full control of memory in this inline assembly
                // block because it will not return to Solidity code. We overwrite the
                // Solidity scratch pad at memory position 0.
                calldatacopy(0, 0, calldatasize())

                // Call the implementation.
                // out and outsize are 0 because we don't know the size yet.
                let result := delegatecall(gas(), executor, 0, calldatasize(), 0, 0)

                // Copy the returned data.
                returndatacopy(0, 0, returndatasize())

                switch result
                // delegatecall returns 0 on error.
                case 0 { revert(0, returndatasize()) }
                default { return(0, returndatasize()) }
            }
        }
    }
}
