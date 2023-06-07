// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IFloodPlain} from "../interfaces/IFloodPlain.sol";
import {ISignatureTransfer, PermitHash} from "permit2/src/libraries/PermitHash.sol";

library OrderHash {
    using OrderHash for IFloodPlain.Order;

    bytes32 public constant _ITEM_TYPEHASH = keccak256("Item(address token,uint256 amount)");
    bytes32 public constant _ORDER_TYPEHASH = keccak256(
        "Order(address offerer,address zone,Item[] offer,Item[] consideration,uint256 deadline,uint256 nonce)Item(address token,uint256 amount)"
    );
    bytes32 public constant _PERMIT_TYPEHASH = keccak256(
        "PermitBatchWitnessTransferFrom(TokenPermissions[] permitted,address spender,uint256 nonce,uint256 deadline,Order witness)Item(address token,uint256 amount)Order(address offerer,address zone,Item[] offer,Item[] consideration,uint256 deadline,uint256 nonce)TokenPermissions(address token,uint256 amount)"
    );
    string public constant _WITNESS_TYPESTRING =
        "Order witness)Item(address token,uint256 amount)Order(address offerer,address zone,Item[] offer,Item[] consideration,uint256 deadline,uint256 nonce)TokenPermissions(address token,uint256 amount)";

    function hash(IFloodPlain.Item calldata item) internal pure returns (bytes32) {
        return keccak256(abi.encode(_ITEM_TYPEHASH, item.token, item.amount));
    }

    function hash(IFloodPlain.Order calldata order) internal pure returns (bytes32) {
        // Move lengths to stack.
        uint256 offerLength = order.offer.length;
        uint256 considerationLength = order.consideration.length;

        // Designate new memory regions for offer and consideration item hashes.
        bytes32[] memory offerHashes = new bytes32[](offerLength);
        bytes32[] memory considerationHashes = new bytes32[](considerationLength);

        // Iterate over each offer on the order.
        for (uint256 i = 0; i < offerLength;) {
            // Hash the offer and place the result into memory.
            offerHashes[i] = hash(order.offer[i]);

            unchecked {
                ++i;
            }
        }

        // Iterate over each consideration on the order.
        for (uint256 i = 0; i < considerationLength;) {
            // Hash the consideration and place the result into memory.
            considerationHashes[i] = hash(order.consideration[i]);

            unchecked {
                ++i;
            }
        }

        // Derive and return the order hash as specified by EIP-712.
        return keccak256(
            abi.encode(
                _ORDER_TYPEHASH,
                order.offerer,
                order.zone,
                keccak256(abi.encodePacked(offerHashes)),
                keccak256(abi.encodePacked(considerationHashes)),
                order.deadline,
                order.nonce
            )
        );
    }

    function hashAsWitness(IFloodPlain.Order calldata order) internal pure returns (bytes32) {
        IFloodPlain.Item[] calldata offer = order.offer;
        uint256 itemsLength = offer.length;

        IFloodPlain.Item calldata item;
        bytes32[] memory tokenPermissionHashes = new bytes32[](itemsLength);
        for (uint256 i = 0; i < itemsLength;) {
            item = offer[i];

            tokenPermissionHashes[i] = keccak256(
                abi.encode(
                    PermitHash._TOKEN_PERMISSIONS_TYPEHASH,
                    ISignatureTransfer.TokenPermissions({token: item.token, amount: item.amount})
                )
            );

            unchecked {
                ++i;
            }
        }

        return keccak256(
            abi.encode(
                _PERMIT_TYPEHASH,
                keccak256(abi.encodePacked(tokenPermissionHashes)),
                order.offerer,
                order.nonce,
                order.deadline,
                order.hash()
            )
        );
    }
}
