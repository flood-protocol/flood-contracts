// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";

import {OrderHash} from "src/libraries/OrderHash.sol";
import {PermitHash} from "permit2/src/libraries/PermitHash.sol";
import {OrderSignature} from "test/utils/OrderSignature.sol";
import {IFloodPlain} from "src/interfaces/IFloodPlain.sol";

contract OrderHashTest is Test {
    OrderSignature orderSignature;
    Account account0;

    function setUp() public {
        orderSignature = new OrderSignature();
        account0 = makeAccount("a");
    }

    function test_itemHash() public {
        IFloodPlain.Item memory item = IFloodPlain.Item({token: address(0), amount: 0});

        bytes32 itemHash = orderSignature.hash(item);

        assertEq(itemHash, 0xcb02862e625f0c341a0dbe9a7495af6982102db3456f3ded5efe8fabd6689904);
    }

    function test_hookHash() public {
        IFloodPlain.Hook memory hook = IFloodPlain.Hook({target: address(0), data: hex""});

        bytes32 hookHash = orderSignature.hash(hook);

        assertEq(hookHash, 0xd3b9e9f5bd8a8b242f8319849becb42f8ce6c5c6e9d24f9539f428e8d7e666bd);
    }

    function test_orderHash() public {
        IFloodPlain.Item memory item = IFloodPlain.Item({token: address(0), amount: 0});
        IFloodPlain.Hook memory hook = IFloodPlain.Hook({target: address(0), data: hex""});

        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](3);
        offer[0] = item;
        offer[1] = item;
        offer[2] = item;

        IFloodPlain.Hook[] memory preHooks = new IFloodPlain.Hook[](1);
        preHooks[0] = hook;

        IFloodPlain.Hook[] memory postHooks = new IFloodPlain.Hook[](2);
        postHooks[0] = hook;
        postHooks[1] = hook;

        IFloodPlain.Order memory order = IFloodPlain.Order({
            offerer: address(0),
            zone: address(0),
            recipient: address(0),
            offer: offer,
            consideration: item,
            deadline: 0,
            nonce: 0,
            preHooks: preHooks,
            postHooks: postHooks
        });

        bytes32 orderHash = orderSignature.hash(order);

        assertEq(orderHash, 0xece53f158244592f601148c3a00ab85c63d4bf4ce04da8375e216dfc40694b32);
    }

    function test_permitHash() public {
        IFloodPlain.Item memory item = IFloodPlain.Item({token: address(0), amount: 0});
        IFloodPlain.Hook memory hook = IFloodPlain.Hook({target: address(0), data: hex""});

        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](3);
        offer[0] = item;
        offer[1] = item;
        offer[2] = item;

        IFloodPlain.Hook[] memory preHooks = new IFloodPlain.Hook[](1);
        preHooks[0] = hook;

        IFloodPlain.Hook[] memory postHooks = new IFloodPlain.Hook[](2);
        postHooks[0] = hook;
        postHooks[1] = hook;

        IFloodPlain.Order memory order = IFloodPlain.Order({
            offerer: address(0),
            zone: address(0),
            recipient: address(0),
            offer: offer,
            consideration: item,
            deadline: 0,
            nonce: 0,
            preHooks: preHooks,
            postHooks: postHooks
        });

        bytes32 permitHash = orderSignature.hashAsWitness(order, address(0x420));

        assertEq(permitHash, 0x8aea3ef4ab58e3cfd67a39b948421def10f4424ee4be0b8c1be0bb6c05bb022a);
    }

    // `test_permitHash` checks the whole PermitBatchWitnessTransferFrom struct typehash, but
    // what is passed to the Permit 2 is only the partial type string. Below tests the partial
    // typestring against the whole typestring, which was implicitly checked to be true in the
    // previous `test_permitHash` test.
    function test_witnessTypeString() public {
        bytes32 permitTypeHash = keccak256(
            bytes(
                string.concat(
                    PermitHash._PERMIT_BATCH_WITNESS_TRANSFER_FROM_TYPEHASH_STUB, OrderHash._WITNESS_TYPESTRING
                )
            )
        );
        assertEq(permitTypeHash, OrderHash._PERMIT_TYPEHASH);
    }
}
