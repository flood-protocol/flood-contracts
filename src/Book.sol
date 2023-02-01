// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {IWETH9} from "./interfaces/IWETH9.sol";
import {IFloodFillCallback} from "./interfaces/IFloodFillCallback.sol";
import {AllKnowingOracle, Request, IOptimisticRequester} from "./AllKnowingOracle.sol";
import {FloodRegistry} from "./FloodRegistry.sol";

interface IBookEvents {
    event SafeBlockThresholdSet(uint256 newSafeBlockThreshold);
    event ParamsCombinationSet(uint256 disputeBondPct, uint256 tradeRebatePct, uint256 relayerRefundPct);
    event FeePctSet(uint256 feePct);
    event TradeRequested(
        address indexed tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 indexed tradeIndex,
        address indexed trader
    );
    event TradeFilled(address indexed relayer, uint256 indexed tradeIndex, uint256 amountOut, address indexed trader);
    event TradeSettled(
        address indexed relayer, uint256 indexed tradeIndex, uint256 filledAtBlock, address indexed trader
    );
    event TradeDisputed(
        address relayer,
        uint256 indexed tradeIndex,
        bytes32 indexed disputeId,
        uint256 filledAtBlock,
        address indexed trader
    );
    event TradeCancelled(uint256 indexed tradeIndex, bytes32 indexed tradeId, address indexed trader);
    event TradeDisputeSettled(
        address relayer, uint256 indexed tradeIndex, bytes32 indexed disputeId, bool answer, address indexed trader
    );
}

error Book__ZeroRegistry();
error Book__ZeroSafeBlockThreshold();
error Book__InvalidToken(address token);
error Book__SameToken();
error Book__ZeroAmount();
error Book__NotWeth();
error Book__InvalidValue();
// the recipient of a transfer was the 0 address
error Book__SentToBlackHole();
error Book__InvalidParamsCombination();
error Book__FeePctTooHigh();
// The trade is not filled yet, doesn't exist or was disputed
error Book__TradeNotInFillableState(bytes32 tradeId);
error Book__TradeNotFilled(bytes32 tradeId);
error Book__TradeNotCancelable(bytes32 tradeId);
error Book__AmountOutTooLow();
error Book__DisputePeriodNotOver(uint256 blocksLeft);
error Book__TradeNotDisputable();
error Book__MaliciousCaller(address caller);
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
 *     @param filledAtBlock The block the trade was filled at. A value of 0 means it has not been filled yet.
 *     @param filledBy The relayer that filled the trade.
 *     @param status The status of the trade.
 *     @param unwrapOutput Whether the recipient wants to unwrap their output token.
 *     @param isEthTrade Whether the trade was requested with ETH.
 *     @param amountPaid The amount of tokenIn paid by far by the contract to the relayer filling the trade, disputers or given as a rebate to users. This should never exceed the `amountIn` of the trade.
 */
struct TradeData {
    uint256 filledAtBlock;
    address filledBy;
    TradeStatus status;
    bool unwrapOutput;
    bool isEthTrade;
    uint256 amountPaid;
}

