// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

// Inheritances
import {IEncodedCalls} from "./interfaces/IEncodedCalls.sol";

// Libraries
import {LEB128Lib} from "leb128/LEB128Lib.sol";

abstract contract EncodedCalls is IEncodedCalls {
    address[] public decoders;

    function addDecoder(address decoder) external returns (uint256 id) {
        id = decoders.length;
        decoders.push(decoder);
        emit DecoderAdded(id, decoder);
    }

    fallback() external {
        // First byte of the calldata is not used. Any first byte that does not clash with
        // other function signatures can be used to enter fallback. From `forge inspect
        // FloodPlainComplete methodIdentifiers` one can find unused first bytes. We test this to
        // ensure such byte exists.

        // Get the decoded decoder ID, and the new calldata ptr.
        (uint256 decoderId, uint256 ptr) = LEB128Lib.rawDecodeUint({ptr: 1});

        // Get the decoder.
        address decoder = decoders[decoderId];

        assembly {
            // Move the effective calldata size to stack.
            let trimmedCalldataSize := sub(calldatasize(), ptr)

            // Check for underflow.
            if gt(trimmedCalldataSize, calldatasize()) { revert(0, 0) }

            // Copy calldata starting from the end of decoder id.
            calldatacopy(0, ptr, trimmedCalldataSize)

            // Staticcall the decoder to get the decoded data.
            let result := staticcall(gas(), decoder, 0, trimmedCalldataSize, 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            // Revert with return data if the call failed.
            if iszero(result) { revert(0, returndatasize()) }

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
