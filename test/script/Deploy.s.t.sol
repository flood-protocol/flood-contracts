// SPDX-License-Identifier: Unlincensed
pragma solidity ^0.8.15;

import "script/Deploy.s.sol";
import "forge-std/Test.sol";

contract DeployTest is DeployScript {
    address internal testOracle = address(0);
    uint256 internal testSafeBlockThreshold = 100;
    uint256 internal testDisputeBondPct = 10;
    uint256 internal testTradeRebatePct = 10;
    uint256 internal testRelayerRefundPct = 80;
    uint256 internal testFeePct = 300;

    function testDeployOracle() public {
        AllKnowingOracle oracle = deployOracle();
        assertTrue(
            address(oracle) != address(0),
            "Oracle address should not be 0"
        );

        assertTrue(
            oracle.settlers(address(this)),
            "Oracle constructor should have run correctly"
        );
    }

    function testDeployBook(
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
        Book book = deployBook(
            _oracle,
            _safeBlockThreshold,
            _disputeBondPct,
            _tradeRebatePct,
            _relayerRefundPct,
            _feePct
        );
        assertTrue(address(book) != address(0), "Book address should not be 0");

        assertEq(
            address(book.oracle()),
            _oracle,
            "Book constructor should have run correctly"
        );

        assertEq(
            book.safeBlockThreshold(),
            _safeBlockThreshold,
            "Book constructor should have run correctly"
        );

        assertEq(
            book.disputeBondPct(),
            _disputeBondPct,
            "Book constructor should have run correctly"
        );

        assertEq(
            book.tradeRebatePct(),
            _tradeRebatePct,
            "Book constructor should have run correctly"
        );

        assertEq(
            book.relayerRefundPct(),
            _relayerRefundPct,
            "Book constructor should have run correctly"
        );

        assertEq(
            book.feePct(),
            _feePct,
            "Book constructor should have run correctly"
        );
    }

    function testCannotDeployIfFeePctTooHigh(uint256 feePct) public {
        vm.assume(feePct > 2500);
        vm.expectRevert(stdError.assertionError);
        deployBook(
            testOracle,
            testSafeBlockThreshold,
            testDisputeBondPct,
            testTradeRebatePct,
            testRelayerRefundPct,
            feePct
        );
    }

    function testCannotDeployIfSafeThresholdIsZero() public {
        vm.expectRevert(stdError.assertionError);
        deployBook(
            testOracle,
            0,
            testDisputeBondPct,
            testTradeRebatePct,
            testRelayerRefundPct,
            testFeePct
        );
    }

    function testCannotDeployBookIfAnyFeeIsZero() public {
        vm.expectRevert(stdError.assertionError);
        deployBook(
            testOracle,
            testSafeBlockThreshold,
            0,
            testTradeRebatePct,
            testRelayerRefundPct,
            testFeePct
        );
        vm.expectRevert(stdError.assertionError);
        deployBook(
            testOracle,
            testSafeBlockThreshold,
            testDisputeBondPct,
            0,
            testRelayerRefundPct,
            testFeePct
        );
        vm.expectRevert(stdError.assertionError);
        deployBook(
            testOracle,
            testSafeBlockThreshold,
            testDisputeBondPct,
            testTradeRebatePct,
            0,
            testFeePct
        );
    }

    function testWhitelistToken(address _token, bool _enable) public {
        AllKnowingOracle oracle = deployOracle();
        Book book = deployBook(
            address(oracle),
            testSafeBlockThreshold,
            testDisputeBondPct,
            testTradeRebatePct,
            testRelayerRefundPct,
            testFeePct
        );
        if (_token == address(0)) {
            vm.expectRevert(stdError.assertionError);
        }
        whitelistTokenForBookAndOracle(oracle, book, _token, _enable);
    }
}
