// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

import {IERC20} from "@openzeppelin/interfaces/IERC20.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {AllKnowingOracle, FloodRegistry} from "src/AllKnowingOracle.sol";
import {BaseFixture} from "../utils/BaseFixture.sol";
import {TokenFixture} from "../utils/TokenFixture.sol";

contract OracleFixture is BaseFixture, TokenFixture {
    FloodRegistry internal registry;
    AllKnowingOracle internal oracle;

    function setUp() public virtual override {
        super.setUp();
        registry = new FloodRegistry(IWETH9(WETH));
        oracle = new AllKnowingOracle(registry);
        registry.setOracle(oracle);
        IERC20(USDC).approve(address(oracle), type(uint256).max);
        IERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.label(address(oracle), "AllKnowingOracle");
        vm.startPrank(alice);
        IERC20(USDC).approve(address(oracle), type(uint256).max);
        IERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(bob);
        IERC20(USDC).approve(address(oracle), type(uint256).max);
        IERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(charlie);
        IERC20(USDC).approve(address(oracle), type(uint256).max);
        IERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.stopPrank();
        oracle.whitelistSettler(charlie, true);
    }
}
