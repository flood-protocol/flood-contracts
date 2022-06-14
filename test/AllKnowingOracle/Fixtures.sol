// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "../utils/BaseFixture.sol";
import "../utils/TokenFixture.sol";
import "src/AllKnowingOracle.sol";

interface IAllKnowingOracleEvents {
    event TokenWhitelisted(address indexed token, bool enabled);
    event NewRequest(
        bytes32 indexed id,
        address indexed proposer,
        address indexed disputer,
        address bondToken,
        uint256 bond,
        uint256 stake
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
    }

    function _settle(bytes32 id, bool answer) internal {
        oracle.settle(id, answer);
    }

    function _ask(
        address proposer,
        address disputer,
        address bondToken,
        uint256 bond,
        uint256 stake
    ) internal returns (bytes32) {
        return oracle.ask(proposer, disputer, bondToken, bond, stake);
    }
}
