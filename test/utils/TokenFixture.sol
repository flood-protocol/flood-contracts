// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.15;

import "forge-std/Test.sol";
import "solmate/tokens/ERC20.sol";

address constant MAINNET_USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
address constant MAINNET_WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

contract MockToken is ERC20 {
    constructor(string memory name, string memory symbol, uint8 decimals)
        ERC20(name, symbol, decimals)
    {}
}

contract TokenFixture is Test {
    address internal USDC = deployIfNotEmpty(
        MAINNET_USDC,
        "TokenFixture.sol:MockToken",
        abi.encode("USDC", "USDC", 6)
    );
    address internal WETH = deployIfNotEmpty(
        MAINNET_WETH,
        "TokenFixture.sol:MockToken",
        abi.encode("Wrapped Ether", "WETH", 18)
    );

    // Deploys a contract to `target` if the address has no existing code.
    // This is used to deploy contracts in tests ONLY if you're not forking.
    function deployIfNotEmpty(
        address target,
        string memory whatContract,
        bytes memory args
    )
        internal
        returns (address)
    {
        uint256 existingCode = target.code.length;
        if (existingCode > 0) {
            return target;
        }
        address deployed = deployCode(whatContract, args);

        ERC20(deployed).symbol();
        return deployed;
    }
}
