// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

// Inheritances
import {EIP712} from "@openzeppelin/utils/cryptography/EIP712.sol";
import {ReentrancyGuard} from "@openzeppelin/security/ReentrancyGuard.sol";
import {IFloodPlain} from "./interfaces/IFloodPlain.sol";

// Libraries
import {SignatureChecker} from "@openzeppelin/utils/cryptography/SignatureChecker.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";

// Interfaces
import {IFulfiller} from "./interfaces/IFulfiller.sol";
import {IZone} from "./interfaces/IZone.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

contract FloodPlain is IFloodPlain, EIP712, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // Precompute type hashes on deployment.
    bytes32 private immutable _ITEM_TYPEHASH;
    bytes32 private immutable _ORDER_TYPEHASH;

    // Only orders signed using an offerer's current counter are fulfillable.
    mapping(address => uint256) private _counters;

    // Track status of each order (validated, cancelled, and fraction filled).
    mapping(bytes32 => OrderStatus) private _orderStatus;

    // Track nonces for contract offerers.
    mapping(address => uint256) private _contractNonces;

    // Ensures function is not cross entered.
    modifier nonCrossEntrant() {
        if (_reentrancyGuardEntered()) {
            revert CrossEntrancy();
        }
        _;
    }

    constructor() EIP712("FloodPlain", "1.0") {
        (_ITEM_TYPEHASH, _ORDER_TYPEHASH) = _getTypeHashes();
    }

    function fulfillOrder(Order calldata order, address fulfiller, bytes calldata extraData) external nonReentrant {
        // Retrieve the order parameters.
        OrderParameters calldata orderParameters = order.parameters;

        // Move offerer address to stack.
        address offerer = orderParameters.offerer;

        // Retrieve the order hash for the counter value of the offerer.
        bytes32 orderHash = _deriveOrderHash(orderParameters, _counters[offerer]);

        // Retrieve the order status using the derived order hash.
        OrderStatus storage orderStatus = _orderStatus[orderHash];

        // Verify that owner is not cancelled or already filled.
        _verifyOrderStatus(orderHash, orderStatus);

        // If the order has not already been validated...
        if (!orderStatus.isValidated) {
            orderStatus.isValidated = true;
            // Verify that the supplied signature recovers to the offerer.
            if (!SignatureChecker.isValidSignatureNow(offerer, orderHash, order.signature)) {
                revert InvalidSignature();
            }
        }

        // Verify that the current timestamp has not passed the deadline.
        if (orderParameters.deadline > block.timestamp) {
            revert DeadlinePassed();
        }

        // Optimistically update order status before any external call.
        orderStatus.isFilled = true;

        // Check zone restrictions.
        address zone = orderParameters.zone;
        if (zone != address(0)) {
            IZone(zone).validateOrder({
                order: orderParameters,
                fulfiller: fulfiller,
                caller: msg.sender,
                orderHash: orderHash,
                context: extraData
            });
        }

        // Transfer each offer item to fulfiller.
        _transferOffer({from: orderParameters.offerer, to: fulfiller, items: orderParameters.offer});

        // Call fulfiller with order data and the caller address to source consideration items.
        // Contracts implementing Fulfiller interface could get all their tokens drained, hence
        // they should not hold any tokens.
        IFulfiller(fulfiller).sourceConsideration({
            order: orderParameters,
            fulfiller: fulfiller,
            caller: msg.sender,
            orderHash: orderHash,
            context: extraData
        });

        // Transfer consideration items from fulfiller to offerer.
        _transferConsideration({from: fulfiller, to: offerer, items: orderParameters.consideration});

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, orderParameters.offerer, fulfiller);
    }

    function cancel(OrderComponents[] calldata orders) external nonCrossEntrant {
        // Declare storage pointer outside of the loop.
        OrderStatus storage orderStatus;

        // Read length of the orders array from memory and place on stack.
        uint256 totalOrders = orders.length;

        // Iterate over each order.
        for (uint256 i = 0; i < totalOrders;) {
            // Retrieve the order.
            OrderComponents calldata order = orders[i];

            address offerer = order.offerer;
            address zone = order.zone;

            // Revert if caller is neither the offerer nor the zone.
            if (msg.sender != offerer && msg.sender != zone) {
                revert InvalidCaller();
            }

            bytes32 orderHash = _deriveOrderHash(order);

            // Retrieve the order status using the derived order hash.
            orderStatus = _orderStatus[orderHash];

            // Update the order status as not valid and cancelled.
            orderStatus.isValidated = false;
            orderStatus.isCancelled = true;

            // Emit an event signifying that the order has been cancelled.
            emit OrderCancelled(orderHash, offerer, zone);

            // Skip overflow check as for loop is indexed starting at zero.
            unchecked {
                ++i;
            }
        }
    }

    function validate(Order[] calldata orders) external nonCrossEntrant {
        // Read length of the orders array from memory and place on stack.
        uint256 totalOrders = orders.length;

        // Initialize local variables only once outside the loop.
        OrderStatus storage orderStatus;
        Order calldata order;
        OrderParameters calldata orderParameters;
        address offerer;
        bytes32 orderHash;

        // Iterate over each order.
        for (uint256 i = 0; i < totalOrders;) {
            // Retrieve the order.
            order = orders[i];

            // Retrieve the order parameters.
            orderParameters = order.parameters;

            // Move offerer from memory to the stack.
            offerer = orderParameters.offerer;

            // Get current counter and use it w/ params to derive order hash.
            orderHash = _deriveOrderHash(orderParameters, _counters[offerer]);

            // Retrieve the order status using the derived order hash.
            orderStatus = _orderStatus[orderHash];

            // Ensure order is fillable and retrieve the filled amount.
            _verifyOrderStatus(orderHash, orderStatus);

            // If the order has not already been validated...
            if (!orderStatus.isValidated) {
                // Verify the supplied signature.
                if (!SignatureChecker.isValidSignatureNow(offerer, orderHash, order.signature)) {
                    revert InvalidSignature();
                }

                // Update order status to mark the order as valid.
                orderStatus.isValidated = true;

                // Emit an event signifying the order has been validated.
                emit OrderValidated(orderHash, orderParameters);
            }

            unchecked {
                ++i;
            }
        }
    }

    function incrementCounter() external nonCrossEntrant returns (uint256 newCounter) {
        // Utilize assembly to access counters storage mapping directly. Skip
        // overflow check as counter cannot be incremented that far.
        assembly {
            // Use second half of previous block hash as a quasi-random number.
            let quasiRandomNumber := shr(0x80, blockhash(sub(number(), 1)))

            // Write the caller to scratch space.
            mstore(0, caller())

            // Write the storage slot for _counters to scratch space.
            mstore(0x20, _counters.slot)

            // Derive the storage pointer for the counter value.
            let storagePointer := keccak256(0, 0x40)

            // Derive new counter value using random number and original value.
            newCounter := add(quasiRandomNumber, sload(storagePointer))

            // Store the updated counter value.
            sstore(storagePointer, newCounter)
        }

        // Emit an event containing the new counter.
        emit CounterIncremented(newCounter, msg.sender);
    }

    function getOrderHash(OrderComponents calldata order) external view returns (bytes32 /* orderHash */ ) {
        // Derive order hash by supplying order parameters along with counter.
        return _deriveOrderHash(order);
    }

    function getOrderStatus(bytes32 orderHash) external view returns (OrderStatus memory /* orderStatus */ ) {
        // Retrieve the order status using the order hash.
        return _orderStatus[orderHash];
    }

    function getCounter(address offerer) external view returns (uint256 /* counter */ ) {
        // Return the counter for the supplied offerer.
        return _counters[offerer];
    }

    function _transferOffer(address from, address to, Item[] calldata items) private {
        Item calldata item;
        uint256 itemsLength = items.length;

        for (uint256 i = 0; i < itemsLength;) {
            item = items[i];

            IERC20(item.token).safeTransferFrom(from, to, item.amount);

            unchecked {
                ++i;
            }
        }
    }

    function _transferConsideration(address from, address to, Item[] calldata items) private {
        Item calldata item;
        uint256 itemsLength = items.length;
        uint256 balanceBefore;

        for (uint256 i = 0; i < itemsLength;) {
            item = items[i];

            if (item.isNative) {
                balanceBefore = to.balance;
                IFulfiller(from).pullNativeToken({to: to, amount: item.amount});
                if (to.balance - balanceBefore != item.amount) {
                    revert InsufficientAmountPulled();
                }
            } else {
                balanceBefore = IERC20(item.token).balanceOf(to);
                IFulfiller(from).pullToken({token: item.token, to: to, amount: item.amount});
                if (IERC20(item.token).balanceOf(to) - balanceBefore != item.amount) {
                    revert InsufficientAmountPulled();
                }
            }

            unchecked {
                ++i;
            }
        }
    }

    function _verifyOrderStatus(bytes32 orderHash, OrderStatus storage orderStatus) private view {
        // Ensure that the order has not been cancelled or filled.
        if (orderStatus.isCancelled || orderStatus.isFilled) {
            revert OrderIsFilledOrCancelled(orderHash);
        }
    }

    // TODO: Efficiently combine this with the function below.
    function _deriveOrderHash(OrderComponents calldata orderComponents) private view returns (bytes32 orderHash) {
        // Move lengths to stack.
        uint256 offerLength = orderComponents.offer.length;
        uint256 considerationLength = orderComponents.consideration.length;

        // Designate new memory regions for offer and consideration item hashes.
        bytes32[] memory offerHashes = new bytes32[](offerLength);
        bytes32[] memory considerationHashes = new bytes32[](considerationLength);

        // Iterate over each offer on the order.
        for (uint256 i = 0; i < offerLength;) {
            // Hash the offer and place the result into memory.
            offerHashes[i] = _hashItem(orderComponents.offer[i]);

            unchecked {
                ++i;
            }
        }

        // Iterate over each consideration on the order.
        for (uint256 i = 0; i < considerationLength;) {
            // Hash the consideration and place the result into memory.
            considerationHashes[i] = _hashItem(orderComponents.consideration[i]);

            unchecked {
                ++i;
            }
        }

        // Derive and return the order hash as specified by EIP-712.
        return keccak256(
            abi.encode(
                _ORDER_TYPEHASH,
                orderComponents.offerer,
                orderComponents.zone,
                keccak256(abi.encodePacked(offerHashes)),
                keccak256(abi.encodePacked(considerationHashes)),
                orderComponents.deadline,
                orderComponents.salt,
                orderComponents.counter
            )
        );
    }

    function _deriveOrderHash(OrderParameters calldata orderParameters, uint256 counter)
        private
        view
        returns (bytes32 orderHash)
    {
        // Move lengths to stack.
        uint256 offerLength = orderParameters.offer.length;
        uint256 considerationLength = orderParameters.consideration.length;

        // Designate new memory regions for offer and consideration item hashes.
        bytes32[] memory offerHashes = new bytes32[](offerLength);
        bytes32[] memory considerationHashes = new bytes32[](considerationLength);

        // Iterate over each offer on the order.
        for (uint256 i = 0; i < offerLength;) {
            // Hash the offer and place the result into memory.
            offerHashes[i] = _hashItem(orderParameters.offer[i]);

            unchecked {
                ++i;
            }
        }

        // Iterate over each consideration on the order.
        for (uint256 i = 0; i < considerationLength;) {
            // Hash the consideration and place the result into memory.
            considerationHashes[i] = _hashItem(orderParameters.consideration[i]);

            unchecked {
                ++i;
            }
        }

        // Derive and return the order hash as specified by EIP-712.
        return keccak256(
            abi.encode(
                _ORDER_TYPEHASH,
                orderParameters.offerer,
                orderParameters.zone,
                keccak256(abi.encodePacked(offerHashes)),
                keccak256(abi.encodePacked(considerationHashes)),
                orderParameters.deadline,
                orderParameters.salt,
                counter
            )
        );
    }

    function _hashItem(Item calldata item) internal view returns (bytes32 /* itemHash */ ) {
        return keccak256(abi.encode(_ITEM_TYPEHASH, item.isNative, item.token, item.amount));
    }

    function _getTypeHashes() private pure returns (bytes32, /* itemTypeHash */ bytes32 /* orderTypehash */ ) {
        // Construct the Item type string.
        bytes memory itemTypeString =
            abi.encodePacked("Item(", "bool isNative,", "address token,", "uint256 amount", ")");

        // Construct the OrderComponents type string.
        bytes memory orderComponentsTypeString = abi.encodePacked(
            "OrderComponents(",
            "address offerer,",
            "address zone,",
            "Item[] offer,",
            "Item[] consideration,",
            "uint256 deadline,",
            "uint256 salt,",
            "uint256 counter",
            ")",
            itemTypeString
        );

        return (keccak256(itemTypeString), keccak256(orderComponentsTypeString));
    }
}
