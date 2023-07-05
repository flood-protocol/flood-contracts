// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IFloodPlainEncodedCalls {
    event DecoderAdded(uint256 indexed decoderId, address indexed decoder);

    /**
     * @notice Get the decoder address corresponding to an identifier.
     *
     * @param decoderId The identifier of the decoder.
     *
     * @return decoder The address of the decoder.
     */
    function getDecoder(uint256 decoderId) external view returns (address decoder);

    /**
     * @notice Add a decoder address to the decoders array.
     *
     * @dev Decoders above identifier 255 will not be accessible. Be mindful when adding decoders.
     *
     * @param decoder The address of the decoder.
     *
     * @return decoderId The identifier of the new decoder added.
     */
    function addDecoder(address decoder) external returns (uint256 decoderId);

    /**
     * @notice Decode an arbitrarily encoded calldata to have reduced calldata length in L2s.
     *
     * @dev The first byte of the calldata is not used. The first byte should be any byte that has
     *      no match with the first byte of a function selector in the contract. It is guaranteed
     *      that such a unique byte exists for a contract with less than 257 external functions.
     *      The second byte should be the identifier of the decoder to decode any arbitrary bytes
     *      passed after the second byte.
     */
    fallback() external;
}
