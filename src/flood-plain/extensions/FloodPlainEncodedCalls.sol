// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {FloodPlain} from "../FloodPlain.sol";
import {IFloodPlainEncodedCalls} from "./IFloodPlainEncodedCalls.sol";
import {Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";

abstract contract FloodPlainEncodedCalls is FloodPlain, IFloodPlainEncodedCalls, Ownable2Step {
    address[] internal _decoders;

    function getDecoder(uint256 decoderId) external view returns (address /* decoder */ ) {
        return _decoders[decoderId];
    }

    function addDecoder(address decoder) external onlyOwner returns (uint256 decoderId) {
        if (decoder.code.length == 0) {
            revert NotAContract();
        }

        decoderId = _decoders.length;
        _decoders.push(decoder);
        emit DecoderAdded(decoderId, decoder);
    }

    fallback() external {
        // The first byte is ignored. It should not match the first byte of any other function
        // selector in the contract to guarantee the fallback is executed. The second byte is the
        // decoder ID. A decoder can employ any decoding scheme.
        uint256 decoderId;
        assembly {
            // Get the decoder identifier from the second byte of calldata.
            decoderId := shr(248, calldataload(0x01))
        }

        // Get the decoder address by its identifier from the decoders mapping.
        address decoder = _decoders[decoderId];

        assembly {
            // Move the effective calldata size to stack.
            let trimmedCalldataSize := sub(calldatasize(), 0x02)

            // Check for underflow.
            if gt(trimmedCalldataSize, calldatasize()) {
                revert(0, 0)
            }

            // Copy calldata starting from third byte to the memory.
            calldatacopy(0x00, 0x02, trimmedCalldataSize)

            // Staticcall the decoder to get the decoded data.
            let result := staticcall(gas(), decoder, 0, trimmedCalldataSize, 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            // Revert with return data if the call failed.
            if iszero(result) {
                revert(0, returndatasize())
            }

            // Delegatecall to this address with the decoded data.
            result := delegatecall(gas(), address(), 0, returndatasize(), 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            switch result
            // delegatecall returns 0 on error.
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }
}
