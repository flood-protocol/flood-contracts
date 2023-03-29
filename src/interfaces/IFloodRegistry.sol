// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

interface IFloodRegistry {
    function isAddressWhitelisted(address token) external view returns (bool);
    function whitelistToken(address token, bool isWhitelisted) external;
    function batchWhitelistTokens(address[] memory tokens, bool[] memory isWhitelisted) external;
}
