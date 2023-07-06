// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {Test} from "forge-std/Test.sol";
import {DecoderWithRegistry} from "src/decoders/extensions/DecoderWithRegistry.sol";
import {FloodPlainTestShared, IFloodPlain} from "../utils/FloodPlainTestShared.sol";

contract DecoderWithRegistryTest is FloodPlainTestShared {
    DecoderWithRegistry registryDecoder;

    function setUp() public override {
        super.setUp();
        registryDecoder = new DecoderWithRegistry();
        registryDecoder.setToken(0, address(token0));
        registryDecoder.setToken(1, address(token1));
        registryDecoder.setZone(0, address(0));
        registryDecoder.setFulfiller(0, address(fulfiller));
        book.addDecoder(address(registryDecoder));
        assertEq(book.getDecoder(0), address(registryDecoder));
    }

    function test_fulfillWithRegistryDecoder() public {
        (IFloodPlain.Order memory order, bytes memory sig) = setup_mostBasicOrder();
        bytes memory encodedDataBegin = abi.encodePacked(
            bytes1(0x00), // fallback selector
            bytes1(0x00), // decoder id
            order.offerer,
            bytes1(0x00) // zone id
        );
        uint8 nonceAndNibbleByteSizes;
        {
            // nonce is 0 --> 1 byte
            uint256 nonceSize = 1;
            // deadline is type(uint).max --> 16 bytes
            uint256 deadlineSize = 16;
            nonceAndNibbleByteSizes = uint8(((nonceSize - 1) << 4) | (deadlineSize - 1));
        }
        // Following this decoder encoding scheme, we first have 1 byte for the nonce and nibble byte sizes.
        bytes memory encodedNonceAndDeadline = abi.encodePacked(
            nonceAndNibbleByteSizes,
            uint8(order.nonce),
            order.deadline,
            bytes1(0x00) // fulfiller id
        );
        // Since there can't be 0 items in offer or consideration, we scale the actual number by 1 so that a nibble of value 0 is 1 item.
        uint8 offerAndConsiderationLength = uint8((order.offer.length - 1) << 4 | (order.consideration.length - 1));
        bytes memory encodedDataOfferAndConsideration = abi.encodePacked(
            offerAndConsiderationLength, // number of items in offer and consideration
            bytes2(0x0000), // offer item 0 is token0, which has id 0
            bytes1(0x01), // offer item 0 amount is 500, so its 2 bytes long. Same principle as 0 byte items at work here.
            uint16(500),
            bytes2(0x0001), // offer item 1 is token1, which has id 1
            bytes1(0x01), // offer item 1 amount is 500, so its 2 bytes long.
            uint16(500)
        );

        (bytes32 r, bytes32 s) = abi.decode(sig, (bytes32, bytes32));
        bytes1 v = sig[64];

        bytes memory encodedData =
            bytes.concat(encodedDataBegin, encodedNonceAndDeadline, encodedDataOfferAndConsideration, v, r, s);

        (bool result,) = address(book).call(encodedData);
        assertTrue(result);

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
    }
}
