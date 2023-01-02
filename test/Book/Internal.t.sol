// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "@openzeppelin/token/ERC20/IERC20.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {Book} from "src/Book.sol";
import {BaseBookFixture} from "./Fixtures.sol";

contract BookWrapper is Book {
    constructor(
        FloodRegistry registry_,
        uint256 safeBlockThreashold_,
        uint256 disputeBondPct_,
        uint256 tradeRebatePct_,
        uint256 relayerRefundPct_,
        uint256 feePct_
    ) Book(registry_, safeBlockThreashold_, disputeBondPct_, tradeRebatePct_, relayerRefundPct_, feePct_) {}

    function transferAndUnwrap(IERC20 token, address sender, address recipient, uint256 amount, bool unwrap) external {
        _transferAndUnwrap(token, sender, recipient, amount, unwrap);
    }

    function isPairSupported(address tokenA, address tokenB) external view {
        _isPairSupported(tokenA, tokenB);
    }
}

contract BookInternalTest is BaseBookFixture {
    BookWrapper internal wrapper;

    function setUp() public override {
        super.setUp();
        wrapper = new BookWrapper(
            registry,
            testSafeBlockThreashold,
            testDisputeBondPct,
            testTradeRebatePct,
            testRelayerRefundPct,
            testFeePct
        );
        vm.label(address(wrapper), "BookWrapper");

        // approve the wrapper to spend the tokens
        vm.startPrank(alice);
        IERC20(USDC).approve(address(wrapper), type(uint256).max);
        IERC20(WETH).approve(address(wrapper), type(uint256).max);
        vm.stopPrank();
    }

    function testTransfer(uint256 amount, bool unwrap) public {
        deal(USDC, alice, amount);
        wrapper.transferAndUnwrap(IERC20(USDC), alice, bob, amount, unwrap);

        assertEq(IERC20(USDC).balanceOf(alice), 0);
        assertEq(IERC20(USDC).balanceOf(bob), amount);
    }

    function testTransferAndUnwrap() public {
        uint256 amount = 1 ether;
        bool unwrap = true;
        deal(WETH, alice, amount);
        wrapper.transferAndUnwrap(IERC20(WETH), alice, bob, amount, unwrap);

        assertEq(IERC20(WETH).balanceOf(alice), 0);
        if (unwrap) {
            assertEq(IERC20(WETH).balanceOf(bob), 0);
            assertEq(bob.balance, amount);
        } else {
            assertEq(IERC20(WETH).balanceOf(bob), amount);
            assertEq(bob.balance, 0);
        }
    }
}
