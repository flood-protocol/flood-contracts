// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "solmate/tokens/ERC20.sol";
import "solmate/auth/Owned.sol";
import "solmate/utils/SafeTransferLib.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./AllKnowingOracle.sol";

error BookSingleChain__InvalidToken(address token);
error BookSingleChain__FeePctTooHigh(uint256 fee);
error BookSingleChain__SameToken();
error BookSingleChain__NewFeePctTooHigh();
error BookSingleChain__ZeroAmount();
// the recipient of a transfer was the 0 address
error BookSingleChain__SentToBlackHole();
error BookSingleChain__TradeAlreadyFilled(bytes32 tradeId);

// The trade was not filled or doesn't exist
error BookSingleChain__TradeNotFilled(bytes32 tradeId);
error BookSingleChain__InvalidSignature();
error BookSingleChain__DisputePeriodNotOver(uint256 blocksLeft);
error BookSingleChain__DisputePeriodOver();

bytes32 constant SIGNATURE_DELIMITER = keccak256("LAGUNA-V1");

struct FilledTrade {
    address trader;
    uint256 amountOut;
    uint256 filledAtBlock;
}

/**
 * @title BookSingleChain
 * @notice A basic RFQ book implementation, where users request trades and off-chain relayers fill them.
 * To ensure relayers fill trades at "fair" price, there is a block range in which trades can be disputed and voided.
 * If a trade is not disputed within the dispute period, the relayer can call `refund` to obtain the other side of the trade it filled.
 * @notice This implementation gives immense power to the owner of the contract, and only allows one relayer / disputer. This is not intended for production use, but rather for a small scale test.
 */
