// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;

import "solmate/auth/Owned.sol";
import "solmate/tokens/ERC20.sol";
import "solmate/utils/SafeTransferLib.sol";

error AllKnowingOracle__AlreadySettled(bytes32 id);
error AllKnowingOracle__RequestAlreadyExists(bytes32 id);
error AllKnowingOracle__NotWhitelisted(address token);
error AllKnowingOracle__BondTooSmall();

enum RequestState {
    Uninitialized,
    Pending,
    Settled
}

struct Request {
    address proposer;
    address disputer;
    address bondToken;
    uint256 bond;
    uint256 stake;
    bool answer;
    RequestState state;
}

interface IOracle {
    function ask(
        address proposer,
        address disputer,
        address bondToken,
        uint256 bond,
        uint256 stake
    ) external returns (bytes32);

    function settle(bytes32 id, bool answer) external;

    function bondForStake(uint256 stake) external view returns (uint256);
}

/**
 * @title All Knowing Oracle
 * @notice This contract is used to test the functionality of the `BookSingleChain` contract.
 * In implementation, only the owner of the contract can settle a request.
 * @dev This should be deployed before `BookSingleChain`
 */
contract AllKnowingOracle is IOracle, Owned {
    using SafeTransferLib for ERC20;

    mapping(bytes32 => Request) public requests;
    mapping(address => bool) public whitelistedTokens;
    // Percentage of stake which must be bonded by disputer, in % points (1%)
    uint256 public disputeBondPct;

    /****************************************
     *                EVENTS                *
     ****************************************/

    event TokenWhitelisted(address indexed token, bool enabled);
    event NewRequest(
        bytes32 indexed id,
        address indexed proposer,
        address indexed disputer,
        address bondToken,
        uint256 bond,
        uint256 stake
    );
    event BondPctChanged(uint256 newPct);
    event RequestSettled(bytes32 indexed id, bool answer);

    constructor() Owned(msg.sender) {
        disputeBondPct = 25;
        emit BondPctChanged(disputeBondPct);
    }

    function setBondPct(uint256 _pct) public onlyOwner {
        disputeBondPct = _pct;
        emit BondPctChanged(disputeBondPct);
    }

    /**************************************
     *          ADMIN FUNCTIONS           *
     **************************************/

    /**
     @notice Whitelist a token for use in the contract.
     @param token Token to whitelist
     @param enabled Whether to enable or disable the token
    */
    function whitelistToken(address token, bool enabled) public onlyOwner {
        whitelistedTokens[token] = enabled;
    }

    /**
     * @notice Settles a request. Only the owner of the contract can call this function.
     * @param id ID of the request to settle
     * @param answer Whether the proposer won the dispute. `true` means the proposer won, `false` means the disputer won.
     */
    function settle(bytes32 id, bool answer) external onlyOwner {
        Request memory request = requests[id];
        // revert if the request is already settled
        if (request.state == RequestState.Settled) {
            revert AllKnowingOracle__AlreadySettled(id);
        }
        // If the answer is correct, the proposer wins the bond
        if (answer == true) {
            ERC20(request.bondToken).transfer(
                request.proposer,
                request.bond + request.stake
            );
        } else {
            ERC20(request.bondToken).transfer(
                request.disputer,
                request.bond + request.stake
            );
        }
        // Update the request state
        request.state = RequestState.Settled;
        request.answer = answer;
        requests[id] = request;

        emit RequestSettled(id, answer);
    }

    /**
     * @notice Calculates the bond amount for a given stake.
     * @param stake Stake to calculate the bond amount for
     * @return The bond amount for the given stake
     */
    function bondForStake(uint256 stake) public view returns (uint256) {
        return _bondForStake(stake);
    }

    /**************************************
     *          EXTERNAL FUNCTIONS         *
     **************************************/

    /**
     * @notice Ask the oracle for an answer to a dispute.
     * @dev The bond is transferred from `msg.sender` rather than from `proposer` to allow contracts to sponsor proposals.
     * For example, in `BookSingleChain`, the relayer has already sent funds to the contract, so they are pulled from the contract directly and the relayer is set as proposer.
     * @param proposer Address of the proposer
     * @param disputer Address of the disputer
     * @param bondToken Token to use for the bond
     * @param bond Amount which must be posted by the disputer
     * @param stake Stake to use for the dispute
     * @return ID of the request
     */
    function ask(
        address proposer,
        address disputer,
        address bondToken,
        uint256 bond,
        uint256 stake
    ) external returns (bytes32) {
        // Check if the token is whitelisted
        if (!whitelistedTokens[bondToken]) {
            revert AllKnowingOracle__NotWhitelisted(bondToken);
        }
        // Check if the bond is sufficient
        if (bond < _bondForStake(stake)) {
            revert AllKnowingOracle__BondTooSmall();
        }

        // Generate a unique id
        bytes32 id = keccak256(
            abi.encode(proposer, disputer, bondToken, bond, stake)
        );
        // Check if the request already exists
        if (requests[id].state == RequestState.Pending) {
            revert AllKnowingOracle__RequestAlreadyExists(id);
        }

        // Create the request
        Request memory request = Request(
            proposer,
            disputer,
            bondToken,
            bond,
            stake,
            false,
            RequestState.Pending
        );

        // Store the request
        requests[id] = request;

        // transfer tokens from disputer and proposer
        ERC20(bondToken).safeTransferFrom(disputer, address(this), bond);
        ERC20(bondToken).safeTransferFrom(msg.sender, address(this), stake);

        emit NewRequest(id, proposer, disputer, bondToken, bond, stake);

        return id;
    }

    /**************************************
     *          INTERNAL FUNCTIONS         *
     **************************************/

    /**
     * @notice Calculates the bond amount for a given stake.
     */
    function _bondForStake(uint256 stake) internal view returns (uint256) {
        return (stake * disputeBondPct) / 100;
    }
}
