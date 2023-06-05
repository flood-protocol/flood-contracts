// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";
import {Pausable} from "@openzeppelin/security/Pausable.sol";
import {IWETH9} from "./interfaces/IWETH9.sol";
import {IFloodFillCallback, IFloodRecipient} from "./interfaces/ICallbacks.sol";
import {IBook} from "./interfaces/IBook.sol";
import {FloodRegistry} from "./FloodRegistry.sol";

error Book__ZeroRegistry();
error Book__InvalidToken(address token);
error Book__NotWeth();
error Book__InvalidBasket();
error Book__ZeroAmount();
error Book__InvalidValue();
error Book__TradeNotFillable();
error Book__TradeNotCancelable();
error Book__AmountOutTooLow();
error Book_UnauthorizedRelayer(address relayer);
error Book__NotTrader();

// The trade is not initialized, meaning it does not exist or was already settled/disputed
enum TradeStatus {
    UNINITIALIZED,
    // The trade is initialized and can be filled
    REQUESTED,
    // The trade is filled but not settled/disputed
    FILLED
}

/**
 * @notice A struct representing data associated with a trade. It is used to store the data of a trade in a mapping.
 *     @param status The status of the trade.
 *     @param unwrapOutput Whether the recipient wants to unwrap their output token.
 *     @param wrapInput Whether the trade was requested with ETH.
 */
struct TradeData {
    TradeStatus status;
    bool unwrapOutput;
    bool wrapInput;
}

