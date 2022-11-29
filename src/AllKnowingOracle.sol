// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./FloodRegistry.sol";
import "@openzeppelin/token/ERC20/IERC20.sol";
import "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/access/Ownable2Step.sol";

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
    IERC20 currency; // ERC20 token used to pay rewards and fees.
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
contract AllKnowingOracle is IAllKnowingOracleEvents, Ownable2Step {
    using SafeERC20 for IERC20;

    mapping(bytes32 => Request) public requests;
    mapping(address => bool) public settlers;
    FloodRegistry public immutable registry;

    modifier onlySettler() {
        if (!settlers[msg.sender]) {
            revert AllKnowingOracle__NonSettler();
        }
        _;
    }

    constructor(FloodRegistry _registry) {
        settlers[msg.sender] = true;
        registry = _registry;
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
     * For example, in `BookSingleChain`, relayer and disputer have already sent funds to the contract, so they are pulled from the contract directly and the relayer is set as proposer.
     * @param proposer Address of the proposer
     * @param disputer Address of the disputer
     * @param currency Token to use for the bond
     * @param bond Bond value which must be posted to dispute
     */
    function ask(address proposer, address disputer, address currency, uint256 bond, bytes calldata data)
        external
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
            currency: IERC20(currency),
            bond: bond,
            state: RequestState.Pending,
            answer: false,
            data: data
        });

        requests[id] = request;

        emit NewRequest(id, proposer, disputer, currency, bond);

        IERC20(currency).safeTransferFrom(msg.sender, address(this), 2 * bond);
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
        // Update the request state
        request.state = RequestState.Settled;
        request.answer = answer;
        emit RequestSettled(id, answer);

        if (answer) {
            request.currency.safeTransfer(request.proposer, payout);
        } else {
            request.currency.safeTransfer(request.disputer, payout);
        }

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
