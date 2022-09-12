pub use book::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod book {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Book was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BOOK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_safeBlockThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_disputeBondPct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_tradeRebatePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_relayerRefundPct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_feePct\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__AmountOutTooLow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blocksLeft\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__DisputePeriodNotOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__DisputePeriodOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__FeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__InvalidFeeCombination\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__InvalidToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__MaliciousCaller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__NotTrader\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__SameToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__SentToBlackHole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotCancelable\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotInFillableState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__UnsafeTokenToWhitelist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__ZeroAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"disputeBondPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeRebatePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"relayerRefundPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeeCombinationSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeePctSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeCancelled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"disputeBondPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputeTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAtBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledBy\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numberOfTrades\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Request\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onPriceSettled\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayerRefundPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requestTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"safeBlockThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"status\",\"outputs\":[{\"internalType\":\"enum TradeStatus\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tradeRebatePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BOOK_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x61014060405260006001553480156200001757600080fd5b50604051620018d6380380620018d68339810160408190526200003a91620001b4565b600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a3506001600160a01b0386166101205260808590526040518581527f882885d0e4612a71677644a9d70e58ca05fc5a1ea1b0875f6e46c315241bfe149060200160405180910390a181620000ce848662000215565b620000da919062000215565b606414620000fb57604051636683d88d60e01b815260040160405180910390fd5b60a084905260c083905260e082905260408051858152602081018590529081018390527ff33486d12ebec978385318eaf8163e096679d7eab14d4def8f26b7a5fda0f5829060600160405180910390a16109c48111156200016f5760405163b768880160e01b815260040160405180910390fd5b6101008190526040518181527f9e67c173f0d1bf66a955764a6b072d74e095af185e539f9f6570fb91d788fca59060200160405180910390a15050505050506200023c565b60008060008060008060c08789031215620001ce57600080fd5b86516001600160a01b0381168114620001e657600080fd5b6020880151604089015160608a015160808b015160a0909b0151939c929b509099909850965090945092505050565b600082198211156200023757634e487b7160e01b600052601160045260246000fd5b500190565b60805160a05160c05160e05161010051610120516115fd620002d96000396000818161024f01528181610482015281816104ab0152818161058e015281816106770152610968015260006102e70152600081816102150152818161090c0152610e4c01526000818161030e01526109d60152600081816101be015261043901526000818161014b01528181610ddf0152610f6c01526115fd6000f3fe608060405234801561001057600080fd5b506004361061012c5760003560e01c80638b2cdb9d116100ad578063c16402bb11610071578063c16402bb14610309578063cd805d5e14610330578063d70e3dfd14610339578063daf9c21014610362578063e66a34ee1461039557600080fd5b80638b2cdb9d146102895780638da5cb5b1461029c5780639170c05b146102af5780639501325f146102c2578063a02cf937146102e257600080fd5b8063391fe4e2116100f4578063391fe4e2146101b957806352ad0d5e146101e057806353906a5914610210578063734d1627146102375780637dc0d1d01461024a57600080fd5b806309796dff146101315780630ff0c00e146101465780630ffb1d8b1461018057806313af4035146101935780632d7e8e1c146101a6575b600080fd5b61014461013f366004611169565b6103a8565b005b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b6040519081526020015b60405180910390f35b61014461018e3660046111f2565b610607565b6101446101a136600461122b565b61076c565b6101446101b436600461124f565b610800565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b6102036101ee3660046112d4565b60056020526000908152604090205460ff1681565b6040516101779190611303565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b61014461024536600461132b565b61095d565b6102717f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610177565b610144610297366004611169565b610ad1565b600054610271906001600160a01b031681565b6101446102bd36600461136f565b610ba7565b61016d6102d03660046112d4565b60036020526000908152604090205481565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b61016d60015481565b6102716103473660046112d4565b6004602052600090815260409020546001600160a01b031681565b61038561037036600461122b565b60026020526000908152604090205460ff1681565b6040519015158152602001610177565b6101446103a3366004611169565b610d6c565b60006103b988888888888888610eec565b6000818152600360205260408120549192508190036103f357604051630f70cbe360e41b8152600481018390526024015b60405180910390fd5b6103fc81610f64565b61041957604051632fe1403160e21b815260040160405180910390fd5b6000828152600460205260408120546001600160a01b031690606461045e7f00000000000000000000000000000000000000000000000000000000000000008b6113e4565b6104689190611403565b905061047384610f98565b6104a76001600160a01b038c167f000000000000000000000000000000000000000000000000000000000000000083610fd2565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f7d3b58b84338f868f8e8e60405160200161050b939291909283526001600160a01b03919091166020830152604082015260600190565b6040516020818303038152906040526040518663ffffffff1660e01b815260040161053a959493929190611425565b6020604051808303816000875af1158015610559573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061057d91906114a6565b90506105b46001600160a01b038d167f00000000000000000000000000000000000000000000000000000000000000006000610fd2565b8087846001600160a01b03167f3ce24c6eab720bcebe9baf9d21eee3175218126f896eb40e25675b054f19a40f876040516105f191815260200190565b60405180910390a4505050505050505050505050565b6000546001600160a01b031633146106505760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b60448201526064016103ea565b8080156106e45750604051630daf9c2160e41b81526001600160a01b0383811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063daf9c21090602401602060405180830381865afa1580156106be573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106e291906114bf565b155b1561070d576040516335e4dbd960e11b81526001600160a01b03831660048201526024016103ea565b6001600160a01b038216600081815260026020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d92910160405180910390a25050565b6000546001600160a01b031633146107b55760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b60448201526064016103ea565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b600061081189898989898989610eec565b9050600160008281526005602052604090205460ff166002811115610838576108386112ed565b14610859576040516337d6ee8b60e11b8152600481018290526024016103ea565b8582101561087a5760405163122e33cf60e31b815260040160405180910390fd5b6000818152600360209081526040808320439055600482528083208054336001600160a01b031990911681179091556005835292819020805460ff19166002179055518481528692917f243be33d9fe04f6f36f3db44a174c069e89c89148c15c7b95df48c25bdf2bba5910160405180910390a36109036001600160a01b03891633878561104f565b600060646109317f00000000000000000000000000000000000000000000000000000000000000008a6113e4565b61093b9190611403565b90506109516001600160a01b038b1633836110d9565b50505050505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109a85760405163347b0c3360e11b81523360048201526024016103ea565b600080806109b960e08501856114dc565b8101906109c6919061152a565b91945092509050600060646109fb7f0000000000000000000000000000000000000000000000000000000000000000866113e4565b610a059190611403565b9050610a1760e0860160c08701611562565b15610a5557610a50610a2f604087016020880161122b565b82610a406080890160608a0161122b565b6001600160a01b031691906110d9565b610a6a565b610a6a8382610a406080890160608a0161122b565b8582610a7c604088016020890161122b565b6001600160a01b03167f5346d9dd24f7f5e01b23ca4dada136d909acecf7ff9e4ed41474140d5d1319f6610ab660e08a0160c08b01611562565b604051901515815260200160405180910390a4505050505050565b336001600160a01b03821614610afa57604051630e020a8960e31b815260040160405180910390fd5b6000610b0b88888888888888610eec565b9050600160008281526005602052604090205460ff166002811115610b3257610b326112ed565b14610b535760405163069572a960e21b8152600481018290526024016103ea565b610b5c81610f98565b604051819084907f68db4a22e668d59f89604eaaf8ffcf0b9af0703c4f481b08926850bb6633f9b490600090a3610b9d6001600160a01b03891683886110d9565b5050505050505050565b6001600160a01b03851660009081526002602052604090205460ff16610beb576040516378abcf6760e11b81526001600160a01b03861660048201526024016103ea565b6001600160a01b03841660009081526002602052604090205460ff16610c2f576040516378abcf6760e11b81526001600160a01b03851660048201526024016103ea565b836001600160a01b0316856001600160a01b031603610c6157604051636f8f06d360e01b815260040160405180910390fd5b821580610c6c575081155b15610c8a576040516305dc4f8360e21b815260040160405180910390fd5b6001600160a01b038116610cb1576040516325f5bdf760e21b815260040160405180910390fd5b60015460408051858152602081018590526001600160a01b03848116828401523360608301529151878316928916917f8a923a6574cd5d908fa4e840c47fb64b874b2116a88d073e5fdbd83403393276919081900360800190a46000610d1e868686868660015433610eec565b6000818152600560205260408120805460ff19166001908117909155805492935090610d498361157f565b90915550610d6490506001600160a01b03871633308761104f565b505050505050565b6000610d7d88888888888888610eec565b600081815260036020526040812054919250819003610db257604051630f70cbe360e41b8152600481018390526024016103ea565b610dbb81610f64565b15610e2257600082815260036020526040812054610dd99043611598565b610e03907f0000000000000000000000000000000000000000000000000000000000000000611598565b905080604051637be8522f60e01b81526004016103ea91815260200190565b6000828152600460205260409020546001600160a01b0316610e4383610f98565b60006064610e717f000000000000000000000000000000000000000000000000000000000000000082611598565b610e7b908b6113e4565b610e859190611403565b9050610e9b6001600160a01b038c1683836110d9565b85826001600160a01b03167f32b1eeadbe2d36ad64238ef29d8064aedff6d8150cf1f0c4d6617bae1c00d92685604051610ed791815260200190565b60405180910390a35050505050505050505050565b6040516bffffffffffffffffffffffff19606089811b8216602084015288811b82166034840152604883018890526068830187905285811b82166088840152609c830185905283901b1660bc82015260009060d001604051602081830303815290604052805190602001209050979650505050505050565b6000610f90827f00000000000000000000000000000000000000000000000000000000000000006115af565b431092915050565b60009081526003602090815260408083208390556004825280832080546001600160a01b031916905560059091529020805460ff19169055565b600060405163095ea7b360e01b8152836004820152826024820152602060006044836000895af13d15601f3d11600160005114161716915050806110495760405162461bcd60e51b815260206004820152600e60248201526d1054141493d59157d1905253115160921b60448201526064016103ea565b50505050565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806110d25760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b60448201526064016103ea565b5050505050565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d11600160005114161716915050806110495760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b60448201526064016103ea565b6001600160a01b038116811461116657600080fd5b50565b600080600080600080600060e0888a03121561118457600080fd5b873561118f81611151565b9650602088013561119f81611151565b9550604088013594506060880135935060808801356111bd81611151565b925060a0880135915060c08801356111d481611151565b8091505092959891949750929550565b801515811461116657600080fd5b6000806040838503121561120557600080fd5b823561121081611151565b91506020830135611220816111e4565b809150509250929050565b60006020828403121561123d57600080fd5b813561124881611151565b9392505050565b600080600080600080600080610100898b03121561126c57600080fd5b883561127781611151565b9750602089013561128781611151565b9650604089013595506060890135945060808901356112a581611151565b935060a0890135925060c08901356112bc81611151565b8092505060e089013590509295985092959890939650565b6000602082840312156112e657600080fd5b5035919050565b634e487b7160e01b600052602160045260246000fd5b602081016003831061132557634e487b7160e01b600052602160045260246000fd5b91905290565b6000806040838503121561133e57600080fd5b82359150602083013567ffffffffffffffff81111561135c57600080fd5b8301610100818603121561122057600080fd5b600080600080600060a0868803121561138757600080fd5b853561139281611151565b945060208601356113a281611151565b9350604086013592506060860135915060808601356113c081611151565b809150509295509295909350565b634e487b7160e01b600052601160045260246000fd5b60008160001904831182151516156113fe576113fe6113ce565b500290565b60008261142057634e487b7160e01b600052601260045260246000fd5b500490565b600060018060a01b038088168352602081881681850152818716604085015285606085015260a06080850152845191508160a085015260005b8281101561147a5785810182015185820160c00152810161145e565b8281111561148c57600060c084870101525b5050601f01601f19169190910160c0019695505050505050565b6000602082840312156114b857600080fd5b5051919050565b6000602082840312156114d157600080fd5b8151611248816111e4565b6000808335601e198436030181126114f357600080fd5b83018035915067ffffffffffffffff82111561150e57600080fd5b60200191503681900382131561152357600080fd5b9250929050565b60008060006060848603121561153f57600080fd5b83359250602084013561155181611151565b929592945050506040919091013590565b60006020828403121561157457600080fd5b8135611248816111e4565b600060018201611591576115916113ce565b5060010190565b6000828210156115aa576115aa6113ce565b500390565b600082198211156115c2576115c26113ce565b50019056fea26469706673582212203778c204836bcb9d98d537d0945f24b0cb1fc07a34ffa3b301c873cf1553491664736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct Book<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Book<M> {
        fn clone(&self) -> Self {
            Book(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Book<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Book<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Book))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Book<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BOOK_ABI.clone(), client).into()
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
                BOOK_ABI.clone(),
                BOOK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `cancelTrade` (0x8b2cdb9d) function"]
        pub fn cancel_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            trader: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [139, 44, 219, 157],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        recipient,
                        trade_index,
                        trader,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disputeBondPct` (0x391fe4e2) function"]
        pub fn dispute_bond_pct(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([57, 31, 228, 226], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disputeTrade` (0x09796dff) function"]
        pub fn dispute_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            trader: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [9, 121, 109, 255],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        recipient,
                        trade_index,
                        trader,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feePct` (0xa02cf937) function"]
        pub fn fee_pct(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([160, 44, 249, 55], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTrade` (0x2d7e8e1c) function"]
        pub fn fill_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            trader: ethers::core::types::Address,
            amount_to_send: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [45, 126, 142, 28],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        recipient,
                        trade_index,
                        trader,
                        amount_to_send,
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
        #[doc = "Calls the contract's `requestTrade` (0x9170c05b) function"]
        pub fn request_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [145, 112, 192, 91],
                    (token_in, token_out, amount_in, min_amount_out, recipient),
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
        #[doc = "Calls the contract's `settleTrade` (0xe66a34ee) function"]
        pub fn settle_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            trader: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [230, 106, 52, 238],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        recipient,
                        trade_index,
                        trader,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `status` (0x52ad0d5e) function"]
        pub fn status(&self, p0: [u8; 32]) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([82, 173, 13, 94], p0)
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
        #[doc = "Gets the contract's `FeePctSet` event"]
        pub fn fee_pct_set_filter(&self) -> ethers::contract::builders::Event<M, FeePctSetFilter> {
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
        #[doc = "Gets the contract's `TradeCancelled` event"]
        pub fn trade_cancelled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeCancelledFilter> {
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
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, BookEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Book<M> {
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
    #[ethevent(name = "FeePctSet", abi = "FeePctSet(uint256)")]
    pub struct FeePctSetFilter {
        pub fee_pct: ethers::core::types::U256,
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
    #[ethevent(name = "TradeCancelled", abi = "TradeCancelled(uint256,bytes32)")]
    pub struct TradeCancelledFilter {
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trade_id: [u8; 32],
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
    #[ethevent(name = "TradeFilled", abi = "TradeFilled(address,uint256,uint256)")]
    pub struct TradeFilledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
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
        abi = "TradeRequested(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct TradeRequestedFilter {
        #[ethevent(indexed)]
        pub token_in: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BookEvents {
        FeeCombinationSetFilter(FeeCombinationSetFilter),
        FeePctSetFilter(FeePctSetFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        SafeBlockThresholdSetFilter(SafeBlockThresholdSetFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
        TradeCancelledFilter(TradeCancelledFilter),
        TradeDisputeSettledFilter(TradeDisputeSettledFilter),
        TradeDisputedFilter(TradeDisputedFilter),
        TradeFilledFilter(TradeFilledFilter),
        TradeRequestedFilter(TradeRequestedFilter),
        TradeSettledFilter(TradeSettledFilter),
    }
    impl ethers::contract::EthLogDecode for BookEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FeeCombinationSetFilter::decode_log(log) {
                return Ok(BookEvents::FeeCombinationSetFilter(decoded));
            }
            if let Ok(decoded) = FeePctSetFilter::decode_log(log) {
                return Ok(BookEvents::FeePctSetFilter(decoded));
            }
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(BookEvents::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SafeBlockThresholdSetFilter::decode_log(log) {
                return Ok(BookEvents::SafeBlockThresholdSetFilter(decoded));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(BookEvents::TokenWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = TradeCancelledFilter::decode_log(log) {
                return Ok(BookEvents::TradeCancelledFilter(decoded));
            }
            if let Ok(decoded) = TradeDisputeSettledFilter::decode_log(log) {
                return Ok(BookEvents::TradeDisputeSettledFilter(decoded));
            }
            if let Ok(decoded) = TradeDisputedFilter::decode_log(log) {
                return Ok(BookEvents::TradeDisputedFilter(decoded));
            }
            if let Ok(decoded) = TradeFilledFilter::decode_log(log) {
                return Ok(BookEvents::TradeFilledFilter(decoded));
            }
            if let Ok(decoded) = TradeRequestedFilter::decode_log(log) {
                return Ok(BookEvents::TradeRequestedFilter(decoded));
            }
            if let Ok(decoded) = TradeSettledFilter::decode_log(log) {
                return Ok(BookEvents::TradeSettledFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BookEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookEvents::FeeCombinationSetFilter(element) => element.fmt(f),
                BookEvents::FeePctSetFilter(element) => element.fmt(f),
                BookEvents::OwnerUpdatedFilter(element) => element.fmt(f),
                BookEvents::SafeBlockThresholdSetFilter(element) => element.fmt(f),
                BookEvents::TokenWhitelistedFilter(element) => element.fmt(f),
                BookEvents::TradeCancelledFilter(element) => element.fmt(f),
                BookEvents::TradeDisputeSettledFilter(element) => element.fmt(f),
                BookEvents::TradeDisputedFilter(element) => element.fmt(f),
                BookEvents::TradeFilledFilter(element) => element.fmt(f),
                BookEvents::TradeRequestedFilter(element) => element.fmt(f),
                BookEvents::TradeSettledFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `cancelTrade` function with signature `cancelTrade(address,address,uint256,uint256,address,uint256,address)` and selector `[139, 44, 219, 157]`"]
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
        name = "cancelTrade",
        abi = "cancelTrade(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct CancelTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `disputeTrade` function with signature `disputeTrade(address,address,uint256,uint256,address,uint256,address)` and selector `[9, 121, 109, 255]`"]
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
        abi = "disputeTrade(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct DisputeTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `feePct` function with signature `feePct()` and selector `[160, 44, 249, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "feePct", abi = "feePct()")]
    pub struct FeePctCall;
    #[doc = "Container type for all input parameters for the `fillTrade` function with signature `fillTrade(address,address,uint256,uint256,address,uint256,address,uint256)` and selector `[45, 126, 142, 28]`"]
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
        abi = "fillTrade(address,address,uint256,uint256,address,uint256,address,uint256)"
    )]
    pub struct FillTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
        pub amount_to_send: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `requestTrade` function with signature `requestTrade(address,address,uint256,uint256,address)` and selector `[145, 112, 192, 91]`"]
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
        abi = "requestTrade(address,address,uint256,uint256,address)"
    )]
    pub struct RequestTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `settleTrade` function with signature `settleTrade(address,address,uint256,uint256,address,uint256,address)` and selector `[230, 106, 52, 238]`"]
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
        abi = "settleTrade(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct SettleTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `status` function with signature `status(bytes32)` and selector `[82, 173, 13, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "status", abi = "status(bytes32)")]
    pub struct StatusCall(pub [u8; 32]);
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
    pub enum BookCalls {
        CancelTrade(CancelTradeCall),
        DisputeBondPct(DisputeBondPctCall),
        DisputeTrade(DisputeTradeCall),
        FeePct(FeePctCall),
        FillTrade(FillTradeCall),
        FilledAtBlock(FilledAtBlockCall),
        FilledBy(FilledByCall),
        NumberOfTrades(NumberOfTradesCall),
        OnPriceSettled(OnPriceSettledCall),
        Oracle(OracleCall),
        Owner(OwnerCall),
        RelayerRefundPct(RelayerRefundPctCall),
        RequestTrade(RequestTradeCall),
        SafeBlockThreshold(SafeBlockThresholdCall),
        SetOwner(SetOwnerCall),
        SettleTrade(SettleTradeCall),
        Status(StatusCall),
        TradeRebatePct(TradeRebatePctCall),
        WhitelistToken(WhitelistTokenCall),
        WhitelistedTokens(WhitelistedTokensCall),
    }
    impl ethers::core::abi::AbiDecode for BookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CancelTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::CancelTrade(decoded));
            }
            if let Ok(decoded) =
                <DisputeBondPctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::DisputeBondPct(decoded));
            }
            if let Ok(decoded) =
                <DisputeTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::DisputeTrade(decoded));
            }
            if let Ok(decoded) = <FeePctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::FeePct(decoded));
            }
            if let Ok(decoded) =
                <FillTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::FillTrade(decoded));
            }
            if let Ok(decoded) =
                <FilledAtBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::FilledAtBlock(decoded));
            }
            if let Ok(decoded) =
                <FilledByCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::FilledBy(decoded));
            }
            if let Ok(decoded) =
                <NumberOfTradesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::NumberOfTrades(decoded));
            }
            if let Ok(decoded) =
                <OnPriceSettledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::OnPriceSettled(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::Oracle(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RelayerRefundPctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::RelayerRefundPct(decoded));
            }
            if let Ok(decoded) =
                <RequestTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::RequestTrade(decoded));
            }
            if let Ok(decoded) =
                <SafeBlockThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::SafeBlockThreshold(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::SetOwner(decoded));
            }
            if let Ok(decoded) =
                <SettleTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::SettleTrade(decoded));
            }
            if let Ok(decoded) = <StatusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::Status(decoded));
            }
            if let Ok(decoded) =
                <TradeRebatePctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::TradeRebatePct(decoded));
            }
            if let Ok(decoded) =
                <WhitelistTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::WhitelistToken(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::WhitelistedTokens(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BookCalls::CancelTrade(element) => element.encode(),
                BookCalls::DisputeBondPct(element) => element.encode(),
                BookCalls::DisputeTrade(element) => element.encode(),
                BookCalls::FeePct(element) => element.encode(),
                BookCalls::FillTrade(element) => element.encode(),
                BookCalls::FilledAtBlock(element) => element.encode(),
                BookCalls::FilledBy(element) => element.encode(),
                BookCalls::NumberOfTrades(element) => element.encode(),
                BookCalls::OnPriceSettled(element) => element.encode(),
                BookCalls::Oracle(element) => element.encode(),
                BookCalls::Owner(element) => element.encode(),
                BookCalls::RelayerRefundPct(element) => element.encode(),
                BookCalls::RequestTrade(element) => element.encode(),
                BookCalls::SafeBlockThreshold(element) => element.encode(),
                BookCalls::SetOwner(element) => element.encode(),
                BookCalls::SettleTrade(element) => element.encode(),
                BookCalls::Status(element) => element.encode(),
                BookCalls::TradeRebatePct(element) => element.encode(),
                BookCalls::WhitelistToken(element) => element.encode(),
                BookCalls::WhitelistedTokens(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BookCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookCalls::CancelTrade(element) => element.fmt(f),
                BookCalls::DisputeBondPct(element) => element.fmt(f),
                BookCalls::DisputeTrade(element) => element.fmt(f),
                BookCalls::FeePct(element) => element.fmt(f),
                BookCalls::FillTrade(element) => element.fmt(f),
                BookCalls::FilledAtBlock(element) => element.fmt(f),
                BookCalls::FilledBy(element) => element.fmt(f),
                BookCalls::NumberOfTrades(element) => element.fmt(f),
                BookCalls::OnPriceSettled(element) => element.fmt(f),
                BookCalls::Oracle(element) => element.fmt(f),
                BookCalls::Owner(element) => element.fmt(f),
                BookCalls::RelayerRefundPct(element) => element.fmt(f),
                BookCalls::RequestTrade(element) => element.fmt(f),
                BookCalls::SafeBlockThreshold(element) => element.fmt(f),
                BookCalls::SetOwner(element) => element.fmt(f),
                BookCalls::SettleTrade(element) => element.fmt(f),
                BookCalls::Status(element) => element.fmt(f),
                BookCalls::TradeRebatePct(element) => element.fmt(f),
                BookCalls::WhitelistToken(element) => element.fmt(f),
                BookCalls::WhitelistedTokens(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CancelTradeCall> for BookCalls {
        fn from(var: CancelTradeCall) -> Self {
            BookCalls::CancelTrade(var)
        }
    }
    impl ::std::convert::From<DisputeBondPctCall> for BookCalls {
        fn from(var: DisputeBondPctCall) -> Self {
            BookCalls::DisputeBondPct(var)
        }
    }
    impl ::std::convert::From<DisputeTradeCall> for BookCalls {
        fn from(var: DisputeTradeCall) -> Self {
            BookCalls::DisputeTrade(var)
        }
    }
    impl ::std::convert::From<FeePctCall> for BookCalls {
        fn from(var: FeePctCall) -> Self {
            BookCalls::FeePct(var)
        }
    }
    impl ::std::convert::From<FillTradeCall> for BookCalls {
        fn from(var: FillTradeCall) -> Self {
            BookCalls::FillTrade(var)
        }
    }
    impl ::std::convert::From<FilledAtBlockCall> for BookCalls {
        fn from(var: FilledAtBlockCall) -> Self {
            BookCalls::FilledAtBlock(var)
        }
    }
    impl ::std::convert::From<FilledByCall> for BookCalls {
        fn from(var: FilledByCall) -> Self {
            BookCalls::FilledBy(var)
        }
    }
    impl ::std::convert::From<NumberOfTradesCall> for BookCalls {
        fn from(var: NumberOfTradesCall) -> Self {
            BookCalls::NumberOfTrades(var)
        }
    }
    impl ::std::convert::From<OnPriceSettledCall> for BookCalls {
        fn from(var: OnPriceSettledCall) -> Self {
            BookCalls::OnPriceSettled(var)
        }
    }
    impl ::std::convert::From<OracleCall> for BookCalls {
        fn from(var: OracleCall) -> Self {
            BookCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for BookCalls {
        fn from(var: OwnerCall) -> Self {
            BookCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RelayerRefundPctCall> for BookCalls {
        fn from(var: RelayerRefundPctCall) -> Self {
            BookCalls::RelayerRefundPct(var)
        }
    }
    impl ::std::convert::From<RequestTradeCall> for BookCalls {
        fn from(var: RequestTradeCall) -> Self {
            BookCalls::RequestTrade(var)
        }
    }
    impl ::std::convert::From<SafeBlockThresholdCall> for BookCalls {
        fn from(var: SafeBlockThresholdCall) -> Self {
            BookCalls::SafeBlockThreshold(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for BookCalls {
        fn from(var: SetOwnerCall) -> Self {
            BookCalls::SetOwner(var)
        }
    }
    impl ::std::convert::From<SettleTradeCall> for BookCalls {
        fn from(var: SettleTradeCall) -> Self {
            BookCalls::SettleTrade(var)
        }
    }
    impl ::std::convert::From<StatusCall> for BookCalls {
        fn from(var: StatusCall) -> Self {
            BookCalls::Status(var)
        }
    }
    impl ::std::convert::From<TradeRebatePctCall> for BookCalls {
        fn from(var: TradeRebatePctCall) -> Self {
            BookCalls::TradeRebatePct(var)
        }
    }
    impl ::std::convert::From<WhitelistTokenCall> for BookCalls {
        fn from(var: WhitelistTokenCall) -> Self {
            BookCalls::WhitelistToken(var)
        }
    }
    impl ::std::convert::From<WhitelistedTokensCall> for BookCalls {
        fn from(var: WhitelistedTokensCall) -> Self {
            BookCalls::WhitelistedTokens(var)
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
    #[doc = "Container type for all return fields from the `feePct` function with signature `feePct()` and selector `[160, 44, 249, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FeePctReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `status` function with signature `status(bytes32)` and selector `[82, 173, 13, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct StatusReturn(pub u8);
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
}
