// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";
import {IFloodPlain} from "./interfaces/IFloodPlain.sol";

// Interfaces
import {IFulfiller} from "./interfaces/IFulfiller.sol";
import {IZone} from "./interfaces/IZone.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";

// Libraries
import {OrderHash} from "./libraries/OrderHash.sol";

contract FloodPlain is IFloodPlain, ReentrancyGuard {
    using OrderHash for Order;

    ISignatureTransfer private immutable _PERMIT2;

    OrderWithSignature[] public etchedOrders;

    constructor(address permit2) {
        _PERMIT2 = ISignatureTransfer(permit2);
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
        OrderWithSignature memory orderWithSignature = etchedOrders[orderId];
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
        orderId = etchedOrders.length;
        etchedOrders.push(orderWithSignature);
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
        return _PERMIT2.nonceBitmap(user, wordPos) & (1 << bitPos) == 0;
    }

    function _permitTransferOffer(Order calldata order, bytes calldata signature, bytes32 orderHash, address to)
        private
    {
        OfferItem[] calldata offer = order.offer;
        uint256 itemsLength = offer.length;

        ISignatureTransfer.TokenPermissions[] memory permitted = new ISignatureTransfer.TokenPermissions[](itemsLength);
        ISignatureTransfer.SignatureTransferDetails[] memory transferDetails =
        new ISignatureTransfer.SignatureTransferDetails[](
                itemsLength
            );

        {
            OfferItem calldata item;
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

        _PERMIT2.permitWitnessTransferFrom({
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

    function _transferConsideration(Order calldata order, address fulfiller, bytes calldata extraData) private {
        ConsiderationItem[] calldata items = order.consideration;
        ConsiderationItem calldata item;
        ConsiderationItem memory deduplicatedItem;
        uint256 itemsLength = items.length;

        ConsiderationItem[] memory deduplicatedItems = new ConsiderationItem[](itemsLength);
        uint256 dedupCount;
        for (uint256 i = 0; i < itemsLength;) {
            item = items[i];
            bool isFound;
            for (uint256 j = 0; j < dedupCount;) {
                deduplicatedItem = deduplicatedItems[j];
                if ((deduplicatedItem.isNative && item.isNative) || (deduplicatedItem.token == item.token)) {
                    deduplicatedItem.amount += item.amount;
                    isFound = true;
                    break;
                }

                unchecked {
                    ++j;
                }
            }

            if (!isFound) {
                deduplicatedItems[dedupCount] = item;

                unchecked {
                    ++dedupCount;
                }
            }
        }

        uint256[] memory requiredAmounts = new uint256[](dedupCount);
        address to = order.offerer;
        for (uint256 i = 0; i < dedupCount;) {
            deduplicatedItem = deduplicatedItems[i];

            if (deduplicatedItem.isNative) {
                requiredAmounts[i] = deduplicatedItem.amount + to.balance;
            } else {
                requiredAmounts[i] = deduplicatedItem.amount + IERC20(deduplicatedItem.token).balanceOf(to);
            }

            unchecked {
                ++i;
            }
        }

        // TODO: Test this!
        assembly {
            mstore(deduplicatedItems, dedupCount)
        }

        // Call fulfiller with order data and the caller address to source consideration items.
        // Contracts implementing Fulfiller interface could get all their tokens drained, hence
        // they should restrict who can call them directly or indirectly.
        IFulfiller(fulfiller).sourceConsideration({
            order: order,
            requestedItems: deduplicatedItems,
            caller: msg.sender,
            context: extraData
        });

        uint256 newBalance;
        for (uint256 i = 0; i < dedupCount;) {
            deduplicatedItem = deduplicatedItems[i];
            newBalance = deduplicatedItem.isNative ? to.balance : IERC20(deduplicatedItem.token).balanceOf(to);
            if (newBalance < requiredAmounts[i]) {
                revert InsufficientAmountPulled();
            }

            unchecked {
                ++i;
            }
        }
    }
}
