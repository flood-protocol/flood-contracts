// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "solmate/tokens/ERC20.sol";
import "solmate/auth/Owned.sol";
import "solmate/utils/SafeTransferLib.sol";
import "solmate/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./AllKnowingOracle.sol";

interface IBookSingleChainEvents {
    event SafeBlockThresholdSet(uint256 newSafeBlockThreshold);
    event FeeCombinationSet(
        uint256 disputeBondPct,
        uint256 tradeRebatePct,
        uint256 relayerPenaltyPct
    );
    event TokenWhitelisted(address indexed token, bool whitelisted);
    event TradeRequested(
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address to,
        uint256 indexed tradeIndex
    );
    event UpdatedFeeForTrade(
        address indexed trader,
        uint256 indexed tradeIndex,
        uint256 newFeePct
    );
    event TradeFilled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        uint256 feePct,
        uint256 amountOut
    );
    event TradeSettled(address indexed relayer, uint256 indexed tradeIndex);
    event TradeDisputed(
        address indexed relayer,
        uint256 indexed tradeIndex,
        bytes32 indexed disputeId
    );
    event TradeDisputeSettled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        bytes32 indexed disputeId,
        bool answer
    );
}

error BookSingleChain__InvalidToken(address token);
error BookSingleChain__FeePctTooHigh(uint256 fee);
error BookSingleChain__SameToken();
error BookSingleChain__NewFeePctTooHigh();
error BookSingleChain__UnsafeTokenToWhitelist(address token);
error BookSingleChain__ZeroAmount();
// the recipient of a transfer was the 0 address
error BookSingleChain__SentToBlackHole();
error BookSingleChain__InvalidFeeCombination();
error BookSingleChain__TradeAlreadyFilled(bytes32 tradeId);
error BookSingleChain__AmountOutTooLow();
// The trade was not filled or doesn't exist
error BookSingleChain__TradeNotFilled(bytes32 tradeId);
error BookSingleChain__InvalidSignature();
error BookSingleChain__DisputePeriodNotOver(uint256 blocksLeft);
error BookSingleChain__DisputePeriodOver();
error BookSingleChain__MaliciousCaller(address caller);

bytes32 constant SIGNATURE_DELIMITER = keccak256("FLOOD-V1");

