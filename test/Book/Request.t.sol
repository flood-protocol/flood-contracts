// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {IBookEvents} from "src/interfaces/IBook.sol";
import {
    TradeData,
    TradeStatus,
    Book__InvalidBasket,
    Book__ZeroAmount,
    Book__InvalidValue,
    Book__NotWeth
} from "src/Book.sol";
import {BookFixture} from "./Fixture.t.sol";

contract RequestTest is BookFixture, IBookEvents {
    function setUp() public override {
        super.setUp();
        IERC20[] memory tokens = new IERC20[](2);
        tokens[0] = DAI;
        tokens[1] = USDC;

        bool[] memory whitelist = new bool[](2);
        whitelist[0] = true;
        whitelist[1] = true;

        registry.batchWhitelistTokens(tokens, whitelist);

        // Alice is going to pre-approve the book to spend her tokens
        vm.startPrank(alice);
        DAI.approve(address(book), type(uint256).max);
        USDC.approve(address(book), type(uint256).max);
        WETH.approve(address(book), type(uint256).max);
        vm.stopPrank();
    }

    function testRequestTrade() public {
        IERC20[] memory basket = new IERC20[](3);
        basket[0] = DAI;
        basket[1] = USDC;
        basket[2] = WETH;

        uint128[] memory amounts = new uint128[](3);
        amounts[0] = 100;
        amounts[1] = 200;
        amounts[2] = 300;

        address recipient = charlie;

        deal(address(basket[0]), alice, 100);
        deal(address(basket[1]), alice, 200);
        deal(address(basket[2]), alice, 300);

        vm.prank(alice);
        vm.expectEmit(address(book));
        emit TradeRequested(basket, amounts, recipient, 0, alice, false, false);
        book.requestTrade(basket, amounts, recipient, false);

        bytes32 tradeId = book.getTradeId(basket, amounts, recipient, 0, alice);
        (TradeStatus status, bool unwrapOutput, bool wrapInput) = book.tradesData(tradeId);

        assertEq(book.numberOfTrades(), 1);
        assertEq(uint8(status), uint8(TradeStatus.REQUESTED));
        assertEq(unwrapOutput, false);
        assertEq(wrapInput, false);

        // Check that the book has the tokens
        for (uint256 i = 0; i < basket.length - 1; i++) {
            assertEq(basket[i].balanceOf(address(book)), amounts[i]);
        }
    }

    function testCannotTradeLessThan2Tokens() public {
        IERC20[] memory basket = new IERC20[](1);
        basket[0] = DAI;

        uint128[] memory amounts = new uint128[](1);
        amounts[0] = 100;

        deal(address(basket[0]), alice, 100);

        vm.prank(alice);
        vm.expectRevert(Book__InvalidBasket.selector);
        book.requestTrade(basket, amounts, address(0), false);
    }

    function testCannotTradeIfAmountsAndTokensHaveDiffLength() public {
        IERC20[] memory basket = new IERC20[](2);
        basket[0] = DAI;
        basket[1] = USDC;

        uint128[] memory amounts = new uint128[](1);
        amounts[0] = 100;

        deal(address(basket[0]), alice, 100);
        deal(address(basket[1]), alice, 200);

        vm.prank(alice);
        vm.expectRevert(Book__InvalidBasket.selector);
        book.requestTrade(basket, amounts, address(0), false);
    }

    function testCannotUnwrapOutputIfNotWETH() public {
        IERC20[] memory basket = new IERC20[](2);
        basket[0] = DAI;
        basket[1] = USDC;

        uint128[] memory amounts = new uint128[](2);
        amounts[0] = 100;
        amounts[1] = 300;

        deal(address(basket[0]), alice, 100);
        deal(address(basket[1]), alice, 200);

        vm.prank(alice);
        vm.expectRevert(Book__NotWeth.selector);
        book.requestTrade(basket, amounts, address(0), true);
    }

    function testCannotTradeSameTokens() public {
        IERC20[] memory basket = new IERC20[](3);
        basket[0] = DAI;
        basket[1] = DAI;
        basket[2] = WETH;

        uint128[] memory amounts = new uint128[](3);
        amounts[0] = 100;
        amounts[1] = 200;
        amounts[2] = 300;

        deal(address(basket[0]), alice, 100);
        deal(address(basket[1]), alice, 200);
        deal(address(basket[2]), alice, 300);

        vm.prank(alice);
        vm.expectRevert(Book__InvalidBasket.selector);
        book.requestTrade(basket, amounts, address(0), false);
    }

    function testCannotRequestTradeWithUnsupportedBasket() public {
        IERC20[] memory basket = new IERC20[](4);
        basket[0] = DAI;
        basket[1] = USDC;
        basket[2] = WETH;
        basket[3] = USDT;

        uint128[] memory amounts = new uint128[](4);
        amounts[0] = 100;
        amounts[1] = 200;
        amounts[2] = 300;
        amounts[3] = 400;

        vm.prank(alice);
        vm.expectRevert(Book__InvalidBasket.selector);
        book.requestTrade(basket, amounts, address(0), false);
    }

    function testCannotRequestTradeWithZeroAmount() public {
        IERC20[] memory basket = new IERC20[](3);
        basket[0] = DAI;
        basket[1] = USDC;
        basket[2] = WETH;

        uint128[] memory amounts = new uint128[](3);
        amounts[0] = 0;
        amounts[1] = 0;
        amounts[2] = 0;

        vm.prank(alice);
        vm.expectRevert(Book__ZeroAmount.selector);
        book.requestTrade(basket, amounts, address(0), false);
    }

    function testCanRequestTradeWithETH() public {
        IERC20[] memory basket = new IERC20[](3);
        basket[0] = DAI;
        basket[1] = IERC20(address(0));
        basket[2] = USDC;

        uint128[] memory amounts = new uint128[](3);
        amounts[0] = 100;
        amounts[1] = 200;
        amounts[2] = 300;

        deal(address(basket[0]), alice, 100);
        deal(alice, 200);

        vm.prank(alice);
        vm.expectEmit(address(book));
        emit TradeRequested(basket, amounts, alice, 0, alice, false, true);
        book.requestTrade{value: 200}(basket, amounts, address(0), false);

        bytes32 tradeId = book.getTradeId(basket, amounts, alice, 0, alice);

        (TradeStatus status, bool unwrapOutput, bool wrapInput) = book.tradesData(tradeId);

        assertEq(book.numberOfTrades(), 1);
        assertEq(uint8(status), uint8(TradeStatus.REQUESTED), "status");
        assertEq(unwrapOutput, false, "unwrapOutput");
        assertEq(wrapInput, true, "wrapInput");

        // Check that the book has the tokens
        for (uint256 i = 0; i < basket.length - 1; i++) {
            if (basket[i] == IERC20(address(0))) {
                assertEq(WETH.balanceOf(address(book)), amounts[i]);
            } else {
                assertEq(basket[i].balanceOf(address(book)), amounts[i]);
            }
        }
    }

    function testCannotRequestTradeWithETHIfAmountsMismatch() public {
        IERC20[] memory basket = new IERC20[](3);
        basket[0] = DAI;
        basket[1] = IERC20(address(0));
        basket[2] = USDC;

        uint128[] memory amounts = new uint128[](3);
        amounts[0] = 100;
        amounts[1] = 200;
        amounts[2] = 300;

        uint256 value = 400;

        deal(address(basket[0]), alice, amounts[0]);
        deal(alice, value);

        vm.prank(alice);
        vm.expectRevert(Book__InvalidValue.selector);
        book.requestTrade{value: value}(basket, amounts, address(0), false);
    }
}
