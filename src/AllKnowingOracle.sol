// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "solmate/auth/Owned.sol";
import "solmate/tokens/ERC20.sol";
import "solmate/utils/SafeTransferLib.sol";


error AllKnowingOracle__AlreadySettled(bytes32 id);
error AllKnowingOracle__NonSettler();
error AllKnowingOracle__NonRequester();
error AllKnowingOracle__RequestAlreadyExists(bytes32 id);
error AllKnowingOracle__BondTooSmall();

enum RequestState {
    Uninitialized,
    Pending,
    Settled
}

struct Request {
    address requester; // who requested an answer.
    address proposer; // Address of the proposer.
    address disputer; // Address of the disputer.
    ERC20 currency; // ERC20 token used to pay rewards and fees.
    uint256 bond; // Bond that the proposer and disputer must pay.
    RequestState state; // State of the request.
    bool answer; // Answer to the request.
    bytes data; // Data associated with the request.
}

interface IOptimisticRequester {
    function onPriceSettled(bytes32 id, Request calldata request) external;
}

/**
 *
 *                EVENTS                *
 *
 */
interface IAllKnowingOracleEvents {
    event TokenWhitelisted(address indexed token, bool enabled);
    event SettlerWhitelisted(address indexed settler, bool enabled);
    event RequesterWhitelisted(address indexed requester, bool enabled);
    event NewRequest(
        bytes32 indexed id, address indexed proposer, address indexed disputer, address currency, uint256 bond
    );
    event RequestSettled(bytes32 indexed id, bool answer);
}

/**
 * @title All Knowing Oracle
 * @notice This contract is used to test the functionality of the `BookSingleChain` contract.
 * In implementation, only the owner of the contract can settle a request.
 * @dev This should be deployed before `BookSingleChain`
 */
contract AllKnowingOracle is IAllKnowingOracleEvents, Owned {
    using SafeTransferLib for ERC20;

    mapping(bytes32 => Request) public requests;
    mapping(address => bool) public whitelistedTokens;
    mapping(address => bool) public settlers;
    mapping(address => bool) public requesters;
  

    modifier onlySettler() {
        if (!settlers[msg.sender]) {
            revert AllKnowingOracle__NonSettler();
        }
        _;
    }

    modifier onlyRequester() {
        if (!requesters[msg.sender]) {
            revert AllKnowingOracle__NonRequester();
        }
        _;
    }

    constructor() Owned(msg.sender) {
        settlers[msg.sender] = true;
    }

    /**
     *
     *          ADMIN FUNCTIONS           *
     *
     */
    function whitelistSettler(address settler, bool enabled) external onlyOwner {
        settlers[settler] = enabled;
        emit SettlerWhitelisted(settler, enabled);
    }

    function whitelistRequester(address requester, bool enabled) external onlyOwner {
        requesters[requester] = enabled;
        emit RequesterWhitelisted(requester, enabled);
    }

    /**
     *
     *          EXTERNAL FUNCTIONS         *
     *
     */

    /**
     * @notice Compute the ID for a request.
     * @dev The bond is transferred from `msg.sender` rather than from `proposer` to allow contracts to sponsor proposals.
     * For example, in `BookSingleChain`, the relayer has already sent funds to the contract, so they are pulled from the contract directly and the relayer is set as proposer.
     * @param sender value of `msg.sender` when the request is created
     * @param proposer Address of the proposer
     * @param disputer Address of the disputer
     * @param currency Token to use for the bond
     * @param bond Value of the bond
     * @return ID of the request
     */
    function getRequestId(address sender, address proposer, address disputer, address currency, uint256 bond)
        external
        pure
        returns (bytes32)
    {
        return _getRequestId(sender, proposer, disputer, currency, bond);
    }

    /**
     * @notice Requests and proposes a price to the oracle. Disputers should set their allowance at each dispute to safely pay the bond.
     * @dev The bond proposer bond is transferred from `msg.sender` rather than from `proposer` to allow contracts to sponsor proposals.
     * For example, in `BookSingleChain`, the relayer has already sent funds to the contract, so they are pulled from the contract directly and the relayer is set as proposer.
     * @param proposer Address of the proposer
     * @param disputer Address of the disputer
     * @param currency Token to use for the bond
     * @param bond Bond value which must be posted to dispute
     */
    function ask(address proposer, address disputer, address currency, uint256 bond, bytes calldata data)
        external
        onlyRequester
        returns (bytes32 id)
    {
        id = _getRequestId(msg.sender, proposer, disputer, currency, bond);
        if (requests[id].state != RequestState.Uninitialized) {
            revert AllKnowingOracle__RequestAlreadyExists(id);
        }
        Request memory request = Request({
            requester: msg.sender,
            proposer: proposer,
            disputer: disputer,
            currency: ERC20(currency),
            bond: bond,
            state: RequestState.Pending,
            answer: false,
            data: data
        });

        requests[id] = request;

        emit NewRequest(id, proposer, disputer, currency, bond);

        ERC20(currency).safeTransferFrom(msg.sender, address(this), bond);
        // Note: This is unsafe for the disputer as a requester could list someone as disputer and give the right answer, making the disputer lose the bond.
        // However, as this method is permissioned and requesters are assumed to be trustworthy, this is "safe".
        ERC20(currency).safeTransferFrom(disputer, address(this), bond);
    }

    /**
     * @notice Settles a request. Only a whitelist `settler` can call this function.
     * @param id ID of the request to settle
     * @param answer Whether the proposer won the dispute. `true` means the proposer won, `false` means the disputer won.
     */
    function settle(bytes32 id, bool answer) external onlySettler {
        Request storage request = requests[id];
        // revert if the request is already settled
        if (request.state == RequestState.Settled) {
            revert AllKnowingOracle__AlreadySettled(id);
        }
        // Whoever wins gets the bond back plus the other party bond.
        uint256 payout = 2 * request.bond;
        if (answer) {
            request.currency.safeTransfer(request.proposer, payout);
        } else {
            request.currency.safeTransfer(request.disputer, payout);
        }
        // Update the request state
        request.state = RequestState.Settled;
        request.answer = answer;

        emit RequestSettled(id, answer);
        // Callback into the proposer if its a contract
        if (request.requester.code.length != 0) {
            IOptimisticRequester(request.requester).onPriceSettled(id, request);
        }
    }

    /**
     *
     *          INTERNAL FUNCTIONS         *
     *
     */

    function _getRequestId(address requester, address proposer, address disputer, address currency, uint256 bond)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(requester, proposer, disputer, currency, bond));
    }
}
