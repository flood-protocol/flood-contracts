// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "../utils/BaseFixture.sol";
import "../utils/TokenFixture.sol";
import "src/AllKnowingOracle.sol";

contract OracleFixture is BaseFixture, TokenFixture {
    FloodRegistry internal registry;
    AllKnowingOracle internal oracle;

    function setUp() public virtual override {
        super.setUp();
        registry = new FloodRegistry();
        oracle = new AllKnowingOracle(registry);
        registry.setOracle(oracle);
        ERC20(USDC).approve(address(oracle), type(uint256).max);
        ERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.label(address(oracle), "AllKnowingOracle");
        vm.startPrank(alice);
        ERC20(USDC).approve(address(oracle), type(uint256).max);
        ERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(bob);
        ERC20(USDC).approve(address(oracle), type(uint256).max);
        ERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(charlie);
        ERC20(USDC).approve(address(oracle), type(uint256).max);
        ERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.stopPrank();
        oracle.whitelistSettler(charlie, true);
    }
}
