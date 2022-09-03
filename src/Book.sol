// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "solmate/tokens/ERC20.sol";
import "solmate/auth/Owned.sol";
import "solmate/utils/SafeTransferLib.sol";
import "solmate/utils/ReentrancyGuard.sol";
import "./AllKnowingOracle.sol";

interface IBookEvents {
    event SafeBlockThresholdSet(uint256 newSafeBlockThreshold);
    event FeeCombinationSet(
        uint256 disputeBondPct,
        uint256 tradeRebatePct,
        uint256 relayerRefundPct
    );
    event FeePctSet(uint256 feePct);
    event TokenWhitelisted(address indexed token, bool whitelisted);
    event TradeRequested(
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 indexed tradeIndex,
        address trader
    );
    event TradeFilled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        uint256 amountOut
    );
    event TradeSettled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        uint256 filledAtBlock
    );
    event TradeDisputed(
        address indexed relayer,
        uint256 indexed tradeIndex,
        bytes32 indexed disputeId,
        uint256 filledAtBlock
    );
    event TradeCancelled(uint256 indexed tradeIndex, bytes32 indexed tradeId);
    event TradeDisputeSettled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        bytes32 indexed disputeId,
        bool answer
    );
}

error Book__InvalidToken(address token);
error Book__SameToken();
error Book__UnsafeTokenToWhitelist(address token);
error Book__ZeroAmount();
// the recipient of a transfer was the 0 address
error Book__SentToBlackHole();
error Book__InvalidFeeCombination();
error Book__FeePctTooHigh();
// The trade is not filled yet, doesn't exist or was disputed
error Book__TradeNotInFillableState(bytes32 tradeId);
error Book__TradeNotFilled(bytes32 tradeId);
error Book__TradeNotCancelable(bytes32 tradeId);
error Book__AmountOutTooLow();
error Book__DisputePeriodNotOver(uint256 blocksLeft);
error Book__DisputePeriodOver();
error Book__MaliciousCaller(address caller);
error Book__NotTrader();

enum TradeStatus {
    // The trade is not initialized, meaning it does not exist or was already settled/disputed
    UNINITIALIZED,
    // The trade is initialized and can be filled
    REQUESTED,
    // The trade is filled but not settled/disputed
    FILLED
}

