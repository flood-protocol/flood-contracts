// SPDX-License-Identifier: Unlincensed
pragma solidity ^0.8.15;

import "src/AllKnowingOracle.sol";
import "src/BookSingleChain.sol";
import "forge-std/Script.sol";
import "forge-std/Test.sol";
import {Solenv} from "solenv/Solenv.sol";

contract DeployScript is Script, Test {
    AllKnowingOracle internal oracle;
    BookSingleChain internal book;

    function run() public {
        Solenv.config();
        uint256 safeBlockThreshold = vm.envUint("SAFE_BLOCK_THRESHOLD");
        uint256 disputeBondPct = vm.envUint("DISPUTE_BOND_PCT");
        uint256 tradeRebatePct = vm.envUint("TRADE_REBATE_PCT");
        uint256 relayerRefundPct = vm.envUint("RELAYER_REFUND_PCT");
        address USDC = vm.envAddress("USDC_ADDRESS");
        address WETH = vm.envAddress("WETH_ADDRESS");
        string memory RPC_URL = vm.envString("DEPLOY_RPC_URL");
        vm.createSelectFork(RPC_URL);
        vm.startBroadcast();
        deployOracle();
        deployBook(
            address(oracle),
            safeBlockThreshold,
            disputeBondPct,
            tradeRebatePct,
            relayerRefundPct
        );
        whitelistTokenForBookAndOracle(
            address(oracle),
            address(book),
            USDC,
            true
        );
        whitelistTokenForBookAndOracle(
            address(oracle),
            address(book),
            WETH,
            true
        );
        vm.stopBroadcast();
    }

    function deployOracle() public {
        oracle = new AllKnowingOracle();
    }

    function deployBook(
        address _oracle,
        uint256 _safeBlockThreshold,
        uint256 _disputeBondPct,
        uint256 _tradeRebatePct,
        uint256 _relayerRefundPct
    ) public {
        assertEq(
            _disputeBondPct + _tradeRebatePct + _relayerRefundPct,
            100,
            "invalid invariant"
        );
        assert(_disputeBondPct > 0);
        assert(_tradeRebatePct > 0);
        assert(_relayerRefundPct > 0);
        assert(_safeBlockThreshold > 0);

        book = new BookSingleChain(
            _oracle,
            _safeBlockThreshold,
            _disputeBondPct,
            _tradeRebatePct,
            _relayerRefundPct
        );
        AllKnowingOracle(_oracle).whitelistRequester(address(book), true);
    }

    function whitelistTokenForBookAndOracle(
        address _oracle,
        address _book,
        address _token,
        bool _enable
    ) public {
        AllKnowingOracle(_oracle).whitelistToken(_token, _enable);
        AllKnowingOracle(_book).whitelistToken(_token, _enable);
    }
}
