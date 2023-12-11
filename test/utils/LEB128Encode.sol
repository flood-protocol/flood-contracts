// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

abstract contract LEB128Encode {
    function _encode(uint256 x) internal pure returns (bytes memory result) {
        if (x == 0) return result = new bytes(1);
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(0x40)
            let offset := add(result, 32)
            let i := offset
            for {} 1 {} {
                let nextByte := and(x, 0x7f)
                x := shr(7, x)
                if x {
                    nextByte := or(nextByte, 0x80)
                    mstore8(i, nextByte)
                    i := add(i, 1)
                    continue
                }
                mstore8(i, nextByte)
                i := add(i, 1)
                break
            }
            mstore(result, sub(i, offset))
            mstore(0x40, i)
        }
    }
}
