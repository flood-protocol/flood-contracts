// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./utils/FloodPlainTestShared.sol";

import {EncodedCalls} from "src/EncodedCalls.sol";
import {IEncodedCalls} from "src/interfaces/IEncodedCalls.sol";

import {LEB128Lib} from "leb128/LEB128Lib.sol";

contract EncodedCallsContract is EncodedCalls {
    uint256 public val;
    bytes1 public constant FALLBACK_SELECTOR = 0x00;

    function changeVal(uint256 a) external {
        val = a;
    }
}

contract MockDecoder {
    fallback(bytes calldata data) external returns (bytes memory) {
        uint256 ptr;
        assembly {
            ptr := data.offset
        }
        (uint256 a, uint256 newPtr) = LEB128Lib.rawDecodeUint(ptr);
        require(data.length >= newPtr - ptr, "DECODING_ERROR");
        return abi.encodeWithSelector(EncodedCallsContract.changeVal.selector, a);
    }
}

contract EncodedCallsTest is FloodPlainTestShared {
    EncodedCallsContract encodedCalls;
    MockDecoder decoder;

    function setUp() public override {
        encodedCalls = new EncodedCallsContract();
        decoder = new MockDecoder();
        super.setUp();
    }

    function test_addDecoder() public {
        // Add a decoder.
        uint256 decoderId = encodedCalls.addDecoder(address(0x69));
        assertEq(decoderId, 0);
        assertEq(encodedCalls.decoders(0), address(0x69));

        decoderId = encodedCalls.addDecoder(address(0x70));
        assertEq(decoderId, 1);
        assertEq(encodedCalls.decoders(1), address(0x70));

        decoderId = encodedCalls.addDecoder(address(0x71));
        assertEq(decoderId, 2);
        assertEq(encodedCalls.decoders(2), address(0x71));

        decoderId = encodedCalls.addDecoder(address(0x72));
        assertEq(decoderId, 3);
        assertEq(encodedCalls.decoders(3), address(0x72));
    }

    function test_useDecoder(uint8 decoders, uint256 a) public {
        assertEq(encodedCalls.val(), 0);

        for (uint256 i; i < decoders; ++i) {
            encodedCalls.addDecoder(address(0x69));
        }
        uint256 decoderId = encodedCalls.addDecoder(address(decoder));
        bytes memory callData = abi.encodePacked(hex"00", LEB128Lib.encode(decoderId), LEB128Lib.encode(a));
        (bool success, bytes memory data) = address(encodedCalls).call(callData);

        assertTrue(success);
        assertEq(data.length, 0);

        assertEq(encodedCalls.val(), a);
    }

    function test_invalidDecoderId() public {
        encodedCalls.addDecoder(address(decoder));
        uint256 val = 69;
        bytes memory callData = abi.encodePacked(hex"00", LEB128Lib.encode(val), LEB128Lib.encode(val));
        (bool success, bytes memory data) = address(encodedCalls).call(callData);
        assertFalse(success);
        assertEq(data, abi.encodeWithSignature("Panic(uint256)", 0x32));
    }

    function test_emptyCalldata() public {
        encodedCalls.addDecoder(address(decoder));
        (bool success, bytes memory data) = address(encodedCalls).call("");
        assertFalse(success);
        assertEq(data, hex"");
    }

    function test_decoderError() public {
        uint256 decoderId = encodedCalls.addDecoder(address(decoder));
        bytes memory callData = abi.encodePacked(hex"00", LEB128Lib.encode(decoderId));
        (bool success, bytes memory data) = address(encodedCalls).call(callData);
        assertFalse(success);
        assertEq(data, abi.encodeWithSignature("Error(string)", "DECODING_ERROR"));
    }

    function test_methodError() public {
        uint256 decoderId = encodedCalls.addDecoder(address(decoder));
        bytes memory callData = abi.encodePacked(hex"00", LEB128Lib.encode(decoderId));
        (bool success, bytes memory data) = address(encodedCalls).call(callData);
        assertFalse(success);
        assertEq(data, abi.encodeWithSignature("Error(string)", "DECODING_ERROR"));
    }

    function test_NoSelectorClash() public {
        string[] memory inputs = new string[](3);
        inputs[0] = "sh";
        inputs[1] = "test/utils/selector_check.sh";
        inputs[2] = vm.toString(uint8(book.FALLBACK_SELECTOR()));

        bytes memory res = vm.ffi(inputs);

        // False in raw hexcode. Means does not match any existing selector.
        assertEq(res.length, 5);
        assertEq(uint8(res[0]), uint8(0x66));
        assertEq(uint8(res[1]), uint8(0x61));
        assertEq(uint8(res[2]), uint8(0x6c));
        assertEq(uint8(res[3]), uint8(0x73));
        assertEq(uint8(res[4]), uint8(0x65));
    }
}