contract BookSingleChain is Owned {
    using SafeTransferLib for ERC20;

    // Number of trades done so far. Used to generate trade ids.
    uint128 public numberOfTrades = 0;
    // The amountIn of blocks in which a trade can be disputed.
    uint256 public safeBlockThreshold;
    // The maximum % off the optimal quote allowed. 1e18 is 100%.
    uint128 public maxFeePct;
    // A mapping with the tokens that are supported by this contract.
    mapping(address => bool) public whitelistedTokens;

    // Maps each trade id to the block it was filled at.
    mapping(bytes32 => uint256) public filledAtBlock;
    // A mapping from a trade id to the relayer filling it.
    mapping(bytes32 => address) public filledBy;
    // A mapping from a trade id to the amount of tokens received by the trader that requested the trade.
    mapping(bytes32 => uint256) public filledAmount;

    // Oracle used for dispute resolution
    IOracle public immutable oracle;

    /****************************************
     *                EVENTS                *
     ****************************************/

    event SafeBlockThresholdChanged(uint256 newSafeBlockThreshold);
    event MaxFeePctChanged(uint128 newMaxFeePct);
    event TokenWhitelisted(address indexed token, bool whitelisted);
    event TradeRequested(
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 feePct,
        address to,
        uint256 indexed tradeIndex
    );
    event UpdatedFeeForTrade(
        address indexed trader,
        bytes32 indexed tradeId,
        uint256 newFeePct
    );
    event TradeFilled(
        address indexed relayer,
        bytes32 indexed tradeId,
        uint256 indexed filledAtBlock,
        uint256 feePct,
        uint256 amountOut
    );
    event TradeSettled(
        address indexed relayer,
        bytes32 indexed tradeId,
        uint256 indexed filledAmount,
        uint256 feePct
    );
    event TradeDisputed(
        address indexed relayer,
        bytes32 indexed tradeId,
        bytes32 indexed disputeId,
        uint256 filledAmount,
        uint256 feePct
    );

    /**
     * @notice Constructs the order book.
     * @param _safeBlockThreshold The number of blocks in which a trade can be disputed.
     * @param _oracleAddress The address of the oracle used for dispute resolution.
     */
    constructor(uint256 _safeBlockThreshold, address _oracleAddress)
        Owned(msg.sender)
    {
        oracle = IOracle(_oracleAddress);
        safeBlockThreshold = _safeBlockThreshold;
        emit SafeBlockThresholdChanged(safeBlockThreshold);
        maxFeePct = 0.25 * 1e18;
        emit MaxFeePctChanged(maxFeePct);
    }

    /**************************************
     *          ADMIN FUNCTIONS           *
     **************************************/

    /**
     * @notice Changes the safe block threshold.
     * @param newSafeBlockThreshold The new safe block threshold.
     */
    function setSafeBlockThreshold(uint256 newSafeBlockThreshold)
        external
        onlyOwner
    {
        safeBlockThreshold = newSafeBlockThreshold;
        emit SafeBlockThresholdChanged(safeBlockThreshold);
    }

    /**
     * @notice Adds a token to the whitelist.
     * @param token The token to add to the whitelist.
     * @param whitelisted If `true` whitelists the token, if `false` it removes it.
     */
    function whitelistToken(address token, bool whitelisted)
        external
        onlyOwner
    {
        whitelistedTokens[token] = whitelisted;
        emit TokenWhitelisted(token, whitelisted);
    }

    /**
     * @notice Changes the maximum fee percentage.
     * @param newMaxFeePct The new maximum fee percentage.
     */
    function setMaxFeePct(uint128 newMaxFeePct) external onlyOwner {
        if (newMaxFeePct >= 1e18) {
            revert BookSingleChain__NewFeePctTooHigh();
        }
        maxFeePct = newMaxFeePct;
        emit MaxFeePctChanged(maxFeePct);
    }

    /**************************************
     *         TRADING FUNCTIONS        *
     **************************************/

    /**
     * @notice Requests to trade `amountIn` of `tokenIn` for `tokenOut` with `feePct` fee off the optimal quote at execution time. Users deposit `tokenIn` to the contract.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param to The address to receive the tokens bought.
     */
    function requestTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 feePct,
        address to
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
        if (feePct > maxFeePct) {
            revert BookSingleChain__FeePctTooHigh(feePct);
        }
        if (amountIn == 0) {
            revert BookSingleChain__ZeroAmount();
        }
        if (to == address(0)) {
            revert BookSingleChain__SentToBlackHole();
        }

        ERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);

        emit TradeRequested(
            tokenIn,
            tokenOut,
            amountIn,
            feePct,
            to,
            numberOfTrades
        );

        numberOfTrades++;
    }

    /**
     * @notice Updates the `feePct` for the trade with the `tradeId` ID. 
        It's possible to update the fee for a trade that does not exist, but it is safe to do so, as relayers trying to fill would get disputed as if they submitted an incorrect trade.
        However, it is never possible to update the fee for a trade on behalf of another trader without their signature.
     * @param trader The address of the trader who initially requested the trade.
     * @param newFeePct The updated fee percentage.
     * @param tradeId The ID of the trade to update.
     * @param traderSignature A signed message by the trader that first requested the trade.
     */
    function updateFeeForTrade(
        address trader,
        bytes32 tradeId,
        uint256 newFeePct,
        bytes calldata traderSignature
    ) external {
        if (newFeePct > maxFeePct) {
            revert BookSingleChain__FeePctTooHigh(newFeePct);
        }
        if (filledAtBlock[tradeId] > 0) {
            revert BookSingleChain__TradeAlreadyFilled(tradeId);
        }

        _verifyFeeUpdateSignature(trader, tradeId, newFeePct, traderSignature);

        emit UpdatedFeeForTrade(trader, tradeId, newFeePct);
    }

    /**
     * @notice Called by relayers to fill a trade posted by users. A relayer takes on the other side of the trade by giving tokenOut to the requestor of the trade.
     * A relayer is expect to uniquely identify the trade by sending the contract unique information about it.
     * Each trade submission is validate off-chain and can be disputed up to `safeBlockThreshold` blocks after it was submitted.
     * If the trade is found to be invalid, the relayer loses its tokens and won't get the other side of the trade.
     * After `safeBlockThreshold` blocks, the trade is considered valid and the relayer can request the other side of the trade by calling `settleTrade`.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param to The address to receive the tokens bought.
     * @param tradeIndex The index of the trade to fill.
     * @param amountToSend The amount of `tokenOut` to send to the requestor.
     */
    function fillTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 feePct,
        address to,
        uint128 tradeIndex,
        uint256 amountToSend
    ) external {
        // We don't need to check if the trade is valid, as the relayer is expected to do that off-chain.
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            feePct,
            to,
            tradeIndex
        );
        // Check if the trade has already been filled, to prevent relayers losing tokens if two or more of them try to fill the same trade.
        if (filledAtBlock[tradeId] > 0) {
            revert BookSingleChain__TradeAlreadyFilled(tradeId);
        }

        _fillTrade(tokenOut, tradeId, amountToSend);

        emit TradeFilled(
            msg.sender,
            tradeId,
            block.number,
            feePct,
            amountToSend
        );
    }

    /**
     * @notice Called by relayers to execute the same logic as `fillTrade` but with `updatedFeePct` instead of `feePct`.
     * This updated fee must have been requested by the `trader` that initiated the trade, by cryptographically signing a message `updatedFeePct`.
     * A trader will usually call `updateFeeForTrade` to signal to relayers the new fee and make its signature available.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param to The address to receive the tokens bought.
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
        uint256 feePct,
        address to,
        uint128 tradeIndex,
        uint256 amountToSend,
        address trader,
        uint256 newFeePct,
        bytes calldata traderSignature
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            feePct,
            to,
            tradeIndex
        );

        // We delegate checking if the trade is already filled to `_verifyFeeUpdateSignature`
        _verifyFeeUpdateSignature(trader, tradeId, newFeePct, traderSignature);
        _fillTrade(tokenOut, tradeId, amountToSend);
        // Emit a TradeFilled event with the new fee percentage.
        emit TradeFilled(
            msg.sender,
            tradeId,
            block.number,
            newFeePct,
            amountToSend
        );
    }

    /**
    @notice It settles a trade by delivering tokens to both the relayer and the trader. Can only be called after `safeBlockThreshold` blocks have passed since the trade was submitted.
    @param tokenIn The token that was sold.
    @param tokenOut The token that was bought.
    @param amountIn The amount of `tokenIn` that was sold.
    @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
    @param to The address to receive the tokens bought.
    @param tradeIndex The index of the trade to settle.
     */
    function settleTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 feePct,
        address to,
        uint128 tradeIndex
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            feePct,
            to,
            tradeIndex
        );
        // Check if the trade has already been settled, is not filled or does not exist.
        if (filledAtBlock[tradeId] == 0) {
            revert BookSingleChain__TradeNotFilled(tradeId);
        }
        if (block.number - filledAtBlock[tradeId] < safeBlockThreshold) {
            // Always > 0 for the check above
            uint256 blocksLeft = safeBlockThreshold -
                (block.number - filledAtBlock[tradeId]);
            revert BookSingleChain__DisputePeriodNotOver(blocksLeft);
        }
        uint256 amountToTrader = filledAmount[tradeId];
        address relayer = filledBy[tradeId];

        delete filledAtBlock[tradeId];
        delete filledBy[tradeId];
        delete filledAmount[tradeId];

        ERC20(tokenOut).safeTransfer(to, amountToTrader);
        ERC20(tokenIn).safeTransfer(relayer, amountIn);

        emit TradeSettled(relayer, tradeId, amountToTrader, feePct);
    }

    /**
     * @notice Called by disputers to dispute a trade.
     * For `safeBlockThreshold` blocks after a trade is submitted off-chain entities can dispute it and claim the relayer's tokens.
     * Dispute resolution is delegated to an external oracle contract.
     * This makes the trade available for relayers to fill again.
     * @param tokenIn The token that was sold.
     * @param tokenOut The token that was bought.
     * @param amountIn The amount of `tokenIn` that was sold.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param to The address to receive the tokens bought.
     * @param tradeIndex The index of the trade to dispute.
     */
    function disputeTrade(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 feePct,
        address to,
        uint128 tradeIndex
    ) external {
        bytes32 tradeId = _getTradeId(
            tokenIn,
            tokenOut,
            amountIn,
            feePct,
            to,
            tradeIndex
        );
        uint256 filledHeight = filledAtBlock[tradeId];

        // Check that the trade has been filled.
        if (filledHeight == 0) {
            revert BookSingleChain__TradeNotFilled(tradeId);
        }
        // Check that the dispute period has not yet ended.
        if (block.number - filledHeight >= safeBlockThreshold) {
            revert BookSingleChain__DisputePeriodOver();
        }

        uint256 amountSent = filledAmount[tradeId];
        address relayer = filledBy[tradeId];
        // Mark the trade as unfilled to allow relayers to fill it again.
        delete filledAtBlock[tradeId];
        delete filledBy[tradeId];
        delete filledAmount[tradeId];

        // Approve the oracle to spend the amountSent by the relayer.
        ERC20(tokenOut).safeApprove(address(oracle), amountSent);
        bytes32 disputeId = oracle.ask(
            relayer,
            msg.sender,
            tokenOut,
            amountSent
        );
        emit TradeDisputed(relayer, tradeId, disputeId, amountSent, feePct);
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
        if (newFeePct > maxFeePct) {
            revert BookSingleChain__FeePctTooHigh(newFeePct);
        }
        if (filledAtBlock[tradeId] > 0) {
            revert BookSingleChain__TradeAlreadyFilled(tradeId);
        }

        bytes32 expectedMessageHash = keccak256(
            abi.encode(SIGNATURE_DELIMITER, tradeId, newFeePct)
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
        address tokenOut,
        bytes32 tradeId,
        uint256 amountToSend
    ) internal {
        filledAtBlock[tradeId] = block.number;
        filledBy[tradeId] = msg.sender;
        filledAmount[tradeId] = amountToSend;

        ERC20(tokenOut).safeTransferFrom(
            msg.sender,
            address(this),
            amountToSend
        );
    }

    /**
     * @notice Computes the trade ID for the given trade by hashing the trade parameters.
     * @param tokenIn The token to be sold.
     * @param tokenOut The token to be bought.
     * @param amountIn The amount of `tokenIn` to be sold.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param to The address to receive the tokens bought.
     * @param tradeIndex The number of trades preceding this one.
     * @return The trade ID.
     */
    function _getTradeId(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 feePct,
        address to,
        uint128 tradeIndex
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encode(tokenIn, tokenOut, amountIn, feePct, to, tradeIndex)
            );
    }
}
