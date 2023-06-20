// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "test/flood-plain/utils/FloodPlainTestShared.sol";

import {IFloodPlainEncodedCalls} from "src/flood-plain/extensions/IFloodPlainEncodedCalls.sol";

contract FloodPlainEncodedCalls is FloodPlainTestShared {
    function test_addDecoder() public {
        // Add a decoder.
        uint256 decoderId = book.addDecoder(address(decoder));
        assertEq(decoderId, 0);

        decoderId = book.addDecoder(address(decoder));
        assertEq(decoderId, 1);

        decoderId = book.addDecoder(address(decoder));
        assertEq(decoderId, 2);

        decoderId = book.addDecoder(address(decoder));
        assertEq(decoderId, 3);
    }

    function test_getDecoder() public {
        // Add a decoder.
        uint256 decoderId = book.addDecoder(address(decoder));
        address decoderAddress = book.getDecoder(decoderId);
        assertEq(decoderAddress, address(decoder));

        decoderId = book.addDecoder(address(this));
        decoderAddress = book.getDecoder(decoderId);
        assertEq(decoderAddress, address(this));

        decoderId = book.addDecoder(address(book));
        decoderAddress = book.getDecoder(decoderId);
        assertEq(decoderAddress, address(book));
    }

    function test_fufillThroughDecoder() public {
        book.addDecoder(address(decoder));

        (IFloodPlain.Order memory order, ) = setup_mostBasicOrder();
        order.deadline = type(uint32).max;
        bytes memory sig = getSignature(order, account0);

        (bytes32 r, bytes32 s) = abi.decode(sig, (bytes32, bytes32));
        bytes1 v = sig[64];

        bytes memory encodedDataBegin = abi.encodePacked(
            bytes1(0x00), // fallback selector
            bytes1(0x00), // decoder id
            order.offerer,
            order.zone,
            uint24(order.nonce),
            uint32(order.deadline),
            address(fulfiller),
            v,
            r,
            s
        );

        bytes memory encodedDataEnd = abi.encodePacked(
            bytes1(0x11), // (0001, 0001)
            address(address(token0)),
            uint112(500),
            address(address(token1)),
            uint112(500)
        );

        bytes memory encodedData = bytes.concat(encodedDataBegin, encodedDataEnd);

        address bookAddress = address(book);

        bool result;
        assembly {
            result := call(gas(), bookAddress, 0, add(encodedData, 0x20), encodedData, 0, 0)
        }
        assertTrue(result);

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
    }

    function test_RevertWhenAddInvalidDecoder() public {
        vm.expectRevert(bytes4(keccak256("NotAContract()")));
        book.addDecoder(address(0xd00d));
    }
}
