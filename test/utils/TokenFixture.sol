// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {ERC20} from "solmate/tokens/ERC20.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";

address constant ARBITRUM_USDC = 0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8;
address constant ARBITRUM_WETH = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
address constant ARBITRUM_DAI = 0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1;

contract MockToken is ERC20 {
    constructor(string memory name, string memory symbol, uint8 decimals) ERC20(name, symbol, decimals) {}
}

contract MockWeth is ERC20 {
    constructor() ERC20("Wrapped Ether", "WETH", 18) {}

    function deposit() external payable {
        _mint(msg.sender, msg.value);
    }

    function withdraw(uint256 amount) external {
        require(balanceOf[msg.sender] >= amount, "ERC20: burn amount exceeds balance");
        _burn(msg.sender, amount);
        payable(msg.sender).transfer(amount);
    }
}

contract TokenFixture is Test {
    IERC20 internal USDC = deployTokenIfEmpty(ARBITRUM_USDC, "USDC", "USDC", 6);
    IWETH9 internal WETH = deployWETHIfEmpty(ARBITRUM_WETH);
    IERC20 internal DAI = deployTokenIfEmpty(ARBITRUM_DAI, "DAI", "DAI", 18);

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
