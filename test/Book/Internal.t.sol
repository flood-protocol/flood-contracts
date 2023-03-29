// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {Book, Book_UnauthorizedRelayer, TradeData, TradeStatus, Book__InvalidBasket} from "src/Book.sol";
import {IBookEvents} from "src/interfaces/IBook.sol";
import {BookFixture} from "./Fixture.t.sol";

contract InternalTest is BookFixture, IBookEvents {
    using stdStorage for StdStorage;

    function setUp() public virtual override {
        super.setUp();
    }

    function testConstructor() public {
        vm.expectEmit();
        emit FeePctSet(testFeePct);

        Book _book = new Book(registry, testFeePct);
        assertEq(address(_book.registry()), address(registry));
        assertEq(_book.feePct(), testFeePct);
    }

    function testGetTradeId(
        address[] memory tokens,
        uint128[] memory amounts,
        address recipient,
        uint256 tradeIndex,
        address trader
    ) public {
        IERC20[] memory tokens_ = new IERC20[](tokens.length);
        for (uint256 i = 0; i < tokens.length; i++) {
            tokens_[i] = IERC20(tokens[i]);
        }
        bytes32 tradeId = book.getTradeId(tokens_, amounts, recipient, tradeIndex, trader);

        assertEq(tradeId, keccak256(abi.encodePacked(tokens, amounts, recipient, tradeIndex, trader)));
    }

    function testIsRelayerAuthorized() public {
        address relayer = address(1);
        registry.whitelistRelayer(relayer, true);
        book.isRelayerAuthorized(relayer);
        address notRelayer = address(2);
        vm.expectRevert(abi.encodeWithSelector(Book_UnauthorizedRelayer.selector, notRelayer));
        book.isRelayerAuthorized(notRelayer);
    }

    function testDeleteTrade() public {
        bytes32 tradeId = bytes32(uint256(1));
        // Fill the storage slot with a random value
        stdstore.target(address(book)).sig(book.tradesData.selector).with_key(tradeId).depth(0).checked_write(2);
        (TradeStatus s,,) = book.tradesData(tradeId);
        assertEq(uint256(s), 2);
        book.deleteTrade(tradeId);
        (TradeStatus sa,,) = book.tradesData(tradeId);
        assertEq(uint256(sa), 0);
    }

    function testIsBasketSupported() public {
        IERC20[] memory basket = new IERC20[](3);
        basket[0] = DAI;
        basket[1] = USDC;
        basket[2] = USDT;

        registry.whitelistToken(DAI, true);
        registry.whitelistToken(USDC, true);
        registry.whitelistToken(USDT, true);
        book.isBasketSupported(basket);
    }

    function testCannotSupportBasketWithSameToken() public {
        IERC20[] memory basket = new IERC20[](2);
        basket[0] = DAI;
        basket[1] = DAI;

        registry.whitelistToken(DAI, true);
        vm.expectRevert((Book__InvalidBasket.selector));
        book.isBasketSupported(basket);
    }

    function testCannotSupportBasketWithUnsupportedToken() public {
        IERC20[] memory basket = new IERC20[](2);
        basket[0] = DAI;
        basket[1] = USDC;

        registry.whitelistToken(DAI, true);
        vm.expectRevert((Book__InvalidBasket.selector));
        book.isBasketSupported(basket);
    }

    function testUniversalTransfer() public {
        deal(address(DAI), address(book), 100);
        book.universalTransfer(DAI, address(1), 100);
        assertEq(DAI.balanceOf(address(1)), 100);

        deal(address(WETH), address(book), 1 ether);
        book.universalTransfer(IERC20(address(0)), address(3), 1 ether);
        assertEq(address(3).balance, 1 ether);
    }
}
