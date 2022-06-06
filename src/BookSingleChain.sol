// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solmate/auth/Owned.sol";
import "solmate/utils/SafeTransferLib.sol";

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
    // The amount of blocks in which a trade can be disputed.
    uint256 public safeBlockThreshold;
    // A mapping with the tokens that are supported by this contract.
    mapping(address => bool) public whitelistedTokens;
    // A mapping from a trade id to a boolean indicating wether the trade has been filled.
    mapping(bytes32 => bool) public isFilled;
    // A mapping from a trade id to a boolean indicating who filled the trade.
    mapping(bytes32 => uint256) public filledAtBlock;
    // A mapping from a trade id to the relayer filling it.
    mapping(bytes32 => address) public filledBy;

    event TradeRequested(
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 feePct,
        uint256 amount,
        address to,
        uint256 indexed tradeIndex
    );
    event SafeBlockThresholdChanged(uint256 newSafeBlockThreshold);
    event TokenWhitelisted(address indexed token, bool whitelisted);

    /**
     * @notice Constructs the order book.
     * @param _safeBlockThreshold The number of blocks in which a trade can be disputed.
     */
    constructor(uint256 _safeBlockThreshold) Owned(msg.sender) {
        safeBlockThreshold = _safeBlockThreshold;
        emit SafeBlockThresholdChanged(safeBlockThreshold);
    }

    /**************************************
     *          ADMIN FUNCTIONS           *
     **************************************/

    /**
     * @notice Changes the safe block threshold.
     * @param newSafeBlockThreshold The new safe block threshold.
     */
    function setSafeBlockThreshold(uint256 newSafeBlockThreshold)
        public
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
    function whitelistToken(address token, bool whitelisted) public onlyOwner {
        whitelistedTokens[token] = whitelisted;
        emit TokenWhitelisted(token, whitelisted);
    }
}
