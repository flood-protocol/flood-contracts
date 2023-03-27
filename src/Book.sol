// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";
import {Pausable} from "@openzeppelin/security/Pausable.sol";
import {IWETH9} from "./interfaces/IWETH9.sol";
import {IFloodFillCallback} from "./interfaces/IFloodFillCallback.sol";
import {IBook} from "./interfaces/IBook.sol";
import {FloodRegistry} from "./FloodRegistry.sol";

error Book__ZeroRegistry();
error Book__InvalidToken(address token);
error Book__InvalidBasket();
error Book__SameToken();
error Book__ZeroAmount();
error Book__InvalidValue();
// The trade is not filled yet, doesn't exist or was disputed
error Book__TradeNotInFillableState(bytes32 tradeId);
error Book__TradeNotFilled(bytes32 tradeId);
error Book__TradeNotCancelable(bytes32 tradeId);
error Book__AmountOutTooLow();
error Book_UnauthorizedRelayer(address relayer);
error Book__NotTrader();

// The trade is not initialized, meaning it does not exist or was already settled/disputed
enum TradeStatus {
    UNINITIALIZED,
    // The trade is initialized and can be filled
    REQUESTED,
    // The trade is filled but not settled/disputed
    FILLED,
    // The trade has been disputed
    DISPUTED
}

/**
 * @notice A struct representing data associated with a trade. It is used to store the data of a trade in a mapping.
 *     @param status The status of the trade.
 *     @param unwrapOutput Whether the recipient wants to unwrap their output token.
 *     @param wrapInput Whether the trade was requested with ETH.
 *     @param amountPaid The amount of tokenIn paid by far by the contract to the relayer filling the trade, disputers or given as a rebate to users. This should never exceed the `amountIn` of the trade.
 */
struct TradeData {
    TradeStatus status;
    uint256 amountPaid;
}

abstract contract Book is IBook, ReentrancyGuard, Pausable {
    using SafeERC20 for IERC20;

    // The fee taken on each trade by relayers, expressed in basis points (so 100% = 10000). Not used in the code, but useful to coordinate off-chain.
    uint256 public immutable feePct;
    FloodRegistry public immutable registry;
    IWETH9 private immutable weth;

    uint256 public numberOfTrades = 0;
    // A mapping from trade ids to trade data.
    mapping(bytes32 => TradeData) public tradesData;

    /////////////////////////////////
    //    CONSTRUCTOR             //
    ///////////////////////////////
    constructor(FloodRegistry _registry, uint256 _feePct) {
        if (address(_registry) == address(0) || address(_registry).code.length == 0) {
            revert Book__ZeroRegistry();
        }
        registry = _registry;
        weth = _registry.WETH();
        feePct = _feePct;
        emit FeePctSet(_feePct);
    }

    /////////////////////////////////
    //    EXTERNAL FUNCTIONS      //
    ///////////////////////////////

    /**
     * @inheritdoc IBook
     */
    function requestTrade(bytes calldata tokens, bytes calldata amounts, address recipient) external {}

    /**
     * @inheritdoc IBook
     */
    function cancelTrade(bytes calldata tokens, bytes calldata amounts, address recipient, uint256 tradeIndex)
        external
    {}

    /**
     * @inheritdoc IBook
     */
    function fillTrade(
        bytes calldata tokens,
        bytes calldata amounts,
        address recipient,
        uint256 tradeIndex,
        address trader,
        uint128 amountToSend,
        bytes calldata data
    ) external nonReentrant {
        _isRelayerAuthorized(msg.sender);
    }

    /////////////////////////////////
    //    INTERNAL FUNCTIONS      //
    ///////////////////////////////

    /**
     * @notice Removes a trade with id `tradeId` from storage.
     * @param tradeId The token that was sold.
     */
    function _deleteTrade(bytes32 tradeId) internal {
        delete tradesData[tradeId];
    }

    /**
     * @notice Checks whether a basket of tokens is supported. Reverts if not.
     * @param basket The basket of tokens.
     * @param tokenOut The token to be bought.
     */
    function _isBasketSupported(IERC20[] memory basket, IERC20 tokenOut) internal view {
        if (!registry.isTokenWhitelisted(tokenOut)) {
            revert Book__InvalidToken(address(tokenOut));
        }
        if (!registry.areTokensWhitelisted(basket)) {
            revert Book__InvalidBasket();
        }

        for (uint256 i = 0; i < basket.length; i++) {
            if (basket[i] == tokenOut) {
                revert Book__SameToken();
            }
        }
    }

    /**
     * @notice Checks whether a relayer is authorized to fill trades. Reverts if not.
     * @param relayer The address of the relayer.
     */
    function _isRelayerAuthorized(address relayer) internal view {
        if (!registry.isRelayerWhitelisted(relayer)) {
            revert Book_UnauthorizedRelayer(relayer);
        }
    }

    /**
     * @notice Computes the trade ID for the given trade by hashing the trade parameters.
     * @param tokens The tokens to be traded. The last token is the token to be bought.
     * @param amounts The amount of each token to be traded. The last amount is the amount of `tokenOut` to be bought.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The number of trades preceding this one.
     * @param trader The address of the trader who initiated the trade.
     * @return The trade ID.
     */

    function _getTradeId(
        IERC20[] memory tokens,
        uint128[] memory amounts,
        address recipient,
        uint256 tradeIndex,
        address trader
    ) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(tokens, amounts, recipient, tradeIndex, trader));
    }

    /**
     * @notice Transfers `amount` of `token` from `sender` to `recipient`.
     * @param token The token to transfer.
     * @param sender The address to transfer the tokens from.
     * @param recipient The address to transfer the tokens to.
     * @param amount The amount of tokens to transfer.
     * @param unwrap Whether to unwrap the tokens if they are WETH.
     */
    function _transferAndUnwrap(IERC20 token, address sender, address recipient, uint256 amount, bool unwrap)
        internal
    {
        if (sender == address(this)) {
            if (unwrap && address(token) == address(weth)) {
                IWETH9(address(token)).withdraw(amount);
                (bool success,) = recipient.call{value: amount}("");
                require(success, "Book: ETH transfer failed");
            } else {
                IERC20(token).safeTransfer(recipient, amount);
            }
            return;
        }
        if (unwrap && address(token) == address(weth)) {
            token.safeTransferFrom(sender, address(this), amount);
            IWETH9(address(token)).withdraw(amount);
            (bool success,) = recipient.call{value: amount}("");
            require(success, "Book: ETH transfer failed");
        } else {
            IERC20(token).safeTransferFrom(sender, recipient, amount);
        }
    }

    /**
     * @notice decodes the tokens and amounts from the encoded bytes.
     * @param encodedTokens The abi encoded packed bytes
     * @return tokens The tokens.
     */
    function _decodeTokens(bytes memory encodedTokens) internal pure returns (address[] memory tokens) {
        // This should NEVER round, as the encodedTokens length is always a multiple of 20.
        uint256 length = encodedTokens.length / 20;
        tokens = new address[](length);

        for (uint256 i = 0; i < length; i++) {
            address token;
            assembly {
                token := mload(add(encodedTokens, mul(20, i)))
            }
            tokens[i] = token;
        }
    }
}
