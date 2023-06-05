// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/token/ERC20/IERC20.sol";

interface IWETH9 is IERC20 {
    /// @notice Deposit ether to get wrapped ether
    function deposit() external payable;

    /// @notice Withdraw wrapped ether to get ether
    function withdraw(uint256) external;
}
