// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {Test} from "forge-std/Test.sol";
import {DecoderWithRegistry} from "src/decoders/extensions/DecoderWithRegistry.sol";
import {FloodPlainTestShared, IFloodPlain} from "../utils/FloodPlainTestShared.sol";

contract DecoderWithRegistryTest is FloodPlainTestShared {
    DecoderWithRegistry registryDecoder;

    function setUp() public override {
        super.setUp();
        registryDecoder = new DecoderWithRegistry(address(this));
        registryDecoder.setToken(0, address(token0));
        registryDecoder.setToken(1, address(token1));
        registryDecoder.setZone(0, address(0));
        registryDecoder.setFulfiller(0, address(fulfiller));
        book.addDecoder(address(registryDecoder));
        assertEq(book.getDecoder(0), address(registryDecoder));
    }

    function test_fulfillWithRegistryDecoder() public {
        IFloodPlain.SignedOrder memory signedOrder = setup_mostBasicOrder();
        signedOrder.order.deadline = uint256(type(uint128).max);
        signedOrder.signature = getSignature(signedOrder.order, account0);
        bytes memory encodedDataBegin = abi.encodePacked(
            bytes1(0x00), // fallback selector
            bytes1(0x00), // decoder id
            signedOrder.order.offerer,
            bytes1(0x00) // zone id
        );
        uint8 nonceAndNibbleByteSizes;
        {
            // nonce is 0 --> 1 byte
            uint256 nonceSize = 1;
            // deadline is type(uint128).max --> 16 bytes
            uint256 deadlineSize = 16;
            nonceAndNibbleByteSizes = uint8(((nonceSize - 1) << 4) | (deadlineSize - 1));
        }
        // Following this decoder encoding scheme, we first have 1 byte for the nonce and nibble byte sizes.
        bytes memory encodedNonceAndDeadline = abi.encodePacked(
            nonceAndNibbleByteSizes,
            uint8(signedOrder.order.nonce),
            uint128(signedOrder.order.deadline),
            bytes1(0x00) // fulfiller id
        );
        // Since there can't be 0 items in offer or consideration, we scale the actual number by 1 so that a nibble of value 0 is 1 item.
        uint8 offerAndConsiderationLength = uint8((signedOrder.order.offer.length - 1) << 4 | (signedOrder.order.consideration.length - 1));
        bytes memory encodedDataOfferAndConsideration = abi.encodePacked(
            offerAndConsiderationLength, // number of items in offer and consideration
            bytes2(0x0000), // offer item 0 is token0, which has id 0
            bytes1(0x01), // offer item 0 amount is 500, so its 2 bytes long. Same principle as 0 byte items at work here.
            uint16(500),
            bytes2(0x0001), // offer item 1 is token1, which has id 1
            bytes1(0x01), // offer item 1 amount is 500, so its 2 bytes long.
            uint16(500)
        );

        (bytes32 r, bytes32 s) = abi.decode(signedOrder.signature, (bytes32, bytes32));
        bytes1 v = signedOrder.signature[64];

        bytes memory extraData =
            hex"016b0100020152dbd72e6903410697b403000f015503ab72aaea8978c1105ec15e04001401220c1d8c0b89ca03001c01524731ddb5146702228e0000690156c05baf54f4abd7734544d3b22a00006c0145022e1ceaef02711c1f1928000076015617a76be89155030a58df46ad8800007e01522563e508042f011f0a000086016206fb6b11ab756f359a1800008101621012c8363cee877b648200008a01561bc7dbb249f4039223c0d76d9a0000890152385591d89a1a01b079010035016219ad16ec6e7e0b05b4830100291036010a0adc1f6f947ce0dc3101004a01670670bf1bd1bcd50736acd082c72f0301004d016871d84a4e12f2e4028f966c3d8a171d5e01003d015601f2c870485d0d9acaf352d08901004701568723b8aacb3b115ed8247b9c1401005301660e48eeb3dbd9a6022ea92ab087c101003901720877887275bca41b37454501004e108602bef04b16ef0e61686ee1d0b9b07b6e03006501620316825edd0c0e17b4c7000000000000000000000000000000000000000000";

        bytes memory encodedData = bytes.concat(
            encodedDataBegin, encodedNonceAndDeadline, encodedDataOfferAndConsideration, v, r, s, extraData
        );

        (bool result,) = address(book).call(encodedData);
        assertTrue(result);

        // Assertions.
        assertEq(token0.balanceOf(address(account0.addr)), 0);
        assertEq(token1.balanceOf(address(account0.addr)), 500);
        assertEq(token0.balanceOf(address(fulfiller)), 500);
        assertEq(token1.balanceOf(address(fulfiller)), 0);
    }
}
