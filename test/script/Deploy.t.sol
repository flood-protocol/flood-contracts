// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {
    FloodRegistry,
    FloodRegistry__TokenAlreadyWhitelisted,
    FloodRegistry__TokenNotWhitelisted
} from "src/FloodRegistry.sol";
import {AllKnowingOracle} from "src/AllKnowingOracle.sol";
import {Book} from "src/Book.sol";
import {DeployScript} from "script/Deploy.s.sol";

contract DeployScriptTest is DeployScript {
    AllKnowingOracle internal testOracle = AllKnowingOracle(address(0));
    FloodRegistry internal testRegistry = FloodRegistry(address(1));
    uint256 internal testSafeBlockThreshold = 100;
    uint256 internal testDisputeBondPct = 10;
    uint256 internal testTradeRebatePct = 10;
    uint256 internal testRelayerRefundPct = 80;
    uint256 internal testFeePct = 300;

    function testDeployOracle(FloodRegistry _registry) public {
        AllKnowingOracle _oracle = deployOracle(_registry);
        assertEq(address(_oracle.registry()), address(_registry), "Oracle constructor should have run correctly");
        assertTrue(_oracle.settlers(address(this)), "Oracle constructor should have run correctly");
    }

    function testDeployBook(
        address _registry,
        address _oracle,
        uint256 _safeBlockThreshold,
        uint256 _disputeBondPct,
        uint256 _tradeRebatePct,
        uint256 _relayerRefundPct,
        uint256 _feePct
    ) public {
        vm.assume(_disputeBondPct <= type(uint256).max / 3);
        vm.assume(_tradeRebatePct <= type(uint256).max / 3);
        vm.assume(_relayerRefundPct <= type(uint256).max / 3);
        vm.assume(_feePct <= 2500);

        if (_tradeRebatePct + _relayerRefundPct + _disputeBondPct != 100) {
            return;
        }
        FloodRegistry(_registry).setOracle(AllKnowingOracle(_oracle));
        Book book = deployBook(
            FloodRegistry(_registry), _safeBlockThreshold, _disputeBondPct, _tradeRebatePct, _relayerRefundPct, _feePct
        );

        assertEq(address(book.registry()), _registry, "Book constructor should have run correctly");
        assertEq(address(book.oracle()), _oracle, "Book constructor should have run correctly");

        assertEq(book.safeBlockThreshold(), _safeBlockThreshold, "Book constructor should have run correctly");

        assertEq(book.disputeBondPct(), _disputeBondPct, "Book constructor should have run correctly");

        assertEq(book.tradeRebatePct(), _tradeRebatePct, "Book constructor should have run correctly");

        assertEq(book.relayerRefundPct(), _relayerRefundPct, "Book constructor should have run correctly");

        assertEq(book.feePct(), _feePct, "Book constructor should have run correctly");
    }

    function testCannotDeployIfFeePctTooHigh(uint256 feePct) public {
        vm.assume(feePct > 2500);
        vm.expectRevert(stdError.assertionError);
        deployBook(
            testRegistry, testSafeBlockThreshold, testDisputeBondPct, testTradeRebatePct, testRelayerRefundPct, feePct
        );
    }

    function testCannotDeployIfSafeThresholdIsZero() public {
        vm.expectRevert(stdError.assertionError);
        deployBook(testRegistry, 0, testDisputeBondPct, testTradeRebatePct, testRelayerRefundPct, testFeePct);
    }

    function testCannotDeployBookIfAnyFeeIsZero() public {
        vm.expectRevert(stdError.assertionError);
        deployBook(testRegistry, testSafeBlockThreshold, 0, testTradeRebatePct, testRelayerRefundPct, testFeePct);
        vm.expectRevert(stdError.assertionError);
        deployBook(testRegistry, testSafeBlockThreshold, testDisputeBondPct, 0, testRelayerRefundPct, testFeePct);
        vm.expectRevert(stdError.assertionError);
        deployBook(testRegistry, testSafeBlockThreshold, testDisputeBondPct, testTradeRebatePct, 0, testFeePct);
    }

    function testWhitelistToken(address _token, bool _enable) public {
        FloodRegistry _registry = deployRegistry();

        if (_enable && _registry.isTokenWhitelisted(_token)) {
            vm.expectRevert(FloodRegistry__TokenAlreadyWhitelisted.selector);
        }
        if (!_enable && !_registry.isTokenWhitelisted(_token)) {
            vm.expectRevert(FloodRegistry__TokenNotWhitelisted.selector);
        }
        whitelistToken(_registry, _token, _enable);
    }
}
