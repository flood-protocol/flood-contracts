// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IExecutor {
    struct Swap {
        address pool;
        uint256 tokenInIndex;
        uint256 tokenOutIndex;
        uint256 amountIn;
        uint256 amountOut;
    }

    /**
     * @notice Return true if this executor has a callback.
     */
    function hasCallback() external view returns (bool);

    /**
     * @notice Get the address that might do a callback during a swap.
     *
     * @param swap The details of the swap that will be made.
     *
     * @dev If zero address is returned no callback is expected. If address(type(uint160).max)) is
     *      returned, then callbacks from any address can be accepted (dangerous).
     *
     * @return callbackSource The address that might do a callback.
     */
    function getCallbackSource(Swap memory swap) external view returns (address callbackSource);

    /**
     * @notice Swap tokens based on the swap argument.
     *
     * @param swap The details of the swap.
     */
    function swap(Swap memory swap) external;
}
