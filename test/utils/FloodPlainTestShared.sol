// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Test} from "forge-std/Test.sol";
import {DeployPermit2} from "permit2/test/utils/DeployPermit2.sol";

import {FloodPlain} from "src/FloodPlain.sol";
import {IFloodPlain} from "src/interfaces/IFloodPlain.sol";
import {MockERC20} from "./MockERC20.sol";
import {MockFeeOnTransferERC20} from "./MockFeeOnTransferERC20.sol";
import {MockFulfiller} from "./MockFulfiller.sol";
//import {MockBadFulfiller} from "test/utils/MockBadFulfiller.sol";
//import {MockBadFulfiller2} from "test/utils/MockBadFulfiller2.sol";
import {MockBatchFulfiller} from "test/utils/MockBatchFulfiller.sol";
//import {MockDecoder} from "test/utils/MockDecoder.sol";
import {MaliciousFulfiller} from "test/utils/MaliciousFulfiller.sol";
import {MaliciousFulfiller2} from "test/utils/MaliciousFulfiller2.sol";
import {MockZone} from "./MockZone.sol";
import {OrderSignature} from "./OrderSignature.sol";
import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";
import {IEIP712} from "permit2/src/interfaces/IEIP712.sol";

abstract contract FloodPlainTestShared is Test, DeployPermit2 {
    ISignatureTransfer permit2;
    FloodPlain book;
    MockZone zone;
    MockFulfiller fulfiller;
    //MockBadFulfiller badFulfiller;
    //MockBadFulfiller2 badFulfiller2;
    MockBatchFulfiller batchFulfiller;
    //MockDecoder decoder;
    MaliciousFulfiller maliciousFulfiller;
    MaliciousFulfiller2 maliciousFulfiller2;
    OrderSignature orderSignature;
    MockERC20 token0;
    MockERC20 token1;
    MockERC20 token2;
    MockERC20 token3;
    MockERC20 token4;
    MockERC20 token5;
    MockFeeOnTransferERC20 token6;
    Account account0;
    Account account1;
    Account account2;
    Account account3;

    function setUp() public virtual {
        permit2 = ISignatureTransfer(deployPermit2());
        book = new FloodPlain(address(permit2));
        fulfiller = new MockFulfiller();
        //badFulfiller = new MockBadFulfiller();
        //badFulfiller2 = new MockBadFulfiller2();
        batchFulfiller = new MockBatchFulfiller();
        //decoder = new MockDecoder();
        maliciousFulfiller = new MaliciousFulfiller();
        maliciousFulfiller2 = new MaliciousFulfiller2();
        zone = new MockZone(address(this));
        orderSignature = new OrderSignature();
        token0 = new MockERC20();
        token1 = new MockERC20();
        token2 = new MockERC20();
        token3 = new MockERC20();
        token4 = new MockERC20();
        token5 = new MockERC20();
        token6 = new MockFeeOnTransferERC20();
        account0 = makeAccount("a");
        account1 = makeAccount("b");
        account2 = makeAccount("c");
        account3 = makeAccount("d");

        zone.setFulfiller(address(fulfiller), true);
        zone.setFulfiller(address(this), true);
        zone.setFulfiller(address(batchFulfiller), true);
    }

    function getSignature(IFloodPlain.Order memory order, Account memory signer)
        internal
        view
        returns (bytes memory sig)
    {
        sig =
            orderSignature.getSignature(order, signer.key, IEIP712(address(permit2)).DOMAIN_SEPARATOR(), address(book));
    }

    function setup_mostBasicOrder() internal returns (IFloodPlain.SignedOrder memory) {
        deal(address(token0), account0.addr, token0.balanceOf(account0.addr) + 500);
        deal(address(token1), address(fulfiller), token1.balanceOf(address(fulfiller)) + 500);

        // Set offer item.
        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](1);
        offer[0].token = address(token0);
        offer[0].amount = 500;

        // Approve permit2 spending.
        uint256 existingAllowance = token0.allowance(account0.addr, address(permit2));
        vm.prank(account0.addr);
        token0.approve(address(permit2), existingAllowance + 500);

        // Set consideration item.
        IFloodPlain.Item memory consideration;
        consideration.token = address(token1);
        consideration.amount = 500;

        IFloodPlain.Hook[] memory hooks = new IFloodPlain.Hook[](0);

        // Construct order.
        IFloodPlain.Order memory order = IFloodPlain.Order({
            offerer: address(account0.addr),
            zone: address(0),
            recipient: account0.addr,
            offer: offer,
            consideration: consideration,
            deadline: type(uint256).max,
            nonce: 0,
            preHooks: hooks,
            postHooks: hooks
        });

        // Sign the order.
        bytes memory sig = getSignature(order, account0);

        return IFloodPlain.SignedOrder({order: order, signature: sig});
    }

    function setup_multiItemOrder() internal returns (IFloodPlain.SignedOrder memory) {
        deal(address(token0), address(account0.addr), 100);
        deal(address(token1), address(account0.addr), 200);
        deal(address(token2), address(account0.addr), 300);
        deal(address(token3), address(fulfiller), 999);

        // Set offer item.
        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](3);
        offer[0].token = address(token0);
        offer[0].amount = 100;
        offer[1].token = address(token1);
        offer[1].amount = 200;
        offer[2].token = address(token2);
        offer[2].amount = 300;

        // Approve permit2 spending.
        vm.prank(account0.addr);
        token0.approve(address(permit2), 100);
        vm.prank(account0.addr);
        token1.approve(address(permit2), 200);
        vm.prank(account0.addr);
        token2.approve(address(permit2), 300);

        // Set consideration item.
        IFloodPlain.Item memory consideration;
        consideration.token = address(token3);
        consideration.amount = 999;

        // Construct order.
        IFloodPlain.Hook[] memory hooks = new IFloodPlain.Hook[](0);
        IFloodPlain.Order memory order = IFloodPlain.Order({
            offerer: address(account0.addr),
            zone: address(0),
            recipient: account0.addr,
            offer: offer,
            consideration: consideration,
            deadline: type(uint256).max,
            nonce: 0,
            preHooks: hooks,
            postHooks: hooks
        });

        // Sign the order.
        bytes memory sig = getSignature(order, account0);

        return IFloodPlain.SignedOrder({order: order, signature: sig});
    }
}