contract Book is IOptimisticRequester, IBookEvents, Owned {
    using SafeTransferLib for ERC20;

    uint256 public immutable safeBlockThreshold;
    uint256 public immutable disputeBondPct;
    uint256 public immutable tradeRebatePct;
    uint256 public immutable relayerRefundPct;
    // The fee taken on each trade by relayers, expressed in basis points (so 100% = 10000)
    uint256 public immutable feePct;

    AllKnowingOracle public immutable oracle;

    uint256 public numberOfTrades = 0;
    mapping(address => bool) public whitelistedTokens;
    // Maps each trade id to the block it was filled at. A value of 0 means it was not filled yet.
    mapping(bytes32 => uint256) public filledAtBlock;
    // A mapping from a trade id to the relayer filling it.
    mapping(bytes32 => address) public filledBy;
    // A mapping from trade id to an enum representing the state of the trade.
    mapping(bytes32 => TradeStatus) public status;

    constructor(
        address _oracle,
        uint256 _safeBlockThreshold,
        uint256 _disputeBondPct,
        uint256 _tradeRebatePct,
        uint256 _relayerRefundPct,
        uint256 _feePct
    ) Owned(msg.sender) {
        oracle = AllKnowingOracle(_oracle);
        safeBlockThreshold = _safeBlockThreshold;
        emit SafeBlockThresholdSet(safeBlockThreshold);

        if (_disputeBondPct + _tradeRebatePct + _relayerRefundPct != 100) {
            revert Book__InvalidFeeCombination();
        }
        disputeBondPct = _disputeBondPct;
        tradeRebatePct = _tradeRebatePct;
        relayerRefundPct = _relayerRefundPct;

        emit FeeCombinationSet(
            _disputeBondPct,
            _tradeRebatePct,
            _relayerRefundPct
        );

        if (_feePct > 2500) {
            revert Book__FeePctTooHigh();
        }

        feePct = _feePct;
        emit FeePctSet(_feePct);
    }

    /**
     * @notice Adds a token to the whitelist.
     * Only allows tokens whitelisted by the oracle to make sure all trades are disputable.
     * @param token The token to add to the whitelist.
     * @param whitelisted If `true` whitelists the token, if `false` it removes it.
     */
    function whitelistToken(address token, bool whitelisted)
        external
        onlyOwner
    {
        if (whitelisted && !oracle.whitelistedTokens(token)) {
            revert Book__UnsafeTokenToWhitelist(token);
        }
        whitelistedTokens[token] = whitelisted;
        emit TokenWhitelisted(token, whitelisted);
    }

    /**
     * @notice Requests to trade `amountIn` of `tokenIn` for `tokenOut` with `feePct` fee off the optimal quote at execution time. Users deposit `tokenIn` to the contract.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought. This should be set offchain based on `feeCombination.tradeRebatePct`, for example, if `feeCombination.tradeRebatePct` is 20%, then `minAmountOut` could be 90% of optimal at the time of request.
     * @param recipient The address to receive the tokens bought.
     */
    function requestTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient
    ) external {
        if (!whitelistedTokens[tokenIn]) {
            revert Book__InvalidToken(tokenIn);
        }
        if (!whitelistedTokens[tokenOut]) {
            revert Book__InvalidToken(tokenOut);
        }
        if (tokenIn == tokenOut) {
            revert Book__SameToken();
        }
        if (amountIn == 0 || minAmountOut == 0) {
            revert Book__ZeroAmount();
        }
        if (recipient == address(0)) {
            revert Book__SentToBlackHole();
        }

        emit TradeRequested(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            recipient,
            numberOfTrades,
            msg.sender
        );

        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            recipient,
            numberOfTrades,
            msg.sender
        );
        status[tradeId] = TradeStatus.REQUESTED;
        numberOfTrades++;

        ERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);
    }

    /**
     * @notice Fills a trade with the optimal quote at the time of execution.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought. This should be set offchain based on `feeCombination.tradeRebatePct`, for example, if `feeCombination.tradeRebatePct` is 20%, then `minAmountOut` could be 90% of optimal at the time of request.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade.
     * @param trader The address of the trader who originally requested the trade. Must equal msg.sender.
     */
    function cancelTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 tradeIndex,
        address trader
    ) external {
        if (msg.sender != trader) {
            revert Book__NotTrader();
        }
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            recipient,
            tradeIndex,
            trader
        );
        if (status[tradeId] != TradeStatus.REQUESTED) {
            revert Book__TradeNotCancelable(tradeId);
        }

        _deleteTrade(tradeId);

        emit TradeCancelled(tradeIndex, tradeId);
        // Refund the trader.
        ERC20(tokenIn).safeTransfer(trader, amountIn);
    }

    /**
     * @notice called internally for filling a trade.
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
     */
    function fillTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        address recipient,
        uint256 tradeIndex,
        address trader,
        uint256 amountToSend
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            recipient,
            tradeIndex,
            trader
        );
        if (status[tradeId] != TradeStatus.REQUESTED) {
            revert Book__TradeNotInFillableState(tradeId);
        }
        if (amountToSend < minAmountOut) {
            revert Book__AmountOutTooLow();
        }
        filledAtBlock[tradeId] = block.number;
        filledBy[tradeId] = msg.sender;
        status[tradeId] = TradeStatus.FILLED;

        emit TradeFilled(msg.sender, tradeIndex, amountToSend);
        // Send the tokens to the recipient.
        ERC20(tokenOut).safeTransferFrom(msg.sender, recipient, amountToSend);
        // Send some of the tokens to the relayer.
        uint256 amountInToRelayer = (amountIn * relayerRefundPct) / 100;

        ERC20(tokenIn).safeTransfer(msg.sender, amountInToRelayer);
    }

    /**
    @notice It settles a trade by delivering the remaining tokens to the relayer. Can only be called after `safeBlockThreshold` blocks have passed since the trade was submitted.
    @param tokenIn The token that was sold.
    @param tokenOut The token that was bought.
    @param amountIn The amount of `tokenIn` that was sold.
    @param minAmountOut The minimum amount of `tokenOut` that was bought.
    @param recipient The address to receive the tokens bought.
    @param tradeIndex The index of the trade to settle.
    @param trader The address of the trader who initiated the trade.
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
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            recipient,
            tradeIndex,
            trader
        );
        uint256 filledHeight = filledAtBlock[tradeId];
        // Check if the trade has already been settled, is not filled or does not exist. We do not use the status as we need to read the filledHeight anyway.
        if (filledHeight == 0) {
            revert Book__TradeNotFilled(tradeId);
        }
        // safe cast as for the check above we know that filledAtBlock[tradeId] > 0.
        if (_isDisputable(filledHeight)) {
            // Always > 0 for the check above
            uint256 blocksLeft = safeBlockThreshold -
                (block.number - filledAtBlock[tradeId]);
            revert Book__DisputePeriodNotOver(blocksLeft);
        }

        address relayer = filledBy[tradeId];

        _deleteTrade(tradeId);

        // Since the trade is valid, the relayer can now receive all the tokens.
        uint256 amountInToRelayer = (amountIn * (100 - relayerRefundPct)) / 100;

        ERC20(tokenIn).safeTransfer(relayer, amountInToRelayer);

        emit TradeSettled(relayer, tradeIndex, filledHeight);
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
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            recipient,
            tradeIndex,
            trader
        );

        uint256 filledHeight = filledAtBlock[tradeId];

        // Check that the trade exist and has not been disputed already. We do not use the status as we need to read the filledHeight anyway and thus save a read.
        if (filledHeight == 0) {
            revert Book__TradeNotFilled(tradeId);
        }

        if (!_isDisputable(filledHeight)) {
            revert Book__DisputePeriodOver();
        }

        address relayer = filledBy[tradeId];
        uint256 bondAmount = (amountIn * (disputeBondPct)) / 100;

        _deleteTrade(tradeId);

        ERC20(tokenIn).safeApprove(address(oracle), bondAmount);
        bytes32 disputeId = oracle.ask(
            relayer,
            msg.sender,
            tokenIn,
            bondAmount,
            abi.encode(amountIn, recipient, tradeIndex)
        );
        ERC20(tokenIn).safeApprove(address(oracle), 0);

        emit TradeDisputed(relayer, tradeIndex, disputeId, filledHeight);
    }

    function onPriceSettled(bytes32 id, Request calldata request) external {
        if (msg.sender != address(oracle)) {
            revert Book__MaliciousCaller(msg.sender);
        }
        (uint256 amountIn, address recipient, uint256 tradeIndex) = abi.decode(
            request.data,
            (uint256, address, uint256)
        );
        // If answer is true, it means the relayer was truthful, so he gets the tradeRebatePct of the trade as no rebate is necessary.
        uint256 rebate = (amountIn * tradeRebatePct) / 100;
        if (request.answer) {
            request.currency.safeTransfer(request.proposer, rebate);
        } else {
            request.currency.safeTransfer(recipient, rebate);
        }

        emit TradeDisputeSettled(
            request.proposer,
            tradeIndex,
            id,
            request.answer
        );
    }

    /**
     *************************************
     *         INTERNAL FUNCTIONS         *
     *************************************
     */

    /**
     * @notice Removes a trade with id `tradeId` from storage.
     * @param tradeId The token that was sold.
     */
    function _deleteTrade(bytes32 tradeId) internal {
        delete filledAtBlock[tradeId];
        delete filledBy[tradeId];
        delete status[tradeId];
    }

    function _isDisputable(uint256 filledHeight) internal view returns (bool) {
        return block.number < safeBlockThreshold + filledHeight;
    }

    /**
     * @notice Computes the trade ID for the given trade by hashing the trade parameters.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The number of trades preceding this one.
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
        return
            keccak256(
                abi.encodePacked(
                    tokenIn,
                    tokenOut,
                    amountIn,
                    minAmountOut,
                    recipient,
                    tradeIndex,
                    trader
                )
            );
    }
}
