// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

// Inheritances
import {IFloodPlain} from "./interfaces/IFloodPlain.sol";
import {EncodedCalls} from "./EncodedCalls.sol";
import {OnChainOrders} from "./OnChainOrders.sol";
import {ReentrancyGuard} from "@openzeppelin/utils/ReentrancyGuard.sol";

// Libraries
import {OrderHash} from "./libraries/OrderHash.sol";
import {Hooks, SELECTOR_EXTENSION} from "./libraries/Hooks.sol";
import {Duplicates} from "./libraries/Duplicates.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";

// Interfaces
import {IFulfiller} from "./interfaces/IFulfiller.sol";
import {IZone} from "./interfaces/IZone.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";

contract FloodPlain is IFloodPlain, EncodedCalls, OnChainOrders, ReentrancyGuard {
    using SafeERC20 for IERC20;
    using OrderHash for Order;
    using Duplicates for Item[];
    using Hooks for Hook[];

    bytes1 public constant FALLBACK_SELECTOR = 0x00;
    ISignatureTransfer public immutable PERMIT2;

    constructor(address permit2) {
        PERMIT2 = ISignatureTransfer(permit2);
    }

    function getOrderStatus(Order calldata order) public view returns (bool /* isValid */ ) {
        if (block.timestamp > order.deadline) return false;
        if (getNonceStatus(order.offerer, order.nonce)) return true;
        else return false;
    }

    function getNonceStatus(address user, uint256 nonce) public view returns (bool /* isValid */ ) {
        return PERMIT2.nonceBitmap(user, uint248(nonce >> 8)) & (1 << uint8(nonce)) == 0;
    }

    function getPermitHash(Order calldata order) external view returns (bytes32 /* permitHash */ ) {
        return order.hashAsWitness(address(this));
    }

    function getOrderHash(Order calldata order) external pure returns (bytes32 /* orderHash */ ) {
        return order.hash();
    }

    receive() external payable {}

    function fulfillOrder(SignedOrder calldata package) external payable nonReentrant {
        Order calldata order = package.order;

        bytes32 orderHash = order.hash();

        // Check zone accepts the fulfiller. Fulfiller is msg.sender in direct fills.
        if (order.zone != address(0)) if (!(IZone(order.zone).validate(order, msg.sender))) revert ZoneDenied();

        // Execute pre hooks.
        order.preHooks.execute(orderHash, address(PERMIT2));

        // Transfer each offer item to msg.sender using Permit2.
        _permitTransferOffer(order, package.signature, orderHash, msg.sender);

        // Transfer consideration item from msg.sender to offerer.
        uint256 amount = order.consideration.amount;
        IERC20(order.consideration.token).safeTransferFrom(msg.sender, order.recipient, amount);

        // Execute post hooks.
        order.postHooks.execute(orderHash, address(PERMIT2));

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, order.zone, msg.sender, amount);
    }

    function fulfillOrder(SignedOrder calldata package, address fulfiller, bytes calldata swapData)
        external
        nonReentrant
    {
        // Get order component from calldata.
        Order calldata order = package.order;

        // Retrieve the order hash for order.
        bytes32 orderHash = order.hash();

        // Check zone accepts the fulfiller. Fulfiller is msg.sender in this case.
        if (order.zone != address(0)) if (!(IZone(order.zone).validate(order, fulfiller))) revert ZoneDenied();

        // Execute pre hooks.
        order.preHooks.execute(orderHash, address(PERMIT2));

        // Transfer each offer item to fulfiller using Permit2.
        _permitTransferOffer(order, package.signature, orderHash, fulfiller);

        // Call fulfiller to perform swaps and return the sourced consideration amount.
        uint256 amount =
            IFulfiller(payable(fulfiller)).sourceConsideration(SELECTOR_EXTENSION, order, msg.sender, swapData);

        // Ensure sufficient consideration amount is sourced by fulfiller.
        if (amount < order.consideration.amount) revert InsufficientAmountReceived();

        // Transfer declared consideration amount from the fulfiller to the offerer.
        IERC20(order.consideration.token).safeTransferFrom(fulfiller, order.recipient, amount);

        // Execute post hooks.
        order.postHooks.execute(orderHash, address(PERMIT2));

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, order.zone, fulfiller, amount);
    }

    function fulfillOrders(SignedOrder[] calldata packages, address fulfiller, bytes calldata swapData)
        external
        nonReentrant
    {
        Order[] memory orders = new Order[](packages.length);
        for (uint256 i; i < packages.length; ++i) {
            // Get order component from calldata.
            Order calldata order = packages[i].order;

            // Check zone accepts the fulfiller.
            if (order.zone != address(0)) if (!(IZone(order.zone).validate(order, fulfiller))) revert ZoneDenied();

            // Execute pre hooks.
            order.preHooks.execute(order.hash(), address(PERMIT2));

            // Transfer each offer item to fulfiller using Permit2.
            _permitTransferOffer(order, packages[i].signature, order.hash(), fulfiller);

            // Move order to memory for later use.
            orders[i] = order;
        }

        uint256[] memory amounts =
            IFulfiller(payable(fulfiller)).sourceConsiderations(SELECTOR_EXTENSION, orders, msg.sender, swapData);

        if (packages.length != amounts.length) revert ArrayLengthMismatch();

        for (uint256 i; i < packages.length; ++i) {
            Order calldata order = packages[i].order;
            uint256 amount = amounts[i];

            // Ensure sufficient consideration amount is sourced by fulfiller.
            if (amount < order.consideration.amount) revert InsufficientAmountReceived();

            // Transfer declared consideration amount from the fulfiller to the offerer.
            IERC20(order.consideration.token).safeTransferFrom(fulfiller, order.recipient, amount);

            // Execute post hooks.
            order.postHooks.execute(order.hash(), address(PERMIT2));

            // Emit an event signifying that the order has been fulfilled.
            emit OrderFulfilled(order.hash(), order.zone, fulfiller, amount);
        }
    }

    function _permitTransferOffer(Order calldata order, bytes calldata signature, bytes32 orderHash, address to)
        internal
    {
        // Ensure there are no duplicate items in offer to simplify calculations in fulfillers.
        Item[] calldata offer = order.offer;
        if (offer.hasDuplicates()) revert DuplicateItems();

        // Construct Permit2 data.
        ISignatureTransfer.TokenPermissions[] memory permitted = new ISignatureTransfer.TokenPermissions[](offer.length);
        ISignatureTransfer.SignatureTransferDetails[] memory transferDetails =
            new ISignatureTransfer.SignatureTransferDetails[](offer.length);
        for (uint256 i; i < offer.length; ++i) {
            Item calldata item = offer[i];
            permitted[i] = ISignatureTransfer.TokenPermissions(item.token, item.amount);
            transferDetails[i] = ISignatureTransfer.SignatureTransferDetails(to, item.amount);
        }

        // Call Permit2 which transfers offer items from offerer to fulfiller (to).
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
}
