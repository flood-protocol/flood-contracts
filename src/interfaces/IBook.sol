// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

interface IBookEvents {
    event FeePctSet(uint256 feePct);
    event TradeRequested(
        IERC20[] tokens,
        uint128[] amounts,
        address recipient,
        uint256 indexed tradeIndex,
        address indexed trader,
        bool unwrapOutput,
        bool wrapInput
    );
    event TradeFilled(address indexed relayer, uint256 indexed tradeIndex, uint128 amountOut, address indexed trader);
    event TradeSettled(
        address indexed relayer, uint256 indexed tradeIndex, uint256 filledAtBlock, address indexed trader
    );

    event TradeCancelled(uint256 indexed tradeIndex, bytes32 indexed tradeId, address indexed trader);
}

interface IBook is IBookEvents {
    /**
     * @notice Requests to trade `amounts` of `tokens` for `tokenOut` with `feePct` fee off the optimal quote at execution time. Users deposit `tokenIn` to the contract.
     * This contract performs no safety checks on `minAmountOut` and the `recipient`, it is up to the caller to specify these correctly.
     * @param tokens An packed array of token addresses to be traded. The last token is the token to be bought.
     * @param amounts An array of amounts of `tokens` to be traded. The last amount is the minimum amount of the last token to be bought.
     * @param recipient The address to receive the tokens bought. If 0, will be set to the caller.
     * @param unwrapOutput Whether to unwrap the output tokens into ETH. If true, token out must be WETH.
     */

    function requestTrade(IERC20[] calldata tokens, uint128[] calldata amounts, address recipient, bool unwrapOutput)
        external
        payable;

    /**
     * @param tokens The tokens involved in the trade. The last token is the token to be bought.
     * @param amounts The amounts of the tokens involved in the trade. The last amount is the amount to be bought. *
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade.
     * @param trader The address of the trader who initiated the trade.
     * @param amountToSend The amount of `tokenOut` given by the relayer to the trader.
     * @param data The data to be passed to the calling contract in the `onFloodFill` callback.
     */
    struct FillTradeArgs {
        IERC20[] tokens;
        uint128[] amounts;
        address recipient;
        uint256 tradeIndex;
        address trader;
        uint128 amountOut;
        bytes callbackData;
    }

    /**
     * @notice called by relayers for filling a trade.
     * All trade are accepted at face value, so no checks are performed. However, invalid trades can be disputed.
     * @dev Relayers are expected to correctly compute the Optimal Execution Price - feePct off-chain.
     * Failing to do so will result in a dispute and the relayer losing tokens.
     * @param trade The trade to fill. See `FillTradeArgs` for details.
     */
    function fillTrade(FillTradeArgs calldata trade) external;

    /**
     * @notice Cancel a previously requested trade.
     * @param tokens An packed array of token addresses to be traded. The last token is the token to be bought.
     * @param amounts A packed array of amounts (as int256) of `tokens` to be traded. The last amount is the minimum amount of the last token to be bought.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade.
     */
    function cancelTrade(IERC20[] calldata tokens, uint128[] calldata amounts, address recipient, uint256 tradeIndex)
        external;
}
