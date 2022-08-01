// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "../utils/BaseFixture.sol";
import "../utils/TokenFixture.sol";
import "src/AllKnowingOracle.sol";

contract OracleFixture is BaseFixture, TokenFixture {
    AllKnowingOracle internal oracle;

    function setUp() public virtual override {
        super.setUp();
        oracle = new AllKnowingOracle();
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
        oracle.whitelistToken(USDC, true);
        oracle.whitelistToken(WETH, true);
        oracle.whitelistSettler(charlie, true);
        oracle.whitelistRequester(charlie, true);
        oracle.whitelistRequester(address(this), true);
    }
}
