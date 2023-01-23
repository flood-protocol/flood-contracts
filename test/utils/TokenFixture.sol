// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";

address constant MAINNET_USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
address constant MAINNET_WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
address constant MAINNET_DAI = 0x6B175474E89094C44Da98b954EedeAC495271d0F;

contract MockToken is ERC20 {
    uint8 private mockDecimals;

    constructor(string memory _name, string memory _symbol, uint8 _decimals) ERC20(_name, _symbol) {
        mockDecimals = _decimals;
    }

    function decimals() public view virtual override returns (uint8) {
        return mockDecimals;
    }
}

contract MockWeth is MockToken, IWETH9, Test {
    constructor() MockToken("Wrapped Ether", "WETH", 18) {}

    function deposit() external payable {
        _mint(msg.sender, msg.value);
    }

    function withdraw(uint256 amount) external {
        deal(address(this), amount);
        _burn(msg.sender, amount);
        payable(msg.sender).transfer(amount);
    }
}

contract TokenFixture is Test {
    address internal USDC = deployERC20IfNotEmpty(MAINNET_USDC, "USDC", "USDC", 6);
    address internal WETH = deployWethIfNotEmpty(MAINNET_WETH);
    address internal DAI = deployERC20IfNotEmpty(MAINNET_DAI, "DAI", "DAI", 18);

    // Deploys a contract to `target` if the address has no existing code.
    // This is used to deploy contracts in tests ONLY if you're not forking.
    function deployERC20IfNotEmpty(address target, string memory name, string memory symbol, uint8 decimals)
        internal
        returns (address)
    {
        uint256 existingCode = target.code.length;
        if (existingCode > 0) {
            return target;
        }
        ERC20 deployed = new MockToken(name, symbol, decimals);
        return address(deployed);
    }

    // Deploys a contract to `target` if the address has no existing code.
    // This is used to deploy contracts in tests ONLY if you're not forking.
    function deployWethIfNotEmpty(address target) internal returns (address) {
        uint256 existingCode = target.code.length;
        if (existingCode > 0) {
            return target;
        }
        IWETH9 deployed = new MockWeth();
        return address(deployed);
    }
}
