// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

interface IEncodedCalls {
    event DecoderAdded(uint256 indexed decoderId, address indexed decoder);

    /**
     * @notice Declares a byte that does not clash with the first byte of any function signature.
     */
    function FALLBACK_SELECTOR() external view returns (bytes1);

    /**
     * @notice Get the decoder address corresponding to an identifier.
     *
     * @param // The identifier of the decoder.
     *
     * @return decoder The address of the decoder.
     */
    function decoders(uint256) external view returns (address decoder);

    /**
     * @notice Add a decoder address to the decoders array.
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
     *      that such a unique byte exists for a contract with less than 256 external functions.
     *      Starting with the second byte must be the Unsigned LEB128 encoded identifier of the
     *      decoder. The identifier maps to a decoder address. The remaining calldata is then
     *      passed to the decoder as staticcall, and returndata is delegatecalled to this contract.
     */
    fallback() external;
}
