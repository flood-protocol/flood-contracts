// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";
import {DeployPermit2} from "permit2/test/utils/DeployPermit2.sol";

import {FloodPlainL2} from "src/flood-plain/FloodPlainL2.sol";
import {IFloodPlain} from "src/flood-plain/IFloodPlain.sol";
import {MockERC20} from "test/flood-plain/utils/MockERC20.sol";
import {MockFeeOnTransferERC20} from "test/flood-plain/utils/MockFeeOnTransferERC20.sol";
import {MockFulfiller} from "test/flood-plain/utils/MockFulfiller.sol";
import {MaliciousFulfiller} from "test/flood-plain/utils/MaliciousFulfiller.sol";
import {MockZone} from "test/flood-plain/utils/MockZone.sol";
import {OrderSignature} from "test/flood-plain/utils/OrderSignature.sol";
import {ISignatureTransfer} from "permit2/src/interfaces/ISignatureTransfer.sol";
import {EIP712} from "permit2/src/EIP712.sol";

abstract contract FloodPlainTestShared is Test, DeployPermit2 {
    ISignatureTransfer permit2;
    FloodPlainL2 book;
    MockZone zone;
    MockFulfiller fulfiller;
    MaliciousFulfiller maliciousFulfiller;
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

    function setUp() public {
        permit2 = ISignatureTransfer(deployPermit2());
        book = new FloodPlainL2(address(permit2));
        fulfiller = new MockFulfiller();
        maliciousFulfiller = new MaliciousFulfiller();
        zone = new MockZone();
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
    }

    function getSignature(IFloodPlain.Order memory order, Account memory signer) internal view returns (bytes memory sig) {
        sig = orderSignature.getSignature(order, signer.key, EIP712(address(permit2)).DOMAIN_SEPARATOR(), address(book));
    }

    function setup_mostBasicOrder() internal returns (IFloodPlain.Order memory order, bytes memory sig) {
        deal(address(token0), address(account0.addr), 500);
        deal(address(token1), address(fulfiller), 500);

        // Set offer item.
        IFloodPlain.Item[] memory offer = new IFloodPlain.Item[](1);
        offer[0].token = address(token0);
        offer[0].amount = 500;

        // Approve permit2 spending.
        vm.prank(account0.addr);
        token0.approve(address(permit2), 500);

        // Set consideration item.
        IFloodPlain.Item[] memory consideration = new IFloodPlain.Item[](1);
        consideration[0].token = address(token1);
        consideration[0].amount = 500;

        // Construct order.
        order = IFloodPlain.Order({
            offerer: address(account0.addr),
            zone: address(0),
            offer: offer,
            consideration: consideration,
            deadline: type(uint256).max,
            nonce: 0
        });

        // Sign the order.
        sig = getSignature(order, account0);

        return (order, sig);
    }

    function setup_multiItemOrder() internal returns (IFloodPlain.Order memory order, bytes memory sig) {
        deal(address(token0), address(account0.addr), 100);
        deal(address(token1), address(account0.addr), 200);
        deal(address(token2), address(account0.addr), 300);
        deal(address(token3), address(fulfiller), 400);
        deal(address(token4), address(fulfiller), 500);
        deal(address(token5), address(fulfiller), 600);

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
        IFloodPlain.Item[] memory consideration = new IFloodPlain.Item[](3);
        consideration[0].token = address(token3);
        consideration[0].amount = 400;
        consideration[1].token = address(token4);
        consideration[1].amount = 500;
        consideration[2].token = address(token5);
        consideration[2].amount = 600;

        // Construct order.
        order = IFloodPlain.Order({
            offerer: address(account0.addr),
            zone: address(0),
            offer: offer,
            consideration: consideration,
            deadline: type(uint256).max,
            nonce: 0
        });

        // Sign the order.
        sig = getSignature(order, account0);

        return (order, sig);
    }
}
