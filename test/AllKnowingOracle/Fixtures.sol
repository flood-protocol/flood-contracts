// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "../utils/BaseFixture.sol";
import "../utils/TokenFixture.sol";
import "src/AllKnowingOracle.sol";

interface IAllKnowingOracleEvents {
    event TokenWhitelisted(address indexed token, bool enabled);
    event SettlerWhitelisted(address indexed settler, bool enabled);
    event NewRequest(
        bytes32 indexed id,
        address indexed proposer,
        address indexed disputer,
        address bondToken,
        uint256 stake,
        uint256 bond
    );
    event BondPctChanged(uint256 newPct);
    event RequestSettled(bytes32 id, bool answer);
}

contract OracleFixture is BaseFixture, TokenFixture {
    AllKnowingOracle internal oracle;

    function setUp() public virtual override {
        super.setUp();
        oracle = new AllKnowingOracle();
        vm.label(address(oracle), "AllKnowingOracle");
        vm.startPrank(alice);
        ERC20(USDC).approve(address(oracle), type(uint256).max);
        ERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(bob);
        ERC20(USDC).approve(address(oracle), type(uint256).max);
        ERC20(WETH).approve(address(oracle), type(uint256).max);
        vm.stopPrank();
        oracle.whitelistToken(USDC, true);
        oracle.whitelistToken(WETH, true);
        oracle.whitelistSettler(charlie, true);
    }
}
