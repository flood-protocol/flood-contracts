// SPDX-License-Identifier: Unlincensed
pragma solidity ^0.8.15;

import "src/AllKnowingOracle.sol";
import "src/Book.sol";
import "forge-std/Script.sol";
import "forge-std/Test.sol";

contract DeployScript is Script, Test {
    AllKnowingOracle internal oracle;

    function run() public {
        uint256 safeBlockThreshold = vm.envUint("SAFE_BLOCK_THRESHOLD");
        uint256 disputeBondPct = vm.envUint("DISPUTE_BOND_PCT");
        uint256 tradeRebatePct = vm.envUint("TRADE_REBATE_PCT");
        uint256 relayerRefundPct = vm.envUint("RELAYER_REFUND_PCT");
        uint256 feePct = vm.envUint("FEE_PCT");
        address USDC = vm.envAddress("USDC_ADDRESS");
        address WETH = vm.envAddress("WETH_ADDRESS");
        address DAI = vm.envAddress("DAI_ADDRESS");
        address WBTC = vm.envAddress("WBTC_ADDRESS");
        address USDT = vm.envAddress("USDT_ADDRESS");
        address ORACLE_ADDRESS = vm.envAddress("ORACLE_ADDRESS");
        vm.startBroadcast();

        if (ORACLE_ADDRESS == address(0)) {
            oracle = deployOracle();
        } else {
            oracle = AllKnowingOracle(ORACLE_ADDRESS);
        }
        Book book = deployBook(
            address(oracle),
            safeBlockThreshold,
            disputeBondPct,
            tradeRebatePct,
            relayerRefundPct,
            feePct
        );
        whitelistTokenForBookAndOracle(oracle, book, USDC, true);
        whitelistTokenForBookAndOracle(oracle, book, WETH, true);
        whitelistTokenForBookAndOracle(oracle, book, DAI, true);
        whitelistTokenForBookAndOracle(oracle, book, WBTC, true);
        whitelistTokenForBookAndOracle(oracle, book, USDT, true);
        vm.stopBroadcast();
    }

    function deployOracle() public returns (AllKnowingOracle newOracle) {
        newOracle = new AllKnowingOracle();
    }

    function deployBook(
        address _oracle,
        uint256 _safeBlockThreshold,
        uint256 _disputeBondPct,
        uint256 _tradeRebatePct,
        uint256 _relayerRefundPct,
        uint256 _feePct
    ) public returns (Book book) {
        assertEq(
            _disputeBondPct + _tradeRebatePct + _relayerRefundPct,
            100,
            "invalid invariant"
        );
        assert(_feePct < 2500);
        assert(_disputeBondPct > 0);
        assert(_tradeRebatePct > 0);
        assert(_relayerRefundPct > 0);
        assert(_safeBlockThreshold > 0);

        book = new Book(
            _oracle,
            _safeBlockThreshold,
            _disputeBondPct,
            _tradeRebatePct,
            _relayerRefundPct,
            _feePct
        );
        AllKnowingOracle(_oracle).whitelistRequester(address(book), true);
    }

    function whitelistTokenForBookAndOracle(
        AllKnowingOracle _oracle,
        Book _book,
        address _token,
        bool _enable
    ) public {
        assert(_token != address(0));
        _oracle.whitelistToken(_token, _enable);
        _book.whitelistToken(_token, _enable);
    }
}
