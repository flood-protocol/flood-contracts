// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

import {IWETH9} from "./IWETH9.sol";

address constant ARBITRUM_USDC = 0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8;
address constant ARBITRUM_WETH = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
address constant ARBITRUM_DAI = 0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1;
address constant ARBITRUM_USDT = 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9;

contract MockToken is ERC20 {
    uint8 internal immutable innerDecimals;

    constructor(string memory name, string memory symbol, uint8 _decimals) ERC20(name, symbol) {
        innerDecimals = _decimals;
    }

    function decimals() public view override returns (uint8) {
        return innerDecimals;
    }
}

contract MockWeth is ERC20 {
    constructor() ERC20("Wrapped Ether", "WETH") {}

    function deposit() public payable {
        require(msg.value > 0, "Deposit amount must be greater than 0");
        _mint(msg.sender, msg.value);
    }

    function withdraw(uint256 amount) public {
        require(balanceOf(msg.sender) >= amount, "ERC20: burn amount exceeds balance");
        _burn(msg.sender, amount);

        (bool _s,) = msg.sender.call{value: amount}("");
        require(_s, "WETH: failed to send ether");
    }

    receive() external payable {
        deposit();
    }
}

contract TokenFixture is Test {
    IERC20 internal USDC = deployTokenIfEmpty(ARBITRUM_USDC, "USDC", "USDC", 6);
    IWETH9 internal WETH = deployWETHIfEmpty(ARBITRUM_WETH);
    IERC20 internal DAI = deployTokenIfEmpty(ARBITRUM_DAI, "DAI", "DAI", 18);
    IERC20 internal USDT = deployTokenIfEmpty(ARBITRUM_USDT, "USDT", "USDT", 6);

    function deployWETHIfEmpty(address target) internal returns (IWETH9) {
        uint256 existingCode = target.code.length;
        if (existingCode > 0) {
            return IWETH9(target);
        }
        MockWeth deployed = new MockWeth();

        return IWETH9(address(deployed));
    }

    // Deploys a contract to `target` if the address has no existing code.
    // This is used to deploy contracts in tests ONLY if you're not forking.
    function deployTokenIfEmpty(address target, string memory name, string memory symbol, uint8 decimals)
        internal
        returns (IERC20)
    {
        uint256 existingCode = target.code.length;
        if (existingCode > 0) {
            return IERC20(target);
        }
        MockToken deployed = new MockToken(name, symbol, decimals);

        return IERC20(address(deployed));
    }
}