contract Book is IBook, Pausable {
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
    function requestTrade(IERC20[] calldata tokens, uint128[] calldata amounts, address recipient, bool unwrapOutput)
        external
        payable
    {
        if (tokens.length != amounts.length) {
            revert Book__InvalidBasket();
        }
        if (tokens.length < 2) {
            revert Book__InvalidBasket();
        }
        // Check if the basket is supported.
        _isBasketSupported(tokens);

        // If no recipient was specified, the trade is sent to the sender.
        address realRecipient = recipient == address(0) ? msg.sender : recipient;

        // If `unwrapOutput` is true, the last token must be WETH.
        if (unwrapOutput && tokens[tokens.length - 1] != weth) {
            revert Book__NotWeth();
        }

        for (uint256 i = 0; i < amounts.length; i++) {
            // Check that the amount is not zero.
            if (amounts[i] == 0) {
                revert Book__ZeroAmount();
            }
            // We check if the user is sending ETH by first checking if they specified address 0.
            // If they did, we check that the amount is equal to the value sent. Note that we already checked that the amount is not zero.
            if (address(tokens[i]) == address(0) && msg.value != amounts[i]) {
                revert Book__InvalidValue();
            }
        }

        emit TradeRequested(tokens, amounts, realRecipient, numberOfTrades, msg.sender, unwrapOutput, msg.value > 0);
        bytes32 tradeId = _getTradeId(tokens, amounts, realRecipient, numberOfTrades, msg.sender);
        tradesData[tradeId] = TradeData(TradeStatus.REQUESTED, unwrapOutput, msg.value > 0);
        numberOfTrades++;

        // Transfer the tokens to the contract. We stop at `amounts.length - 1` because the last token is the one that the user wants to receive and the last amount is the minimum amount they want to receive.
        for (uint256 i = 0; i < amounts.length - 1; i++) {
            if (address(tokens[i]) == address(0)) {
                // If the token is ETH, we wrap it. Note that we already checked that msg.value is equal to the amount.
                weth.deposit{value: amounts[i]}();
            } else {
                // Otherwise, we transfer the token.
                tokens[i].safeTransferFrom(msg.sender, address(this), amounts[i]);
            }
        }
    }

    /**
     * @inheritdoc IBook
     */
    function cancelTrade(IERC20[] calldata tokens, uint128[] calldata amounts, address recipient, uint256 tradeIndex)
        external
    {
        bytes32 tradeId = _getTradeId(tokens, amounts, recipient, tradeIndex, msg.sender);

        TradeData memory tradeData = tradesData[tradeId];
        if (tradeData.status != TradeStatus.REQUESTED) {
            revert Book__TradeNotCancelable();
        }
        _deleteTrade(tradeId);
        emit TradeCancelled(tradeIndex, tradeId, msg.sender);

        // Refund the tokens to the trader.
        for (uint256 i = 0; i < tokens.length - 1; i++) {
            if (address(tokens[i]) == address(0)) {} else {
                _universalTransfer(tokens[i], msg.sender, amounts[i]);
            }
        }
    }

    /**
     * @inheritdoc IBook
     */
    function fillTrade(FillTradeArgs calldata trade) external {
        _isRelayerAuthorized(msg.sender);
        TradeData memory tradeData;
        {
            bytes32 id = _getTradeId(trade.tokens, trade.amounts, trade.recipient, trade.tradeIndex, trade.trader);
            tradeData = tradesData[id];
            if (tradeData.status != TradeStatus.REQUESTED) {
                revert Book__TradeNotFillable();
            }
            tradeData.status = TradeStatus.FILLED;
            tradesData[id] = tradeData;
        }

        // Transfer tokens to the relayer. We stop at `amounts.length - 1` because the last token is tokenOut.
        for (uint256 i = 0; i < trade.tokens.length - 1; i++) {
            if (address(trade.tokens[i]) == address(0)) {
                // If the token was ETH, we wrapped it and send WETH.
                _universalTransfer(weth, msg.sender, trade.amounts[i]);
            } else {
                // Otherwise, we transfer the token.
                _universalTransfer(trade.tokens[i], msg.sender, trade.amounts[i]);
            }
        }

        uint128 realAmountOut = trade.amountOut;

        if (trade.callbackData.length > 0) {
            uint128 amountFromCallback = IFloodFillCallback(msg.sender).onFloodFill(trade.callbackData);

            if (amountFromCallback > trade.amountOut) {
                realAmountOut = uint128(amountFromCallback);
            }
        }

        if (trade.amounts[trade.amounts.length - 1] > realAmountOut) {
            revert Book__AmountOutTooLow();
        }

        emit TradeFilled(msg.sender, trade.tradeIndex, realAmountOut, trade.trader);

        // Send the tokens to the recipient.
        if (tradeData.unwrapOutput) {
            trade.tokens[trade.tokens.length - 1].safeTransferFrom(msg.sender, address(this), realAmountOut);
            weth.withdraw(realAmountOut);
            (bool s,) = trade.recipient.call{value: realAmountOut}("");
            require(s, "Book: failed to send ETH");
        } else {
            trade.tokens[trade.tokens.length - 1].safeTransferFrom(msg.sender, trade.recipient, realAmountOut);
        }

        // If the recipient is a contract, we call the callback on it.
        if (trade.recipient.code.length > 0) {
            IFloodRecipient(trade.recipient).onTradeFilled(
                trade.trader,
                trade.tokens,
                trade.amounts,
                realAmountOut,
                _getTradeId(trade.tokens, trade.amounts, trade.recipient, trade.tradeIndex, trade.trader)
            );
        }
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
     */
    function _isBasketSupported(IERC20[] memory basket) internal view {
        if (!registry.areTokensWhitelisted(basket)) {
            revert Book__InvalidBasket();
        }
        // Check that the basket does not contain duplicates.
        for (uint256 i = 0; i < basket.length; i++) {
            for (uint256 j = i + 1; j < basket.length; j++) {
                if (basket[i] == basket[j]) {
                    revert Book__InvalidBasket();
                }
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
     * @notice Transfers `amount` of `token` to `recipient`. Supports ETH and WETH.
     * @param token The token to transfer. If it is address 0, ETH is transferred.
     * @param recipient The address to transfer the tokens to.
     * @param amount The amount of tokens to transfer.
     */
    function _universalTransfer(IERC20 token, address recipient, uint256 amount) internal {
        if (address(token) == address(0)) {
            weth.withdraw(amount);
            (bool success,) = recipient.call{value: amount}("");
            require(success, "Book: ETH transfer failed");
        } else {
            IERC20(token).safeTransfer(recipient, amount);
        }
    }

    /**
     * @notice Standard function to receive ETH.
     */
    receive() external payable {}
}
