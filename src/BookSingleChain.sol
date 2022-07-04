// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "solmate/tokens/ERC20.sol";
import "solmate/auth/Owned.sol";
import "solmate/utils/SafeTransferLib.sol";
import "solmate/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./AllKnowingOracle.sol";
import "forge-std/Test.sol";

error BookSingleChain__InvalidToken(address token);
error BookSingleChain__FeePctTooHigh(uint256 fee);
error BookSingleChain__SameToken();
error BookSingleChain__NewFeePctTooHigh();
error BookSingleChain__UnsafeTokenToWhitelist(address token);
error BookSingleChain__ZeroAmount();
// the recipient of a transfer was the 0 address
error BookSingleChain__SentToBlackHole();
error BookSingleChain__TradeAlreadyFilled(uint256 tradeIndex);

// The trade was not filled or doesn't exist
error BookSingleChain__TradeNotFilled(uint256 tradeIndex);
error BookSingleChain__InvalidSignature();
error BookSingleChain__DisputePeriodNotOver(uint256 blocksLeft);
error BookSingleChain__DisputePeriodOver();

bytes32 constant SIGNATURE_DELIMITER = keccak256("LAGUNA-V1");

struct FilledTrade {
    // Address of the relayer who filled the trade
    address relayer;
    // Amount of tokens received by the trader who requested the trade
    uint256 amountOut;
    // block the trade was filled at
    uint256 blockHeight;
}

/**
 * @title BookSingleChain
 * @notice A basic RFQ book implementation, where users request trades and off-chain relayers fill them.
 * To ensure relayers fill trades at "fair" price, there is a block range in which trades can be disputed and voided.
 * If a trade is not disputed within the dispute period, the relayer can call `refund` to obtain the other side of the trade it filled.
 * @notice This implementation gives immense power to the owner of the contract, and only allows one relayer / disputer. This is not intended for production use, but rather for a small scale test.
 */
