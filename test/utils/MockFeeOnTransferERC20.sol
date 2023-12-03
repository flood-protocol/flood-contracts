// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

contract MockFeeOnTransferERC20 is ERC20("TestToken", "TEST") {
    function _update(address from, address to, uint256 amount) internal override {
        super._update(from, to, amount);

        if (amount > 0 && to != address(0)) _burn(to, 1);
    }
}
