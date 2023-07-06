// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IFulfiller} from "./IFulfiller.sol";
import {IFulfillerWithCallback} from "./extensions/IFulfillerWithCallback.sol";
import {Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";
import {Pausable} from "@openzeppelin/security/Pausable.sol";
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";

// Libraries
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";

// Interfaces
import {IFloodPlain} from "../flood-plain/IFloodPlain.sol";
import {IExecutor} from "../executors/IExecutor.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

contract Fulfiller is IFulfiller, IFulfillerWithCallback, Ownable2Step, Pausable, ReentrancyGuard {
    using SafeERC20 for IERC20;
    using Address for address payable;

    address public immutable ZONE;

    ExecutorInfo[] internal _executors;

    mapping(address => bool) internal _books;

    CallbackInfo internal _callbackInfo;

    constructor(address zone) {
        if (zone.code.length == 0) {
            revert NotAContract();
        }

        ZONE = zone;

        _callbackInfo.alwaysTrue = true;
    }

    function getExecutor(uint256 executorId) external view returns (ExecutorInfo memory /* executorInfo */ ) {
        return _executors[executorId];
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
        ExecutorInfo memory executorInfo =
            ExecutorInfo({executor: executor, hasCallback: IExecutor(executor).hasCallback(), isEnabled: true});

        executorId = _executors.length;
        _executors.push(executorInfo);

        emit ExecutorAdded(executorId, executor);
    }

    function disableExecutor(uint256 executorId) external onlyOwner {
        _executors[executorId].isEnabled = false;

        emit ExecutorDisabled(executorId);
    }

    function enableExecutor(uint256 executorId) external onlyOwner {
        _executors[executorId].isEnabled = true;

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

        for (uint256 i; i < length;) {
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
    ) external whenNotPaused nonReentrant returns (uint256[] memory) {
        if (!_books[msg.sender]) {
            revert InvalidBook();
        }
        if (order.zone != ZONE) {
            revert InvalidZone();
        }

        uint256 itemsLength = requestedItems.length;
        uint256[] memory gatheredAmounts = new uint256[](itemsLength);

        // Book must have sent offer items prior to this call. The swap data is ought to use the
        // received offer items. However, this fulfiller will not have any checks to ensure not
        // more than the received offer items are spent. It will therefore trust `caller` to supply
        // honest swap data. The caller is a trusted address and the access control is enforced
        // through the zone.

        // Record requested item balances before the swaps.
        address token;
        for (uint256 i; i < itemsLength;) {
            token = requestedItems[i].token;

            if (token == address(0)) {
                gatheredAmounts[i] = address(this).balance;
            } else {
                gatheredAmounts[i] = IERC20(token).balanceOf(address(this));
            }

            unchecked {
                ++i;
            }
        }

        // Execute swaps based on the swap instructions provided.
        _executeSwaps(context);

        // Get the gathered amounts, and approve book to spend them.
        uint256 gatheredAmount;
        for (uint256 i; i < itemsLength;) {
            token = requestedItems[i].token;

            if (token == address(0)) {
                gatheredAmount = address(this).balance - gatheredAmounts[i];
                payable(msg.sender).sendValue(gatheredAmount);
            } else {
                gatheredAmount = IERC20(token).balanceOf(address(this)) - gatheredAmounts[i];
                IERC20(token).safeIncreaseAllowance(msg.sender, gatheredAmount);
            }

            gatheredAmounts[i] = gatheredAmount;

            unchecked {
                ++i;
            }
        }

        // Let Book pull the tokens and send to offerer. Book checks if gathered amounts cover
        // requested amounts.
        return gatheredAmounts;
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
        assembly {
            ptr := swaps.offset
        }

        // Variables reset at each iteration, defined outside the loop to save gas.
        IExecutor.Swap memory swap;
        bytes memory swapData;
        uint64 executorId;
        ExecutorInfo memory executorInfo;
        CallbackInfo storage callbackInfo = _callbackInfo;
        address executor;
        bool hasCallback;

        // Execute a swap with each iteration.
        while (msg.data.length > ptr) {
            // Decode first swap instructions from ptr, ensuring executor is not disabled.
            (ptr, executorId, swap) = _decodeSwap(ptr);

            // Get executor details.
            executorInfo = _executors[executorId];
            executor = executorInfo.executor;

            // Ensure executor is not disabled.
            if (!executorInfo.isEnabled) {
                revert DisabledExecutor();
            }

            // If executor requires a callback...
            hasCallback = executorInfo.hasCallback;
            if (hasCallback) {
                // Set callback info pseudo-transient storage accordingly.
                callbackInfo.expectingCallback = true;
                callbackInfo.activeExecutorId = executorId;
                callbackInfo.callbackSource = IExecutor(executor).getCallbackSource(swap);
            }

            // Construct calldata to pass to the executor.
            swapData = abi.encodeWithSelector(IExecutor.swap.selector, swap);
            assembly {
                // Delegate swap execution to the executor.
                let result := delegatecall(gas(), executor, add(swapData, 0x20), mload(swapData), 0, 0)

                // Copy the returned data.
                returndatacopy(0, 0, returndatasize())

                if iszero(result) { revert(0, returndatasize()) }
            }

            // If executor had a callback...
            if (hasCallback) {
                // Unset callback info pseudo-transient storage after the swap is completed.
                callbackInfo.expectingCallback = false;
                callbackInfo.activeExecutorId = 0;
                callbackInfo.callbackSource = address(0);
            }
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
    function _decodeSwap(uint256 ptr)
        internal
        pure
        returns (uint256 endPtr, uint64 executorId, IExecutor.Swap memory swap)
    {
        address pool;
        uint256 tokenInIndex;
        uint256 tokenOutIndex;
        uint256 amountIn;
        uint256 amountOut;

        // Decode instructions based on the above-described scheme.
        assembly {
            executorId := shr(248, calldataload(ptr))
            ptr := add(ptr, 1)

            let amountsSizes := calldataload(ptr)
            ptr := add(ptr, 1)

            let amountInSize := add(shr(252, amountsSizes), 1)
            let amountOutSize := add(and(shr(248, amountsSizes), 0x0f), 1)

            amountIn := shr(sub(256, shl(3, amountInSize)), calldataload(ptr))
            ptr := add(ptr, amountInSize)

            amountOut := shr(sub(256, shl(3, amountOutSize)), calldataload(ptr))
            ptr := add(ptr, amountOutSize)

            pool := shr(96, calldataload(ptr))
            ptr := add(ptr, 20)

            let tokenIndices := calldataload(ptr)
            endPtr := add(ptr, 1)

            tokenInIndex := shr(252, tokenIndices)
            tokenOutIndex := and(shr(248, tokenIndices), 0x0f)
        }

        swap = IExecutor.Swap({
            pool: pool,
            tokenInIndex: tokenInIndex,
            tokenOutIndex: tokenOutIndex,
            amountIn: amountIn,
            amountOut: amountOut
        });
    }

    function _fallback() internal {
        CallbackInfo memory callbackInfo = _callbackInfo;
        if (!callbackInfo.expectingCallback) {
            revert FallbackNotThroughExecutor();
        } else {
            // Ensure only the pool can callback.
            if (callbackInfo.callbackSource != msg.sender) {
                revert CallbackNotByPool();
            }

            // Get executor address corresponding to an identifier.
            address executor = _executors[callbackInfo.activeExecutorId].executor;

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

                switch result
                // delegatecall returns 0 on error.
                case 0 {
                    // Copy the returned data.
                    returndatacopy(0, 0, returndatasize())
                    revert(0, returndatasize())
                }
                default { return(0, returndatasize()) }
            }
        }
    }
}
