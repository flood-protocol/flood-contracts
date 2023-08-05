// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {IFloodPlain} from "./IFloodPlain.sol";
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";

// Libraries
import {OrderHash} from "./libraries/OrderHash.sol";
import {Duplicates} from "./libraries/Duplicates.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/utils/Address.sol";

// Interfaces
import {IFulfiller} from "../fulfiller/IFulfiller.sol";
import {IZone} from "../zone/IZone.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";

contract FloodPlain is IFloodPlain, ReentrancyGuard {
    using SafeERC20 for IERC20;
    using Address for address payable;
    using OrderHash for Order;
    using Duplicates for Item[];

    ISignatureTransfer public immutable PERMIT2;

    constructor(address permit2) {
        if (permit2.code.length == 0) {
            revert NotAContract();
        }

        PERMIT2 = ISignatureTransfer(permit2);
    }

    function fulfillOrder(SignedOrder calldata signedOrder, address fulfiller, bytes calldata swapData)
        external
        nonReentrant
    {
        // Get order component from calldata.
        Order calldata order = signedOrder.order;

        // Retrieve the order hash for order.
        bytes32 orderHash = order.hash();

        // Check zone restrictions.
        address zone = order.zone;
        if (zone != address(0)) {
            if (
                IZone(zone).validateOrder({
                    order: order,
                    book: address(this),
                    fulfiller: fulfiller,
                    caller: msg.sender,
                    orderHash: orderHash,
                    context: signedOrder.zoneData
                }) != IZone.validateOrder.selector
            ) {
                revert ZoneDenied();
            }
        }

        // Transfer each offer item to fulfiller using Permit2.
        _permitTransferOffer({order: order, signature: signedOrder.signature, orderHash: orderHash, to: fulfiller});

        // Transfer consideration items from fulfiller to offerer.
        _transferConsideration({order: order, fulfiller: fulfiller, swapData: swapData});

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, order.offerer, fulfiller);
    }

    function getPermitHash(Order calldata order) external view returns (bytes32 /* permitHash */ ) {
        // Derive permit2 hash with order hash as witness by supplying order parameters.
        return order.hashAsWitness(address(this));
    }

    function getOrderHash(Order calldata order) external pure returns (bytes32 /* orderHash */ ) {
        // Derive order hash by supplying order parameters.
        return order.hash();
    }

    function getOrderValidity(Order calldata order, address fulfiller, address caller, bytes calldata zoneData)
        external
        view
        returns (bool /* isValid */ )
    {
        if (!getOrderStatus({order: order})) {
            return false;
        }

        if (order.zone == address(0)) {
            return true;
        }

        if (
            IZone(order.zone).validateOrder({
                book: address(this),
                order: order,
                fulfiller: fulfiller,
                caller: caller,
                orderHash: order.hash(),
                context: zoneData
            }) == IZone.validateOrder.selector
        ) {
            return true;
        }

        return false;
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
        // Derive the word and bit position within the word from the nonce.
        uint256 wordPos = uint248(nonce >> 8);
        uint256 bitPos = uint8(nonce);
        // Check if the bit is set by ANDing the word with a bit mask.
        return PERMIT2.nonceBitmap(user, wordPos) & (1 << bitPos) == 0;
    }

    function _permitTransferOffer(Order calldata order, bytes calldata signature, bytes32 orderHash, address to)
        internal
    {
        Item[] calldata offer = order.offer;
        uint256 itemsLength = offer.length;

        if (offer.hasDuplicates()) {
            revert DuplicateItems();
        }

        ISignatureTransfer.TokenPermissions[] memory permitted = new ISignatureTransfer.TokenPermissions[](itemsLength);
        ISignatureTransfer.SignatureTransferDetails[] memory transferDetails =
        new ISignatureTransfer.SignatureTransferDetails[](
                itemsLength
            );

        {
            Item calldata item;
            uint256 amount;
            for (uint256 i; i < itemsLength;) {
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

    function _transferConsideration(Order calldata order, address fulfiller, bytes calldata swapData) internal {
        // Call fulfiller with order data and the caller address to source consideration items.
        // Contracts implementing Fulfiller interface could get all their tokens drained, hence
        // they should restrict who can call them directly or indirectly.
        uint256[] memory returnedAmounts = IFulfiller(payable(fulfiller)).sourceConsideration({
            order: order,
            caller: msg.sender,
            context: swapData
        });

        // Get consideration items, and their count.
        Item[] calldata consideration = order.consideration;
        uint256 length = consideration.length;

        // Ensure returned array length is valid.
        if (length != returnedAmounts.length) {
            revert ArrayLengthMismatch();
        }

        // Do not accept consideration with duplicate items.
        if (consideration.hasDuplicates()) {
            revert DuplicateItems();
        }

        // Move offerer to the stack.
        address to = order.offerer;

        // Create stack elements outside the loop.
        Item calldata item;
        uint256 amount;
        address token;

        for (uint256 i; i < length;) {
            amount = returnedAmounts[i];
            item = consideration[i];

            if (amount < item.amount) {
                revert InsufficientAmountReceived();
            }

            token = item.token;
            if (token == address(0)) {
                // Expect Fulfiller to have sent Ether when sourceConsideration was called. No
                // check is made that Fulfiller sent correct amount. Because FloodPlain is not
                // supposed to hold any Ether, and if less than the returnedAmount indicated by
                // Fulfiller is received, below transfer will revert.
                payable(to).sendValue(amount);
            } else {
                // Simply transferFrom, without using Permit2.
                IERC20(token).safeTransferFrom(fulfiller, to, amount);
            }

            unchecked {
                ++i;
            }
        }
    }

    receive() external payable {}
}