contract BookSingleChain is Owned, ReentrancyGuard {
    using SafeTransferLib for ERC20;

    // Number of trades done so far. Used to generate trade ids.
    uint256 public numberOfTrades = 0;
    // The amountIn of blocks in which a trade can be disputed.
    uint256 public safeBlockThreshold;
    // The maximum % off the optimal quote allowed. 1e18 is 100%.
    uint256 public maxFeePct;
    // A mapping with the tokens that are supported by this contract.
    mapping(address => bool) public whitelistedTokens;

    // A mapping of trades that have been filled. They are indexed by trade_index so the array might be sparse.
    mapping(uint256 => FilledTrade) public filledTrades;

    // Oracle used for dispute resolution
    IOracle public immutable oracle;

    /****************************************
     *                EVENTS                *
     ****************************************/

    event SafeBlockThresholdChanged(uint256 newSafeBlockThreshold);
    event MaxFeePctChanged(uint256 newMaxFeePct);
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
        uint256 indexed tradeIndex,
        uint256 newFeePct
    );
    event TradeFilled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        uint256 indexed filledAtBlock,
        uint256 feePct,
        uint256 amountOut
    );
    event TradeSettled(
        address indexed relayer,
        uint256 indexed tradeIndex,
        uint256 indexed filledAmount,
        uint256 feePct
    );
    event TradeDisputed(
        address indexed relayer,
        uint256 indexed tradeIndex,
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
     * @notice Changes the maximum fee percentage.
     * @param newMaxFeePct The new maximum fee percentage.
     */
    function setMaxFeePct(uint256 newMaxFeePct) external onlyOwner {
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
     * @param tradeIndex The ID of the trade to update.
     * @param traderSignature A signed message by the trader that first requested the trade.
     */
    function updateFeeForTrade(
        address trader,
        uint256 newFeePct,
        uint256 tradeIndex,
        bytes calldata traderSignature
    ) external {
        if (newFeePct > maxFeePct) {
            revert BookSingleChain__FeePctTooHigh(newFeePct);
        }
        if (_isTradeFilled(tradeIndex)) {
            revert BookSingleChain__TradeAlreadyFilled(tradeIndex);
        }

        _verifyFeeUpdateSignature(
            trader,
            tradeIndex,
            newFeePct,
            traderSignature
        );

        emit UpdatedFeeForTrade(trader, tradeIndex, newFeePct);
    }

    /**
     * @notice Called by relayers to fill a trade posted by users. A relayer takes on the other side of the trade by giving tokenOut to the requestor of the trade.
     * A relayer is expect to uniquely identify the trade by sending the contract unique information about it.
     * Each trade submission is validate off-chain and can be disputed up to `safeBlockThreshold` blocks after it was submitted.
     * If the trade is found to be invalid, the relayer loses its tokens and won't get the other side of the trade.
     * After `safeBlockThreshold` blocks, the trade is considered valid and the relayer can request the other side of the trade by calling `settleTrade`.
     * @param tokenOut The token to be bought.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param tradeIndex The index of the trade to fill.
     * @param amountToSend The amount of `tokenOut` to send to the requestor.
     */
    function fillTrade(
        address tokenOut,
        uint256 feePct,
        uint256 tradeIndex,
        uint256 amountToSend
    ) external {
        // Check if the trade has already been filled, to prevent relayers losing tokens if two or more of them try to fill the same trade.
        if (filledTrades[tradeIndex].blockHeight > 0) {
            revert BookSingleChain__TradeAlreadyFilled(tradeIndex);
        }

        _fillTrade(tokenOut, tradeIndex, amountToSend);

        emit TradeFilled(
            msg.sender,
            tradeIndex,
            block.number,
            feePct,
            amountToSend
        );
    }

    /**
     * @notice Called by relayers to execute the same logic as `fillTrade` but with `updatedFeePct` instead of `feePct`.
     * This updated fee must have been requested by the `trader` that initiated the trade, by cryptographically signing a message `updatedFeePct`.
     * A trader will usually call `updateFeeForTrade` to signal to relayers the new fee and make its signature available.
     * @param tokenOut The token to send to the requestor.
     * @param tradeIndex The ID of the trade to fill.
     * @param amountToSend The amount of `tokenOut` to send to the requestor.
     * @param trader The address of the trader who initially requested the trade.
     * @param newFeePct The updated fee percentage.
     * @param traderSignature A signed message by the trader that first requested the trade.
     */
    function fillTradeWithUpdatedFee(
        address tokenOut,
        uint256 tradeIndex,
        uint256 amountToSend,
        address trader,
        uint256 newFeePct,
        bytes calldata traderSignature
    ) external {
        // We delegate checking if the trade is already filled to `_verifyFeeUpdateSignature`
        _verifyFeeUpdateSignature(
            trader,
            tradeIndex,
            newFeePct,
            traderSignature
        );
        _fillTrade(tokenOut, tradeIndex, amountToSend);
        // Emit a TradeFilled event with the new fee percentage.
        emit TradeFilled(
            msg.sender,
            tradeIndex,
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
        uint256 tradeIndex
    ) external {
        // Check if the trade has already been settled, is not filled or does not exist.
        if (!_isTradeFilled(tradeIndex)) {
            revert BookSingleChain__TradeNotFilled(tradeIndex);
        }
        FilledTrade memory maybeTrade = filledTrades[tradeIndex];
        if (block.number - maybeTrade.blockHeight < safeBlockThreshold) {
            // Always > 0 for the check above
            uint256 blocksLeft = safeBlockThreshold -
                (block.number - maybeTrade.blockHeight);
            revert BookSingleChain__DisputePeriodNotOver(blocksLeft);
        }
        uint256 amountToTrader = maybeTrade.amountOut;
        address relayer = maybeTrade.relayer;

        delete filledTrades[tradeIndex];

        ERC20(tokenOut).safeTransfer(to, amountToTrader);
        ERC20(tokenIn).safeTransfer(relayer, amountIn);

        emit TradeSettled(relayer, tradeIndex, amountToTrader, feePct);
    }

    /**
     * @notice Called by disputers to dispute a trade.
     * For `safeBlockThreshold` blocks after a trade is submitted off-chain entities can dispute it and claim the relayer's tokens.
     * Dispute resolution is delegated to an external oracle contract.
     * This makes the trade available for relayers to fill again.
     * @param tokenOut The token that was bought.
     * @param feePct The fee percentage. This is to be interpreted as a "distance" from the optimal execution price.
     * @param tradeIndex The ID of the trade to dispute.
     */
    function disputeTrade(
        address tokenOut,
        uint256 feePct,
        uint256 tradeIndex
    ) external nonReentrant {
        // Check that the trade has been filled.
        if (!_isTradeFilled(tradeIndex)) {
            revert BookSingleChain__TradeNotFilled(tradeIndex);
        }
        FilledTrade memory maybeTrade = filledTrades[tradeIndex];
        // Check that the dispute period has not yet ended.
        if (block.number - maybeTrade.blockHeight >= safeBlockThreshold) {
            revert BookSingleChain__DisputePeriodOver();
        }

        uint256 amountSent = maybeTrade.amountOut;
        address relayer = maybeTrade.relayer;
        // Mark the trade as unfilled to allow relayers to fill it again.
        delete filledTrades[tradeIndex];

        bytes32 disputeId = oracle.getRequestId(
            relayer,
            msg.sender,
            tokenOut,
            amountSent
        );
        emit TradeDisputed(relayer, tradeIndex, disputeId, amountSent, feePct);
        // Approve the oracle to spend the amountSent by the relayer.
        ERC20(tokenOut).safeApprove(address(oracle), amountSent);
        oracle.ask(relayer, msg.sender, tokenOut, amountSent);
    }

    /**************************************
     *         INTERNAL FUNCTIONS         *
     **************************************/

    function _verifyFeeUpdateSignature(
        address trader,
        uint256 tradeIndex,
        uint256 newFeePct,
        bytes calldata traderSignature
    ) internal view {
        if (newFeePct > maxFeePct) {
            revert BookSingleChain__FeePctTooHigh(newFeePct);
        }
        if (_isTradeFilled(tradeIndex)) {
            revert BookSingleChain__TradeAlreadyFilled(tradeIndex);
        }

        bytes32 expectedMessageHash = keccak256(
            abi.encode(SIGNATURE_DELIMITER, tradeIndex, newFeePct)
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
        uint256 tradeIndex,
        uint256 amountToSend
    ) internal {
        FilledTrade memory trade = FilledTrade(
            msg.sender,
            amountToSend,
            block.number
        );
        filledTrades[tradeIndex] = trade;

        ERC20(tokenOut).safeTransferFrom(
            msg.sender,
            address(this),
            amountToSend
        );
    }

    /**
     * @notice Called to check if a trade is filled.
     */
    function _isTradeFilled(uint256 tradeIndex) internal view returns (bool) {
        FilledTrade memory trade = filledTrades[tradeIndex];
        return trade.blockHeight > 0;
    }
}
