// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {Book} from "src/Book.sol";
import {IBookEvents} from "src/interfaces/IBook.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {TokenFixture} from "../utils/TokenFixture.sol";
import {BaseFixture} from "../utils/BaseFixture.sol";

/// Export internal functions for testing
contract BookExposeExternalFunctions is Book {
    constructor(FloodRegistry registry, uint256 feePct) Book(registry, feePct) {}

    function getTradeId(
        IERC20[] memory tokens,
        uint128[] memory amounts,
        address recipient,
        uint256 tradeIndex,
        address trader
    ) external pure returns (bytes32) {
        return _getTradeId(tokens, amounts, recipient, tradeIndex, trader);
    }

    function deleteTrade(bytes32 tradeId) external {
        _deleteTrade(tradeId);
    }

    function isBasketSupported(IERC20[] memory basket) external view {
        _isBasketSupported(basket);
    }

    function isRelayerAuthorized(address relayer) external view {
        _isRelayerAuthorized(relayer);
    }

    function universalTransfer(IERC20 token, address recipient, uint256 amount) external {
        _universalTransfer(token, recipient, amount);
    }
}

contract BookFixture is TokenFixture, BaseFixture {
    FloodRegistry internal registry;
    BookExposeExternalFunctions internal book;
    uint256 internal testFeePct = 100;

    function setUp() public virtual override {
        super.setUp();
        vm.label(address(DAI), "DAI");
        vm.label(address(USDC), "USDC");
        vm.label(address(WETH), "WETH");
        // Give some ETH to the WETH contract
        deal(address(WETH), 100000 ether);
        registry = new FloodRegistry(WETH);
        vm.label(address(registry), "FloodRegistry");
        book = new BookExposeExternalFunctions(registry, testFeePct);
        vm.label(address(book), "Book");
    }
}

contract TradeFixture is BookFixture {
    address internal testTrader = alice;
    address internal testRecipient = charlie;
    uint128[] internal testAmounts;
    IERC20[] internal testBasket;
    uint256 internal testTradeIndex = 0;
    bytes32 internal testTradeId;
    bool internal testUnwrapOutput = false;
    uint256[] internal balancesBefore;

    function setUp() public virtual override {
        super.setUp();

        testBasket = new IERC20[](2);
        testBasket[0] = DAI;
        testBasket[1] = USDC;

        testAmounts = new uint128[](2);
        testAmounts[0] = 100;
        testAmounts[1] = 200;

        bool[] memory whitelist = new bool[](2);
        whitelist[0] = true;
        whitelist[1] = true;

        registry.batchWhitelistTokens(testBasket, whitelist);

        // Alice is going to send a trade to the book
        deal(address(testBasket[0]), alice, testAmounts[0]);
        deal(address(testBasket[1]), alice, testAmounts[1]);

        balancesBefore = new uint256[](testBasket.length - 1);
        // We don't check the last one as its the tokenOut
        for (uint256 i = 0; i < testBasket.length - 1; i++) {
            balancesBefore[i] = testBasket[i].balanceOf(testTrader);
        }
        // Alice is going to pre-approve the book to spend her tokens
        vm.startPrank(testTrader);
        DAI.approve(address(book), type(uint256).max);
        USDC.approve(address(book), type(uint256).max);
        WETH.approve(address(book), type(uint256).max);
        book.requestTrade(testBasket, testAmounts, testRecipient, testUnwrapOutput);
        vm.stopPrank();

        testTradeId = book.getTradeId(testBasket, testAmounts, testRecipient, testTradeIndex, testTrader);
    }
}