contract BookSingleChain is
    IOptimisticRequester,
    IBookSingleChainEvents,
    Owned
{
    using SafeTransferLib for ERC20;

    uint256 constant MAX_FEE_PCT = 0.25 * 1e18;
    uint256 public immutable safeBlockThreshold;
    uint256 public immutable disputeBondPct;
    uint256 public immutable tradeRebatePct;
    uint256 public immutable relayerPenaltyPct;

    AllKnowingOracle public immutable oracle;

    uint256 public numberOfTrades = 0;
    mapping(address => bool) public whitelistedTokens;
    // Maps each trade id to the block it was filled at.
    mapping(bytes32 => uint256) public filledAtBlock;
    // A mapping from a trade id to the relayer filling it.
    mapping(bytes32 => address) public filledBy;

    constructor(
        address _oracle,
        uint256 _safeBlockThreshold,
        uint256 _disputeBondPct,
        uint256 _tradeRebatePct,
        uint256 _relayerPenaltyPct
    ) Owned(msg.sender) {
        oracle = AllKnowingOracle(_oracle);
        safeBlockThreshold = _safeBlockThreshold;
        emit SafeBlockThresholdSet(safeBlockThreshold);

        if (_disputeBondPct + _tradeRebatePct + _relayerPenaltyPct != 100) {
            revert BookSingleChain__InvalidFeeCombination();
        }
        disputeBondPct = _disputeBondPct;
        tradeRebatePct = _tradeRebatePct;
        relayerPenaltyPct = _relayerPenaltyPct;

        emit FeeCombinationSet(
            _disputeBondPct,
            _tradeRebatePct,
            _relayerPenaltyPct
        );
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
            revert BookSingleChain__UnsafeTokenToWhitelist(token);
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
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param recipient The address to receive the tokens bought.
     */
    function requestTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address recipient
    ) external {
        if (!whitelistedTokens[tokenIn]) {
            revert BookSingleChain__InvalidToken(tokenIn);
        }
        if (!whitelistedTokens[tokenOut]) {
            revert BookSingleChain__InvalidToken(tokenOut);
        }
        if (tokenIn == tokenOut) {
            revert BookSingleChain__SameToken();
        }
        if (feePct > MAX_FEE_PCT) {
            revert BookSingleChain__FeePctTooHigh(feePct);
        }
        if (amountIn == 0 || minAmountOut == 0) {
            revert BookSingleChain__ZeroAmount();
        }
        if (recipient == address(0)) {
            revert BookSingleChain__SentToBlackHole();
        }

        emit TradeRequested(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            feePct,
            recipient,
            numberOfTrades
        );

        numberOfTrades++;

        ERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);
    }

    /**
     * @notice Updates the `feePct` for the trade with the `tradeId` ID. 
        It's possible to update the fee for a trade that does not exist, but it is safe to do so, as relayers trying to fill would get disputed as if they submitted an incorrect trade.
        However, it is never possible to update the fee for a trade on behalf of another trader without their signature.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade to update.
     * @param trader The address of the trader who initially requested the trade.
     * @param newFeePct The updated fee percentage.
     * @param traderSignature A signed message by the trader that first requested the trade.
     */
    function updateFeeForTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address recipient,
        uint256 tradeIndex,
        address trader,
        uint256 newFeePct,
        bytes calldata traderSignature
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            feePct,
            recipient,
            tradeIndex
        );
        if (newFeePct > MAX_FEE_PCT) {
            revert BookSingleChain__FeePctTooHigh(newFeePct);
        }
        if (filledAtBlock[tradeId] > 0) {
            revert BookSingleChain__TradeAlreadyFilled(tradeId);
        }

        _verifyFeeUpdateSignature(trader, tradeId, newFeePct, traderSignature);

        emit UpdatedFeeForTrade(trader, tradeIndex, newFeePct);
    }

    function fillTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address recipient,
        uint256 tradeIndex,
        uint256 amountToSend
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            feePct,
            recipient,
            tradeIndex
        );
        _fillTrade(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            feePct,
            recipient,
            tradeIndex,
            tradeId,
            amountToSend,
            msg.sender
        );
    }

    /**
     * @notice Called by relayers to execute the same logic as `fillTrade` but with `updatedFeePct` instead of `feePct`.
     * This updated fee must have been requested by the `trader` that initiated the trade, by cryptographically signing a message `updatedFeePct`.
     * A trader will usually call `updateFeeForTrade` to signal to relayers the new fee and make its signature available.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade to fill.
     * @param amountToSend The amount of `tokenOut` to send to the requestor.
     * @param trader The address of the trader who initially requested the trade.
     * @param newFeePct The updated fee percentage.
     * @param traderSignature A signed message by the trader that first requested the trade.
     */
    function fillTradeWithUpdatedFee(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address recipient,
        uint256 tradeIndex,
        uint256 amountToSend,
        address trader,
        uint256 newFeePct,
        bytes calldata traderSignature
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            feePct,
            recipient,
            tradeIndex
        );

        // We delegate checking if the trade is already filled to `_verifyFeeUpdateSignature`
        _verifyFeeUpdateSignature(trader, tradeId, newFeePct, traderSignature);
        _fillTrade(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            // Emit a TradeFilled event with the new fee percentage.
            newFeePct,
            recipient,
            tradeIndex,
            tradeId,
            amountToSend,
            trader
        );
    }

    /**
    @notice It settles a trade by delivering the remaining tokens to the relayer. Can only be called after `safeBlockThreshold` blocks have passed since the trade was submitted.
    @param tokenIn The token that was sold.
    @param tokenOut The token that was bought.
    @param amountIn The amount of `tokenIn` that was sold.
    @param minAmountOut The minimum amount of `tokenOut` that was bought.
    @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
    @param recipient The address to receive the tokens bought.
    @param tradeIndex The index of the trade to settle.
    */
    function settleTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address recipient,
        uint256 tradeIndex
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            feePct,
            recipient,
            tradeIndex
        );
        // Check if the trade has already been settled, is not filled or does not exist.
        if (filledAtBlock[tradeId] == 0) {
            revert BookSingleChain__TradeNotFilled(tradeId);
        }
        // block.number - filledAtBlock[tradeId] < safeBlockThreshold -> block.number < safeBlockThreshold + filledAtBlock[tradeId]
        if (block.number - filledAtBlock[tradeId] < safeBlockThreshold) {
            // Always > 0 for the check above
            uint256 blocksLeft = safeBlockThreshold -
                (block.number - filledAtBlock[tradeId]);
            revert BookSingleChain__DisputePeriodNotOver(blocksLeft);
        }

        address relayer = filledBy[tradeId];

        delete filledAtBlock[tradeId];
        delete filledBy[tradeId];

        // Since the trade is valid, the relayer can now receive all the tokens.
        uint256 amountInToRelayer = (amountIn * (relayerPenaltyPct)) / 100;

        ERC20(tokenIn).safeTransfer(relayer, amountInToRelayer);

        emit TradeSettled(relayer, tradeIndex);
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
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The index of the trade to dispute.
     */
    function disputeTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address recipient,
        uint256 tradeIndex
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            minAmountOut,
            feePct,
            recipient,
            tradeIndex
        );
        // nest to avoid stack too deep, yes this is still a thing in 2022
        {
            uint256 filledHeight = filledAtBlock[tradeId];

            // Check that the trade has been filled.
            if (filledHeight == 0) {
                revert BookSingleChain__TradeNotFilled(tradeId);
            }
            // Check that the dispute period has not yet ended.
            if (block.number - filledHeight >= safeBlockThreshold) {
                revert BookSingleChain__DisputePeriodOver();
            }
        }

        address relayer = filledBy[tradeId];
        // Mark the trade as unfilled to allow relayers to fill it again.
        delete filledAtBlock[tradeId];
        delete filledBy[tradeId];

        uint256 bondAmount = (amountIn * (disputeBondPct)) / 100;

        ERC20(tokenIn).safeApprove(address(oracle), bondAmount);
        oracle.ask(
            relayer,
            msg.sender,
            tokenIn,
            (amountIn * (disputeBondPct)) / 100,
            abi.encode(amountIn, recipient, tradeIndex)
        );
        ERC20(tokenIn).safeApprove(address(oracle), 0);

        bytes32 disputeId = oracle.getRequestId(
            address(this),
            relayer,
            msg.sender,
            tokenIn,
            bondAmount
        );

        emit TradeDisputed(relayer, tradeIndex, disputeId);
    }

    function onPriceSettled(bytes32 id, Request calldata request) external {
        if (msg.sender != address(oracle)) {
            revert BookSingleChain__MaliciousCaller(msg.sender);
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

    /**************************************
     *         INTERNAL FUNCTIONS         *
     **************************************/

    function _verifyFeeUpdateSignature(
        address trader,
        bytes32 tradeId,
        uint256 newFeePct,
        bytes calldata traderSignature
    ) internal view {
        if (newFeePct > MAX_FEE_PCT) {
            revert BookSingleChain__FeePctTooHigh(newFeePct);
        }
        if (filledAtBlock[tradeId] > 0) {
            revert BookSingleChain__TradeAlreadyFilled(tradeId);
        }

        bytes32 expectedMessageHash = keccak256(
            abi.encodePacked(SIGNATURE_DELIMITER, tradeId, newFeePct)
        );

        bytes32 ethSignedMessageHash = ECDSA.toEthSignedMessageHash(
            expectedMessageHash
        );
        address maybeTrader = ECDSA.recover(
            ethSignedMessageHash,
            traderSignature
        );
        if (maybeTrader != trader) {
            revert BookSingleChain__InvalidSignature();
        }
    }

    /**
     * @notice called internally for filling a trade.
     * All trade are accepted at face value, so no checks are performed. However, invalid trades can be disputed.
     * @dev Relayers are expected to correctly compute the Optimal Execution Price - feePct off-chain.
     * Failing to do so will result in a dispute and the relayer losing tokens.
     */
    function _fillTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address recipient,
        uint256 tradeIndex,
        bytes32 tradeId,
        uint256 amountToSend,
        address relayer
    ) internal {
        if (filledAtBlock[tradeId] > 0) {
            revert BookSingleChain__TradeAlreadyFilled(tradeId);
        }
        if (amountToSend < minAmountOut) {
            revert BookSingleChain__AmountOutTooLow();
        }
        filledAtBlock[tradeId] = block.number;
        filledBy[tradeId] = relayer;

        emit TradeFilled(relayer, tradeIndex, feePct, amountToSend);
        // Send the tokens to the recipient.
        ERC20(tokenOut).safeTransferFrom(relayer, recipient, amountToSend);
        // Send some of the tokens to the relayer.
        uint256 amountInToRelayer = (amountIn * (100 - relayerPenaltyPct)) /
            100;

        ERC20(tokenIn).safeTransferFrom(
            address(this),
            relayer,
            amountInToRelayer
        );
    }

    /**
     * @notice Computes the trade ID for the given trade by hashing the trade parameters.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param minAmountOut The minimum amount of `tokenOut` to be bought.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param recipient The address to receive the tokens bought.
     * @param tradeIndex The number of trades preceding this one.
     * @return The trade ID.
     */
    function _getTradeId(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 feePct,
        address recipient,
        uint256 tradeIndex
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encodePacked(
                    tokenIn,
                    tokenOut,
                    amountIn,
                    minAmountOut,
                    feePct,
                    recipient,
                    tradeIndex
                )
            );
    }
}
