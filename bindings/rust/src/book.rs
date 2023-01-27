pub use book::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod book {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///Book was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    use ::ethers::core::{
        abi::{Abi, Token, Detokenize, InvalidOutputType, Tokenizable},
        types::*,
    };
    use ::ethers::contract::{
        Contract, builders::{ContractCall, Event},
        Lazy,
    };
    use ::ethers::providers::Middleware;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract FloodRegistry\",\"name\":\"_registry\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_safeBlockThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_disputeBondPct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_tradeRebatePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_relayerRefundPct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_feePct\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__AmountOutTooLow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blocksLeft\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__DisputePeriodNotOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__FeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__InvalidParamsCombination\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__InvalidToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__InvalidValue\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__MaliciousCaller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__NotWeth\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__SameToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__SentToBlackHole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotCancelable\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__TradeNotDisputable\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"Book__TradeNotInFillableState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__ZeroAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__ZeroRegistry\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Book__ZeroSafeBlockThreshold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeePctSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"disputeBondPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeRebatePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"relayerRefundPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ParamsCombinationSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeCancelled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeDisputeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"disputeBondPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputeTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numberOfTrades\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Request\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onPriceSettled\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"registry\",\"outputs\":[{\"internalType\":\"contract FloodRegistry\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayerRefundPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"receiveETH\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"requestTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"safeBlockThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tradeRebatePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tradesData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"filledBy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"enum TradeStatus\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"unwrapOutput\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isEthTrade\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountPaid\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static BOOK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    /// Bytecode of the #name contract
    pub static BOOK_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> = ::ethers::contract::Lazy::new(||
    {
        "0x610180604052600080553480156200001657600080fd5b506040516200263a3803806200263a8339810160408190526200003991620002da565b6001600160a01b038616620000615760405163dacc695960e01b815260040160405180910390fd5b84600003620000835760405163a95604b160e01b815260040160405180910390fd5b6001600160a01b03861661012081905260408051630175727b60e51b81529051632eae4f60916004808201926020929091908290030181865afa158015620000cf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000f5919062000330565b6001600160a01b0316610140816001600160a01b031681525050610120516001600160a01b031663ad5c46486040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000151573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000177919062000330565b6001600160a01b03166101605260808590526040518581527f882885d0e4612a71677644a9d70e58ca05fc5a1ea1b0875f6e46c315241bfe149060200160405180910390a1620001c984848462000282565b60a084905260c083905260e082905260408051858152602081018590529081018390527f8be1eb1cba497386c77933500158ebaaef9e51125fa5ebdfaa92139d320d43299060600160405180910390a16109c48111156200023d5760405163b768880160e01b815260040160405180910390fd5b6101008190526040518181527f9e67c173f0d1bf66a955764a6b072d74e095af185e539f9f6570fb91d788fca59060200160405180910390a15050505050506200037f565b806200028f838562000357565b6200029b919062000357565b606414620002bc57604051639327bc8760e01b815260040160405180910390fd5b505050565b6001600160a01b0381168114620002d757600080fd5b50565b60008060008060008060c08789031215620002f457600080fd5b86516200030181620002c1565b6020880151604089015160608a015160808b015160a0909b0151939c929b509099909850965090945092505050565b6000602082840312156200034357600080fd5b81516200035081620002c1565b9392505050565b808201808211156200037957634e487b7160e01b600052601160045260246000fd5b92915050565b60805160a05160c05160e051610100516101205161014051610160516121ec6200044e60003960008181610d7a01528181610dda01528181610e9d0152818161105c0152818161157601526116d50152600081816102e0015281816105dd015281816106530152610aa50152600081816102940152818161184a01526118fc015260006103140152600081816101a701526108d10152600061037b015260008181610173015281816104c0015261060001526000818161012c01528181611200015261137101526121ec6000f3fe6080604052600436106100ec5760003560e01c80637b1039991161008a578063ab6552e511610059578063ab6552e514610356578063c16402bb14610369578063cd805d5e1461039d578063e66a34ee146103b357600080fd5b80637b103999146102825780637dc0d1d0146102ce578063a02cf93714610302578063aa311ab41461033657600080fd5b806353906a59116100c657806353906a59146101955780636b16e8e6146101c9578063734d1627146101e957806375ef5c991461020957600080fd5b806309796dff146100f85780630ff0c00e1461011a578063391fe4e21461016157600080fd5b366100f357005b600080fd5b34801561010457600080fd5b50610118610113366004611c0d565b6103d3565b005b34801561012657600080fd5b5061014e7f000000000000000000000000000000000000000000000000000000000000000081565b6040519081526020015b60405180910390f35b34801561016d57600080fd5b5061014e7f000000000000000000000000000000000000000000000000000000000000000081565b3480156101a157600080fd5b5061014e7f000000000000000000000000000000000000000000000000000000000000000081565b3480156101d557600080fd5b506101186101e4366004611c88565b6107a9565b3480156101f557600080fd5b50610118610204366004611d6d565b610a9a565b34801561021557600080fd5b50610270610224366004611dbc565b60016020819052600091825260409091208054918101546002909101546001600160a01b0382169160ff600160a01b8204811692600160a81b8304821692600160b01b90049091169086565b60405161015896959493929190611deb565b34801561028e57600080fd5b506102b67f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610158565b3480156102da57600080fd5b506102b67f000000000000000000000000000000000000000000000000000000000000000081565b34801561030e57600080fd5b5061014e7f000000000000000000000000000000000000000000000000000000000000000081565b34801561034257600080fd5b50610118610351366004611e43565b610c3c565b610118610364366004611eb8565b610d6e565b34801561037557600080fd5b5061014e7f000000000000000000000000000000000000000000000000000000000000000081565b3480156103a957600080fd5b5061014e60005481565b3480156103bf57600080fd5b506101186103ce366004611c0d565b6110f1565b60006103e4888888888888886112d5565b6000818152600160208181526040808420815160c08101835281548152938101546001600160a01b0381169385019390935294955092939192830190600160a01b900460ff16600381111561043b5761043b611dd5565b600381111561044c5761044c611dd5565b8152600182015460ff600160a81b8204811615156020840152600160b01b9091041615156040808301919091526002909201546060909101528151908201519192509061049a90829061134d565b6104b7576040516331c0890d60e21b815260040160405180910390fd5b600060646104e57f00000000000000000000000000000000000000000000000000000000000000008b611f3e565b6104ef9190611f55565b9050808360a001516105019190611f77565b60a084015260036040840181905250600084815260016020818152604092839020865181559086015191810180546001600160a01b031981166001600160a01b039094169384178255938701518794929390926001600160a81b03191617600160a01b83600381111561057657610576611dd5565b0217905550606082015160018201805460808501511515600160b01b0260ff60b01b19931515600160a81b029390931661ffff60a81b199091161791909117905560a0909101516002909101556105d86001600160a01b038c163330846113a1565b61064f7f000000000000000000000000000000000000000000000000000000000000000060647f000000000000000000000000000000000000000000000000000000000000000061062a8d6002611f3e565b6106349190611f3e565b61063e9190611f55565b6001600160a01b038e169190611412565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f7d3b58b8560200151338f868f8e8e8e8e6040516020016106ca9594939291909485526001600160a01b03938416602086015260408501929092529091166060830152608082015260a00190565b6040516020818303038152906040526040518663ffffffff1660e01b81526004016106f9959493929190611fda565b6020604051808303816000875af1158015610718573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061073c9190612015565b9050856001600160a01b031681887f611d2790eebc2dfbd3329eb8ce89302d94d2b42c489cfd3ba8dae42977a3f9428760200151876040516107939291906001600160a01b03929092168252602082015260400190565b60405180910390a4505050505050505050505050565b60006107ba8b8b8b8b8b8b8b6112d5565b6000818152600160208181526040808420815160c08101835281548152938101546001600160a01b0381169385019390935294955092939192830190600160a01b900460ff16600381111561081157610811611dd5565b600381111561082257610822611dd5565b815260018281015460ff600160a81b8204811615156020850152600160b01b90910416151560408301526002909201546060909101529091508160400151600381111561087157610871611dd5565b14610897576040516337d6ee8b60e11b8152600481018390526024015b60405180910390fd5b888510156108b85760405163122e33cf60e31b815260040160405180910390fd5b43815233602082015260026040820152600060646108f67f00000000000000000000000000000000000000000000000000000000000000008d611f3e565b6109009190611f55565b60a08301819052600084815260016020818152604092839020865181559086015191810180546001600160a01b031981166001600160a01b0390941693841782559387015194955086949193919290916001600160a81b03191617600160a01b83600381111561097257610972611dd5565b0217905550606082015160018201805460808501511515600160b01b0260ff60b01b19931515600160a81b029390931661ffff60a81b199091161791909117905560a0909101516002909101556040516001600160a01b03881690899033907f81f3dddde62cb8d590d391999bc1a4c362a03cf74718fb3bd8195b46fc1c100990610a00908b815260200190565b60405180910390a4610a1c6001600160a01b038e16338361152c565b8315610a7a57604051633ce7083360e11b815233906379ce106690610a47908890889060040161202e565b600060405180830381600087803b158015610a6157600080fd5b505af1158015610a75573d6000803e3d6000fd5b505050505b610a8b8c338b89866060015161155c565b50505050505050505050505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610ae55760405163347b0c3360e11b815233600482015260240161088e565b600080808080610af860e087018761205d565b810190610b0591906120ab565b600081815260016020526040812060020154959a509398509196509450925090610b2f90876120fd565b9050610b3a826117cf565b6001600160a01b03831688857ff27fbe6db06bb8ac8a2b4206eef25be7c007d9fd8d53a772c5ffa4d7487952b2610b7760408c0160208d01612110565b610b8760e08d0160c08e01612134565b604080516001600160a01b03909316835290151560208301520160405180910390a4610bb960e0880160c08901612134565b15610bf757610bf2610bd16040890160208a01612110565b82610be260808b0160608c01612110565b6001600160a01b0316919061152c565b610c32565b610c32610c0a6080890160608a01612110565b60008481526001602081905260409091200154309088908590600160a81b900460ff1661155c565b5050505050505050565b6000610c4d878787878787336112d5565b6000818152600160208181526040808420815160c08101835281548152938101546001600160a01b0381169385019390935294955092939192830190600160a01b900460ff166003811115610ca457610ca4611dd5565b6003811115610cb557610cb5611dd5565b815260018281015460ff600160a81b8204811615156020850152600160b01b909104161515604083015260029092015460609091015290915081604001516003811115610d0457610d04611dd5565b14610d255760405163069572a960e21b81526004810183905260240161088e565b610d2e826117cf565b6040513390839085907f4aaea9d289fcf796ee2d93ab7730c9a0afa79d03debf6528dd7f13a83613b7f490600090a4610c3288303389856080015161155c565b610d7886866117f9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316866001600160a01b031614158015610dba5750600034115b15610dd8576040516398a5b0bf60e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316866001600160a01b0316148015610e195750600034115b8015610e255750833414155b15610e4357604051631d57a23960e31b815260040160405180910390fd5b831580610e4e575082155b15610e6c576040516305dc4f8360e21b815260040160405180910390fd5b6001600160a01b038216610e93576040516325f5bdf760e21b815260040160405180910390fd5b808015610ed257507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316856001600160a01b031614155b15610ef0576040516398a5b0bf60e01b815260040160405180910390fd5b600054604080516001600160a01b0388811682526020820188905291810186905284821660608201523392918916907f8a923a6574cd5d908fa4e840c47fb64b874b2116a88d073e5fdbd834033932769060800160405180910390a46000610f5f8787878787600054336112d5565b6040805160c0810182526000808252602080830182815260018486018181528915156060870152341515608087015260a08601859052878552928190529490922083518155915193820180546001600160a01b039095166001600160a01b03198616811782559151959650929491939192916001600160a81b03191617600160a01b836003811115610ff357610ff3611dd5565b0217905550606082015160018201805460808501511515600160b01b0260ff60b01b19931515600160a81b029390931661ffff60a81b199091161791909117905560a09091015160029091015560008054908061104f83612151565b909155505034156110d3577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0866040518263ffffffff1660e01b81526004016000604051808303818588803b1580156110b557600080fd5b505af11580156110c9573d6000803e3d6000fd5b50505050506110e8565b6110e86001600160a01b0388163330886113a1565b50505050505050565b6000611102888888888888886112d5565b6000818152600160208181526040808420815160c08101835281548152938101546001600160a01b0381169385019390935294955092939192830190600160a01b900460ff16600381111561115957611159611dd5565b600381111561116a5761116a611dd5565b8152600182015460ff600160a81b8204811615156020840152600160b01b90910416151560408201526002918201546060909101528151919250826040015160038111156111ba576111ba611dd5565b146111db57604051630f70cbe360e41b81526004810184905260240161088e565b6111e981836040015161134d565b156112435760006111fa82436120fd565b611224907f00000000000000000000000000000000000000000000000000000000000000006120fd565b905080604051637be8522f60e01b815260040161088e91815260200190565b602082015160a083015160009061125a908b6120fd565b9050611265856117cf565b856001600160a01b031687836001600160a01b03167fdeb6ee1bbbaaac1d6b7873a679d462110dd54d72915436622cc00525c2561cab866040516112ab91815260200190565b60405180910390a46112c76001600160a01b038d16838361152c565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606089811b8216602084015288811b82166034840152604883018890526068830187905285811b82166088840152609c830185905283901b1660bc82015260009060d001604051602081830303815290604052805190602001209050979650505050505050565b6000600282600381111561136357611363611dd5565b1480156113985750611395837f0000000000000000000000000000000000000000000000000000000000000000611f77565b43105b90505b92915050565b6040516001600160a01b038085166024830152831660448201526064810182905261140c9085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152611993565b50505050565b80158061148c5750604051636eb1769f60e11b81523060048201526001600160a01b03838116602483015284169063dd62ed3e90604401602060405180830381865afa158015611466573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061148a9190612015565b155b6114f75760405162461bcd60e51b815260206004820152603660248201527f5361666545524332303a20617070726f76652066726f6d206e6f6e2d7a65726f60448201527520746f206e6f6e2d7a65726f20616c6c6f77616e636560501b606482015260840161088e565b6040516001600160a01b03831660248201526044810182905261152790849063095ea7b360e01b906064016113d5565b505050565b6040516001600160a01b03831660248201526044810182905261152790849063a9059cbb60e01b906064016113d5565b306001600160a01b038516036116cb578080156115aa57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316856001600160a01b0316145b156116b257604051632e1a7d4d60e01b8152600481018390526001600160a01b03861690632e1a7d4d90602401600060405180830381600087803b1580156115f157600080fd5b505af1158015611605573d6000803e3d6000fd5b505050506000836001600160a01b03168360405160006040518083038185875af1925050503d8060008114611656576040519150601f19603f3d011682016040523d82523d6000602084013e61165b565b606091505b50509050806116ac5760405162461bcd60e51b815260206004820152601960248201527f426f6f6b3a20455448207472616e73666572206661696c656400000000000000604482015260640161088e565b506117c8565b6116c66001600160a01b038616848461152c565b6117c8565b80801561170957507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316856001600160a01b0316145b156117b3576117236001600160a01b0386168530856113a1565b604051632e1a7d4d60e01b8152600481018390526001600160a01b03861690632e1a7d4d90602401600060405180830381600087803b15801561176557600080fd5b505af1158015611779573d6000803e3d6000fd5b50506040516001600160a01b038616925084156108fc02915084906000818181858888f193505050501580156116ac573d6000803e3d6000fd5b6117c86001600160a01b0386168585856113a1565b5050505050565b6000908152600160208190526040822082815590810180546001600160b81b031916905560020155565b806001600160a01b0316826001600160a01b03160361182b57604051636f8f06d360e01b815260040160405180910390fd5b60405163b5af090f60e01b81526001600160a01b0383811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063b5af090f90602401602060405180830381865afa158015611891573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118b5919061216a565b6118dd576040516378abcf6760e11b81526001600160a01b038316600482015260240161088e565b60405163b5af090f60e01b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063b5af090f90602401602060405180830381865afa158015611943573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611967919061216a565b61198f576040516378abcf6760e11b81526001600160a01b038216600482015260240161088e565b5050565b60006119e8826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316611a659092919063ffffffff16565b8051909150156115275780806020019051810190611a06919061216a565b6115275760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b606482015260840161088e565b6060611a748484600085611a7c565b949350505050565b606082471015611add5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b606482015260840161088e565b600080866001600160a01b03168587604051611af99190612187565b60006040518083038185875af1925050503d8060008114611b36576040519150601f19603f3d011682016040523d82523d6000602084013e611b3b565b606091505b5091509150611b4c87838387611b57565b979650505050505050565b60608315611bc6578251600003611bbf576001600160a01b0385163b611bbf5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e7472616374000000604482015260640161088e565b5081611a74565b611a748383815115611bdb5781518083602001fd5b8060405162461bcd60e51b815260040161088e91906121a3565b6001600160a01b0381168114611c0a57600080fd5b50565b600080600080600080600060e0888a031215611c2857600080fd5b8735611c3381611bf5565b96506020880135611c4381611bf5565b955060408801359450606088013593506080880135611c6181611bf5565b925060a0880135915060c0880135611c7881611bf5565b8091505092959891949750929550565b6000806000806000806000806000806101208b8d031215611ca857600080fd5b8a35611cb381611bf5565b995060208b0135611cc381611bf5565b985060408b0135975060608b0135965060808b0135611ce181611bf5565b955060a08b0135945060c08b0135611cf881611bf5565b935060e08b013592506101008b013567ffffffffffffffff80821115611d1d57600080fd5b818d0191508d601f830112611d3157600080fd5b813581811115611d4057600080fd5b8e6020828501011115611d5257600080fd5b6020830194508093505050509295989b9194979a5092959850565b60008060408385031215611d8057600080fd5b82359150602083013567ffffffffffffffff811115611d9e57600080fd5b83016101008186031215611db157600080fd5b809150509250929050565b600060208284031215611dce57600080fd5b5035919050565b634e487b7160e01b600052602160045260246000fd5b8681526001600160a01b038616602082015260c0810160048610611e1f57634e487b7160e01b600052602160045260246000fd5b60408201959095529215156060840152901515608083015260a09091015292915050565b60008060008060008060c08789031215611e5c57600080fd5b8635611e6781611bf5565b95506020870135611e7781611bf5565b945060408701359350606087013592506080870135611e9581611bf5565b8092505060a087013590509295509295509295565b8015158114611c0a57600080fd5b60008060008060008060c08789031215611ed157600080fd5b8635611edc81611bf5565b95506020870135611eec81611bf5565b945060408701359350606087013592506080870135611f0a81611bf5565b915060a0870135611f1a81611eaa565b809150509295509295509295565b634e487b7160e01b600052601160045260246000fd5b808202811582820484141761139b5761139b611f28565b600082611f7257634e487b7160e01b600052601260045260246000fd5b500490565b8082018082111561139b5761139b611f28565b60005b83811015611fa5578181015183820152602001611f8d565b50506000910152565b60008151808452611fc6816020860160208601611f8a565b601f01601f19169290920160200192915050565b6001600160a01b0386811682528581166020830152841660408201526060810183905260a060808201819052600090611b4c90830184611fae565b60006020828403121561202757600080fd5b5051919050565b60208152816020820152818360408301376000818301604090810191909152601f909201601f19160101919050565b6000808335601e1984360301811261207457600080fd5b83018035915067ffffffffffffffff82111561208f57600080fd5b6020019150368190038213156120a457600080fd5b9250929050565b600080600080600060a086880312156120c357600080fd5b8535945060208601356120d581611bf5565b93506040860135925060608601356120ec81611bf5565b949793965091946080013592915050565b8181038181111561139b5761139b611f28565b60006020828403121561212257600080fd5b813561212d81611bf5565b9392505050565b60006020828403121561214657600080fd5b813561212d81611eaa565b60006001820161216357612163611f28565b5060010190565b60006020828403121561217c57600080fd5b815161212d81611eaa565b60008251612199818460208701611f8a565b9190910192915050565b6020815260006113986020830184611fae56fea2646970667358221220c1bebc07addd0433422bd7a4d73a2a35395a3ab91b5f0870ae98ed00091fa1e464736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct Book<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for Book<M> {
        fn clone(&self) -> Self {
            Book(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Book<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Book<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Book)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Book<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BOOK_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// 1. If there are no constructor arguments, you should pass `()` as the argument.
        /// 1. The default poll duration is 7 seconds.
        /// 1. The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter,"../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                BOOK_ABI.clone(),
                BOOK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `cancelTrade` (0xaa311ab4) function
        pub fn cancel_trade(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            min_amount_out: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            trade_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [170, 49, 26, 180],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        recipient,
                        trade_index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disputeBondPct` (0x391fe4e2) function
        pub fn dispute_bond_pct(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([57, 31, 228, 226], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disputeTrade` (0x09796dff) function
        pub fn dispute_trade(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            min_amount_out: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            trade_index: ::ethers::core::types::U256,
            trader: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `feePct` (0xa02cf937) function
        pub fn fee_pct(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 44, 249, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fillTrade` (0x6b16e8e6) function
        pub fn fill_trade(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            min_amount_out: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            trade_index: ::ethers::core::types::U256,
            trader: ::ethers::core::types::Address,
            amount_to_send: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [107, 22, 232, 230],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        recipient,
                        trade_index,
                        trader,
                        amount_to_send,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numberOfTrades` (0xcd805d5e) function
        pub fn number_of_trades(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([205, 128, 93, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onPriceSettled` (0x734d1627) function
        pub fn on_price_settled(
            &self,
            id: [u8; 32],
            request: Request,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 77, 22, 39], (id, request))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracle` (0x7dc0d1d0) function
        pub fn oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registry` (0x7b103999) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([123, 16, 57, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerRefundPct` (0x53906a59) function
        pub fn relayer_refund_pct(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([83, 144, 106, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestTrade` (0xab6552e5) function
        pub fn request_trade(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            min_amount_out: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            receive_eth: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [171, 101, 82, 229],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        min_amount_out,
                        recipient,
                        receive_eth,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeBlockThreshold` (0x0ff0c00e) function
        pub fn safe_block_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([15, 240, 192, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settleTrade` (0xe66a34ee) function
        pub fn settle_trade(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            min_amount_out: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            trade_index: ::ethers::core::types::U256,
            trader: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `tradeRebatePct` (0xc16402bb) function
        pub fn trade_rebate_pct(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([193, 100, 2, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tradesData` (0x75ef5c99) function
        pub fn trades_data(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                u8,
                bool,
                bool,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([117, 239, 92, 153], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FeePctSet` event
        pub fn fee_pct_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, FeePctSetFilter> {
            self.0.event()
        }
        ///Gets the contract's `ParamsCombinationSet` event
        pub fn params_combination_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, ParamsCombinationSetFilter> {
            self.0.event()
        }
        ///Gets the contract's `SafeBlockThresholdSet` event
        pub fn safe_block_threshold_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, SafeBlockThresholdSetFilter> {
            self.0.event()
        }
        ///Gets the contract's `TradeCancelled` event
        pub fn trade_cancelled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TradeCancelledFilter> {
            self.0.event()
        }
        ///Gets the contract's `TradeDisputeSettled` event
        pub fn trade_dispute_settled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TradeDisputeSettledFilter> {
            self.0.event()
        }
        ///Gets the contract's `TradeDisputed` event
        pub fn trade_disputed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TradeDisputedFilter> {
            self.0.event()
        }
        ///Gets the contract's `TradeFilled` event
        pub fn trade_filled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TradeFilledFilter> {
            self.0.event()
        }
        ///Gets the contract's `TradeRequested` event
        pub fn trade_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TradeRequestedFilter> {
            self.0.event()
        }
        ///Gets the contract's `TradeSettled` event
        pub fn trade_settled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TradeSettledFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, BookEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Book<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Book__AmountOutTooLow` with signature `Book__AmountOutTooLow()` and selector `0x91719e78`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__AmountOutTooLow", abi = "Book__AmountOutTooLow()")]
    pub struct Book__AmountOutTooLow;
    ///Custom Error type `Book__DisputePeriodNotOver` with signature `Book__DisputePeriodNotOver(uint256)` and selector `0x7be8522f`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "Book__DisputePeriodNotOver",
        abi = "Book__DisputePeriodNotOver(uint256)"
    )]
    pub struct Book__DisputePeriodNotOver {
        pub blocks_left: ::ethers::core::types::U256,
    }
    ///Custom Error type `Book__FeePctTooHigh` with signature `Book__FeePctTooHigh()` and selector `0xb7688801`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__FeePctTooHigh", abi = "Book__FeePctTooHigh()")]
    pub struct Book__FeePctTooHigh;
    ///Custom Error type `Book__InvalidParamsCombination` with signature `Book__InvalidParamsCombination()` and selector `0x9327bc87`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "Book__InvalidParamsCombination",
        abi = "Book__InvalidParamsCombination()"
    )]
    pub struct Book__InvalidParamsCombination;
    ///Custom Error type `Book__InvalidToken` with signature `Book__InvalidToken(address)` and selector `0xf1579ece`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__InvalidToken", abi = "Book__InvalidToken(address)")]
    pub struct Book__InvalidToken {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `Book__InvalidValue` with signature `Book__InvalidValue()` and selector `0xeabd11c8`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__InvalidValue", abi = "Book__InvalidValue()")]
    pub struct Book__InvalidValue;
    ///Custom Error type `Book__MaliciousCaller` with signature `Book__MaliciousCaller(address)` and selector `0x68f61866`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__MaliciousCaller", abi = "Book__MaliciousCaller(address)")]
    pub struct Book__MaliciousCaller {
        pub caller: ::ethers::core::types::Address,
    }
    ///Custom Error type `Book__NotWeth` with signature `Book__NotWeth()` and selector `0x98a5b0bf`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__NotWeth", abi = "Book__NotWeth()")]
    pub struct Book__NotWeth;
    ///Custom Error type `Book__SameToken` with signature `Book__SameToken()` and selector `0x6f8f06d3`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__SameToken", abi = "Book__SameToken()")]
    pub struct Book__SameToken;
    ///Custom Error type `Book__SentToBlackHole` with signature `Book__SentToBlackHole()` and selector `0x97d6f7dc`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__SentToBlackHole", abi = "Book__SentToBlackHole()")]
    pub struct Book__SentToBlackHole;
    ///Custom Error type `Book__TradeNotCancelable` with signature `Book__TradeNotCancelable(bytes32)` and selector `0x1a55caa4`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "Book__TradeNotCancelable",
        abi = "Book__TradeNotCancelable(bytes32)"
    )]
    pub struct Book__TradeNotCancelable {
        pub trade_id: [u8; 32],
    }
    ///Custom Error type `Book__TradeNotDisputable` with signature `Book__TradeNotDisputable()` and selector `0xc7022434`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__TradeNotDisputable", abi = "Book__TradeNotDisputable()")]
    pub struct Book__TradeNotDisputable;
    ///Custom Error type `Book__TradeNotFilled` with signature `Book__TradeNotFilled(bytes32)` and selector `0xf70cbe30`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__TradeNotFilled", abi = "Book__TradeNotFilled(bytes32)")]
    pub struct Book__TradeNotFilled {
        pub trade_id: [u8; 32],
    }
    ///Custom Error type `Book__TradeNotInFillableState` with signature `Book__TradeNotInFillableState(bytes32)` and selector `0x6faddd16`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "Book__TradeNotInFillableState",
        abi = "Book__TradeNotInFillableState(bytes32)"
    )]
    pub struct Book__TradeNotInFillableState {
        pub trade_id: [u8; 32],
    }
    ///Custom Error type `Book__ZeroAmount` with signature `Book__ZeroAmount()` and selector `0x17713e0c`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__ZeroAmount", abi = "Book__ZeroAmount()")]
    pub struct Book__ZeroAmount;
    ///Custom Error type `Book__ZeroRegistry` with signature `Book__ZeroRegistry()` and selector `0xdacc6959`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(name = "Book__ZeroRegistry", abi = "Book__ZeroRegistry()")]
    pub struct Book__ZeroRegistry;
    ///Custom Error type `Book__ZeroSafeBlockThreshold` with signature `Book__ZeroSafeBlockThreshold()` and selector `0xa95604b1`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "Book__ZeroSafeBlockThreshold",
        abi = "Book__ZeroSafeBlockThreshold()"
    )]
    pub struct Book__ZeroSafeBlockThreshold;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum BookErrors {
        Book__AmountOutTooLow(Book__AmountOutTooLow),
        Book__DisputePeriodNotOver(Book__DisputePeriodNotOver),
        Book__FeePctTooHigh(Book__FeePctTooHigh),
        Book__InvalidParamsCombination(Book__InvalidParamsCombination),
        Book__InvalidToken(Book__InvalidToken),
        Book__InvalidValue(Book__InvalidValue),
        Book__MaliciousCaller(Book__MaliciousCaller),
        Book__NotWeth(Book__NotWeth),
        Book__SameToken(Book__SameToken),
        Book__SentToBlackHole(Book__SentToBlackHole),
        Book__TradeNotCancelable(Book__TradeNotCancelable),
        Book__TradeNotDisputable(Book__TradeNotDisputable),
        Book__TradeNotFilled(Book__TradeNotFilled),
        Book__TradeNotInFillableState(Book__TradeNotInFillableState),
        Book__ZeroAmount(Book__ZeroAmount),
        Book__ZeroRegistry(Book__ZeroRegistry),
        Book__ZeroSafeBlockThreshold(Book__ZeroSafeBlockThreshold),
    }
    impl ::ethers::core::abi::AbiDecode for BookErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <Book__AmountOutTooLow as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__AmountOutTooLow(decoded));
            }
            if let Ok(decoded)
                = <Book__DisputePeriodNotOver as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__DisputePeriodNotOver(decoded));
            }
            if let Ok(decoded)
                = <Book__FeePctTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__FeePctTooHigh(decoded));
            }
            if let Ok(decoded)
                = <Book__InvalidParamsCombination as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__InvalidParamsCombination(decoded));
            }
            if let Ok(decoded)
                = <Book__InvalidToken as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__InvalidToken(decoded));
            }
            if let Ok(decoded)
                = <Book__InvalidValue as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__InvalidValue(decoded));
            }
            if let Ok(decoded)
                = <Book__MaliciousCaller as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__MaliciousCaller(decoded));
            }
            if let Ok(decoded)
                = <Book__NotWeth as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__NotWeth(decoded));
            }
            if let Ok(decoded)
                = <Book__SameToken as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__SameToken(decoded));
            }
            if let Ok(decoded)
                = <Book__SentToBlackHole as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__SentToBlackHole(decoded));
            }
            if let Ok(decoded)
                = <Book__TradeNotCancelable as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__TradeNotCancelable(decoded));
            }
            if let Ok(decoded)
                = <Book__TradeNotDisputable as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__TradeNotDisputable(decoded));
            }
            if let Ok(decoded)
                = <Book__TradeNotFilled as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__TradeNotFilled(decoded));
            }
            if let Ok(decoded)
                = <Book__TradeNotInFillableState as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__TradeNotInFillableState(decoded));
            }
            if let Ok(decoded)
                = <Book__ZeroAmount as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__ZeroAmount(decoded));
            }
            if let Ok(decoded)
                = <Book__ZeroRegistry as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__ZeroRegistry(decoded));
            }
            if let Ok(decoded)
                = <Book__ZeroSafeBlockThreshold as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookErrors::Book__ZeroSafeBlockThreshold(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BookErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                BookErrors::Book__AmountOutTooLow(element) => element.encode(),
                BookErrors::Book__DisputePeriodNotOver(element) => element.encode(),
                BookErrors::Book__FeePctTooHigh(element) => element.encode(),
                BookErrors::Book__InvalidParamsCombination(element) => element.encode(),
                BookErrors::Book__InvalidToken(element) => element.encode(),
                BookErrors::Book__InvalidValue(element) => element.encode(),
                BookErrors::Book__MaliciousCaller(element) => element.encode(),
                BookErrors::Book__NotWeth(element) => element.encode(),
                BookErrors::Book__SameToken(element) => element.encode(),
                BookErrors::Book__SentToBlackHole(element) => element.encode(),
                BookErrors::Book__TradeNotCancelable(element) => element.encode(),
                BookErrors::Book__TradeNotDisputable(element) => element.encode(),
                BookErrors::Book__TradeNotFilled(element) => element.encode(),
                BookErrors::Book__TradeNotInFillableState(element) => element.encode(),
                BookErrors::Book__ZeroAmount(element) => element.encode(),
                BookErrors::Book__ZeroRegistry(element) => element.encode(),
                BookErrors::Book__ZeroSafeBlockThreshold(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BookErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookErrors::Book__AmountOutTooLow(element) => element.fmt(f),
                BookErrors::Book__DisputePeriodNotOver(element) => element.fmt(f),
                BookErrors::Book__FeePctTooHigh(element) => element.fmt(f),
                BookErrors::Book__InvalidParamsCombination(element) => element.fmt(f),
                BookErrors::Book__InvalidToken(element) => element.fmt(f),
                BookErrors::Book__InvalidValue(element) => element.fmt(f),
                BookErrors::Book__MaliciousCaller(element) => element.fmt(f),
                BookErrors::Book__NotWeth(element) => element.fmt(f),
                BookErrors::Book__SameToken(element) => element.fmt(f),
                BookErrors::Book__SentToBlackHole(element) => element.fmt(f),
                BookErrors::Book__TradeNotCancelable(element) => element.fmt(f),
                BookErrors::Book__TradeNotDisputable(element) => element.fmt(f),
                BookErrors::Book__TradeNotFilled(element) => element.fmt(f),
                BookErrors::Book__TradeNotInFillableState(element) => element.fmt(f),
                BookErrors::Book__ZeroAmount(element) => element.fmt(f),
                BookErrors::Book__ZeroRegistry(element) => element.fmt(f),
                BookErrors::Book__ZeroSafeBlockThreshold(element) => element.fmt(f),
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
    impl ::std::convert::From<Book__FeePctTooHigh> for BookErrors {
        fn from(var: Book__FeePctTooHigh) -> Self {
            BookErrors::Book__FeePctTooHigh(var)
        }
    }
    impl ::std::convert::From<Book__InvalidParamsCombination> for BookErrors {
        fn from(var: Book__InvalidParamsCombination) -> Self {
            BookErrors::Book__InvalidParamsCombination(var)
        }
    }
    impl ::std::convert::From<Book__InvalidToken> for BookErrors {
        fn from(var: Book__InvalidToken) -> Self {
            BookErrors::Book__InvalidToken(var)
        }
    }
    impl ::std::convert::From<Book__InvalidValue> for BookErrors {
        fn from(var: Book__InvalidValue) -> Self {
            BookErrors::Book__InvalidValue(var)
        }
    }
    impl ::std::convert::From<Book__MaliciousCaller> for BookErrors {
        fn from(var: Book__MaliciousCaller) -> Self {
            BookErrors::Book__MaliciousCaller(var)
        }
    }
    impl ::std::convert::From<Book__NotWeth> for BookErrors {
        fn from(var: Book__NotWeth) -> Self {
            BookErrors::Book__NotWeth(var)
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
    impl ::std::convert::From<Book__TradeNotDisputable> for BookErrors {
        fn from(var: Book__TradeNotDisputable) -> Self {
            BookErrors::Book__TradeNotDisputable(var)
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
    impl ::std::convert::From<Book__ZeroRegistry> for BookErrors {
        fn from(var: Book__ZeroRegistry) -> Self {
            BookErrors::Book__ZeroRegistry(var)
        }
    }
    impl ::std::convert::From<Book__ZeroSafeBlockThreshold> for BookErrors {
        fn from(var: Book__ZeroSafeBlockThreshold) -> Self {
            BookErrors::Book__ZeroSafeBlockThreshold(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "FeePctSet", abi = "FeePctSet(uint256)")]
    pub struct FeePctSetFilter {
        pub fee_pct: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "ParamsCombinationSet",
        abi = "ParamsCombinationSet(uint256,uint256,uint256)"
    )]
    pub struct ParamsCombinationSetFilter {
        pub dispute_bond_pct: ::ethers::core::types::U256,
        pub trade_rebate_pct: ::ethers::core::types::U256,
        pub relayer_refund_pct: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "SafeBlockThresholdSet", abi = "SafeBlockThresholdSet(uint256)")]
    pub struct SafeBlockThresholdSetFilter {
        pub new_safe_block_threshold: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "TradeCancelled", abi = "TradeCancelled(uint256,bytes32,address)")]
    pub struct TradeCancelledFilter {
        #[ethevent(indexed)]
        pub trade_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trade_id: [u8; 32],
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "TradeDisputeSettled",
        abi = "TradeDisputeSettled(address,uint256,bytes32,bool,address)"
    )]
    pub struct TradeDisputeSettledFilter {
        pub relayer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub answer: bool,
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "TradeDisputed",
        abi = "TradeDisputed(address,uint256,bytes32,uint256,address)"
    )]
    pub struct TradeDisputedFilter {
        pub relayer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub filled_at_block: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "TradeFilled",
        abi = "TradeFilled(address,uint256,uint256,address)"
    )]
    pub struct TradeFilledFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "TradeRequested",
        abi = "TradeRequested(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct TradeRequestedFilter {
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub min_amount_out: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "TradeSettled",
        abi = "TradeSettled(address,uint256,uint256,address)"
    )]
    pub struct TradeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ::ethers::core::types::U256,
        pub filled_at_block: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum BookEvents {
        FeePctSetFilter(FeePctSetFilter),
        ParamsCombinationSetFilter(ParamsCombinationSetFilter),
        SafeBlockThresholdSetFilter(SafeBlockThresholdSetFilter),
        TradeCancelledFilter(TradeCancelledFilter),
        TradeDisputeSettledFilter(TradeDisputeSettledFilter),
        TradeDisputedFilter(TradeDisputedFilter),
        TradeFilledFilter(TradeFilledFilter),
        TradeRequestedFilter(TradeRequestedFilter),
        TradeSettledFilter(TradeSettledFilter),
    }
    impl ::ethers::contract::EthLogDecode for BookEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FeePctSetFilter::decode_log(log) {
                return Ok(BookEvents::FeePctSetFilter(decoded));
            }
            if let Ok(decoded) = ParamsCombinationSetFilter::decode_log(log) {
                return Ok(BookEvents::ParamsCombinationSetFilter(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BookEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookEvents::FeePctSetFilter(element) => element.fmt(f),
                BookEvents::ParamsCombinationSetFilter(element) => element.fmt(f),
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
    ///Container type for all input parameters for the `cancelTrade` function with signature `cancelTrade(address,address,uint256,uint256,address,uint256)` and selector `0xaa311ab4`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "cancelTrade",
        abi = "cancelTrade(address,address,uint256,uint256,address,uint256)"
    )]
    pub struct CancelTradeCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub min_amount_out: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub trade_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `disputeBondPct` function with signature `disputeBondPct()` and selector `0x391fe4e2`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "disputeBondPct", abi = "disputeBondPct()")]
    pub struct DisputeBondPctCall;
    ///Container type for all input parameters for the `disputeTrade` function with signature `disputeTrade(address,address,uint256,uint256,address,uint256,address)` and selector `0x09796dff`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "disputeTrade",
        abi = "disputeTrade(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct DisputeTradeCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub min_amount_out: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub trade_index: ::ethers::core::types::U256,
        pub trader: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `feePct` function with signature `feePct()` and selector `0xa02cf937`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "feePct", abi = "feePct()")]
    pub struct FeePctCall;
    ///Container type for all input parameters for the `fillTrade` function with signature `fillTrade(address,address,uint256,uint256,address,uint256,address,uint256,bytes)` and selector `0x6b16e8e6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "fillTrade",
        abi = "fillTrade(address,address,uint256,uint256,address,uint256,address,uint256,bytes)"
    )]
    pub struct FillTradeCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub min_amount_out: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub trade_index: ::ethers::core::types::U256,
        pub trader: ::ethers::core::types::Address,
        pub amount_to_send: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `numberOfTrades` function with signature `numberOfTrades()` and selector `0xcd805d5e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "numberOfTrades", abi = "numberOfTrades()")]
    pub struct NumberOfTradesCall;
    ///Container type for all input parameters for the `onPriceSettled` function with signature `onPriceSettled(bytes32,(address,address,address,address,uint256,uint8,bool,bytes))` and selector `0x734d1627`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "onPriceSettled",
        abi = "onPriceSettled(bytes32,(address,address,address,address,uint256,uint8,bool,bytes))"
    )]
    pub struct OnPriceSettledCall {
        pub id: [u8; 32],
        pub request: Request,
    }
    ///Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    ///Container type for all input parameters for the `registry` function with signature `registry()` and selector `0x7b103999`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
    ///Container type for all input parameters for the `relayerRefundPct` function with signature `relayerRefundPct()` and selector `0x53906a59`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "relayerRefundPct", abi = "relayerRefundPct()")]
    pub struct RelayerRefundPctCall;
    ///Container type for all input parameters for the `requestTrade` function with signature `requestTrade(address,address,uint256,uint256,address,bool)` and selector `0xab6552e5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "requestTrade",
        abi = "requestTrade(address,address,uint256,uint256,address,bool)"
    )]
    pub struct RequestTradeCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub min_amount_out: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub receive_eth: bool,
    }
    ///Container type for all input parameters for the `safeBlockThreshold` function with signature `safeBlockThreshold()` and selector `0x0ff0c00e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "safeBlockThreshold", abi = "safeBlockThreshold()")]
    pub struct SafeBlockThresholdCall;
    ///Container type for all input parameters for the `settleTrade` function with signature `settleTrade(address,address,uint256,uint256,address,uint256,address)` and selector `0xe66a34ee`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "settleTrade",
        abi = "settleTrade(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct SettleTradeCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub min_amount_out: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub trade_index: ::ethers::core::types::U256,
        pub trader: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `tradeRebatePct` function with signature `tradeRebatePct()` and selector `0xc16402bb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "tradeRebatePct", abi = "tradeRebatePct()")]
    pub struct TradeRebatePctCall;
    ///Container type for all input parameters for the `tradesData` function with signature `tradesData(bytes32)` and selector `0x75ef5c99`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "tradesData", abi = "tradesData(bytes32)")]
    pub struct TradesDataCall(pub [u8; 32]);
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum BookCalls {
        CancelTrade(CancelTradeCall),
        DisputeBondPct(DisputeBondPctCall),
        DisputeTrade(DisputeTradeCall),
        FeePct(FeePctCall),
        FillTrade(FillTradeCall),
        NumberOfTrades(NumberOfTradesCall),
        OnPriceSettled(OnPriceSettledCall),
        Oracle(OracleCall),
        Registry(RegistryCall),
        RelayerRefundPct(RelayerRefundPctCall),
        RequestTrade(RequestTradeCall),
        SafeBlockThreshold(SafeBlockThresholdCall),
        SettleTrade(SettleTradeCall),
        TradeRebatePct(TradeRebatePctCall),
        TradesData(TradesDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for BookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <CancelTradeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::CancelTrade(decoded));
            }
            if let Ok(decoded)
                = <DisputeBondPctCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::DisputeBondPct(decoded));
            }
            if let Ok(decoded)
                = <DisputeTradeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::DisputeTrade(decoded));
            }
            if let Ok(decoded)
                = <FeePctCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BookCalls::FeePct(decoded));
            }
            if let Ok(decoded)
                = <FillTradeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::FillTrade(decoded));
            }
            if let Ok(decoded)
                = <NumberOfTradesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::NumberOfTrades(decoded));
            }
            if let Ok(decoded)
                = <OnPriceSettledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::OnPriceSettled(decoded));
            }
            if let Ok(decoded)
                = <OracleCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BookCalls::Oracle(decoded));
            }
            if let Ok(decoded)
                = <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::Registry(decoded));
            }
            if let Ok(decoded)
                = <RelayerRefundPctCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::RelayerRefundPct(decoded));
            }
            if let Ok(decoded)
                = <RequestTradeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::RequestTrade(decoded));
            }
            if let Ok(decoded)
                = <SafeBlockThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::SafeBlockThreshold(decoded));
            }
            if let Ok(decoded)
                = <SettleTradeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::SettleTrade(decoded));
            }
            if let Ok(decoded)
                = <TradeRebatePctCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::TradeRebatePct(decoded));
            }
            if let Ok(decoded)
                = <TradesDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(BookCalls::TradesData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BookCalls::CancelTrade(element) => element.encode(),
                BookCalls::DisputeBondPct(element) => element.encode(),
                BookCalls::DisputeTrade(element) => element.encode(),
                BookCalls::FeePct(element) => element.encode(),
                BookCalls::FillTrade(element) => element.encode(),
                BookCalls::NumberOfTrades(element) => element.encode(),
                BookCalls::OnPriceSettled(element) => element.encode(),
                BookCalls::Oracle(element) => element.encode(),
                BookCalls::Registry(element) => element.encode(),
                BookCalls::RelayerRefundPct(element) => element.encode(),
                BookCalls::RequestTrade(element) => element.encode(),
                BookCalls::SafeBlockThreshold(element) => element.encode(),
                BookCalls::SettleTrade(element) => element.encode(),
                BookCalls::TradeRebatePct(element) => element.encode(),
                BookCalls::TradesData(element) => element.encode(),
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
                BookCalls::NumberOfTrades(element) => element.fmt(f),
                BookCalls::OnPriceSettled(element) => element.fmt(f),
                BookCalls::Oracle(element) => element.fmt(f),
                BookCalls::Registry(element) => element.fmt(f),
                BookCalls::RelayerRefundPct(element) => element.fmt(f),
                BookCalls::RequestTrade(element) => element.fmt(f),
                BookCalls::SafeBlockThreshold(element) => element.fmt(f),
                BookCalls::SettleTrade(element) => element.fmt(f),
                BookCalls::TradeRebatePct(element) => element.fmt(f),
                BookCalls::TradesData(element) => element.fmt(f),
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
    impl ::std::convert::From<SettleTradeCall> for BookCalls {
        fn from(var: SettleTradeCall) -> Self {
            BookCalls::SettleTrade(var)
        }
    }
    impl ::std::convert::From<TradeRebatePctCall> for BookCalls {
        fn from(var: TradeRebatePctCall) -> Self {
            BookCalls::TradeRebatePct(var)
        }
    }
    impl ::std::convert::From<TradesDataCall> for BookCalls {
        fn from(var: TradesDataCall) -> Self {
            BookCalls::TradesData(var)
        }
    }
    ///Container type for all return fields from the `disputeBondPct` function with signature `disputeBondPct()` and selector `0x391fe4e2`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct DisputeBondPctReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feePct` function with signature `feePct()` and selector `0xa02cf937`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct FeePctReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `numberOfTrades` function with signature `numberOfTrades()` and selector `0xcd805d5e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct NumberOfTradesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct OracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `registry` function with signature `registry()` and selector `0x7b103999`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `relayerRefundPct` function with signature `relayerRefundPct()` and selector `0x53906a59`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct RelayerRefundPctReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `safeBlockThreshold` function with signature `safeBlockThreshold()` and selector `0x0ff0c00e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct SafeBlockThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tradeRebatePct` function with signature `tradeRebatePct()` and selector `0xc16402bb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TradeRebatePctReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tradesData` function with signature `tradesData(bytes32)` and selector `0x75ef5c99`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TradesDataReturn {
        pub filled_at_block: ::ethers::core::types::U256,
        pub filled_by: ::ethers::core::types::Address,
        pub status: u8,
        pub unwrap_output: bool,
        pub is_eth_trade: bool,
        pub amount_paid: ::ethers::core::types::U256,
    }
    ///`Request(address,address,address,address,uint256,uint8,bool,bytes)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct Request {
        pub requester: ::ethers::core::types::Address,
        pub proposer: ::ethers::core::types::Address,
        pub disputer: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub bond: ::ethers::core::types::U256,
        pub state: u8,
        pub answer: bool,
        pub data: ::ethers::core::types::Bytes,
    }
}
