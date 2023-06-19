// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";

import {OrderHash} from "src/flood-plain/libraries/OrderHash.sol";
import {PermitHash} from "permit2/src/libraries/PermitHash.sol";
import {OrderSignature} from "test/flood-plain/utils/OrderSignature.sol";
import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";

contract OrderHashTest is Test {
    OrderSignature orderSignature;
    Account account0;

    function setUp() public {
        orderSignature = new OrderSignature();
        account0 = makeAccount("a");
    }

    // Testing against following ethers v6 script output:
    //
    // ```
    // const ethers = require('ethers');
    //
    // const ITEM = {
    //   Item: [{
    //     name: 'token',
    //     type: 'address'
    //     },
    //     {
    //     name: 'amount',
    //     type: 'uint256'
    //     }
    //   ],
    // }
    //
    // const item = {
    //   token: ethers.ZeroAddress,
    //   amount: 0
    // }
    //
    // const itemHash = ethers.TypedDataEncoder.hashStruct("Item", ITEM, item)
    // console.log(itemHash);
    // ```
    function test_itemHash() public {
        IFloodPlain.Item memory item = IFloodPlain.Item({
            token: address(0),
            amount: 0
        });

        bytes32 itemHash = orderSignature.hash(item);

        assertEq(itemHash, 0xcb02862e625f0c341a0dbe9a7495af6982102db3456f3ded5efe8fabd6689904);
    }

    // Testing against following ethers v6 script output:
    //
    // ```
    // const ethers = require('ethers');
    //
    // const ORDER = {
    //   Order: [{
    //     name: 'offerer',
    //     type: 'address'
    //     },
    //     {
    //     name: 'zone',
    //     type: 'address'
    //     },
    //     {
    //     name: 'offer',
    //     type: 'Item[]'
    //     },
    //     {
    //     name: 'consideration',
    //     type: 'Item[]'
    //     },
    //     {
    //     name: 'deadline',
    //     type: 'uint256'
    //     },
    //     {
    //     name: 'nonce',
    //     type: 'uint256'
    //     }],
    //   Item: [{
    //     name: 'token',
    //     type: 'address'
    //     },
    //     {
    //     name: 'amount',
    //     type: 'uint256'
    //     }]
    // }
    //
    // const item = {
    //   token: ethers.ZeroAddress,
    //   amount: 0
    // }
    //
    // const order = {
    //   offerer: ethers.ZeroAddress,
    //   zone: ethers.ZeroAddress,
    //   offer: [item],
    //   consideration: [item, item, item],
    //   deadline: 0,
    //   nonce: 0
    // }
    //
    //
    // const orderHash = ethers.TypedDataEncoder.hashStruct("Order", ORDER, order)
    //
    // console.log(orderHash);
    // ```
    function test_orderHash() public {
        IFloodPlain.Item memory item = IFloodPlain.Item({
            token: address(0),
            amount: 0
        });

        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](1);
        offer[0] = item;

        IFloodPlain.Item[] memory consideration = new IFloodPlain.Item[](3);
        consideration[0] = item;
        consideration[1] = item;
        consideration[2] = item;

        IFloodPlain.Order memory order = IFloodPlain.Order({
            offerer: address(0),
            zone: address(0),
            offer: offer,
            consideration: consideration,
            deadline: 0,
            nonce: 0
        });

        bytes32 orderHash = orderSignature.hash(order);

        assertEq(orderHash, 0x7c035b41df11b270e3f066ebabf4714b283035deadec175a375ee86e1304a31b);
    }

    // Testing against following ethers v6 script output:
    //
    // ```
    // const ethers = require('ethers');
    //
    // const PERMIT = {
    //   PermitBatchWitnessTransferFrom: [{
    //     name: 'permitted',
    //     type: 'TokenPermissions[]'
    //     },
    //     {
    //     name: 'spender',
    //     type: 'address'
    //     },
    //     {
    //     name: 'nonce',
    //     type: 'uint256'
    //     },
    //     {
    //     name: 'deadline',
    //     type: 'uint256'
    //     },
    //     {
    //     name: 'witness',
    //     type: 'Order'
    //   }],
    //   TokenPermissions: [{
    //     name: 'token',
    //     type: 'address'
    //     },
    //     {
    //     name: 'amount',
    //     type: 'uint256'
    //   }],
    //   Item: [{
    //     name: 'token',
    //     type: 'address'
    //     },
    //     {
    //     name: 'amount',
    //     type: 'uint256'
    //   }],
    //   Order: [{
    //     name: 'offerer',
    //     type: 'address'
    //     },
    //     {
    //     name: 'zone',
    //     type: 'address'
    //     },
    //     {
    //     name: 'offer',
    //     type: 'Item[]'
    //     },
    //     {
    //     name: 'consideration',
    //     type: 'Item[]'
    //     },
    //     {
    //     name: 'deadline',
    //     type: 'uint256'
    //     },
    //     {
    //     name: 'nonce',
    //     type: 'uint256'
    //   }]
    // }
    //
    // const item = {
    //   token: ethers.ZeroAddress,
    //   amount: 0
    // }
    //
    // const order = {
    //   offerer: ethers.ZeroAddress,
    //   zone: ethers.ZeroAddress,
    //   offer: [item],
    //   consideration: [item, item, item],
    //   deadline: 0,
    //   nonce: 0
    // }
    //
    // const permit = {
    //   permitted: [item],
    //   spender: ethers.getAddress('0x0000000000000000000000000000000000000001'),
    //   nonce: 0,
    //   deadline: 0,
    //   witness: order
    // }
    //
    // const permitHash = ethers.TypedDataEncoder.hashStruct("PermitBatchWitnessTransferFrom", PERMIT, permit)
    //
    // console.log(permitHash);
    // ```
    function test_permitHash() public {
        IFloodPlain.Item memory item = IFloodPlain.Item({
            token: address(0),
            amount: 0
        });

        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](1);
        offer[0] = item;

        IFloodPlain.Item[] memory consideration = new IFloodPlain.Item[](3);
        consideration[0] = item;
        consideration[1] = item;
        consideration[2] = item;

        IFloodPlain.Order memory order = IFloodPlain.Order({
            offerer: address(0),
            zone: address(0),
            offer: offer,
            consideration: consideration,
            deadline: 0,
            nonce: 0
        });

        bytes32 permitHash = orderSignature.hashAsWitness(order, address(0x1));

        assertEq(permitHash, 0x374fc27db7eb383f2dae14bfde20441cd0dd5c9d45ba10b826c03d978d5472f3);
    }

    // `test_permitHash` checks the whole PermitBatchWitnessTransferFrom struct typehash, but
    // what is passed to the Permit 2 is only the partial type string. Below tests the partial
    // typestring against the whole typestring, which was implicitly checked to be true in the
    // previous `test_permitHash` test.
    function test_witnessTypeString() public {
        bytes32 permitTypeHash = keccak256(bytes(string.concat(
            PermitHash._PERMIT_BATCH_WITNESS_TRANSFER_FROM_TYPEHASH_STUB,
            OrderHash._WITNESS_TYPESTRING
        )));
        assertEq(permitTypeHash, OrderHash._PERMIT_TYPEHASH);
    }
}
