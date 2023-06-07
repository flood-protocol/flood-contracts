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

    function swap(Swap memory swap) external;
}
