// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {IFloodPlain} from "../interfaces/IFloodPlain.sol";

bytes28 constant SELECTOR_EXTENSION = bytes28(keccak256("IFulfiller.sourceConsiderations"));

library Hooks {
    function execute(IFloodPlain.Hook calldata hook, bytes32 orderHash, address permit2) internal {
        address target = hook.target;
        bytes calldata data = hook.data;

        bytes28 extension;
        assembly ("memory-safe") {
            extension := shl(32, calldataload(data.offset))
        }
        require(extension != SELECTOR_EXTENSION && target != permit2, "MALICIOUS_CALL");

        assembly ("memory-safe") {
            let fmp := mload(0x40)
            calldatacopy(fmp, data.offset, data.length)
            mstore(add(fmp, data.length), orderHash)
            if iszero(call(gas(), target, 0, fmp, add(data.length, 32), 0, 0)) {
                returndatacopy(0, 0, returndatasize())
                revert(0, returndatasize())
            }
        }
    }

    function execute(IFloodPlain.Hook[] calldata hooks, bytes32 orderHash, address permit2) internal {
        for (uint256 i; i < hooks.length; ++i) {
            execute(hooks[i], orderHash, permit2);
        }
    }
}
