pub use book_single_chain::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod book_single_chain {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "BookSingleChain was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BOOKSINGLECHAIN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_safeBlockThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_disputeBondPct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_tradeRebatePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_relayerRefundPct\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__AmountOutTooLow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blocksLeft\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__DisputePeriodNotOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__DisputePeriodOver\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__FeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidFeeCombination\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidSignature\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__MaliciousCaller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__SameToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__SentToBlackHole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__TradeNotFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__TradeNotInFillableState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__UnsafeTokenToWhitelist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__ZeroAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"disputeBondPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeRebatePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"relayerRefundPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeeCombinationSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UpdatedFeeForTrade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"disputeBondPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputeTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"traderSignature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTradeWithUpdatedFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAtBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledBy\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isInitialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numberOfTrades\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Request\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onPriceSettled\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayerRefundPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requestTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"safeBlockThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tradeRebatePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"traderSignature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFeeForTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BOOKSINGLECHAIN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x61012060405260006001553480156200001757600080fd5b5060405162001f7c38038062001f7c8339810160408190526200003a9162000156565b600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a3506001600160a01b0385166101005260808490526040518481527f882885d0e4612a71677644a9d70e58ca05fc5a1ea1b0875f6e46c315241bfe149060200160405180910390a180620000ce8385620001ad565b620000da9190620001ad565b606414620000fb5760405163bcf1e5b360e01b815260040160405180910390fd5b60a083905260c082905260e081905260408051848152602081018490529081018290527ff33486d12ebec978385318eaf8163e096679d7eab14d4def8f26b7a5fda0f5829060600160405180910390a15050505050620001d4565b600080600080600060a086880312156200016f57600080fd5b85516001600160a01b03811681146200018757600080fd5b602087015160408801516060890151608090990151929a91995097965090945092505050565b60008219821115620001cf57634e487b7160e01b600052601160045260246000fd5b500190565b60805160a05160c05160e05161010051611d0f6200026d6000396000818161021f015281816103fc015281816106dc01528181610705015281816107e8015261086c0152600081816101d201528181610d6001526110060152600081816102a401526108da0152600081816101ab0152610664015260008181610136015281816105fd01528181610c830152610cd10152611d0f6000f3fe608060405234801561001057600080fd5b506004361061012c5760003560e01c80639501325f116100ad578063cd805d5e11610071578063cd805d5e146102ec578063d70e3dfd146102f5578063daf9c2101461031e578063ee35a4f914610351578063f7b637bb1461036457600080fd5b80639501325f1461026c578063ad3e76251461028c578063c16402bb1461029f578063c3f6f431146102c6578063cb7b1ec8146102d957600080fd5b806353906a59116100f457806353906a59146101cd57806369cf50c1146101f4578063734d1627146102075780637dc0d1d01461021a5780638da5cb5b1461025957600080fd5b80630ff0c00e146101315780630ffb1d8b1461016b57806313af4035146101805780631655b32314610193578063391fe4e2146101a6575b600080fd5b6101587f000000000000000000000000000000000000000000000000000000000000000081565b6040519081526020015b60405180910390f35b61017e6101793660046116ee565b610387565b005b61017e61018e366004611727565b6104f1565b61017e6101a136600461174b565b610585565b6101587f000000000000000000000000000000000000000000000000000000000000000081565b6101587f000000000000000000000000000000000000000000000000000000000000000081565b61017e6102023660046117c5565b6105b5565b61017e610215366004611837565b610861565b6102417f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610162565b600054610241906001600160a01b031681565b61015861027a36600461187b565b60036020526000908152604090205481565b61017e61029a366004611894565b6109d5565b6101587f000000000000000000000000000000000000000000000000000000000000000081565b61017e6102d436600461193d565b610bc9565b61017e6102e73660046117c5565b610c3b565b61015860015481565b61024161030336600461187b565b6004602052600090815260409020546001600160a01b031681565b61034161032c366004611727565b60026020526000908152604090205460ff1681565b6040519015158152602001610162565b61017e61035f366004611a01565b610e00565b61034161037236600461187b565b60056020526000908152604090205460ff1681565b6000546001600160a01b031633146103d55760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b60448201526064015b60405180910390fd5b8080156104695750604051630daf9c2160e41b81526001600160a01b0383811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063daf9c21090602401602060405180830381865afa158015610443573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104679190611ad1565b155b15610492576040516313c42eef60e21b81526001600160a01b03831660048201526024016103cc565b6001600160a01b038216600081815260026020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d92910160405180910390a25050565b6000546001600160a01b0316331461053a5760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b60448201526064016103cc565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b600061059689898989898989610e85565b90506105aa89898989898989888a33610ef3565b505050505050505050565b60006105c688888888888888610e85565b6000818152600360205260408120549192508190036105fb5760405163cc4f06a160e01b8152600481018390526024016103cc565b7f00000000000000000000000000000000000000000000000000000000000000006106268243611b04565b1061064457604051632c02744560e11b815260040160405180910390fd5b6000828152600460205260408120546001600160a01b03169060646106897f00000000000000000000000000000000000000000000000000000000000000008b611b1b565b6106939190611b3a565b60008581526003602090815260408083208390556004825280832080546001600160a01b031916905560059091529020805460ff1916905590506107016001600160a01b038c167f000000000000000000000000000000000000000000000000000000000000000083611058565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f7d3b58b84338f868f8d8d604051602001610765939291909283526001600160a01b03919091166020830152604082015260600190565b6040516020818303038152906040526040518663ffffffff1660e01b8152600401610794959493929190611b5c565b6020604051808303816000875af11580156107b3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107d79190611bdd565b905061080e6001600160a01b038d167f00000000000000000000000000000000000000000000000000000000000000006000611058565b8086846001600160a01b03167f3ce24c6eab720bcebe9baf9d21eee3175218126f896eb40e25675b054f19a40f8760405161084b91815260200190565b60405180910390a4505050505050505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146108ac5760405163179a2eb160e01b81523360048201526024016103cc565b600080806108bd60e0850185611bf6565b8101906108ca9190611c3d565b91945092509050600060646108ff7f000000000000000000000000000000000000000000000000000000000000000086611b1b565b6109099190611b3a565b905061091b60e0860160c08701611c75565b15610959576109546109336040870160208801611727565b826109446080890160608a01611727565b6001600160a01b031691906110d5565b61096e565b61096e83826109446080890160608a01611727565b85826109806040880160208901611727565b6001600160a01b03167f5346d9dd24f7f5e01b23ca4dada136d909acecf7ff9e4ed41474140d5d1319f66109ba60e08a0160c08b01611c75565b604051901515815260200160405180910390a4505050505050565b6001600160a01b03861660009081526002602052604090205460ff16610a195760405163f602627d60e01b81526001600160a01b03871660048201526024016103cc565b6001600160a01b03851660009081526002602052604090205460ff16610a5d5760405163f602627d60e01b81526001600160a01b03861660048201526024016103cc565b846001600160a01b0316866001600160a01b031603610a8f57604051631d2792fb60e31b815260040160405180910390fd5b6703782dace9d90000821115610abb576040516305bf279d60e41b8152600481018390526024016103cc565b831580610ac6575082155b15610ae45760405163abc5ee6f60e01b815260040160405180910390fd5b6001600160a01b038116610b0b57604051631feef77d60e01b815260040160405180910390fd5b60015460408051868152602081018690529081018490526001600160a01b03838116606083015280881691908916907ff4650f30e27746417929b97bf256a6022b15957ffef5971ddbe48867d9e01d459060800160405180910390a46000610b7a878787878787600154610e85565b6000818152600560205260408120805460ff19166001908117909155805492935090610ba583611c92565b90915550610bc090506001600160a01b03881633308861114d565b50505050505050565b6000610bda8c8c8c8c8c8c8c610e85565b9050610be985828686866111d7565b85856001600160a01b03167f6ab91dbc42f726b630639350395426be048c50255f12e82d28e2dffac417459386604051610c2591815260200190565b60405180910390a3505050505050505050505050565b6000610c4c88888888888888610e85565b600081815260036020526040812054919250819003610c815760405163cc4f06a160e01b8152600481018390526024016103cc565b7f0000000000000000000000000000000000000000000000000000000000000000610cac8243611b04565b1015610d1457600082815260036020526040812054610ccb9043611b04565b610cf5907f0000000000000000000000000000000000000000000000000000000000000000611b04565b9050806040516325797e0360e11b81526004016103cc91815260200190565b600082815260046020908152604080832080546003845282852085905581546001600160a01b03191690915560059092528220805460ff191690556001600160a01b0316906064610d857f000000000000000000000000000000000000000000000000000000000000000082611b04565b610d8f908b611b1b565b610d999190611b3a565b9050610daf6001600160a01b038c1683836110d5565b84826001600160a01b03167f32b1eeadbe2d36ad64238ef29d8064aedff6d8150cf1f0c4d6617bae1c00d92685604051610deb91815260200190565b60405180910390a35050505050505050505050565b6000610e118d8d8d8d8d8d8d610e85565b9050610e2085828686866111d7565b86856001600160a01b03167f6ab91dbc42f726b630639350395426be048c50255f12e82d28e2dffac417459386604051610e5c91815260200190565b60405180910390a3610e768d8d8d8d888d8d888e33610ef3565b50505050505050505050505050565b604080516bffffffffffffffffffffffff196060998a1b8116602080840191909152988a1b81166034830152604882019790975260688101959095526088850193909352951b90921660a882015260bc808201949094528151808203909401845260dc019052815191012090565b60008381526005602052604090205460ff16610f2557604051636074a2a360e11b8152600481018490526024016103cc565b60008381526003602052604090205415610f5557604051636074a2a360e11b8152600481018490526024016103cc565b86821015610f765760405163a53754a760e01b815260040160405180910390fd5b6000838152600360209081526040808320439055600482529182902080546001600160a01b0319166001600160a01b0385169081179091558251898152918201859052869290917f942417ccf4f356e8d909c054f8a8147622647605cbeafd9c63b4fc3cc1dd2a53910160405180910390a3610ffd6001600160a01b038a1682878561114d565b6000606461102b7f00000000000000000000000000000000000000000000000000000000000000008b611b1b565b6110359190611b3a565b905061104b6001600160a01b038c1683836110d5565b5050505050505050505050565b600060405163095ea7b360e01b8152836004820152826024820152602060006044836000895af13d15601f3d11600160005114161716915050806110cf5760405162461bcd60e51b815260206004820152600e60248201526d1054141493d59157d1905253115160921b60448201526064016103cc565b50505050565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d11600160005114161716915050806110cf5760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b60448201526064016103cc565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806111d05760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b60448201526064016103cc565b5050505050565b6703782dace9d90000831115611203576040516305bf279d60e41b8152600481018490526024016103cc565b6000848152600360205260409020541561123357604051636074a2a360e11b8152600481018590526024016103cc565b604080517f582b2ba4cf2b931b2e1a054db15a958a1d2222e9e884ffc3c15f79da7d0177ba6020808301919091528183018790526060808301879052835180840390910181526080830184528051908201207f19457468657265756d205369676e6564204d6573736167653a0a33320000000060a084015260bc8084018290528451808503909101815260dc9093019093528151910120600061130c8286868080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061134a92505050565b9050876001600160a01b0316816001600160a01b031614611340576040516324a0dbd760e21b815260040160405180910390fd5b5050505050505050565b6000806000611359858561136e565b91509150611366816113dc565b509392505050565b60008082516041036113a45760208301516040840151606085015160001a61139887828585611595565b945094505050506113d5565b82516040036113cd57602083015160408401516113c2868383611682565b9350935050506113d5565b506000905060025b9250929050565b60008160048111156113f0576113f0611cab565b036113f85750565b600181600481111561140c5761140c611cab565b036114595760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e6174757265000000000000000060448201526064016103cc565b600281600481111561146d5761146d611cab565b036114ba5760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e6774680060448201526064016103cc565b60038160048111156114ce576114ce611cab565b036115265760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b60648201526084016103cc565b600481600481111561153a5761153a611cab565b036115925760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b60648201526084016103cc565b50565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156115cc5750600090506003611679565b8460ff16601b141580156115e457508460ff16601c14155b156115f55750600090506004611679565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611649573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b03811661167257600060019250925050611679565b9150600090505b94509492505050565b6000806001600160ff1b0383168161169f60ff86901c601b611cc1565b90506116ad87828885611595565b935093505050935093915050565b6001600160a01b038116811461159257600080fd5b80356116db816116bb565b919050565b801515811461159257600080fd5b6000806040838503121561170157600080fd5b823561170c816116bb565b9150602083013561171c816116e0565b809150509250929050565b60006020828403121561173957600080fd5b8135611744816116bb565b9392505050565b600080600080600080600080610100898b03121561176857600080fd5b8835611773816116bb565b97506020890135611783816116bb565b965060408901359550606089013594506080890135935060a08901356117a8816116bb565b979a969950949793969295929450505060c08201359160e0013590565b600080600080600080600060e0888a0312156117e057600080fd5b87356117eb816116bb565b965060208801356117fb816116bb565b955060408801359450606088013593506080880135925060a0880135611820816116bb565b8092505060c0880135905092959891949750929550565b6000806040838503121561184a57600080fd5b82359150602083013567ffffffffffffffff81111561186857600080fd5b8301610100818603121561171c57600080fd5b60006020828403121561188d57600080fd5b5035919050565b60008060008060008060c087890312156118ad57600080fd5b86356118b8816116bb565b955060208701356118c8816116bb565b945060408701359350606087013592506080870135915060a08701356118ed816116bb565b809150509295509295509295565b60008083601f84011261190d57600080fd5b50813567ffffffffffffffff81111561192557600080fd5b6020830191508360208285010111156113d557600080fd5b60008060008060008060008060008060006101408c8e03121561195f57600080fd5b8b3561196a816116bb565b9a5060208c013561197a816116bb565b995060408c0135985060608c0135975060808c0135965060a08c013561199f816116bb565b955060c08c0135945060e08c01356119b6816116bb565b93506101008c013592506101208c013567ffffffffffffffff8111156119db57600080fd5b6119e78e828f016118fb565b915080935050809150509295989b509295989b9093969950565b6000806000806000806000806000806000806101608d8f031215611a2457600080fd5b611a2e8d356116bb565b8c359b50611a3f60208e01356116bb565b60208d01359a5060408d0135995060608d0135985060808d01359750611a6760a08e016116d0565b965060c08d0135955060e08d01359450611a846101008e016116d0565b93506101208d0135925067ffffffffffffffff6101408e01351115611aa857600080fd5b611ab98e6101408f01358f016118fb565b81935080925050509295989b509295989b509295989b565b600060208284031215611ae357600080fd5b8151611744816116e0565b634e487b7160e01b600052601160045260246000fd5b600082821015611b1657611b16611aee565b500390565b6000816000190483118215151615611b3557611b35611aee565b500290565b600082611b5757634e487b7160e01b600052601260045260246000fd5b500490565b600060018060a01b038088168352602081881681850152818716604085015285606085015260a06080850152845191508160a085015260005b82811015611bb15785810182015185820160c001528101611b95565b82811115611bc357600060c084870101525b5050601f01601f19169190910160c0019695505050505050565b600060208284031215611bef57600080fd5b5051919050565b6000808335601e19843603018112611c0d57600080fd5b83018035915067ffffffffffffffff821115611c2857600080fd5b6020019150368190038213156113d557600080fd5b600080600060608486031215611c5257600080fd5b833592506020840135611c64816116bb565b929592945050506040919091013590565b600060208284031215611c8757600080fd5b8135611744816116e0565b600060018201611ca457611ca4611aee565b5060010190565b634e487b7160e01b600052602160045260246000fd5b60008219821115611cd457611cd4611aee565b50019056fea264697066735822122069a687f951594f2417b996830924214228e92f674e950e068f910b1a7838c3be64736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct BookSingleChain<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BookSingleChain<M> {
        fn clone(&self) -> Self {
            BookSingleChain(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for BookSingleChain<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BookSingleChain<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BookSingleChain))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> BookSingleChain<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BOOKSINGLECHAIN_ABI.clone(), client)
                .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                BOOKSINGLECHAIN_ABI.clone(),
                BOOKSINGLECHAIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `disputeBondPct` (0x391fe4e2) function"]
        pub fn dispute_bond_pct(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([57, 31, 228, 226], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disputeTrade` (0x69cf50c1) function"]
        pub fn dispute_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 207, 80, 193],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        fee_pct,
                        recipient,
                        trade_index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTrade` (0x1655b323) function"]
        pub fn fill_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            amount_to_send: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [22, 85, 179, 35],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        fee_pct,
                        recipient,
                        trade_index,
                        amount_to_send,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTradeWithUpdatedFee` (0xee35a4f9) function"]
        pub fn fill_trade_with_updated_fee(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            amount_to_send: ethers::core::types::U256,
            trader: ethers::core::types::Address,
            new_fee_pct: ethers::core::types::U256,
            trader_signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [238, 53, 164, 249],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        fee_pct,
                        recipient,
                        trade_index,
                        amount_to_send,
                        trader,
                        new_fee_pct,
                        trader_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledAtBlock` (0x9501325f) function"]
        pub fn filled_at_block(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([149, 1, 50, 95], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledBy` (0xd70e3dfd) function"]
        pub fn filled_by(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([215, 14, 61, 253], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isInitialized` (0xf7b637bb) function"]
        pub fn is_initialized(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([247, 182, 55, 187], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numberOfTrades` (0xcd805d5e) function"]
        pub fn number_of_trades(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([205, 128, 93, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onPriceSettled` (0x734d1627) function"]
        pub fn on_price_settled(
            &self,
            id: [u8; 32],
            request: Request,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 77, 22, 39], (id, request))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `relayerRefundPct` (0x53906a59) function"]
        pub fn relayer_refund_pct(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([83, 144, 106, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `requestTrade` (0xad3e7625) function"]
        pub fn request_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [173, 62, 118, 37],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        fee_pct,
                        recipient,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeBlockThreshold` (0x0ff0c00e) function"]
        pub fn safe_block_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 240, 192, 14], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOwner` (0x13af4035) function"]
        pub fn set_owner(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settleTrade` (0xcb7b1ec8) function"]
        pub fn settle_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 123, 30, 200],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        fee_pct,
                        recipient,
                        trade_index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tradeRebatePct` (0xc16402bb) function"]
        pub fn trade_rebate_pct(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([193, 100, 2, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFeeForTrade` (0xc3f6f431) function"]
        pub fn update_fee_for_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            trader: ethers::core::types::Address,
            new_fee_pct: ethers::core::types::U256,
            trader_signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [195, 246, 244, 49],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        fee_pct,
                        recipient,
                        trade_index,
                        trader,
                        new_fee_pct,
                        trader_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `whitelistToken` (0x0ffb1d8b) function"]
        pub fn whitelist_token(
            &self,
            token: ethers::core::types::Address,
            whitelisted: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 251, 29, 139], (token, whitelisted))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `whitelistedTokens` (0xdaf9c210) function"]
        pub fn whitelisted_tokens(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([218, 249, 194, 16], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `FeeCombinationSet` event"]
        pub fn fee_combination_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FeeCombinationSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerUpdated` event"]
        pub fn owner_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SafeBlockThresholdSet` event"]
        pub fn safe_block_threshold_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SafeBlockThresholdSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenWhitelisted` event"]
        pub fn token_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeDisputeSettled` event"]
        pub fn trade_dispute_settled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeDisputeSettledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeDisputed` event"]
        pub fn trade_disputed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeDisputedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeFilled` event"]
        pub fn trade_filled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeFilledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeRequested` event"]
        pub fn trade_requested_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeRequestedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeSettled` event"]
        pub fn trade_settled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeSettledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdatedFeeForTrade` event"]
        pub fn updated_fee_for_trade_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdatedFeeForTradeFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, BookSingleChainEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for BookSingleChain<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "FeeCombinationSet",
        abi = "FeeCombinationSet(uint256,uint256,uint256)"
    )]
    pub struct FeeCombinationSetFilter {
        pub dispute_bond_pct: ethers::core::types::U256,
        pub trade_rebate_pct: ethers::core::types::U256,
        pub relayer_refund_pct: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "OwnerUpdated", abi = "OwnerUpdated(address,address)")]
    pub struct OwnerUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "SafeBlockThresholdSet", abi = "SafeBlockThresholdSet(uint256)")]
    pub struct SafeBlockThresholdSetFilter {
        pub new_safe_block_threshold: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TokenWhitelisted", abi = "TokenWhitelisted(address,bool)")]
    pub struct TokenWhitelistedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub whitelisted: bool,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "TradeDisputeSettled",
        abi = "TradeDisputeSettled(address,uint256,bytes32,bool)"
    )]
    pub struct TradeDisputeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub answer: bool,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "TradeDisputed",
        abi = "TradeDisputed(address,uint256,bytes32,uint256)"
    )]
    pub struct TradeDisputedFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub filled_at_block: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "TradeFilled",
        abi = "TradeFilled(address,uint256,uint256,uint256)"
    )]
    pub struct TradeFilledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub amount_out: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "TradeRequested",
        abi = "TradeRequested(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct TradeRequestedFilter {
        #[ethevent(indexed)]
        pub token_in: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TradeSettled", abi = "TradeSettled(address,uint256,uint256)")]
    pub struct TradeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub filled_at_block: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "UpdatedFeeForTrade",
        abi = "UpdatedFeeForTrade(address,uint256,uint256)"
    )]
    pub struct UpdatedFeeForTradeFilter {
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub new_fee_pct: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BookSingleChainEvents {
        FeeCombinationSetFilter(FeeCombinationSetFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        SafeBlockThresholdSetFilter(SafeBlockThresholdSetFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
        TradeDisputeSettledFilter(TradeDisputeSettledFilter),
        TradeDisputedFilter(TradeDisputedFilter),
        TradeFilledFilter(TradeFilledFilter),
        TradeRequestedFilter(TradeRequestedFilter),
        TradeSettledFilter(TradeSettledFilter),
        UpdatedFeeForTradeFilter(UpdatedFeeForTradeFilter),
    }
    impl ethers::contract::EthLogDecode for BookSingleChainEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FeeCombinationSetFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::FeeCombinationSetFilter(decoded));
            }
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SafeBlockThresholdSetFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::SafeBlockThresholdSetFilter(decoded));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TokenWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = TradeDisputeSettledFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeDisputeSettledFilter(decoded));
            }
            if let Ok(decoded) = TradeDisputedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeDisputedFilter(decoded));
            }
            if let Ok(decoded) = TradeFilledFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeFilledFilter(decoded));
            }
            if let Ok(decoded) = TradeRequestedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeRequestedFilter(decoded));
            }
            if let Ok(decoded) = TradeSettledFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeSettledFilter(decoded));
            }
            if let Ok(decoded) = UpdatedFeeForTradeFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::UpdatedFeeForTradeFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BookSingleChainEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookSingleChainEvents::FeeCombinationSetFilter(element) => element.fmt(f),
                BookSingleChainEvents::OwnerUpdatedFilter(element) => element.fmt(f),
                BookSingleChainEvents::SafeBlockThresholdSetFilter(element) => element.fmt(f),
                BookSingleChainEvents::TokenWhitelistedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeDisputeSettledFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeDisputedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeFilledFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeRequestedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeSettledFilter(element) => element.fmt(f),
                BookSingleChainEvents::UpdatedFeeForTradeFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `disputeBondPct` function with signature `disputeBondPct()` and selector `[57, 31, 228, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "disputeBondPct", abi = "disputeBondPct()")]
    pub struct DisputeBondPctCall;
    #[doc = "Container type for all input parameters for the `disputeTrade` function with signature `disputeTrade(address,address,uint256,uint256,uint256,address,uint256)` and selector `[105, 207, 80, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "disputeTrade",
        abi = "disputeTrade(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct DisputeTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fillTrade` function with signature `fillTrade(address,address,uint256,uint256,uint256,address,uint256,uint256)` and selector `[22, 85, 179, 35]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "fillTrade",
        abi = "fillTrade(address,address,uint256,uint256,uint256,address,uint256,uint256)"
    )]
    pub struct FillTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub amount_to_send: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fillTradeWithUpdatedFee` function with signature `fillTradeWithUpdatedFee(address,address,uint256,uint256,uint256,address,uint256,uint256,address,uint256,bytes)` and selector `[238, 53, 164, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "fillTradeWithUpdatedFee",
        abi = "fillTradeWithUpdatedFee(address,address,uint256,uint256,uint256,address,uint256,uint256,address,uint256,bytes)"
    )]
    pub struct FillTradeWithUpdatedFeeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub amount_to_send: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
        pub new_fee_pct: ethers::core::types::U256,
        pub trader_signature: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `filledAtBlock` function with signature `filledAtBlock(bytes32)` and selector `[149, 1, 50, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "filledAtBlock", abi = "filledAtBlock(bytes32)")]
    pub struct FilledAtBlockCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `filledBy` function with signature `filledBy(bytes32)` and selector `[215, 14, 61, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "filledBy", abi = "filledBy(bytes32)")]
    pub struct FilledByCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `isInitialized` function with signature `isInitialized(bytes32)` and selector `[247, 182, 55, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isInitialized", abi = "isInitialized(bytes32)")]
    pub struct IsInitializedCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `numberOfTrades` function with signature `numberOfTrades()` and selector `[205, 128, 93, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "numberOfTrades", abi = "numberOfTrades()")]
    pub struct NumberOfTradesCall;
    #[doc = "Container type for all input parameters for the `onPriceSettled` function with signature `onPriceSettled(bytes32,(address,address,address,address,uint256,uint8,bool,bytes))` and selector `[115, 77, 22, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "onPriceSettled",
        abi = "onPriceSettled(bytes32,(address,address,address,address,uint256,uint8,bool,bytes))"
    )]
    pub struct OnPriceSettledCall {
        pub id: [u8; 32],
        pub request: Request,
    }
    #[doc = "Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `relayerRefundPct` function with signature `relayerRefundPct()` and selector `[83, 144, 106, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "relayerRefundPct", abi = "relayerRefundPct()")]
    pub struct RelayerRefundPctCall;
    #[doc = "Container type for all input parameters for the `requestTrade` function with signature `requestTrade(address,address,uint256,uint256,uint256,address)` and selector `[173, 62, 118, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "requestTrade",
        abi = "requestTrade(address,address,uint256,uint256,uint256,address)"
    )]
    pub struct RequestTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `safeBlockThreshold` function with signature `safeBlockThreshold()` and selector `[15, 240, 192, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "safeBlockThreshold", abi = "safeBlockThreshold()")]
    pub struct SafeBlockThresholdCall;
    #[doc = "Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `settleTrade` function with signature `settleTrade(address,address,uint256,uint256,uint256,address,uint256)` and selector `[203, 123, 30, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "settleTrade",
        abi = "settleTrade(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct SettleTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `tradeRebatePct` function with signature `tradeRebatePct()` and selector `[193, 100, 2, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tradeRebatePct", abi = "tradeRebatePct()")]
    pub struct TradeRebatePctCall;
    #[doc = "Container type for all input parameters for the `updateFeeForTrade` function with signature `updateFeeForTrade(address,address,uint256,uint256,uint256,address,uint256,address,uint256,bytes)` and selector `[195, 246, 244, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "updateFeeForTrade",
        abi = "updateFeeForTrade(address,address,uint256,uint256,uint256,address,uint256,address,uint256,bytes)"
    )]
    pub struct UpdateFeeForTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
        pub new_fee_pct: ethers::core::types::U256,
        pub trader_signature: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `whitelistToken` function with signature `whitelistToken(address,bool)` and selector `[15, 251, 29, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "whitelistToken", abi = "whitelistToken(address,bool)")]
    pub struct WhitelistTokenCall {
        pub token: ethers::core::types::Address,
        pub whitelisted: bool,
    }
    #[doc = "Container type for all input parameters for the `whitelistedTokens` function with signature `whitelistedTokens(address)` and selector `[218, 249, 194, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "whitelistedTokens", abi = "whitelistedTokens(address)")]
    pub struct WhitelistedTokensCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BookSingleChainCalls {
        DisputeBondPct(DisputeBondPctCall),
        DisputeTrade(DisputeTradeCall),
        FillTrade(FillTradeCall),
        FillTradeWithUpdatedFee(FillTradeWithUpdatedFeeCall),
        FilledAtBlock(FilledAtBlockCall),
        FilledBy(FilledByCall),
        IsInitialized(IsInitializedCall),
        NumberOfTrades(NumberOfTradesCall),
        OnPriceSettled(OnPriceSettledCall),
        Oracle(OracleCall),
        Owner(OwnerCall),
        RelayerRefundPct(RelayerRefundPctCall),
        RequestTrade(RequestTradeCall),
        SafeBlockThreshold(SafeBlockThresholdCall),
        SetOwner(SetOwnerCall),
        SettleTrade(SettleTradeCall),
        TradeRebatePct(TradeRebatePctCall),
        UpdateFeeForTrade(UpdateFeeForTradeCall),
        WhitelistToken(WhitelistTokenCall),
        WhitelistedTokens(WhitelistedTokensCall),
    }
    impl ethers::core::abi::AbiDecode for BookSingleChainCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DisputeBondPctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::DisputeBondPct(decoded));
            }
            if let Ok(decoded) =
                <DisputeTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::DisputeTrade(decoded));
            }
            if let Ok(decoded) =
                <FillTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FillTrade(decoded));
            }
            if let Ok(decoded) =
                <FillTradeWithUpdatedFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FillTradeWithUpdatedFee(decoded));
            }
            if let Ok(decoded) =
                <FilledAtBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FilledAtBlock(decoded));
            }
            if let Ok(decoded) =
                <FilledByCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FilledBy(decoded));
            }
            if let Ok(decoded) =
                <IsInitializedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::IsInitialized(decoded));
            }
            if let Ok(decoded) =
                <NumberOfTradesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::NumberOfTrades(decoded));
            }
            if let Ok(decoded) =
                <OnPriceSettledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::OnPriceSettled(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::Oracle(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RelayerRefundPctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::RelayerRefundPct(decoded));
            }
            if let Ok(decoded) =
                <RequestTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::RequestTrade(decoded));
            }
            if let Ok(decoded) =
                <SafeBlockThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SafeBlockThreshold(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SetOwner(decoded));
            }
            if let Ok(decoded) =
                <SettleTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SettleTrade(decoded));
            }
            if let Ok(decoded) =
                <TradeRebatePctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::TradeRebatePct(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeForTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::UpdateFeeForTrade(decoded));
            }
            if let Ok(decoded) =
                <WhitelistTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::WhitelistToken(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::WhitelistedTokens(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BookSingleChainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BookSingleChainCalls::DisputeBondPct(element) => element.encode(),
                BookSingleChainCalls::DisputeTrade(element) => element.encode(),
                BookSingleChainCalls::FillTrade(element) => element.encode(),
                BookSingleChainCalls::FillTradeWithUpdatedFee(element) => element.encode(),
                BookSingleChainCalls::FilledAtBlock(element) => element.encode(),
                BookSingleChainCalls::FilledBy(element) => element.encode(),
                BookSingleChainCalls::IsInitialized(element) => element.encode(),
                BookSingleChainCalls::NumberOfTrades(element) => element.encode(),
                BookSingleChainCalls::OnPriceSettled(element) => element.encode(),
                BookSingleChainCalls::Oracle(element) => element.encode(),
                BookSingleChainCalls::Owner(element) => element.encode(),
                BookSingleChainCalls::RelayerRefundPct(element) => element.encode(),
                BookSingleChainCalls::RequestTrade(element) => element.encode(),
                BookSingleChainCalls::SafeBlockThreshold(element) => element.encode(),
                BookSingleChainCalls::SetOwner(element) => element.encode(),
                BookSingleChainCalls::SettleTrade(element) => element.encode(),
                BookSingleChainCalls::TradeRebatePct(element) => element.encode(),
                BookSingleChainCalls::UpdateFeeForTrade(element) => element.encode(),
                BookSingleChainCalls::WhitelistToken(element) => element.encode(),
                BookSingleChainCalls::WhitelistedTokens(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BookSingleChainCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookSingleChainCalls::DisputeBondPct(element) => element.fmt(f),
                BookSingleChainCalls::DisputeTrade(element) => element.fmt(f),
                BookSingleChainCalls::FillTrade(element) => element.fmt(f),
                BookSingleChainCalls::FillTradeWithUpdatedFee(element) => element.fmt(f),
                BookSingleChainCalls::FilledAtBlock(element) => element.fmt(f),
                BookSingleChainCalls::FilledBy(element) => element.fmt(f),
                BookSingleChainCalls::IsInitialized(element) => element.fmt(f),
                BookSingleChainCalls::NumberOfTrades(element) => element.fmt(f),
                BookSingleChainCalls::OnPriceSettled(element) => element.fmt(f),
                BookSingleChainCalls::Oracle(element) => element.fmt(f),
                BookSingleChainCalls::Owner(element) => element.fmt(f),
                BookSingleChainCalls::RelayerRefundPct(element) => element.fmt(f),
                BookSingleChainCalls::RequestTrade(element) => element.fmt(f),
                BookSingleChainCalls::SafeBlockThreshold(element) => element.fmt(f),
                BookSingleChainCalls::SetOwner(element) => element.fmt(f),
                BookSingleChainCalls::SettleTrade(element) => element.fmt(f),
                BookSingleChainCalls::TradeRebatePct(element) => element.fmt(f),
                BookSingleChainCalls::UpdateFeeForTrade(element) => element.fmt(f),
                BookSingleChainCalls::WhitelistToken(element) => element.fmt(f),
                BookSingleChainCalls::WhitelistedTokens(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DisputeBondPctCall> for BookSingleChainCalls {
        fn from(var: DisputeBondPctCall) -> Self {
            BookSingleChainCalls::DisputeBondPct(var)
        }
    }
    impl ::std::convert::From<DisputeTradeCall> for BookSingleChainCalls {
        fn from(var: DisputeTradeCall) -> Self {
            BookSingleChainCalls::DisputeTrade(var)
        }
    }
    impl ::std::convert::From<FillTradeCall> for BookSingleChainCalls {
        fn from(var: FillTradeCall) -> Self {
            BookSingleChainCalls::FillTrade(var)
        }
    }
    impl ::std::convert::From<FillTradeWithUpdatedFeeCall> for BookSingleChainCalls {
        fn from(var: FillTradeWithUpdatedFeeCall) -> Self {
            BookSingleChainCalls::FillTradeWithUpdatedFee(var)
        }
    }
    impl ::std::convert::From<FilledAtBlockCall> for BookSingleChainCalls {
        fn from(var: FilledAtBlockCall) -> Self {
            BookSingleChainCalls::FilledAtBlock(var)
        }
    }
    impl ::std::convert::From<FilledByCall> for BookSingleChainCalls {
        fn from(var: FilledByCall) -> Self {
            BookSingleChainCalls::FilledBy(var)
        }
    }
    impl ::std::convert::From<IsInitializedCall> for BookSingleChainCalls {
        fn from(var: IsInitializedCall) -> Self {
            BookSingleChainCalls::IsInitialized(var)
        }
    }
    impl ::std::convert::From<NumberOfTradesCall> for BookSingleChainCalls {
        fn from(var: NumberOfTradesCall) -> Self {
            BookSingleChainCalls::NumberOfTrades(var)
        }
    }
    impl ::std::convert::From<OnPriceSettledCall> for BookSingleChainCalls {
        fn from(var: OnPriceSettledCall) -> Self {
            BookSingleChainCalls::OnPriceSettled(var)
        }
    }
    impl ::std::convert::From<OracleCall> for BookSingleChainCalls {
        fn from(var: OracleCall) -> Self {
            BookSingleChainCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for BookSingleChainCalls {
        fn from(var: OwnerCall) -> Self {
            BookSingleChainCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RelayerRefundPctCall> for BookSingleChainCalls {
        fn from(var: RelayerRefundPctCall) -> Self {
            BookSingleChainCalls::RelayerRefundPct(var)
        }
    }
    impl ::std::convert::From<RequestTradeCall> for BookSingleChainCalls {
        fn from(var: RequestTradeCall) -> Self {
            BookSingleChainCalls::RequestTrade(var)
        }
    }
    impl ::std::convert::From<SafeBlockThresholdCall> for BookSingleChainCalls {
        fn from(var: SafeBlockThresholdCall) -> Self {
            BookSingleChainCalls::SafeBlockThreshold(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for BookSingleChainCalls {
        fn from(var: SetOwnerCall) -> Self {
            BookSingleChainCalls::SetOwner(var)
        }
    }
    impl ::std::convert::From<SettleTradeCall> for BookSingleChainCalls {
        fn from(var: SettleTradeCall) -> Self {
            BookSingleChainCalls::SettleTrade(var)
        }
    }
    impl ::std::convert::From<TradeRebatePctCall> for BookSingleChainCalls {
        fn from(var: TradeRebatePctCall) -> Self {
            BookSingleChainCalls::TradeRebatePct(var)
        }
    }
    impl ::std::convert::From<UpdateFeeForTradeCall> for BookSingleChainCalls {
        fn from(var: UpdateFeeForTradeCall) -> Self {
            BookSingleChainCalls::UpdateFeeForTrade(var)
        }
    }
    impl ::std::convert::From<WhitelistTokenCall> for BookSingleChainCalls {
        fn from(var: WhitelistTokenCall) -> Self {
            BookSingleChainCalls::WhitelistToken(var)
        }
    }
    impl ::std::convert::From<WhitelistedTokensCall> for BookSingleChainCalls {
        fn from(var: WhitelistedTokensCall) -> Self {
            BookSingleChainCalls::WhitelistedTokens(var)
        }
    }
    #[doc = "Container type for all return fields from the `disputeBondPct` function with signature `disputeBondPct()` and selector `[57, 31, 228, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DisputeBondPctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `filledAtBlock` function with signature `filledAtBlock(bytes32)` and selector `[149, 1, 50, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FilledAtBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `filledBy` function with signature `filledBy(bytes32)` and selector `[215, 14, 61, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FilledByReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isInitialized` function with signature `isInitialized(bytes32)` and selector `[247, 182, 55, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsInitializedReturn(pub bool);
    #[doc = "Container type for all return fields from the `numberOfTrades` function with signature `numberOfTrades()` and selector `[205, 128, 93, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NumberOfTradesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `relayerRefundPct` function with signature `relayerRefundPct()` and selector `[83, 144, 106, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RelayerRefundPctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `safeBlockThreshold` function with signature `safeBlockThreshold()` and selector `[15, 240, 192, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SafeBlockThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `tradeRebatePct` function with signature `tradeRebatePct()` and selector `[193, 100, 2, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TradeRebatePctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `whitelistedTokens` function with signature `whitelistedTokens(address)` and selector `[218, 249, 194, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct WhitelistedTokensReturn(pub bool);
    #[doc = "`Request(address,address,address,address,uint256,uint8,bool,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Request {
        pub requester: ethers::core::types::Address,
        pub proposer: ethers::core::types::Address,
        pub disputer: ethers::core::types::Address,
        pub currency: ethers::core::types::Address,
        pub bond: ethers::core::types::U256,
        pub state: u8,
        pub answer: bool,
        pub data: ethers::core::types::Bytes,
    }
}
