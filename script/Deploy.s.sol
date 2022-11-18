// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "src/AllKnowingOracle.sol";
import "src/Book.sol";
import "src/FloodRegistry.sol";
import "forge-std/Script.sol";
import "forge-std/Test.sol";

contract DeployScript is Script, Test {
    AllKnowingOracle internal oracle;
    FloodRegistry internal registry;

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
        address REGISTRY_ADDRESS = vm.envAddress("REGISTRY_ADDRESS");
        address ORACLE_ADDRESS = vm.envAddress("ORACLE_ADDRESS");
        vm.startBroadcast();

        if (REGISTRY_ADDRESS == address(0)) {  registry = deployRegistry(); } else {
            registry = FloodRegistry(REGISTRY_ADDRESS);
        }

        if (ORACLE_ADDRESS == address(0)) {
            oracle = deployOracle(registry);
        } else {
            oracle = AllKnowingOracle(ORACLE_ADDRESS);
        }
        registry.setOracle(oracle);
        Book book =
            deployBook(registry,safeBlockThreshold, disputeBondPct, tradeRebatePct, relayerRefundPct, feePct);
        whitelistToken(registry, USDC, true);
        whitelistToken(registry,WETH, true);
        whitelistToken(registry,DAI, true);
        whitelistToken(registry,WBTC, true);
        whitelistToken(registry,USDT, true);
        vm.stopBroadcast();
    }

    function deployOracle(FloodRegistry _registry) public returns (AllKnowingOracle newOracle) {
        newOracle = new AllKnowingOracle(_registry);
    }

      function deployRegistry() public returns (FloodRegistry newRegistry) {
        newRegistry = new FloodRegistry();
    }

    function deployBook(
        FloodRegistry _registry,
        uint256 _safeBlockThreshold,
        uint256 _disputeBondPct,
        uint256 _tradeRebatePct,
        uint256 _relayerRefundPct,
        uint256 _feePct
    ) public returns (Book book) {
        assertEq(_disputeBondPct + _tradeRebatePct + _relayerRefundPct, 100, "invalid invariant");
        assert(_feePct < 2500);
        assert(_disputeBondPct > 0);
        assert(_tradeRebatePct > 0);
        assert(_relayerRefundPct > 0);
        assert(_safeBlockThreshold > 0);

        book = new Book(
            _registry,
            _safeBlockThreshold,
            _disputeBondPct,
            _tradeRebatePct,
            _relayerRefundPct,
            _feePct
        );
        registry.latestOracle().whitelistRequester(address(book), true);
    }

    function whitelistToken(FloodRegistry _registry,address _token, bool _enable)
        public
    {
        _registry.whitelistToken(_token, _enable);
    }
}
