// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IDecoder {
    /**
     * @notice Takes an encoded calldata, and decodes it based on whatever arbitrary encoding
     *         scheme it uses.
     *
     * @dev If there are any other external functions in Decoder, a unique first byte of calldata
     *      must be used for triggering fallback.
     */
    fallback() external; /* view */
}
