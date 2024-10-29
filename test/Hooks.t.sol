// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";

import {Hooks} from "src/libraries/Hooks.sol";
// import {PermitHash} from "permit2/src/libraries/PermitHash.sol";
// import {OrderSignature} from "test/utils/OrderSignature.sol";
import {IFloodPlain} from "src/interfaces/IFloodPlain.sol";

contract HookContract {
    function execute(IFloodPlain.Hook calldata hook, bytes32 orderHash, address permit2) external {
        Hooks.execute(hook, orderHash, permit2);
    }

    function execute(IFloodPlain.Hook[] calldata hooks, bytes32 orderHash, address permit2) external {
        Hooks.execute(hooks, orderHash, permit2);
    }
}

contract MockHook {
    uint256 public val;
    uint256 public val1;
    uint256 public val2;
    bytes32 public orderHash;

    function updateVal(uint256 a) external {
        val = a;
    }

    function updateVal1(uint256 a) external {
        val1 = a;
    }

    function updateVal2(uint256 a) external {
        val2 = a;
    }

    function updateOrderHash() external {
        bytes32 orderHash_;
        // orderHash_ = bytes32(msg.data[msg.data.length - 32:msg.data.length]);
        assembly ("memory-safe") {
            orderHash_ := calldataload(sub(calldatasize(), 32))
        }
        orderHash = orderHash_;
    }

    function unauthorized() external pure {
        revert("random revert message");
    }
}

contract HooksTest is Test {
    HookContract hookHelper;
    address hooked;
    Account account0;

    function setUp() public {
        hookHelper = new HookContract();
        hooked = address(new MockHook());
        account0 = makeAccount("a");
    }

    function test_NoopWhenExtcodesizeIsZero(bytes calldata data, bytes32 orderHash, address permit2) public {
        vm.assume(permit2 != hooked);
        if (data.length >= 32) {
            uint256 ext = abi.decode(data, (uint256));
            vm.assume(ext << 32 != 0x138beaebd34676ddcaaba2ac75bbd144c652c8c6d933f962245c61ff00000000);
        }
        // This is noop. We allow it because checking extcodesize on each hook is expensive.
        hookHelper.execute(IFloodPlain.Hook({target: address(0x6969696969), data: data}), orderHash, permit2);
    }

    function test_RevertWhenSelectorExtensionClash(
        bytes4 data0,
        bytes calldata data2,
        bytes32 orderHash,
        address permit2
    ) public {
        vm.assume(permit2 != hooked);
        bytes28 data1 = 0x138beaebd34676ddcaaba2ac75bbd144c652c8c6d933f962245c61ff;
        bytes memory data = abi.encodePacked(data0, data1, data2);

        vm.expectRevert();
        hookHelper.execute(IFloodPlain.Hook({target: address(0x6969696969), data: data}), orderHash, permit2);
    }

    function test_RevertWhenHookReverts(bytes32 orderHash, address permit2) public {
        vm.assume(permit2 != hooked);
        vm.expectRevert("random revert message");
        hookHelper.execute(IFloodPlain.Hook({target: hooked, data: hex"518d3e42"}), orderHash, permit2);
    }

    function test_RevertWhenHookIsPermit2(bytes calldata data, bytes32 orderHash, address permit2) public {
        if (data.length >= 32) {
            uint256 ext = abi.decode(data, (uint256));
            vm.assume(ext << 32 != 0x138beaebd34676ddcaaba2ac75bbd144c652c8c6d933f962245c61ff00000000);
        }

        vm.expectRevert("MALICIOUS_CALL");
        hookHelper.execute(IFloodPlain.Hook({target: permit2, data: data}), orderHash, permit2);
    }

    function test_Execute(uint256 a, bytes32 orderHash, address permit2) public {
        vm.assume(permit2 != hooked);
        uint256 selectorExt = uint256(0x138beaebd34676ddcaaba2ac75bbd144c652c8c6d933f962245c61ff);
        vm.assume(a >> 32 != selectorExt);

        bytes memory data = abi.encodeWithSelector(MockHook.updateVal.selector, a);
        hookHelper.execute(IFloodPlain.Hook({target: hooked, data: data}), orderHash, permit2);
        assertEq(a, MockHook(hooked).val());
    }

    function test_ExecuteLoop(uint256 a, uint256 b, uint256 c, bytes32 orderHash, address permit2) public {
        vm.assume(permit2 != hooked);
        uint256 selectorExt = uint256(0x138beaebd34676ddcaaba2ac75bbd144c652c8c6d933f962245c61ff);
        vm.assume(a >> 32 != selectorExt);
        vm.assume(b >> 32 != selectorExt);
        vm.assume(c >> 32 != selectorExt);

        bytes memory data0 = abi.encodeWithSelector(MockHook.updateVal.selector, a);
        bytes memory data1 = abi.encodeWithSelector(MockHook.updateVal1.selector, b);
        bytes memory data2 = abi.encodeWithSelector(MockHook.updateVal2.selector, c);
        bytes memory data3 = abi.encodeWithSelector(MockHook.updateOrderHash.selector, orderHash);

        IFloodPlain.Hook memory hook0 = IFloodPlain.Hook({target: hooked, data: data0});
        IFloodPlain.Hook memory hook1 = IFloodPlain.Hook({target: hooked, data: data1});
        IFloodPlain.Hook memory hook2 = IFloodPlain.Hook({target: hooked, data: data2});
        IFloodPlain.Hook memory hook3 = IFloodPlain.Hook({target: hooked, data: data3});

        IFloodPlain.Hook[] memory hooks = new IFloodPlain.Hook[](4);
        hooks[0] = hook0;
        hooks[1] = hook1;
        hooks[2] = hook2;
        hooks[3] = hook3;

        hookHelper.execute(hooks, orderHash, permit2);
        assertEq(a, MockHook(hooked).val());
        assertEq(b, MockHook(hooked).val1());
        assertEq(c, MockHook(hooked).val2());
        assertEq(orderHash, MockHook(hooked).orderHash());
    }
}
