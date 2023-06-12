// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IFloodPlain} from "./IFloodPlain.sol";
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";
import {Ownable2Step} from "@openzeppelin/access/Ownable2Step.sol";

// Libraries
import {OrderHash} from "./libraries/OrderHash.sol";

// Interfaces
import {IFulfiller} from "../fulfiller/IFulfiller.sol";
import {IZone} from "../zone/IZone.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";

contract FloodPlain is IFloodPlain, ReentrancyGuard, Ownable2Step {
    using OrderHash for Order;

    ISignatureTransfer public immutable PERMIT2;

    OrderWithSignature[] internal _etchedOrders;

    address[] internal _decoders;

    constructor(address permit2) {
        if (permit2.code.length == 0) {
            revert NotAContract();
        }

        PERMIT2 = ISignatureTransfer(permit2);
    }

    function getDecoder(uint256 decoderId) external view returns (address /* decoder */ ) {
        return _decoders[decoderId];
    }

    function getEtchedOrder(uint256 etchedOrderId) external view returns (OrderWithSignature memory /* etchedOrder */ ) {
        return _etchedOrders[etchedOrderId];
    }

    function addDecoder(address decoder) external onlyOwner returns (uint256 decoderId) {
        if (decoder.code.length == 0) {
            revert NotAContract();
        }

        decoderId = _decoders.length;
        _decoders.push(decoder);
        emit DecoderAdded(decoderId, decoder);
    }

    function fulfillOrder(Order calldata order, bytes calldata signature, address fulfiller, bytes calldata extraData)
        external
        nonReentrant
    {
        // Retrieve the order hash for order.
        bytes32 orderHash = order.hash();

        // Check zone restrictions.
        address zone = order.zone;
        if (zone != address(0)) {
            IZone(zone).validateOrder({
                order: order,
                book: address(this),
                fulfiller: fulfiller,
                caller: msg.sender,
                orderHash: orderHash,
                context: extraData
            });
        }

        // Transfer each offer item to fulfiller using Permit2.
        _permitTransferOffer({order: order, signature: signature, orderHash: orderHash, to: fulfiller});

        // Transfer consideration items from fulfiller to offerer.
        _transferConsideration({order: order, fulfiller: fulfiller, extraData: extraData});

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, order.offerer, fulfiller);
    }

    function fulfillEtchedOrder(uint256 orderId, address fulfiller, bytes calldata extraData) external {
        OrderWithSignature memory orderWithSignature = _etchedOrders[orderId];

        // Fulfill the etched order using the standard fulfillment function.
        bytes memory data = abi.encodeWithSelector(
            this.fulfillOrder.selector, orderWithSignature.order, orderWithSignature.signature, fulfiller, extraData
        );
        assembly {
            let result := delegatecall(gas(), address(), add(data, 0x20), mload(data), 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            switch result
            // delegatecall returns 0 on error.
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }

    function etchOrder(OrderWithSignature calldata orderWithSignature)
        external
        nonReentrant
        returns (uint256 orderId)
    {
        orderId = _etchedOrders.length;
        _etchedOrders.push(orderWithSignature);
        emit OrderEtched({
            orderId: orderId,
            orderHash: orderWithSignature.order.hash(),
            order: orderWithSignature.order,
            signature: orderWithSignature.signature
        });
    }

    function getPermitHash(Order calldata order) external pure returns (bytes32 /* permitHash */ ) {
        // Derive permit2 hash with order hash as witness by supplying order parameters.
        return order.hashAsWitness();
    }

    function getOrderHash(Order calldata order) external pure returns (bytes32 /* orderHash */ ) {
        // Derive order hash by supplying order parameters.
        return order.hash();
    }

    function getOrderValidity(Order calldata order, address fulfiller, address caller, bytes calldata extraData)
        external
        view
        returns (bool /* isValid */ )
    {
        if (!getOrderStatus({order: order})) {
            return false;
        }

        if (order.zone == address(0)) {
            return true;
        } else {
            try IZone(order.zone).validateOrder({
                book: address(this),
                order: order,
                fulfiller: fulfiller,
                caller: caller,
                orderHash: order.hash(),
                context: extraData
            }) {
                return true;
            } catch {
                return false;
            }
        }
    }

    function getOrderStatus(Order calldata order) public view returns (bool /* isValid */ ) {
        if (block.timestamp > order.deadline) {
            return false;
        }
        if (!getNonceStatus({user: order.offerer, nonce: order.nonce})) {
            return false;
        }
        return true;
    }

    function getNonceStatus(address user, uint256 nonce) public view returns (bool /* isValid */ ) {
        uint256 wordPos = uint248(nonce >> 8);
        uint256 bitPos = uint8(nonce);
        return PERMIT2.nonceBitmap(user, wordPos) & (1 << bitPos) == 0;
    }

    function _permitTransferOffer(Order calldata order, bytes calldata signature, bytes32 orderHash, address to)
        internal
    {
        Item[] calldata offer = order.offer;
        uint256 itemsLength = offer.length;

        ISignatureTransfer.TokenPermissions[] memory permitted = new ISignatureTransfer.TokenPermissions[](itemsLength);
        ISignatureTransfer.SignatureTransferDetails[] memory transferDetails =
        new ISignatureTransfer.SignatureTransferDetails[](
                itemsLength
            );

        {
            Item calldata item;
            uint256 amount;
            for (uint256 i = 0; i < itemsLength;) {
                item = offer[i];
                amount = item.amount;

                permitted[i] = ISignatureTransfer.TokenPermissions({token: item.token, amount: amount});
                transferDetails[i] = ISignatureTransfer.SignatureTransferDetails({to: to, requestedAmount: amount});

                unchecked {
                    ++i;
                }
            }
        }

        PERMIT2.permitWitnessTransferFrom({
            permit: ISignatureTransfer.PermitBatchTransferFrom({
                permitted: permitted,
                nonce: order.nonce,
                deadline: order.deadline
            }),
            transferDetails: transferDetails,
            owner: order.offerer,
            witness: orderHash,
            witnessTypeString: OrderHash._WITNESS_TYPESTRING,
            signature: signature
        });
    }

    function _transferConsideration(Order calldata order, address fulfiller, bytes calldata extraData) internal {
        Item[] calldata items = order.consideration;
        Item calldata item;
        Item memory deduplicatedItem;
        uint256 itemsLength = items.length;

        Item[] memory deduplicatedItems = new Item[](itemsLength);
        uint256 dedupCount;
        // For each consideration item...
        for (uint256 i = 0; i < itemsLength;) {
            item = items[i];
            bool isFound;
            // Check if it is the same as a previous consideration item in the array.
            for (uint256 j = 0; j < dedupCount;) {
                deduplicatedItem = deduplicatedItems[j];
                if (deduplicatedItem.token == item.token) {
                    // And add the amounts of the two consideration items with identical tokens.
                    deduplicatedItem.amount += item.amount;
                    isFound = true;
                    break;
                }

                unchecked {
                    ++j;
                }
            }

            // If the consideration item is encountered the first time...
            if (!isFound) {
                // Add it to the deduplicated consideration items array.
                deduplicatedItems[dedupCount] = item;

                // And increase the actual length of the array.
                unchecked {
                    ++dedupCount;
                }
            }
        }

        // Get the minimum required final balances of the consideration items.
        uint256[] memory requiredAmounts = new uint256[](dedupCount);
        address to = order.offerer;
        for (uint256 i = 0; i < dedupCount;) {
            deduplicatedItem = deduplicatedItems[i];

            requiredAmounts[i] = deduplicatedItem.token == address(0)
                ? deduplicatedItem.amount + to.balance
                : deduplicatedItem.amount + IERC20(deduplicatedItem.token).balanceOf(to);

            unchecked {
                ++i;
            }
        }

        assembly {
            // Trim the array to its actual size. TODO: Test to ensure it works.
            mstore(deduplicatedItems, dedupCount)
        }

        // Call fulfiller with order data and the caller address to source consideration items.
        // Contracts implementing Fulfiller interface could get all their tokens drained, hence
        // they should restrict who can call them directly or indirectly.
        IFulfiller(payable(fulfiller)).sourceConsideration({
            order: order,
            requestedItems: deduplicatedItems,
            caller: msg.sender,
            context: extraData
        });

        // Check the offerer has received necessary amounts of all the consideration items.
        uint256 newBalance;
        for (uint256 i = 0; i < dedupCount;) {
            deduplicatedItem = deduplicatedItems[i];
            newBalance = deduplicatedItem.token == address(0) ? to.balance : IERC20(deduplicatedItem.token).balanceOf(to);
            if (newBalance < requiredAmounts[i]) {
                revert InsufficientAmountPulled();
            }

            unchecked {
                ++i;
            }
        }
    }

    fallback() external {
        // The first byte is ignored. It should not match the first byte of any other function
        // selector in the contract to guarantee the fallback is executed. The second byte is the
        // decoder ID. A decoder can employ any decoding scheme.
        uint256 decoderId;
        assembly {
            // Get the decoder identifier from the second byte of calldata.
            decoderId := shr(248, calldataload(0x01))
        }

        // Get the decoder address by its identifier from the decoders mapping.
        address decoder = _decoders[decoderId];

        assembly {
            // Move the effective calldata size to stack.
            let trimmedCalldataSize := sub(calldatasize(), 0x02)

            // Copy calldata starting from third byte to the memory.
            calldatacopy(0x00, 0x02, trimmedCalldataSize)

            // Staticcall the decoder to get the decoded data.
            let result := staticcall(gas(), decoder, 0, trimmedCalldataSize, 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            // Revert with return data if the call failed.
            if iszero(result) {
                revert(0, returndatasize())
            }

            // Delegatecall to this address with the decoded data.
            result := delegatecall(gas(), address(), 0, returndatasize(), 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            switch result
            // delegatecall returns 0 on error.
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }
}
