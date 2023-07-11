// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {EnumerableMap} from "@openzeppelin/utils/structs/EnumerableMap.sol";
import {Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";
import {IDecoder} from "../IDecoder.sol";
import {IFloodPlain} from "../../flood-plain/IFloodPlain.sol";

struct IdToAddress {
    uint256 id;
    address addr;
}

/**
 * @title DecoderWithRegistry
 * @notice A decoder using an internal registry mapping addresses to indexes.
 */
contract DecoderWithRegistry is IDecoder, Ownable2Step {
    using EnumerableMap for EnumerableMap.UintToAddressMap;

    EnumerableMap.UintToAddressMap internal _zones;
    EnumerableMap.UintToAddressMap internal _fulfillers;
    EnumerableMap.UintToAddressMap internal _tokens;

    constructor() Ownable2Step() {}

    function setZone(uint8 zoneId, address zone) external onlyOwner {
        if (zone.code.length == 0) {
            revert IFloodPlain.NotAContract();
        }
        _zones.set(zoneId, zone);
    }

    function setFulfiller(uint8 fulfillerId, address fulfiller) external onlyOwner {
        if (fulfiller.code.length == 0) {
            revert IFloodPlain.NotAContract();
        }
        _fulfillers.set(fulfillerId, fulfiller);
    }

    function setToken(uint16 tokenId, address token) external onlyOwner {
        if (token.code.length == 0) {
            revert IFloodPlain.NotAContract();
        }
        _tokens.set(tokenId, token);
    }

    function zones() external view returns (IdToAddress[] memory) {
        return _allEntries(_zones);
    }

    function fulfillers() external view returns (IdToAddress[] memory) {
        return _allEntries(_fulfillers);
    }

    function tokens() external view returns (IdToAddress[] memory) {
        return _allEntries(_tokens);
    }

    function _allEntries(EnumerableMap.UintToAddressMap storage _map) private view returns (IdToAddress[] memory) {
        uint256 length = _map.length();
        IdToAddress[] memory result = new IdToAddress[](length);
        for (uint256 i; i < length;) {
            (result[i].id, result[i].addr) = _map.at(i);
            unchecked {
                ++i;
            }
        }
        return result;
    }

    /*
        * @notice Decodes a uint in calldata encoded with variable length of `size` bytes. 
        * @param size Size of the uint.
        * @param ptr Pointer to the start of the uint in calldata.
        * @return The decoded uint.
        */
    function _variableLengthDecoding(uint256 size, uint256 ptr) private pure returns (uint256 decoded) {
        assembly {
            decoded := calldataload(ptr)
            decoded := shr(sub(256, mul(size, 8)), decoded)
        }
    }

    function _decodeUint8(uint256 ptr) private pure returns (uint8 decoded) {
        assembly {
            decoded := shr(248, calldataload(ptr))
        }
    }

    function _decodeUint16(uint256 ptr) private pure returns (uint16 decoded) {
        assembly {
            decoded := shr(240, calldataload(ptr))
        }
    }

    function _decodeAddress(uint256 ptr) private pure returns (address decoded) {
        assembly {
            decoded := shr(96, calldataload(ptr))
        }
    }

    function _decodeBytes32(uint256 ptr) private pure returns (bytes32 decoded) {
        assembly {
            decoded := calldataload(ptr)
        }
    }

    function _decodeByte(uint256 ptr) private pure returns (bytes1 decoded) {
        assembly {
            decoded := calldataload(ptr)
        }
    }

    // We start from the first byte. In decoders with other external functions, first byte
    // would have to be skipped, as it should be a unique byte to bypass all other function
    // signatures. However, this is a simple decoder with only a fallback, so we can start from
    // the first byte.

    // This decoder is for decoding to `FloodPlain.fulfillOrder`, to reduce the calldata size.

    // Scheme:
    // 20 bytes - offerer address.
    // 1 bytes - zone address
    // 1 bytes - nonce & deadline sizes in bytes, each in a nibble
    // n bytes - nonce
    // m bytes - deadline
    // 1 bytes - fulfiller
    // 1 byte - num of offer items and num of consideration items, each in a nibble
    // 2 + 1 + n_amount bytes - Each offer item address + 1 byte for amount size + n amount bytes.
    // 2 + 1 + m_amount bytes - Each consideration item address + 1 byte for amount size + m amount bytes.
    // 1 byte - signature v
    // 32 bytes - signature r
    // 32 bytes - signature s
    // ?? bytes - extraData
    fallback(bytes calldata data) external returns (bytes memory) {
        IFloodPlain.Order memory order;
        bytes memory signature;
        address fulfiller;

        uint256 ptr;
        unchecked {
            order.offerer = _decodeAddress(ptr);
            ptr += 20;

            order.zone = _zones.get(_decodeUint8(ptr));
            ++ptr;

            uint8 nonceDeadlineSize = _decodeUint8(ptr);
            ++ptr;

            uint256 nonceSize = (nonceDeadlineSize >> 4) + 1;
            order.nonce = _variableLengthDecoding(nonceSize, ptr);
            ptr += nonceSize;

            uint256 deadlineSize = (nonceDeadlineSize & 0x0f) + 1;
            order.deadline = _variableLengthDecoding(deadlineSize, ptr);
            ptr += deadlineSize;

            fulfiller = _fulfillers.get(_decodeUint8(ptr));
            ++ptr;
        }
        unchecked {
            uint256 itemCounts = _decodeUint8(ptr);
            ++ptr;
            uint256 offerCount = (itemCounts >> 4) + 1;
            uint256 considerationCount = (itemCounts & 0x0f) + 1;

            IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](offerCount);
            IFloodPlain.Item[] memory consideration = new IFloodPlain.Item[](considerationCount);
            uint256 amountSize;
            for (uint256 i; i < offerCount;) {
                offer[i].token = _tokens.get(_decodeUint16(ptr));
                ptr += 2;

                amountSize = _decodeUint8(ptr) + 1;
                ++ptr;

                offer[i].amount = _variableLengthDecoding(amountSize, ptr);
                ptr += amountSize;

                ++i;
            }
            order.offer = offer;
            for (uint256 i; i < considerationCount;) {
                consideration[i].token = _tokens.get(_decodeUint16(ptr));
                ptr += 2;

                amountSize = _decodeUint8(ptr) + 1;
                ++ptr;
                consideration[i].amount = _variableLengthDecoding(amountSize, ptr);
                ptr += amountSize;

                ++i;
            }
            order.consideration = consideration;

            bytes1 v = _decodeByte(ptr);
            ++ptr;
            bytes32 r = _decodeBytes32(ptr);
            ptr += 32;
            bytes32 s = _decodeBytes32(ptr);
            ptr += 32;

            signature = bytes.concat(r, s, v);
        }

        return abi.encodeWithSelector(IFloodPlain.fulfillOrder.selector, order, signature, fulfiller, data[ptr:]);
    }
}
