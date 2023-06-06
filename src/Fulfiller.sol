// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {IFulfiller} from "./interfaces/IFulfiller.sol";
import {IFloodPlain} from "./interfaces/IFloodPlain.sol";
import {Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";
import {Pausable} from "@openzeppelin/security/Pausable.sol";

import {IERC20, SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";

// fulfiller holds both operating capital and accumulated fees.
//
contract Fulfiller is IFulfiller, Ownable2Step, Pausable {
    using SafeERC20 for IERC20;
    using Address for address payable;

    address private constant _LOGICAL_ZERO_ADDRESS = address(0xdead);
    address private _activeExecutor = _LOGICAL_ZERO_ADDRESS;

    // A list of executors that can be used to perform swaps.
    address[] private _executors;

    mapping(address => bool) private _books;
    mapping(uint256 => bool) private _disabledExecutors;

    address private immutable _zone;

    error InvalidFunctionSelector();
    error FallbackNotThroughExecutor();
    error InvalidBook();
    error InvalidZone();

    modifier divertCallback() {
        address activeExecutor = _activeExecutor;
        if (_activeExecutor == _LOGICAL_ZERO_ADDRESS) {
            _;
        } else {
            // Selector clash, go to fallback.
            _fallback(activeExecutor);
        }
    }

    constructor(address zone) {
        _zone = zone;
    }

    function getZone() external /*view*/ divertCallback returns (address) {
        return _zone;
    }

    function getExecutorById(uint256 id) external /*view*/ divertCallback returns (address, bool) {
        return (_executors[id], _disabledExecutors[id]);
    }

    function getBookValidity(address book) external /*view*/ divertCallback returns (bool) {
        return _books[book];
    }

    function pause() external onlyOwner divertCallback {
        _pause();
    }

    function unpause() external onlyOwner divertCallback {
        _unpause();
    }

    function addExecutor(address executor) external onlyOwner divertCallback {
        _executors.push(executor);
    }

    function disableExecutor(uint256 executorId) external onlyOwner divertCallback {
        _disabledExecutors[executorId] = true;
    }

    function enableExecutor(uint256 executorId) external onlyOwner divertCallback {
        _disabledExecutors[executorId] = false;
    }

    // WARNING: Ensure added book uses zone for access restriction, otherwise ALL in this contract will be lost.
    function enableBook(address book) external onlyOwner divertCallback {
        _books[book] = true;
    }

    function disableBook(address book) external onlyOwner divertCallback {
        _books[book] = false;
    }

    /**
     * @notice Withdraws tokens from the contract.
     * @param tokens the tokens to withdraw.
     */
    function batchWithdraw(address[] calldata tokens) external onlyOwner divertCallback {
        uint256 length = tokens.length;
        for (uint256 i = 0; i < length;) {
            address token = tokens[i];

            if (token == address(0)) {
                payable(msg.sender).sendValue(address(this).balance);
            } else {
                IERC20(token).safeTransfer(msg.sender, IERC20(token).balanceOf(address(this)));
            }

            unchecked {
                ++i;
            }
        }
    }

    function sourceConsideration(
        IFloodPlain.Order calldata order,
        IFloodPlain.ConsiderationItem[] calldata requestedItems,
        address, /* caller */
        bytes calldata /* context */
    ) external whenNotPaused divertCallback {
        if (!_books[msg.sender]) {
            revert InvalidBook();
        }
        if (order.zone != _zone) {
            revert InvalidZone();
        }

        // EXECUTE SWAPS HERE
        // for each swap set and unset active executor before and after
        // Ideally add a check that Fulfiller only swaps out the offer items received from the book, otherwsie fulfiller could be in loss.

        // SEND REQUESTED ITEMS TO ORDER.OFFERER HERE
        // Ideally add a check that Fulfiller made the requested amounts during the above swaps, otherwise fulfiller would be in loss.
        {
            uint256 itemsLength = requestedItems.length;
            address to = order.offerer;
            IFloodPlain.ConsiderationItem calldata item;
            for (uint256 i = 0; i < itemsLength;) {
                item = requestedItems[i];

                if (item.isNative) {
                    payable(to).sendValue(item.amount);
                } else {
                    IERC20(item.token).safeTransfer(to, item.amount);
                }

                unchecked {
                    ++i;
                }
            }
        }
    }

    function _fallback(address executor) private {
        if (executor == _LOGICAL_ZERO_ADDRESS) {
            revert FallbackNotThroughExecutor();
        } else {
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

    fallback() external payable {
        _fallback(_activeExecutor);
    }

    receive() external payable {
        _fallback(_activeExecutor);
    }
}
