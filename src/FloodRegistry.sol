// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "solmate/auth/Owned.sol";
import "./AllKnowingOracle.sol";


interface IFloodRegistryEvents {
    event TokenWhitelisted(address indexed token, bool whitelisted);
    event OracleChanged(AllKnowingOracle indexed oracle);
}


error FloodRegistry__TokenNotWhitelisted(address token);
error FloodRegistry__TokenAlreadyWhitelisted(address token);

/**
 * @title FloodRegistry
 * @notice This contract is the central store of information about the Flood Protocol.
 * Here its possible to query which tokens can be used in the protocol, as well as what is the latest oracle used. 
 */
contract FloodRegistry is IFloodRegistryEvents, Owned {
    // Mapping from token address to its index in the tokens array. A token index of 0 means the token is not whitelisted. 
    mapping(address => uint) public tokenIndexes;
    address[] private tokens;

    // The latest oracle used in the protocol.
    AllKnowingOracle public latestOracle;

    constructor() Owned(msg.sender) {}




    /**
     * @notice Whitelists a token. Reverts if trying to add a token that is already whitelisted or if trying to remove a token that is not whitelisted.
     * @param token The token to whitelist.
     * @param enabled Whether the token should be whitelisted or not.
     */
    function whitelistToken(address token, bool enabled) external onlyOwner {
        if (enabled) {
           _addToken(token); 
        } else {
            _removeToken(token);
        }
        emit TokenWhitelisted(token, enabled);
    }


    /**
    * @notice Sets the latest oracle used in the protocol.
    * @param _oracle The oracle to set.
    */
    function setOracle(AllKnowingOracle _oracle) external onlyOwner {
        latestOracle = _oracle;
        emit OracleChanged(_oracle);
    }



    /**
     * @notice Checks if a token is whitelisted. 
     * @param token The token to check.
     * @return True if the token is whitelisted, false otherwise.
     */
    function isTokenWhitelisted(address token) external view returns (bool) {
        return _isTokenWhitelisted(token); 
    }


    function _isTokenWhitelisted(address token) internal view returns (bool) {
        return tokenIndexes[token] != 0;
    }

    function _addToken(address token) internal {
        if (_isTokenWhitelisted(token)) {
            revert FloodRegistry__TokenAlreadyWhitelisted(token);
        }
        tokens.push(token);
        // We add 1 to the index to avoid having a 0 index, which would mean the token is not whitelisted.
        tokenIndexes[token] = tokens.length;
    }

    function _removeToken(address token) internal {
        uint256 index_plus_one = tokenIndexes[token];
        if(index_plus_one == 0) {
            revert FloodRegistry__TokenNotWhitelisted(token);
        }
        // We set the index to 0 to signal to callers that the token is not whitelisted.
        tokenIndexes[token] = 0;
        // We swap the token to remove with the last token in the array.
        tokens[index_plus_one - 1] = tokens[tokens.length - 1];
        tokens.pop();
    }

}
