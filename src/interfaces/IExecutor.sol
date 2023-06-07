// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IExecutor {
    function swap(
        address pool,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn,
        uint256 amountOut
    ) external;
}
