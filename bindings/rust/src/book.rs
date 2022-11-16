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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"contract FloodRegistry\",\"name\":\"_registry\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_safeBlockThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_disputeBondPct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_tradeRebatePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_relayerRefundPct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_feePct\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__AmountOutTooLow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blocksLeft\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__DisputePeriodNotOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__DisputePeriodOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__FeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__InvalidFeeCombination\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__InvalidToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__MaliciousCaller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__NotTrader\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__SameToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__SentToBlackHole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotCancelable\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotInFillableState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__ZeroAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"disputeBondPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeRebatePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"relayerRefundPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeeCombinationSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeePctSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeCancelled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeDisputeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"disputeBondPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputeTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAtBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledBy\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numberOfTrades\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Request\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onPriceSettled\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"registry\",\"outputs\":[{\"internalType\":\"contract FloodRegistry\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayerRefundPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requestTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"safeBlockThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"status\",\"outputs\":[{\"internalType\":\"enum TradeStatus\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tradeRebatePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static BOOK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BOOK_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x61016060405260006001553480156200001757600080fd5b50604051620018c7380380620018c78339810160408190526200003a91620001d5565b600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a3506001600160a01b038781166101205286166101405260808590526040518581527f882885d0e4612a71677644a9d70e58ca05fc5a1ea1b0875f6e46c315241bfe149060200160405180910390a181620000d5848662000241565b620000e1919062000241565b6064146200010257604051636683d88d60e01b815260040160405180910390fd5b60a084905260c083905260e082905260408051858152602081018590529081018390527ff33486d12ebec978385318eaf8163e096679d7eab14d4def8f26b7a5fda0f5829060600160405180910390a16109c4811115620001765760405163b768880160e01b815260040160405180910390fd5b6101008190526040518181527f9e67c173f0d1bf66a955764a6b072d74e095af185e539f9f6570fb91d788fca59060200160405180910390a15050505050505062000269565b6001600160a01b0381168114620001d257600080fd5b50565b600080600080600080600060e0888a031215620001f157600080fd5b8751620001fe81620001bc565b60208901519097506200021181620001bc565b604089015160608a015160808b015160a08c015160c0909c01519a9d939c50919a90999198509650945092505050565b808201808211156200026357634e487b7160e01b600052601160045260246000fd5b92915050565b60805160a05160c05160e0516101005161012051610140516115af620003186000396000818161027b01528181610496015281816104bf015281816105ac015261083201526000818161023c01528181610fe8015261109a015260006102fb015260008181610202015281816107d60152610c88015260008181610322015261089f0152600081816101ab015261044d01526000818161014b01528181610c1b0152610db201526115af6000f3fe608060405234801561001057600080fd5b506004361061012c5760003560e01c80638b2cdb9d116100ad578063c16402bb11610071578063c16402bb1461031d578063cd805d5e14610344578063d70e3dfd1461034d578063daf9c21014610376578063e66a34ee146103a957600080fd5b80638b2cdb9d1461029d5780638da5cb5b146102b05780639170c05b146102c35780639501325f146102d6578063a02cf937146102f657600080fd5b806352ad0d5e116100f457806352ad0d5e146101cd57806353906a59146101fd578063734d1627146102245780637b103999146102375780637dc0d1d01461027657600080fd5b806309796dff146101315780630ff0c00e1461014657806313af4035146101805780632d7e8e1c14610193578063391fe4e2146101a6575b600080fd5b61014461013f366004611149565b6103bc565b005b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b6040519081526020015b60405180910390f35b61014461018e3660046111c4565b61062a565b6101446101a13660046111e8565b6106be565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b6101f06101db36600461126d565b60056020526000908152604090205460ff1681565b604051610177919061129c565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b6101446102323660046112c4565b610827565b61025e7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610177565b61025e7f000000000000000000000000000000000000000000000000000000000000000081565b6101446102ab366004611149565b6109b0565b60005461025e906001600160a01b031681565b6101446102d1366004611313565b610a91565b61016d6102e436600461126d565b60036020526000908152604090205481565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b61016d60015481565b61025e61035b36600461126d565b6004602052600090815260409020546001600160a01b031681565b6103996103843660046111c4565b60026020526000908152604090205460ff1681565b6040519015158152602001610177565b6101446103b7366004611149565b610ba8565b60006103cd88888888888888610d32565b60008181526003602052604081205491925081900361040757604051630f70cbe360e41b8152600481018390526024015b60405180910390fd5b61041081610daa565b61042d57604051632fe1403160e21b815260040160405180910390fd5b6000828152600460205260408120546001600160a01b03169060646104727f00000000000000000000000000000000000000000000000000000000000000008b611388565b61047c91906113a5565b905061048784610dde565b6104bb6001600160a01b038c167f000000000000000000000000000000000000000000000000000000000000000083610e18565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f7d3b58b84338f868f8e8e8e60405160200161052994939291909384526001600160a01b039283166020850152604084019190915216606082015260800190565b6040516020818303038152906040526040518663ffffffff1660e01b81526004016105589594939291906113c7565b6020604051808303816000875af1158015610577573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061059b9190611441565b90506105d26001600160a01b038d167f00000000000000000000000000000000000000000000000000000000000000006000610e18565b604080516001600160a01b0385811682526020820187905288169183918a917f611d2790eebc2dfbd3329eb8ce89302d94d2b42c489cfd3ba8dae42977a3f942910160405180910390a4505050505050505050505050565b6000546001600160a01b031633146106735760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b60448201526064016103fe565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b60006106cf89898989898989610d32565b9050600160008281526005602052604090205460ff1660028111156106f6576106f6611286565b14610717576040516337d6ee8b60e11b8152600481018290526024016103fe565b858210156107385760405163122e33cf60e31b815260040160405180910390fd5b6000818152600360209081526040808320439055600482528083208054336001600160a01b031990911681179091556005835292819020805460ff19166002179055518481526001600160a01b03861692879290917f81f3dddde62cb8d590d391999bc1a4c362a03cf74718fb3bd8195b46fc1c1009910160405180910390a46107cd6001600160a01b038916338785610e95565b600060646107fb7f00000000000000000000000000000000000000000000000000000000000000008a611388565b61080591906113a5565b905061081b6001600160a01b038b163383610f1f565b50505050505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146108725760405163347b0c3360e11b81523360048201526024016103fe565b600080808061088460e086018661145a565b81019061089191906114a8565b9350935093509350600060647f0000000000000000000000000000000000000000000000000000000000000000866108c99190611388565b6108d391906113a5565b90506108e560e0870160c08801611500565b156109235761091e6108fd60408801602089016111c4565b8261090e60808a0160608b016111c4565b6001600160a01b03169190610f1f565b610938565b610938848261090e60808a0160608b016111c4565b6001600160a01b03821687847ff27fbe6db06bb8ac8a2b4206eef25be7c007d9fd8d53a772c5ffa4d7487952b261097560408b0160208c016111c4565b61098560e08c0160c08d01611500565b604080516001600160a01b03909316835290151560208301520160405180910390a450505050505050565b336001600160a01b038216146109d957604051630e020a8960e31b815260040160405180910390fd5b60006109ea88888888888888610d32565b9050600160008281526005602052604090205460ff166002811115610a1157610a11611286565b14610a325760405163069572a960e21b8152600481018290526024016103fe565b610a3b81610dde565b816001600160a01b031681847f4aaea9d289fcf796ee2d93ab7730c9a0afa79d03debf6528dd7f13a83613b7f460405160405180910390a4610a876001600160a01b0389168388610f1f565b5050505050505050565b610a9b8585610f97565b821580610aa6575081155b15610ac4576040516305dc4f8360e21b815260040160405180910390fd5b6001600160a01b038116610aeb576040516325f5bdf760e21b815260040160405180910390fd5b600154604080516001600160a01b0387811682526020820187905291810185905283821660608201523392918816907f8a923a6574cd5d908fa4e840c47fb64b874b2116a88d073e5fdbd834033932769060800160405180910390a46000610b5a868686868660015433610d32565b6000818152600560205260408120805460ff19166001908117909155805492935090610b858361151d565b90915550610ba090506001600160a01b038716333087610e95565b505050505050565b6000610bb988888888888888610d32565b600081815260036020526040812054919250819003610bee57604051630f70cbe360e41b8152600481018390526024016103fe565b610bf781610daa565b15610c5e57600082815260036020526040812054610c159043611536565b610c3f907f0000000000000000000000000000000000000000000000000000000000000000611536565b905080604051637be8522f60e01b81526004016103fe91815260200190565b6000828152600460205260409020546001600160a01b0316610c7f83610dde565b60006064610cad7f000000000000000000000000000000000000000000000000000000000000000082611536565b610cb7908b611388565b610cc191906113a5565b9050610cd76001600160a01b038c168383610f1f565b846001600160a01b031686836001600160a01b03167fdeb6ee1bbbaaac1d6b7873a679d462110dd54d72915436622cc00525c2561cab86604051610d1d91815260200190565b60405180910390a45050505050505050505050565b6040516bffffffffffffffffffffffff19606089811b8216602084015288811b82166034840152604883018890526068830187905285811b82166088840152609c830185905283901b1660bc82015260009060d001604051602081830303815290604052805190602001209050979650505050505050565b6000610dd6827f0000000000000000000000000000000000000000000000000000000000000000611549565b431092915050565b60009081526003602090815260408083208390556004825280832080546001600160a01b031916905560059091529020805460ff19169055565b600060405163095ea7b360e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080610e8f5760405162461bcd60e51b815260206004820152600e60248201526d1054141493d59157d1905253115160921b60448201526064016103fe565b50505050565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080610f185760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b60448201526064016103fe565b5050505050565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080610e8f5760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b60448201526064016103fe565b806001600160a01b0316826001600160a01b031603610fc957604051636f8f06d360e01b815260040160405180910390fd5b60405163b5af090f60e01b81526001600160a01b0383811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063b5af090f90602401602060405180830381865afa15801561102f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611053919061155c565b61107b576040516378abcf6760e11b81526001600160a01b03831660048201526024016103fe565b60405163b5af090f60e01b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063b5af090f90602401602060405180830381865afa1580156110e1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611105919061155c565b61112d576040516378abcf6760e11b81526001600160a01b03821660048201526024016103fe565b5050565b6001600160a01b038116811461114657600080fd5b50565b600080600080600080600060e0888a03121561116457600080fd5b873561116f81611131565b9650602088013561117f81611131565b95506040880135945060608801359350608088013561119d81611131565b925060a0880135915060c08801356111b481611131565b8091505092959891949750929550565b6000602082840312156111d657600080fd5b81356111e181611131565b9392505050565b600080600080600080600080610100898b03121561120557600080fd5b883561121081611131565b9750602089013561122081611131565b96506040890135955060608901359450608089013561123e81611131565b935060a0890135925060c089013561125581611131565b8092505060e089013590509295985092959890939650565b60006020828403121561127f57600080fd5b5035919050565b634e487b7160e01b600052602160045260246000fd5b60208101600383106112be57634e487b7160e01b600052602160045260246000fd5b91905290565b600080604083850312156112d757600080fd5b82359150602083013567ffffffffffffffff8111156112f557600080fd5b8301610100818603121561130857600080fd5b809150509250929050565b600080600080600060a0868803121561132b57600080fd5b853561133681611131565b9450602086013561134681611131565b93506040860135925060608601359150608086013561136481611131565b809150509295509295909350565b634e487b7160e01b600052601160045260246000fd5b808202811582820484141761139f5761139f611372565b92915050565b6000826113c257634e487b7160e01b600052601260045260246000fd5b500490565b600060018060a01b038088168352602081881681850152818716604085015285606085015260a06080850152845191508160a085015260005b8281101561141c5785810182015185820160c001528101611400565b5050600060c0828501015260c0601f19601f8301168401019150509695505050505050565b60006020828403121561145357600080fd5b5051919050565b6000808335601e1984360301811261147157600080fd5b83018035915067ffffffffffffffff82111561148c57600080fd5b6020019150368190038213156114a157600080fd5b9250929050565b600080600080608085870312156114be57600080fd5b8435935060208501356114d081611131565b92506040850135915060608501356114e781611131565b939692955090935050565b801515811461114657600080fd5b60006020828403121561151257600080fd5b81356111e1816114f2565b60006001820161152f5761152f611372565b5060010190565b8181038181111561139f5761139f611372565b8082018082111561139f5761139f611372565b60006020828403121561156e57600080fd5b81516111e1816114f256fea2646970667358221220a3ff25d9f9b7e4c65671283052fbed5612d45409cc544965a2dafc3dd160a0fb64736f6c63430008110033" . parse () . expect ("invalid bytecode")
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
    impl<M> std::fmt::Debug for Book<M> {
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
        #[doc = "Calls the contract's `registry` (0x7b103999) function"]
        pub fn registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([123, 16, 57, 153], ())
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
    #[doc = "Custom Error type `Book__AmountOutTooLow` with signature `Book__AmountOutTooLow()` and selector `[145, 113, 158, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__AmountOutTooLow", abi = "Book__AmountOutTooLow()")]
    pub struct Book__AmountOutTooLow;
    #[doc = "Custom Error type `Book__DisputePeriodNotOver` with signature `Book__DisputePeriodNotOver(uint256)` and selector `[123, 232, 82, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "Book__DisputePeriodNotOver",
        abi = "Book__DisputePeriodNotOver(uint256)"
    )]
    pub struct Book__DisputePeriodNotOver {
        pub blocks_left: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `Book__DisputePeriodOver` with signature `Book__DisputePeriodOver()` and selector `[191, 133, 0, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__DisputePeriodOver", abi = "Book__DisputePeriodOver()")]
    pub struct Book__DisputePeriodOver;
    #[doc = "Custom Error type `Book__FeePctTooHigh` with signature `Book__FeePctTooHigh()` and selector `[183, 104, 136, 1]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__FeePctTooHigh", abi = "Book__FeePctTooHigh()")]
    pub struct Book__FeePctTooHigh;
    #[doc = "Custom Error type `Book__InvalidFeeCombination` with signature `Book__InvalidFeeCombination()` and selector `[102, 131, 216, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "Book__InvalidFeeCombination",
        abi = "Book__InvalidFeeCombination()"
    )]
    pub struct Book__InvalidFeeCombination;
    #[doc = "Custom Error type `Book__InvalidToken` with signature `Book__InvalidToken(address)` and selector `[241, 87, 158, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__InvalidToken", abi = "Book__InvalidToken(address)")]
    pub struct Book__InvalidToken {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Custom Error type `Book__MaliciousCaller` with signature `Book__MaliciousCaller(address)` and selector `[104, 246, 24, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__MaliciousCaller", abi = "Book__MaliciousCaller(address)")]
    pub struct Book__MaliciousCaller {
        pub caller: ethers::core::types::Address,
    }
    #[doc = "Custom Error type `Book__NotTrader` with signature `Book__NotTrader()` and selector `[112, 16, 84, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__NotTrader", abi = "Book__NotTrader()")]
    pub struct Book__NotTrader;
    #[doc = "Custom Error type `Book__SameToken` with signature `Book__SameToken()` and selector `[111, 143, 6, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__SameToken", abi = "Book__SameToken()")]
    pub struct Book__SameToken;
    #[doc = "Custom Error type `Book__SentToBlackHole` with signature `Book__SentToBlackHole()` and selector `[151, 214, 247, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__SentToBlackHole", abi = "Book__SentToBlackHole()")]
    pub struct Book__SentToBlackHole;
    #[doc = "Custom Error type `Book__TradeNotCancelable` with signature `Book__TradeNotCancelable(bytes32)` and selector `[26, 85, 202, 164]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "Book__TradeNotCancelable",
        abi = "Book__TradeNotCancelable(bytes32)"
    )]
    pub struct Book__TradeNotCancelable {
        pub trade_id: [u8; 32],
    }
    #[doc = "Custom Error type `Book__TradeNotFilled` with signature `Book__TradeNotFilled(bytes32)` and selector `[247, 12, 190, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__TradeNotFilled", abi = "Book__TradeNotFilled(bytes32)")]
    pub struct Book__TradeNotFilled {
        pub trade_id: [u8; 32],
    }
    #[doc = "Custom Error type `Book__TradeNotInFillableState` with signature `Book__TradeNotInFillableState(bytes32)` and selector `[111, 173, 221, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "Book__TradeNotInFillableState",
        abi = "Book__TradeNotInFillableState(bytes32)"
    )]
    pub struct Book__TradeNotInFillableState {
        pub trade_id: [u8; 32],
    }
    #[doc = "Custom Error type `Book__ZeroAmount` with signature `Book__ZeroAmount()` and selector `[23, 113, 62, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Book__ZeroAmount", abi = "Book__ZeroAmount()")]
    pub struct Book__ZeroAmount;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BookErrors {
        Book__AmountOutTooLow(Book__AmountOutTooLow),
        Book__DisputePeriodNotOver(Book__DisputePeriodNotOver),
        Book__DisputePeriodOver(Book__DisputePeriodOver),
        Book__FeePctTooHigh(Book__FeePctTooHigh),
        Book__InvalidFeeCombination(Book__InvalidFeeCombination),
        Book__InvalidToken(Book__InvalidToken),
        Book__MaliciousCaller(Book__MaliciousCaller),
        Book__NotTrader(Book__NotTrader),
        Book__SameToken(Book__SameToken),
        Book__SentToBlackHole(Book__SentToBlackHole),
        Book__TradeNotCancelable(Book__TradeNotCancelable),
        Book__TradeNotFilled(Book__TradeNotFilled),
        Book__TradeNotInFillableState(Book__TradeNotInFillableState),
        Book__ZeroAmount(Book__ZeroAmount),
    }
    impl ethers::core::abi::AbiDecode for BookErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <Book__AmountOutTooLow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__AmountOutTooLow(decoded));
            }
            if let Ok(decoded) =
                <Book__DisputePeriodNotOver as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__DisputePeriodNotOver(decoded));
            }
            if let Ok(decoded) =
                <Book__DisputePeriodOver as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__DisputePeriodOver(decoded));
            }
            if let Ok(decoded) =
                <Book__FeePctTooHigh as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__FeePctTooHigh(decoded));
            }
            if let Ok(decoded) =
                <Book__InvalidFeeCombination as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__InvalidFeeCombination(decoded));
            }
            if let Ok(decoded) =
                <Book__InvalidToken as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__InvalidToken(decoded));
            }
            if let Ok(decoded) =
                <Book__MaliciousCaller as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__MaliciousCaller(decoded));
            }
            if let Ok(decoded) =
                <Book__NotTrader as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__NotTrader(decoded));
            }
            if let Ok(decoded) =
                <Book__SameToken as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__SameToken(decoded));
            }
            if let Ok(decoded) =
                <Book__SentToBlackHole as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__SentToBlackHole(decoded));
            }
            if let Ok(decoded) =
                <Book__TradeNotCancelable as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__TradeNotCancelable(decoded));
            }
            if let Ok(decoded) =
                <Book__TradeNotFilled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__TradeNotFilled(decoded));
            }
            if let Ok(decoded) =
                <Book__TradeNotInFillableState as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BookErrors::Book__TradeNotInFillableState(decoded));
            }
            if let Ok(decoded) =
                <Book__ZeroAmount as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookErrors::Book__ZeroAmount(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BookErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                BookErrors::Book__AmountOutTooLow(element) => element.encode(),
                BookErrors::Book__DisputePeriodNotOver(element) => element.encode(),
                BookErrors::Book__DisputePeriodOver(element) => element.encode(),
                BookErrors::Book__FeePctTooHigh(element) => element.encode(),
                BookErrors::Book__InvalidFeeCombination(element) => element.encode(),
                BookErrors::Book__InvalidToken(element) => element.encode(),
                BookErrors::Book__MaliciousCaller(element) => element.encode(),
                BookErrors::Book__NotTrader(element) => element.encode(),
                BookErrors::Book__SameToken(element) => element.encode(),
                BookErrors::Book__SentToBlackHole(element) => element.encode(),
                BookErrors::Book__TradeNotCancelable(element) => element.encode(),
                BookErrors::Book__TradeNotFilled(element) => element.encode(),
                BookErrors::Book__TradeNotInFillableState(element) => element.encode(),
                BookErrors::Book__ZeroAmount(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BookErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookErrors::Book__AmountOutTooLow(element) => element.fmt(f),
                BookErrors::Book__DisputePeriodNotOver(element) => element.fmt(f),
                BookErrors::Book__DisputePeriodOver(element) => element.fmt(f),
                BookErrors::Book__FeePctTooHigh(element) => element.fmt(f),
                BookErrors::Book__InvalidFeeCombination(element) => element.fmt(f),
                BookErrors::Book__InvalidToken(element) => element.fmt(f),
                BookErrors::Book__MaliciousCaller(element) => element.fmt(f),
                BookErrors::Book__NotTrader(element) => element.fmt(f),
                BookErrors::Book__SameToken(element) => element.fmt(f),
                BookErrors::Book__SentToBlackHole(element) => element.fmt(f),
                BookErrors::Book__TradeNotCancelable(element) => element.fmt(f),
                BookErrors::Book__TradeNotFilled(element) => element.fmt(f),
                BookErrors::Book__TradeNotInFillableState(element) => element.fmt(f),
                BookErrors::Book__ZeroAmount(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<Book__AmountOutTooLow> for BookErrors {
        fn from(var: Book__AmountOutTooLow) -> Self {
            BookErrors::Book__AmountOutTooLow(var)
        }
    }
    impl ::std::convert::From<Book__DisputePeriodNotOver> for BookErrors {
        fn from(var: Book__DisputePeriodNotOver) -> Self {
            BookErrors::Book__DisputePeriodNotOver(var)
        }
    }
    impl ::std::convert::From<Book__DisputePeriodOver> for BookErrors {
        fn from(var: Book__DisputePeriodOver) -> Self {
            BookErrors::Book__DisputePeriodOver(var)
        }
    }
    impl ::std::convert::From<Book__FeePctTooHigh> for BookErrors {
        fn from(var: Book__FeePctTooHigh) -> Self {
            BookErrors::Book__FeePctTooHigh(var)
        }
    }
    impl ::std::convert::From<Book__InvalidFeeCombination> for BookErrors {
        fn from(var: Book__InvalidFeeCombination) -> Self {
            BookErrors::Book__InvalidFeeCombination(var)
        }
    }
    impl ::std::convert::From<Book__InvalidToken> for BookErrors {
        fn from(var: Book__InvalidToken) -> Self {
            BookErrors::Book__InvalidToken(var)
        }
    }
    impl ::std::convert::From<Book__MaliciousCaller> for BookErrors {
        fn from(var: Book__MaliciousCaller) -> Self {
            BookErrors::Book__MaliciousCaller(var)
        }
    }
    impl ::std::convert::From<Book__NotTrader> for BookErrors {
        fn from(var: Book__NotTrader) -> Self {
            BookErrors::Book__NotTrader(var)
        }
    }
    impl ::std::convert::From<Book__SameToken> for BookErrors {
        fn from(var: Book__SameToken) -> Self {
            BookErrors::Book__SameToken(var)
        }
    }
    impl ::std::convert::From<Book__SentToBlackHole> for BookErrors {
        fn from(var: Book__SentToBlackHole) -> Self {
            BookErrors::Book__SentToBlackHole(var)
        }
    }
    impl ::std::convert::From<Book__TradeNotCancelable> for BookErrors {
        fn from(var: Book__TradeNotCancelable) -> Self {
            BookErrors::Book__TradeNotCancelable(var)
        }
    }
    impl ::std::convert::From<Book__TradeNotFilled> for BookErrors {
        fn from(var: Book__TradeNotFilled) -> Self {
            BookErrors::Book__TradeNotFilled(var)
        }
    }
    impl ::std::convert::From<Book__TradeNotInFillableState> for BookErrors {
        fn from(var: Book__TradeNotInFillableState) -> Self {
            BookErrors::Book__TradeNotInFillableState(var)
        }
    }
    impl ::std::convert::From<Book__ZeroAmount> for BookErrors {
        fn from(var: Book__ZeroAmount) -> Self {
            BookErrors::Book__ZeroAmount(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
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
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "FeePctSet", abi = "FeePctSet(uint256)")]
    pub struct FeePctSetFilter {
        pub fee_pct: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
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
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SafeBlockThresholdSet", abi = "SafeBlockThresholdSet(uint256)")]
    pub struct SafeBlockThresholdSetFilter {
        pub new_safe_block_threshold: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeCancelled",
        abi = "TradeCancelled(uint256,bytes32,address)"
    )]
    pub struct TradeCancelledFilter {
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trade_id: [u8; 32],
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeDisputeSettled",
        abi = "TradeDisputeSettled(address,uint256,bytes32,bool,address)"
    )]
    pub struct TradeDisputeSettledFilter {
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub answer: bool,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeDisputed",
        abi = "TradeDisputed(address,uint256,bytes32,uint256,address)"
    )]
    pub struct TradeDisputedFilter {
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub filled_at_block: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeFilled",
        abi = "TradeFilled(address,uint256,uint256,address)"
    )]
    pub struct TradeFilledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub amount_out: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeRequested",
        abi = "TradeRequested(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct TradeRequestedFilter {
        #[ethevent(indexed)]
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeSettled",
        abi = "TradeSettled(address,uint256,uint256,address)"
    )]
    pub struct TradeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub filled_at_block: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BookEvents {
        FeeCombinationSetFilter(FeeCombinationSetFilter),
        FeePctSetFilter(FeePctSetFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        SafeBlockThresholdSetFilter(SafeBlockThresholdSetFilter),
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
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
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
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "disputeBondPct", abi = "disputeBondPct()")]
    pub struct DisputeBondPctCall;
    #[doc = "Container type for all input parameters for the `disputeTrade` function with signature `disputeTrade(address,address,uint256,uint256,address,uint256,address)` and selector `[9, 121, 109, 255]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
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
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feePct", abi = "feePct()")]
    pub struct FeePctCall;
    #[doc = "Container type for all input parameters for the `fillTrade` function with signature `fillTrade(address,address,uint256,uint256,address,uint256,address,uint256)` and selector `[45, 126, 142, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
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
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "filledAtBlock", abi = "filledAtBlock(bytes32)")]
    pub struct FilledAtBlockCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `filledBy` function with signature `filledBy(bytes32)` and selector `[215, 14, 61, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "filledBy", abi = "filledBy(bytes32)")]
    pub struct FilledByCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `numberOfTrades` function with signature `numberOfTrades()` and selector `[205, 128, 93, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "numberOfTrades", abi = "numberOfTrades()")]
    pub struct NumberOfTradesCall;
    #[doc = "Container type for all input parameters for the `onPriceSettled` function with signature `onPriceSettled(bytes32,(address,address,address,address,uint256,uint8,bool,bytes))` and selector `[115, 77, 22, 39]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
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
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `registry` function with signature `registry()` and selector `[123, 16, 57, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
    #[doc = "Container type for all input parameters for the `relayerRefundPct` function with signature `relayerRefundPct()` and selector `[83, 144, 106, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "relayerRefundPct", abi = "relayerRefundPct()")]
    pub struct RelayerRefundPctCall;
    #[doc = "Container type for all input parameters for the `requestTrade` function with signature `requestTrade(address,address,uint256,uint256,address)` and selector `[145, 112, 192, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
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
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "safeBlockThreshold", abi = "safeBlockThreshold()")]
    pub struct SafeBlockThresholdCall;
    #[doc = "Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `settleTrade` function with signature `settleTrade(address,address,uint256,uint256,address,uint256,address)` and selector `[230, 106, 52, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
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
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "status", abi = "status(bytes32)")]
    pub struct StatusCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `tradeRebatePct` function with signature `tradeRebatePct()` and selector `[193, 100, 2, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tradeRebatePct", abi = "tradeRebatePct()")]
    pub struct TradeRebatePctCall;
    #[doc = "Container type for all input parameters for the `whitelistedTokens` function with signature `whitelistedTokens(address)` and selector `[218, 249, 194, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
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
        Registry(RegistryCall),
        RelayerRefundPct(RelayerRefundPctCall),
        RequestTrade(RequestTradeCall),
        SafeBlockThreshold(SafeBlockThresholdCall),
        SetOwner(SetOwnerCall),
        SettleTrade(SettleTradeCall),
        Status(StatusCall),
        TradeRebatePct(TradeRebatePctCall),
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
                <RegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookCalls::Registry(decoded));
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
                BookCalls::Registry(element) => element.encode(),
                BookCalls::RelayerRefundPct(element) => element.encode(),
                BookCalls::RequestTrade(element) => element.encode(),
                BookCalls::SafeBlockThreshold(element) => element.encode(),
                BookCalls::SetOwner(element) => element.encode(),
                BookCalls::SettleTrade(element) => element.encode(),
                BookCalls::Status(element) => element.encode(),
                BookCalls::TradeRebatePct(element) => element.encode(),
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
                BookCalls::Registry(element) => element.fmt(f),
                BookCalls::RelayerRefundPct(element) => element.fmt(f),
                BookCalls::RequestTrade(element) => element.fmt(f),
                BookCalls::SafeBlockThreshold(element) => element.fmt(f),
                BookCalls::SetOwner(element) => element.fmt(f),
                BookCalls::SettleTrade(element) => element.fmt(f),
                BookCalls::Status(element) => element.fmt(f),
                BookCalls::TradeRebatePct(element) => element.fmt(f),
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
    impl ::std::convert::From<RegistryCall> for BookCalls {
        fn from(var: RegistryCall) -> Self {
            BookCalls::Registry(var)
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
    impl ::std::convert::From<WhitelistedTokensCall> for BookCalls {
        fn from(var: WhitelistedTokensCall) -> Self {
            BookCalls::WhitelistedTokens(var)
        }
    }
    #[doc = "Container type for all return fields from the `disputeBondPct` function with signature `disputeBondPct()` and selector `[57, 31, 228, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DisputeBondPctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `feePct` function with signature `feePct()` and selector `[160, 44, 249, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeePctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `filledAtBlock` function with signature `filledAtBlock(bytes32)` and selector `[149, 1, 50, 95]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FilledAtBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `filledBy` function with signature `filledBy(bytes32)` and selector `[215, 14, 61, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FilledByReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `numberOfTrades` function with signature `numberOfTrades()` and selector `[205, 128, 93, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NumberOfTradesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `registry` function with signature `registry()` and selector `[123, 16, 57, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RegistryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `relayerRefundPct` function with signature `relayerRefundPct()` and selector `[83, 144, 106, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RelayerRefundPctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `safeBlockThreshold` function with signature `safeBlockThreshold()` and selector `[15, 240, 192, 14]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SafeBlockThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `status` function with signature `status(bytes32)` and selector `[82, 173, 13, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StatusReturn(pub u8);
    #[doc = "Container type for all return fields from the `tradeRebatePct` function with signature `tradeRebatePct()` and selector `[193, 100, 2, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TradeRebatePctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `whitelistedTokens` function with signature `whitelistedTokens(address)` and selector `[218, 249, 194, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WhitelistedTokensReturn(pub bool);
}