contract Book is IOptimisticRequester, IBookEvents {
    using SafeERC20 for IERC20;

    uint256 public immutable safeBlockThreshold;
    uint256 public immutable disputeBondPct;
    uint256 public immutable tradeRebatePct;
    uint256 public immutable relayerRefundPct;
    // The fee taken on each trade by relayers, expressed in basis points (so 100% = 10000). Not used in the code, but useful to coordinate off-chain.
    uint256 public immutable feePct;
    FloodRegistry public immutable registry;
    // The oracle to use for dispute resolution. At deployment, this is the latest oracle used in the protocol and it cannot be changed.
    AllKnowingOracle public immutable oracle;
    IWETH9 private immutable weth;

    uint256 public numberOfTrades = 0;
    // A mapping from trade ids to trade data.
    mapping(bytes32 => TradeData) public tradesData;

    constructor(
        FloodRegistry _registry,
        uint256 _safeBlockThreshold,
        uint256 _disputeBondPct,
        uint256 _tradeRebatePct,
        uint256 _relayerRefundPct,
        uint256 _feePct
    ) {
        if (address(_registry) == address(0)) {
            revert Book__ZeroRegistry();
        }
        if (_safeBlockThreshold == 0) {
            revert Book__ZeroSafeBlockThreshold();
        }
        registry = _registry;
        oracle = registry.latestOracle();
        weth = registry.WETH();
        safeBlockThreshold = _safeBlockThreshold;
        emit SafeBlockThresholdSet(safeBlockThreshold);

        _validateParams(_disputeBondPct, _tradeRebatePct, _relayerRefundPct);
        disputeBondPct = _disputeBondPct;
        tradeRebatePct = _tradeRebatePct;
        relayerRefundPct = _relayerRefundPct;

        emit ParamsCombinationSet(_disputeBondPct, _tradeRebatePct, _relayerRefundPct);

        if (_feePct > 2500) {
            revert Book__FeePctTooHigh();
        }

        feePct = _feePct;
        emit FeePctSet(_feePct);
    }

    /**
     * @notice Requests to trade `amountIn` of `tokenIn` for `tokenOut` with `feePct` fee off the optimal quote at execution time. Users deposit `tokenIn` to the contract.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought. This should be set offchain based on `tradeRebatePct`, for example, if `tradeRebatePct` is 20%, then `minAmountOut` could be 90% of optimal at the time of request.
     * @param recipient The address to receive the tokens bought.
     * @param receiveETH If true, the recipient will receive ETH instead of WETH.
     */
    function requestTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        bool receiveETH
    ) external payable {
        // checks whether the tokens are whitelisted
        _isPairSupported(tokenIn, tokenOut);

        if (tokenIn != address(weth) && msg.value > 0) {
            revert Book__NotWeth();
        }
        if (tokenIn == address(weth) && msg.value > 0 && msg.value != amountIn) {
            revert Book__InvalidValue();
        }
        if (amountIn == 0 || minAmountOut == 0) {
            revert Book__ZeroAmount();
        }
        if (recipient == address(0)) {
            revert Book__SentToBlackHole();
        }
        if (receiveETH && tokenOut != address(weth)) {
            revert Book__NotWeth();
        }

        emit TradeRequested(tokenIn, tokenOut, amountIn, minAmountOut, recipient, numberOfTrades, msg.sender);

        bytes32 tradeId = _getTradeId(tokenIn, tokenOut, amountIn, minAmountOut, recipient, numberOfTrades, msg.sender);
        tradesData[tradeId] = TradeData(0, address(0), TradeStatus.REQUESTED, receiveETH, msg.value > 0, 0);
        numberOfTrades++;
        if (msg.value > 0) {
            weth.deposit{value: amountIn}();
        } else {
            IERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);
        }
    }

    /**
     * @notice Cancel a previously requested trade.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought. This should be set offchain based on `feeCombination.tradeRebatePct`, for example, if `feeCombination.tradeRebatePct` is 20%, then `minAmountOut` could be 90% of optimal at the time of request.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade.
     */
    function cancelTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 tradeIndex
    ) external {
        bytes32 tradeId = _getTradeId(tokenIn, tokenOut, amountIn, minAmountOut, recipient, tradeIndex, msg.sender);
        TradeData memory tradeData = tradesData[tradeId];
        if (tradeData.status != TradeStatus.REQUESTED) {
            revert Book__TradeNotCancelable(tradeId);
        }

        _deleteTrade(tradeId);

        emit TradeCancelled(tradeIndex, tradeId, msg.sender);
        // Refund the trader.
        _transferAndUnwrap(IERC20(tokenIn), address(this), msg.sender, amountIn, tradeData.isEthTrade);
    }

    /**
     * @notice called by relayers for filling a trade.
     * All trade are accepted at face value, so no checks are performed. However, invalid trades can be disputed.
     * @dev Relayers are expected to correctly compute the Optimal Execution Price - feePct off-chain.
     * Failing to do so will result in a dispute and the relayer losing tokens.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought. This should be set offchain based on `feeCombination.tradeRebatePct`, for example, if `feeCombination.tradeRebatePct` is 20%, then `minAmountOut` could be 90% of optimal at the time of request.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade.
     * @param trader The address of the trader who initiated the trade.
     * @param amountToSend The amount of `tokenOut` given by the relayer to the trader.
     * @param data The data to be passed to the calling contract in the `onFloodFill` callback.
     */
    function fillTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 tradeIndex,
        address trader,
        uint256 amountToSend,
        bytes calldata data
    ) external {
        bytes32 tradeId = _getTradeId(tokenIn, tokenOut, amountIn, minAmountOut, recipient, tradeIndex, trader);
        TradeData memory tradeData = tradesData[tradeId];
        if (tradeData.status != TradeStatus.REQUESTED) {
            revert Book__TradeNotInFillableState(tradeId);
        }
        if (amountToSend < minAmountOut) {
            revert Book__AmountOutTooLow();
        }
        tradeData.filledAtBlock = block.number;
        tradeData.filledBy = msg.sender;
        tradeData.status = TradeStatus.FILLED;
        uint256 amountInToRelayer = (amountIn * relayerRefundPct) / 100;
        tradeData.amountPaid = amountInToRelayer;
        // Set the modified trade data in storage.
        tradesData[tradeId] = tradeData;
        emit TradeFilled(msg.sender, tradeIndex, amountToSend, trader);

        // Send some of the tokens to the relayer.
        IERC20(tokenIn).safeTransfer(msg.sender, amountInToRelayer);
        // Relayers can use the tokens they receive to pay for the swaps
        if (data.length > 0) {
            IFloodFillCallback(msg.sender).onFloodFill(data);
        }

        // Send the tokens to the recipient.
        _transferAndUnwrap(IERC20(tokenOut), msg.sender, recipient, amountToSend, tradeData.unwrapOutput);
    }

    /**
     * @notice It settles a trade by delivering the remaining tokens to the relayer. Can only be called after `safeBlockThreshold` blocks have passed since the trade was submitted.
     * @param tokenIn The token that was sold.
     * @param tokenOut The token that was bought.
     * @param amountIn The amount of `tokenIn` that was sold.
     * @param minAmountOut The minimum amount of `tokenOut` that was bought.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade to settle.
     * @param trader The address of the trader who initiated the trade.
     */
    function settleTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 tradeIndex,
        address trader
    ) external {
        bytes32 tradeId = _getTradeId(tokenIn, tokenOut, amountIn, minAmountOut, recipient, tradeIndex, trader);
        TradeData memory tradeData = tradesData[tradeId];
        uint256 filledHeight = tradeData.filledAtBlock;
        if (tradeData.status != TradeStatus.FILLED) {
            revert Book__TradeNotFilled(tradeId);
        }
        // Check if the trade has already been settled, is not filled or does not exist.
        if (_isDisputable(filledHeight, tradeData.status)) {
            // Always > 0 for the check above
            uint256 blocksLeft = safeBlockThreshold - (block.number - filledHeight);
            revert Book__DisputePeriodNotOver(blocksLeft);
        }

        address relayer = tradeData.filledBy;
        // Since the trade is valid, the relayer can now receive all the tokens. We calculate this by subtracting the amount already paid to the relayer from the total amountIn.
        uint256 amountInToRelayer = amountIn - tradeData.amountPaid;
        _deleteTrade(tradeId);
        emit TradeSettled(relayer, tradeIndex, filledHeight, trader);

        IERC20(tokenIn).safeTransfer(relayer, amountInToRelayer);
    }

    /**
     * @notice Called by disputers to dispute a trade.
     * For `safeBlockThreshold` blocks after a trade is submitted off-chain entities can dispute it and claim the relayer's tokens.
     * Dispute resolution is delegated to an external oracle contract.
     * This makes the trade available for relayers to fill again.
     * @param tokenIn The token that was sold.
     * @param tokenOut The token that was bought.
     * @param amountIn The amount of `tokenIn` that was sold.
     * @param minAmountOut The minimum amount of `tokenOut` that was bought.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade to dispute.
     * @param trader The address of the trader who initiated the trade.
     */
    function disputeTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 tradeIndex,
        address trader
    ) external {
        bytes32 tradeId = _getTradeId(tokenIn, tokenOut, amountIn, minAmountOut, recipient, tradeIndex, trader);
        TradeData memory tradeData = tradesData[tradeId];
        uint256 filledHeight = tradeData.filledAtBlock;

        // Check that the trade exist and has not been disputed already.
        if (!_isDisputable(filledHeight, tradeData.status)) {
            revert Book__TradeNotDisputable();
        }

        uint256 bondAmount = amountIn * disputeBondPct / 100;
        tradeData.amountPaid = tradeData.amountPaid + bondAmount;
        tradeData.status = TradeStatus.DISPUTED;
        tradesData[tradeId] = tradeData;

        // Pull the bond from the disputer
        IERC20(tokenIn).safeTransferFrom(msg.sender, address(this), bondAmount);
        // Now the book is going to sponsor the dispute. We approve 2 times. We don't re-use `bondAmount` directly as its less precise than the calculation below.
        IERC20(tokenIn).safeApprove(address(oracle), 2 * amountIn * disputeBondPct / 100);

        bytes32 disputeId = oracle.ask(
            tradeData.filledBy,
            msg.sender,
            tokenIn,
            bondAmount,
            abi.encode(amountIn, recipient, tradeIndex, trader, tradeId)
        );
        emit TradeDisputed(tradeData.filledBy, tradeIndex, disputeId, filledHeight, trader);
    }

    function onPriceSettled(bytes32 id, Request calldata request) external {
        if (msg.sender != address(oracle)) {
            revert Book__MaliciousCaller(msg.sender);
        }
        (uint256 amountIn, address recipient, uint256 tradeIndex, address trader, bytes32 tradeId) =
            abi.decode(request.data, (uint256, address, uint256, address, bytes32));
        // If answer is true, it means the relayer was truthful, so he gets the tradeRebatePct of the trade as no rebate is necessary.
        uint256 rebate = amountIn - tradesData[tradeId].amountPaid;
        _deleteTrade(tradeId);
        emit TradeDisputeSettled(request.proposer, tradeIndex, id, request.answer, trader);

        if (request.answer) {
            request.currency.safeTransfer(request.proposer, rebate);
        } else {
            _transferAndUnwrap(request.currency, address(this), recipient, rebate, tradesData[tradeId].unwrapOutput);
        }
    }

    /**
     *
     *         INTERNAL FUNCTIONS         *
     *
     */

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
     * @notice Removes a trade with id `tradeId` from storage.
     * @param tradeId The token that was sold.
     */
    function _deleteTrade(bytes32 tradeId) internal {
        delete tradesData[tradeId];
    }

    function _isDisputable(uint256 filledHeight, TradeStatus status) internal view returns (bool) {
        return status == TradeStatus.FILLED && block.number < safeBlockThreshold + filledHeight;
    }

    /**
     * @notice Checks whether a pair of tokens is supported by the flood. Reverts if not.
     * @param tokenA The first token of the pair.
     * @param tokenB The second token of the pair.
     */
    function _isPairSupported(address tokenA, address tokenB) internal view {
        if (tokenA == tokenB) {
            revert Book__SameToken();
        }
        if (!registry.isTokenWhitelisted(tokenA)) {
            revert Book__InvalidToken(tokenA);
        }
        if (!registry.isTokenWhitelisted(tokenB)) {
            revert Book__InvalidToken(tokenB);
        }
    }

    /**
     * @notice Validates the input parameters of a book instance. If the sum of the input parameters is not equal to 100, reverts.
     * @param _tradeRebatePct The trade rebate percentage, that is, how much a trader gets back if the trade is disputed and the relayer is found to be lying.
     * @param _relayerRefundPct The relayer refund percentage, that is, how much a relayer gets back immediately from a filled trade.
     * @param _disputeBondPct The dispute bond percentage, that is, how much a disputer has to pay to dispute a trade.
     */
    function _validateParams(uint256 _tradeRebatePct, uint256 _relayerRefundPct, uint256 _disputeBondPct)
        internal
        pure
    {
        if (_tradeRebatePct + _relayerRefundPct + _disputeBondPct != 100) {
            revert Book__InvalidParamsCombination();
        }
    }
    /**
     * @notice Computes the trade ID for the given trade by hashing the trade parameters.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The number of trades preceding this one.
     * @param trader The address of the trader who initiated the trade.
     * @return The trade ID.
     */

    function _getTradeId(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 tradeIndex,
        address trader
    ) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(tokenIn, tokenOut, amountIn, minAmountOut, recipient, tradeIndex, trader));
    }

    /**
     * @notice Standard function to receive ETH.
     */
    receive() external payable {}
}
