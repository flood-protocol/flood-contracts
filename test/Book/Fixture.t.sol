// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {Book} from "src/Book.sol";
import {IWETH9} from "src/interfaces/IWETH9.sol";
import {FloodRegistry} from "src/FloodRegistry.sol";
import {TokenFixture} from "../utils/TokenFixture.sol";

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

    function isBasketSupported(IERC20[] memory basket, IERC20 tokenOut) external view {
        _isBasketSupported(basket, tokenOut);
    }

    function isRelayerAuthorized(address relayer) internal view {
        _isRelayerAuthorized(relayer);
    }

    function transferAndUnwrap(IERC20 token, address sender, address recipient, uint256 amount, bool unwrap) external {
        _transferAndUnwrap(token, sender, recipient, amount, unwrap);
    }
}

contract BookFixture is TokenFixture {
    FloodRegistry internal registry;
    BookExposeExternalFunctions internal book;
    uint256 internal testFeePct = 100;

    function setUp() public virtual {
        vm.label(address(DAI), "DAI");
        vm.label(address(USDC), "USDC");
        vm.label(address(WETH), "WETH");
        registry = new FloodRegistry(WETH);
        vm.label(address(registry), "FloodRegistry");
        book = new BookExposeExternalFunctions(registry, testFeePct);
        vm.label(address(book), "Book");
    }
}
