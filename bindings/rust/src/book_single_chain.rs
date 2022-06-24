pub use booksinglechain_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod booksinglechain_mod {
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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_safeBlockThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracleAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blocksLeft\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__DisputePeriodNotOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__DisputePeriodOver\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__FeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidSignature\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__NewFeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__SameToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__SentToBlackHole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__TradeAlreadyFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__TradeNotFilled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__ZeroAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newMaxFeePct\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MaxFeePctChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UpdatedFeeForTrade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"tradeIndex\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputeTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"tradeIndex\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"tradeIndex\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"traderSignature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTradeWithUpdatedFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAtBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledBy\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxFeePct\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numberOfTrades\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract IOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requestTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"safeBlockThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newMaxFeePct\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMaxFeePct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSafeBlockThreshold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"tradeIndex\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"traderSignature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFeeForTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BOOKSINGLECHAIN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a0604052600180546001600160801b031916905534801561002057600080fd5b5060405161194a38038061194a83398101604081905261003f9161011c565b600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a3506001600160a01b03811660805260028290556040518281527fcf29a5174acb8c175d760a7381ffc52c6ae644e3a4ba3fa7e01344f959cd76159060200160405180910390a1600380546001600160801b0319166703782dace9d900009081179091556040519081527f248707dbfdce07115740191c2a4ec04702f5d55967ca82b9dc65c55801531f7f9060200160405180910390a15050610159565b6000806040838503121561012f57600080fd5b825160208401519092506001600160a01b038116811461014e57600080fd5b809150509250929050565b6080516117c8610182600039600081816101c801528181610b020152610bb801526117c86000f3fe608060405234801561001057600080fd5b50600436106101215760003560e01c80638da5cb5b116100ad578063d70e3dfd11610071578063d70e3dfd14610299578063d8642295146102c2578063daf9c210146102d5578063fbc534ef14610308578063fc711c3a1461031b57600080fd5b80638da5cb5b146102155780639170c05b146102285780639362776f1461023b5780639501325f1461024e578063cd805d5e1461026e57600080fd5b80632613f307116100f45780632613f3071461018a578063436146791461019d5780636f1335b9146101b05780637dc0d1d0146101c357806382b2ec3e1461020257600080fd5b80630b20b7bc146101265780630ff0c00e146101595780630ffb1d8b1461016257806313af403514610177575b600080fd5b61014661013436600461134c565b60076020526000908152604090205481565b6040519081526020015b60405180910390f35b61014660025481565b61017561017036600461138f565b61032e565b005b6101756101853660046113c6565b6103c0565b61017561019836600461134c565b610435565b6101756101ab3660046113ff565b61049b565b6101756101be366004611465565b6105fe565b6101ea7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610150565b610175610210366004611480565b6106b0565b6000546101ea906001600160a01b031681565b6101756102363660046114f1565b610747565b61017561024936600461158a565b610932565b61014661025c36600461134c565b60056020526000908152604090205481565b600154610281906001600160801b031681565b6040516001600160801b039091168152602001610150565b6101ea6102a736600461134c565b6006602052600090815260409020546001600160a01b031681565b6101756102d03660046115f2565b6109e9565b6102f86102e33660046113c6565b60046020526000908152604090205460ff1681565b6040519015158152602001610150565b6101756103163660046113ff565b610a60565b600354610281906001600160801b031681565b6000546001600160a01b031633146103615760405162461bcd60e51b8152600401610358906116b5565b60405180910390fd5b6001600160a01b038216600081815260046020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d92910160405180910390a25050565b6000546001600160a01b031633146103ea5760405162461bcd60e51b8152600401610358906116b5565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b6000546001600160a01b0316331461045f5760405162461bcd60e51b8152600401610358906116b5565b60028190556040518181527fcf29a5174acb8c175d760a7381ffc52c6ae644e3a4ba3fa7e01344f959cd7615906020015b60405180910390a150565b60006104ab878787878787610ca5565b600081815260056020526040812054919250036104de5760405163cc4f06a160e01b815260048101829052602401610358565b6002546000828152600560205260409020546104fa90436116f1565b10156105455760008181526005602052604081205461051990436116f1565b60025461052691906116f1565b9050806040516325797e0360e11b815260040161035891815260200190565b600081815260076020818152604080842080546006845282862080546005865293872087905580546001600160a01b031916905593909252929055906001600160a01b039081169061059a9089168684610d09565b6105ae6001600160a01b038a168289610d09565b8183826001600160a01b03167ff74daea4b4b80ee368b87f2feb712805f4ef30acd087e35c5d16df8736c2d6f2896040516105eb91815260200190565b60405180910390a4505050505050505050565b6000546001600160a01b031633146106285760405162461bcd60e51b8152600401610358906116b5565b670de0b6b3a7640000816001600160801b03161061065957604051636a143fdd60e11b815260040160405180910390fd5b600380546fffffffffffffffffffffffffffffffff19166001600160801b0383169081179091556040519081527f248707dbfdce07115740191c2a4ec04702f5d55967ca82b9dc65c55801531f7f90602001610490565b60006106c0888888888888610ca5565b600081815260056020526040902054909150156106f3576040516304daa62560e21b815260048101829052602401610358565b6106fe878284610d87565b60408051868152602081018490524391839133917f9e2cd6515276369145ad3c2ee5eb7c8f09ca9c99e8e7d6e68de40895a72f3464910160405180910390a45050505050505050565b6001600160a01b03851660009081526004602052604090205460ff1661078b5760405163f602627d60e01b81526001600160a01b0386166004820152602401610358565b6001600160a01b03841660009081526004602052604090205460ff166107cf5760405163f602627d60e01b81526001600160a01b0385166004820152602401610358565b836001600160a01b0316856001600160a01b03160361080157604051631d2792fb60e31b815260040160405180910390fd5b6003546001600160801b0316821115610830576040516305bf279d60e41b815260048101839052602401610358565b826000036108515760405163abc5ee6f60e01b815260040160405180910390fd5b6001600160a01b03811661087857604051631feef77d60e01b815260040160405180910390fd5b61088d6001600160a01b038616333086610ddb565b60015460408051858152602081018590526001600160a01b038481168284015291516001600160801b0390931692878316928916917f7361c265d28ece9d5df249995186533440e0b7a1310ae54d496fa1783056e3da919081900360600190a4600180546001600160801b031690600061090683611708565b91906101000a8154816001600160801b0302191690836001600160801b03160217905550505050505050565b6003546001600160801b0316831115610961576040516305bf279d60e41b815260048101849052602401610358565b60008481526005602052604090205415610991576040516304daa62560e21b815260048101859052602401610358565b61099e8585858585610e65565b83856001600160a01b03167f2932e14c54a9749927aed2cd6d014eb7cd6a224f5fe6dfb944d9ced9b04c4f47856040516109da91815260200190565b60405180910390a35050505050565b60006109f98c8c8c8c8c8c610ca5565b9050610a088582868686610e65565b610a138b8288610d87565b60408051858152602081018890524391839133917f9e2cd6515276369145ad3c2ee5eb7c8f09ca9c99e8e7d6e68de40895a72f3464910160405180910390a4505050505050505050505050565b6000610a70878787878787610ca5565b600081815260056020526040812054919250819003610aa55760405163cc4f06a160e01b815260048101839052602401610358565b600254610ab282436116f1565b10610ad057604051632c02744560e11b815260040160405180910390fd5b60008281526007602090815260408083205460069092529182902054915163095ea7b360e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811660048301526024820183905291928216918a169063095ea7b3906044016020604051808303816000875af1158015610b5f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b83919061172e565b50604051632bd6267f60e21b81526001600160a01b0382811660048301523360248301528a81166044830152606482018490527f0000000000000000000000000000000000000000000000000000000000000000169063af5899fc906084016020604051808303816000875af1158015610c01573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c25919061174b565b5060008481526005602090815260408083208390556006825280832080546001600160a01b031916905560078252808320929092559051888152839186916001600160a01b038516917fbfb0e6d8f5102fdad9ead2713eec1014861fe90e38a91786fb5c610873c38ad8910160405180910390a450505050505050505050565b604080516001600160a01b039788166020808301919091529688168183015260608101959095526080850193909352941660a08301526001600160801b0390931660c0808301919091528351808303909101815260e0909101909252815191012090565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080610d815760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610358565b50505050565b60008281526005602090815260408083204390556006825280832080546001600160a01b031916339081179091556007909252909120829055610dd6906001600160a01b038516903084610ddb565b505050565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080610e5e5760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610358565b5050505050565b6003546001600160801b0316831115610e94576040516305bf279d60e41b815260048101849052602401610358565b60008481526005602052604090205415610ec4576040516304daa62560e21b815260048101859052602401610358565b604080517f0efb9dda140a951df4393d44ca40349032d31811466afd20eacd4b4136c3f4986020808301919091528183018790526060808301879052835180840390910181526080830184528051908201207f19457468657265756d205369676e6564204d6573736167653a0a33320000000060a084015260bc8084018290528451808503909101815260dc90930190935281519101206000610f9d8286868080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250610fdb92505050565b9050876001600160a01b0316816001600160a01b031614610fd1576040516324a0dbd760e21b815260040160405180910390fd5b5050505050505050565b6000806000610fea8585610fff565b91509150610ff78161106d565b509392505050565b60008082516041036110355760208301516040840151606085015160001a61102987828585611226565b94509450505050611066565b825160400361105e5760208301516040840151611053868383611313565b935093505050611066565b506000905060025b9250929050565b600081600481111561108157611081611764565b036110895750565b600181600481111561109d5761109d611764565b036110ea5760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610358565b60028160048111156110fe576110fe611764565b0361114b5760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610358565b600381600481111561115f5761115f611764565b036111b75760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610358565b60048160048111156111cb576111cb611764565b036112235760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b6064820152608401610358565b50565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561125d575060009050600361130a565b8460ff16601b1415801561127557508460ff16601c14155b15611286575060009050600461130a565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156112da573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166113035760006001925092505061130a565b9150600090505b94509492505050565b6000806001600160ff1b0383168161133060ff86901c601b61177a565b905061133e87828885611226565b935093505050935093915050565b60006020828403121561135e57600080fd5b5035919050565b80356001600160a01b038116811461137c57600080fd5b919050565b801515811461122357600080fd5b600080604083850312156113a257600080fd5b6113ab83611365565b915060208301356113bb81611381565b809150509250929050565b6000602082840312156113d857600080fd5b6113e182611365565b9392505050565b80356001600160801b038116811461137c57600080fd5b60008060008060008060c0878903121561141857600080fd5b61142187611365565b955061142f60208801611365565b9450604087013593506060870135925061144b60808801611365565b915061145960a088016113e8565b90509295509295509295565b60006020828403121561147757600080fd5b6113e1826113e8565b600080600080600080600060e0888a03121561149b57600080fd5b6114a488611365565b96506114b260208901611365565b955060408801359450606088013593506114ce60808901611365565b92506114dc60a089016113e8565b915060c0880135905092959891949750929550565b600080600080600060a0868803121561150957600080fd5b61151286611365565b945061152060208701611365565b9350604086013592506060860135915061153c60808701611365565b90509295509295909350565b60008083601f84011261155a57600080fd5b50813567ffffffffffffffff81111561157257600080fd5b60208301915083602082850101111561106657600080fd5b6000806000806000608086880312156115a257600080fd5b6115ab86611365565b94506020860135935060408601359250606086013567ffffffffffffffff8111156115d557600080fd5b6115e188828901611548565b969995985093965092949392505050565b60008060008060008060008060008060006101408c8e03121561161457600080fd5b61161d8c611365565b9a5061162b60208d01611365565b995060408c0135985060608c0135975061164760808d01611365565b965061165560a08d016113e8565b955060c08c0135945061166a60e08d01611365565b93506101008c013592506101208c013567ffffffffffffffff81111561168f57600080fd5b61169b8e828f01611548565b915080935050809150509295989b509295989b9093969950565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b634e487b7160e01b600052601160045260246000fd5b600082821015611703576117036116db565b500390565b60006001600160801b03808316818103611724576117246116db565b6001019392505050565b60006020828403121561174057600080fd5b81516113e181611381565b60006020828403121561175d57600080fd5b5051919050565b634e487b7160e01b600052602160045260246000fd5b6000821982111561178d5761178d6116db565b50019056fea2646970667358221220b144609d5e94d9780a67f8606701da17b69128c8d7de725234b2a7005a9c983e64736f6c634300080d0033" . parse () . expect ("invalid bytecode")
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
        ) -> Result<
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
        #[doc = "Calls the contract's `disputeTrade` (0xfbc534ef) function"]
        pub fn dispute_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            to: ethers::core::types::Address,
            trade_index: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [251, 197, 52, 239],
                    (token_in, token_out, amount_in, fee_pct, to, trade_index),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTrade` (0x82b2ec3e) function"]
        pub fn fill_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            to: ethers::core::types::Address,
            trade_index: u128,
            amount_to_send: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [130, 178, 236, 62],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        fee_pct,
                        to,
                        trade_index,
                        amount_to_send,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTradeWithUpdatedFee` (0xd8642295) function"]
        pub fn fill_trade_with_updated_fee(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            to: ethers::core::types::Address,
            trade_index: u128,
            amount_to_send: ethers::core::types::U256,
            trader: ethers::core::types::Address,
            new_fee_pct: ethers::core::types::U256,
            trader_signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [216, 100, 34, 149],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        fee_pct,
                        to,
                        trade_index,
                        amount_to_send,
                        trader,
                        new_fee_pct,
                        trader_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledAmount` (0x0b20b7bc) function"]
        pub fn filled_amount(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([11, 32, 183, 188], p0)
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
        #[doc = "Calls the contract's `maxFeePct` (0xfc711c3a) function"]
        pub fn max_fee_pct(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([252, 113, 28, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numberOfTrades` (0xcd805d5e) function"]
        pub fn number_of_trades(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([205, 128, 93, 94], ())
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
        #[doc = "Calls the contract's `requestTrade` (0x9170c05b) function"]
        pub fn request_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [145, 112, 192, 91],
                    (token_in, token_out, amount_in, fee_pct, to),
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
        #[doc = "Calls the contract's `setMaxFeePct` (0x6f1335b9) function"]
        pub fn set_max_fee_pct(
            &self,
            new_max_fee_pct: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 19, 53, 185], new_max_fee_pct)
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
        #[doc = "Calls the contract's `setSafeBlockThreshold` (0x2613f307) function"]
        pub fn set_safe_block_threshold(
            &self,
            new_safe_block_threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 19, 243, 7], new_safe_block_threshold)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settleTrade` (0x43614679) function"]
        pub fn settle_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            to: ethers::core::types::Address,
            trade_index: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [67, 97, 70, 121],
                    (token_in, token_out, amount_in, fee_pct, to, trade_index),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFeeForTrade` (0x9362776f) function"]
        pub fn update_fee_for_trade(
            &self,
            trader: ethers::core::types::Address,
            trade_id: [u8; 32],
            new_fee_pct: ethers::core::types::U256,
            trader_signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [147, 98, 119, 111],
                    (trader, trade_id, new_fee_pct, trader_signature),
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
        #[doc = "Gets the contract's `MaxFeePctChanged` event"]
        pub fn max_fee_pct_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MaxFeePctChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerUpdated` event"]
        pub fn owner_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SafeBlockThresholdChanged` event"]
        pub fn safe_block_threshold_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SafeBlockThresholdChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenWhitelisted` event"]
        pub fn token_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
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
    #[ethevent(name = "MaxFeePctChanged", abi = "MaxFeePctChanged(uint128)")]
    pub struct MaxFeePctChangedFilter {
        pub new_max_fee_pct: u128,
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
    #[ethevent(
        name = "SafeBlockThresholdChanged",
        abi = "SafeBlockThresholdChanged(uint256)"
    )]
    pub struct SafeBlockThresholdChangedFilter {
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
        name = "TradeDisputed",
        abi = "TradeDisputed(address,bytes32,uint256,uint256)"
    )]
    pub struct TradeDisputedFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_id: [u8; 32],
        #[ethevent(indexed)]
        pub filled_amount: ethers::core::types::U256,
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
    #[ethevent(
        name = "TradeFilled",
        abi = "TradeFilled(address,bytes32,uint256,uint256,uint256)"
    )]
    pub struct TradeFilledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_id: [u8; 32],
        #[ethevent(indexed)]
        pub filled_at_block: ethers::core::types::U256,
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
        abi = "TradeRequested(address,address,uint256,uint256,address,uint256)"
    )]
    pub struct TradeRequestedFilter {
        #[ethevent(indexed)]
        pub token_in: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
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
    #[ethevent(
        name = "TradeSettled",
        abi = "TradeSettled(address,bytes32,uint256,uint256)"
    )]
    pub struct TradeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_id: [u8; 32],
        #[ethevent(indexed)]
        pub filled_amount: ethers::core::types::U256,
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
    #[ethevent(
        name = "UpdatedFeeForTrade",
        abi = "UpdatedFeeForTrade(address,bytes32,uint256)"
    )]
    pub struct UpdatedFeeForTradeFilter {
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_id: [u8; 32],
        pub new_fee_pct: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BookSingleChainEvents {
        MaxFeePctChangedFilter(MaxFeePctChangedFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        SafeBlockThresholdChangedFilter(SafeBlockThresholdChangedFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
        TradeDisputedFilter(TradeDisputedFilter),
        TradeFilledFilter(TradeFilledFilter),
        TradeRequestedFilter(TradeRequestedFilter),
        TradeSettledFilter(TradeSettledFilter),
        UpdatedFeeForTradeFilter(UpdatedFeeForTradeFilter),
    }
    impl ethers::contract::EthLogDecode for BookSingleChainEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MaxFeePctChangedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::MaxFeePctChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SafeBlockThresholdChangedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::SafeBlockThresholdChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TokenWhitelistedFilter(decoded));
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
                BookSingleChainEvents::MaxFeePctChangedFilter(element) => element.fmt(f),
                BookSingleChainEvents::OwnerUpdatedFilter(element) => element.fmt(f),
                BookSingleChainEvents::SafeBlockThresholdChangedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TokenWhitelistedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeDisputedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeFilledFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeRequestedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeSettledFilter(element) => element.fmt(f),
                BookSingleChainEvents::UpdatedFeeForTradeFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `disputeTrade`function with signature `disputeTrade(address,address,uint256,uint256,address,uint128)` and selector `[251, 197, 52, 239]`"]
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
        abi = "disputeTrade(address,address,uint256,uint256,address,uint128)"
    )]
    pub struct DisputeTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub trade_index: u128,
    }
    #[doc = "Container type for all input parameters for the `fillTrade`function with signature `fillTrade(address,address,uint256,uint256,address,uint128,uint256)` and selector `[130, 178, 236, 62]`"]
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
        abi = "fillTrade(address,address,uint256,uint256,address,uint128,uint256)"
    )]
    pub struct FillTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub trade_index: u128,
        pub amount_to_send: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fillTradeWithUpdatedFee`function with signature `fillTradeWithUpdatedFee(address,address,uint256,uint256,address,uint128,uint256,address,uint256,bytes)` and selector `[216, 100, 34, 149]`"]
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
        abi = "fillTradeWithUpdatedFee(address,address,uint256,uint256,address,uint128,uint256,address,uint256,bytes)"
    )]
    pub struct FillTradeWithUpdatedFeeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub trade_index: u128,
        pub amount_to_send: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
        pub new_fee_pct: ethers::core::types::U256,
        pub trader_signature: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `filledAmount`function with signature `filledAmount(bytes32)` and selector `[11, 32, 183, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "filledAmount", abi = "filledAmount(bytes32)")]
    pub struct FilledAmountCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `filledAtBlock`function with signature `filledAtBlock(bytes32)` and selector `[149, 1, 50, 95]`"]
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
    #[doc = "Container type for all input parameters for the `filledBy`function with signature `filledBy(bytes32)` and selector `[215, 14, 61, 253]`"]
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
    #[doc = "Container type for all input parameters for the `maxFeePct`function with signature `maxFeePct()` and selector `[252, 113, 28, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxFeePct", abi = "maxFeePct()")]
    pub struct MaxFeePctCall;
    #[doc = "Container type for all input parameters for the `numberOfTrades`function with signature `numberOfTrades()` and selector `[205, 128, 93, 94]`"]
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
    #[doc = "Container type for all input parameters for the `oracle`function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
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
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
    #[doc = "Container type for all input parameters for the `requestTrade`function with signature `requestTrade(address,address,uint256,uint256,address)` and selector `[145, 112, 192, 91]`"]
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
        pub fee_pct: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `safeBlockThreshold`function with signature `safeBlockThreshold()` and selector `[15, 240, 192, 14]`"]
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
    #[doc = "Container type for all input parameters for the `setMaxFeePct`function with signature `setMaxFeePct(uint128)` and selector `[111, 19, 53, 185]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMaxFeePct", abi = "setMaxFeePct(uint128)")]
    pub struct SetMaxFeePctCall {
        pub new_max_fee_pct: u128,
    }
    #[doc = "Container type for all input parameters for the `setOwner`function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
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
    #[doc = "Container type for all input parameters for the `setSafeBlockThreshold`function with signature `setSafeBlockThreshold(uint256)` and selector `[38, 19, 243, 7]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSafeBlockThreshold", abi = "setSafeBlockThreshold(uint256)")]
    pub struct SetSafeBlockThresholdCall {
        pub new_safe_block_threshold: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `settleTrade`function with signature `settleTrade(address,address,uint256,uint256,address,uint128)` and selector `[67, 97, 70, 121]`"]
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
        abi = "settleTrade(address,address,uint256,uint256,address,uint128)"
    )]
    pub struct SettleTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub trade_index: u128,
    }
    #[doc = "Container type for all input parameters for the `updateFeeForTrade`function with signature `updateFeeForTrade(address,bytes32,uint256,bytes)` and selector `[147, 98, 119, 111]`"]
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
        abi = "updateFeeForTrade(address,bytes32,uint256,bytes)"
    )]
    pub struct UpdateFeeForTradeCall {
        pub trader: ethers::core::types::Address,
        pub trade_id: [u8; 32],
        pub new_fee_pct: ethers::core::types::U256,
        pub trader_signature: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `whitelistToken`function with signature `whitelistToken(address,bool)` and selector `[15, 251, 29, 139]`"]
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
    #[doc = "Container type for all input parameters for the `whitelistedTokens`function with signature `whitelistedTokens(address)` and selector `[218, 249, 194, 16]`"]
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
        DisputeTrade(DisputeTradeCall),
        FillTrade(FillTradeCall),
        FillTradeWithUpdatedFee(FillTradeWithUpdatedFeeCall),
        FilledAmount(FilledAmountCall),
        FilledAtBlock(FilledAtBlockCall),
        FilledBy(FilledByCall),
        MaxFeePct(MaxFeePctCall),
        NumberOfTrades(NumberOfTradesCall),
        Oracle(OracleCall),
        Owner(OwnerCall),
        RequestTrade(RequestTradeCall),
        SafeBlockThreshold(SafeBlockThresholdCall),
        SetMaxFeePct(SetMaxFeePctCall),
        SetOwner(SetOwnerCall),
        SetSafeBlockThreshold(SetSafeBlockThresholdCall),
        SettleTrade(SettleTradeCall),
        UpdateFeeForTrade(UpdateFeeForTradeCall),
        WhitelistToken(WhitelistTokenCall),
        WhitelistedTokens(WhitelistedTokensCall),
    }
    impl ethers::core::abi::AbiDecode for BookSingleChainCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
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
                <FilledAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FilledAmount(decoded));
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
                <MaxFeePctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::MaxFeePct(decoded));
            }
            if let Ok(decoded) =
                <NumberOfTradesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::NumberOfTrades(decoded));
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
                <SetMaxFeePctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SetMaxFeePct(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SetOwner(decoded));
            }
            if let Ok(decoded) =
                <SetSafeBlockThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SetSafeBlockThreshold(decoded));
            }
            if let Ok(decoded) =
                <SettleTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SettleTrade(decoded));
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
                BookSingleChainCalls::DisputeTrade(element) => element.encode(),
                BookSingleChainCalls::FillTrade(element) => element.encode(),
                BookSingleChainCalls::FillTradeWithUpdatedFee(element) => element.encode(),
                BookSingleChainCalls::FilledAmount(element) => element.encode(),
                BookSingleChainCalls::FilledAtBlock(element) => element.encode(),
                BookSingleChainCalls::FilledBy(element) => element.encode(),
                BookSingleChainCalls::MaxFeePct(element) => element.encode(),
                BookSingleChainCalls::NumberOfTrades(element) => element.encode(),
                BookSingleChainCalls::Oracle(element) => element.encode(),
                BookSingleChainCalls::Owner(element) => element.encode(),
                BookSingleChainCalls::RequestTrade(element) => element.encode(),
                BookSingleChainCalls::SafeBlockThreshold(element) => element.encode(),
                BookSingleChainCalls::SetMaxFeePct(element) => element.encode(),
                BookSingleChainCalls::SetOwner(element) => element.encode(),
                BookSingleChainCalls::SetSafeBlockThreshold(element) => element.encode(),
                BookSingleChainCalls::SettleTrade(element) => element.encode(),
                BookSingleChainCalls::UpdateFeeForTrade(element) => element.encode(),
                BookSingleChainCalls::WhitelistToken(element) => element.encode(),
                BookSingleChainCalls::WhitelistedTokens(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BookSingleChainCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookSingleChainCalls::DisputeTrade(element) => element.fmt(f),
                BookSingleChainCalls::FillTrade(element) => element.fmt(f),
                BookSingleChainCalls::FillTradeWithUpdatedFee(element) => element.fmt(f),
                BookSingleChainCalls::FilledAmount(element) => element.fmt(f),
                BookSingleChainCalls::FilledAtBlock(element) => element.fmt(f),
                BookSingleChainCalls::FilledBy(element) => element.fmt(f),
                BookSingleChainCalls::MaxFeePct(element) => element.fmt(f),
                BookSingleChainCalls::NumberOfTrades(element) => element.fmt(f),
                BookSingleChainCalls::Oracle(element) => element.fmt(f),
                BookSingleChainCalls::Owner(element) => element.fmt(f),
                BookSingleChainCalls::RequestTrade(element) => element.fmt(f),
                BookSingleChainCalls::SafeBlockThreshold(element) => element.fmt(f),
                BookSingleChainCalls::SetMaxFeePct(element) => element.fmt(f),
                BookSingleChainCalls::SetOwner(element) => element.fmt(f),
                BookSingleChainCalls::SetSafeBlockThreshold(element) => element.fmt(f),
                BookSingleChainCalls::SettleTrade(element) => element.fmt(f),
                BookSingleChainCalls::UpdateFeeForTrade(element) => element.fmt(f),
                BookSingleChainCalls::WhitelistToken(element) => element.fmt(f),
                BookSingleChainCalls::WhitelistedTokens(element) => element.fmt(f),
            }
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
    impl ::std::convert::From<FilledAmountCall> for BookSingleChainCalls {
        fn from(var: FilledAmountCall) -> Self {
            BookSingleChainCalls::FilledAmount(var)
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
    impl ::std::convert::From<MaxFeePctCall> for BookSingleChainCalls {
        fn from(var: MaxFeePctCall) -> Self {
            BookSingleChainCalls::MaxFeePct(var)
        }
    }
    impl ::std::convert::From<NumberOfTradesCall> for BookSingleChainCalls {
        fn from(var: NumberOfTradesCall) -> Self {
            BookSingleChainCalls::NumberOfTrades(var)
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
    impl ::std::convert::From<SetMaxFeePctCall> for BookSingleChainCalls {
        fn from(var: SetMaxFeePctCall) -> Self {
            BookSingleChainCalls::SetMaxFeePct(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for BookSingleChainCalls {
        fn from(var: SetOwnerCall) -> Self {
            BookSingleChainCalls::SetOwner(var)
        }
    }
    impl ::std::convert::From<SetSafeBlockThresholdCall> for BookSingleChainCalls {
        fn from(var: SetSafeBlockThresholdCall) -> Self {
            BookSingleChainCalls::SetSafeBlockThreshold(var)
        }
    }
    impl ::std::convert::From<SettleTradeCall> for BookSingleChainCalls {
        fn from(var: SettleTradeCall) -> Self {
            BookSingleChainCalls::SettleTrade(var)
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
}
