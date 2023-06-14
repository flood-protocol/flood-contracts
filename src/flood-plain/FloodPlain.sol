// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IFloodPlain} from "./IFloodPlain.sol";
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";

// Libraries
import {OrderHash} from "./libraries/OrderHash.sol";
import {ItemDeduplicator} from "./libraries/ItemDeduplicator.sol";
import {Address} from "@openzeppelin/utils/Address.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";

// Interfaces
import {IFulfiller} from "../fulfiller/IFulfiller.sol";
import {IZone} from "../zone/IZone.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";

contract FloodPlain is IFloodPlain, ReentrancyGuard {
    using Address for address payable;
    using SafeERC20 for IERC20;
    using OrderHash for Order;
    using ItemDeduplicator for Item[];

    ISignatureTransfer public immutable PERMIT2;

    constructor(address permit2) {
        if (permit2.code.length == 0) {
            revert NotAContract();
        }

        PERMIT2 = ISignatureTransfer(permit2);
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

    function directFulfillOrder(Order calldata order, bytes calldata signature) external payable nonReentrant {
        // Retrieve the order hash for order.
        bytes32 orderHash = order.hash();

        // Check zone restrictions.
        address zone = order.zone;
        if (zone != address(0)) {
            IZone(zone).validateOrder({
                order: order,
                book: address(this),
                caller: msg.sender,
                orderHash: orderHash
            });
        }

        // Transfer each offer item to msg.sender using Permit2.
        _permitTransferOffer({order: order, signature: signature, orderHash: orderHash, to: msg.sender});

        // Transfer consideration items from fulfiller to offerer.
        _directTransferConsideration({order: order, fulfiller: msg.sender});

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, order.offerer, msg.sender);
    }

    function getPermitHash(Order calldata order) external pure returns (bytes32 /* permitHash */ ) {
        // Derive permit2 hash with order hash as witness by supplying order parameters.
        return order.hashAsWitness();
    }

    function getOrderHash(Order calldata order) external pure returns (bytes32 /* orderHash */ ) {
        // Derive order hash by supplying order parameters.
        return order.hash();
    }

    function getOrderValidity(Order calldata order, address caller)
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
                caller: caller,
                orderHash: order.hash()
            }) {
                return true;
            } catch {
                return false;
            }
        }
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
        // Define deduplicatedItems pointer once outside the loops.
        Item memory deduplicatedItem;

        // Deduplicate consideration items, and move the length to stack.
        Item[] memory deduplicatedItems = order.consideration.deduplicate();
        uint256 dedupCount = deduplicatedItems.length;

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

    function _directTransferConsideration(Order calldata order, address fulfiller) internal {
        // Define pointers once outside the loops.
        Item memory item;
        uint256 requiredAmount;
        uint256 amount;
        address token;

        // Deduplicate consideration items, and move the length to stack.
        Item[] memory deduplicatedItems = order.consideration.deduplicate();
        uint256 dedupCount = deduplicatedItems.length;

        address to = order.offerer;
        for (uint256 i = 0; i < dedupCount;) {
            item = deduplicatedItems[i];
            token = item.token;
            amount = item.amount;
            if (token == address(0)) {
                requiredAmount = amount + to.balance;

                if (msg.value != amount) {
                    revert IncorrectValueReceived();
                }

                payable(to).sendValue(amount);

                if (to.balance < requiredAmount) {
                    revert InsufficientAmountPulled();
                }
            } else {
                requiredAmount = amount + IERC20(token).balanceOf(to);

                IERC20(token).safeTransferFrom(fulfiller, to, amount);

                if (IERC20(token).balanceOf(to) < requiredAmount) {
                    revert InsufficientAmountPulled();
                }
            }

            unchecked {
                ++i;
            }
        }
    }
}
