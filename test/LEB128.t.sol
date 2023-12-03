// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import {Test} from "forge-std/Test.sol";
import {LEB128} from "src/libraries/LEB128.sol";

// Helpers
import {LibBit} from "solady/utils/LibBit.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

contract LEB128Contract {
    fallback(bytes calldata) external returns (bytes memory) {
        (uint256 result, uint256 ptr) = LEB128.rawDecodeUint(0);
        return abi.encode(result, ptr);
    }
}

contract LEB128Test is Test {
    LEB128Contract public leb128;

    function setUp() public {
        leb128 = new LEB128Contract();
    }

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

    function _encodedUintLength(uint256 x) internal pure returns (uint256) {
        return x == 0 ? 1 : FixedPointMathLib.divUp(LibBit.fls(x) + 1, 7);
    }

    function testUnsignedEncode() public {
        assertEq(_encode(uint256(0)), hex"00");
        assertEq(_encode(uint256(1)), hex"01");
        assertEq(_encode(uint256(69)), hex"45");
        assertEq(_encode(uint256(420)), hex"a403");
        assertEq(_encode(uint256(666)), hex"9a05");
        assertEq(_encode(uint256(1 ether)), hex"808090bbbad6adf00d");
        assertEq(
            _encode(type(uint256).max - 1),
            hex"feffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0f"
        );
        assertEq(
            _encode(type(uint256).max),
            hex"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0f"
        );
    }

    function testUnsignedEncodeLength(uint256 x) public {
        vm.assume(x != 0);
        assertEq(_encode(x).length, _encodedUintLength(x));
    }

    function testEncodeDecode(uint256 x) public {
        bytes memory uencoded = _encode(x);

        (bool success, bytes memory result) = address(leb128).call(abi.encodePacked(uencoded));
        require(success);
        (uint256 decoded, uint256 length) = abi.decode(result, (uint256, uint256));
        assertEq(decoded, x);
        assertEq(length, uencoded.length);
    }

    function testNoRevertOnOutOfBoundsRawDecoding(uint256 x) public {
        bytes memory uencoded = _encode(x);

        uint256 uencodedPtr;
        /// @solidity memory-safe-assembly
        assembly {
            uencodedPtr := add(uencoded, 0x20)
        }

        uencoded[uencoded.length - 1] ^= 0x80;

        (bool success,) = address(leb128).call(abi.encodePacked(uencoded));
        require(success);
    }

    function testBenchCompress() public {
        // Test that encoding these values takes 72 bytes.
        uint256[] memory a = new uint256[](21);
        a[0] = 0x0000000000000000000000000000000000000000000000000000000000000020;
        a[1] = 0x000000000000000000000000ca1694433e499862ee242f2f403cb1e73ae91cfb;
        a[2] = 0x0000000000000000000000000000000000000000000000000000000000000001;
        a[3] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[4] = 0x0000000000000000000000001f5d295778796a8b9f29600a585ab73d452acb1c;
        a[5] = 0x0000000000000000000000000000000000000000000000000000000000000001;
        a[6] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[7] = 0x00000000000000000000000000000000000000000000000000000000ffffffff;
        a[8] = 0x0000000000000000000000000000000000000000000000000000000000000200;
        a[9] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[10] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[11] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[12] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[13] = 0x0000000000000000000000000000000000000000000000000000000000000220;
        a[14] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[15] = 0x0000000000000000000000000000000000000000000000000000000000000260;
        a[16] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[17] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[18] = 0x0000000000000000000000000000000000000000000000000000000000000014;
        a[19] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        a[20] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        bytes memory encodedData;
        for (uint256 i; i < a.length; ++i) {
            encodedData = abi.encodePacked(encodedData, _encode(a[i]));
        }
        assertEq(encodedData.length, 72);
    }
}
