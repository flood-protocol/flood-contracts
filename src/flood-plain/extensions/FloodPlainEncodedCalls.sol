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
        // First byte of the calldata is not used. Any first byte that does not clash with other
        // function signatures can be used to enter fallback. From the `forge inspect FloodPlainL2
        // methodIdentifiers`, we see that `0x00` is available. We suggest using that since zeroes
        // in calldata is cheaper.
        //
        // "PERMIT2()": "6afdd850",
        // "acceptOwnership()": "79ba5097",
        // "addDecoder(address)": "9d481b66",
        // "etchOrder(((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),bytes))": "1d5473a2",
        // "fulfillEtchedOrder(uint256,address,bytes)": "a15e7907",
        // "fulfillOrder((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),bytes)": "6f01c5e2",
        // "fulfillOrder((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),bytes,address,bytes)": "064d5bc3",
        // "getDecoder(uint256)": "e77876cc",
        // "getEtchedOrder(uint256)": "4d599400",
        // "getNonceStatus(address,uint256)": "e9ba1e97",
        // "getOrderHash((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256))": "1b8b792c",
        // "getOrderStatus((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256))": "093de1d2",
        // "getOrderValidity((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),address)": "fcb0caf2",
        // "getOrderValidity((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),address,address,bytes)": "a77dd3e4",
        // "getPermitHash((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256))": "729d540d",
        // "owner()": "8da5cb5b",
        // "pendingOwner()": "e30c3978",
        // "renounceOwnership()": "715018a6",
        // "transferOwnership(address)": "f2fde38b"

        // The second byte is the decoder ID. A decoder can employ any decoding scheme.
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
            if gt(trimmedCalldataSize, calldatasize()) { revert(0, 0) }

            // Copy calldata starting from third byte to the memory.
            calldatacopy(0x00, 0x02, trimmedCalldataSize)

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
